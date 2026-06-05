//! ProcessBinaryData engine.
//!
//! Many maker-note sub-records are not IFDs but fixed-layout binary blobs: an
//! array of values where each *index position* is a named tag (ExifTool's
//! `ProcessBinaryData`). A table declares a default FORMAT and a FIRST_ENTRY;
//! the byte offset of tag index `N` is `N * sizeof(default FORMAT)`, and the
//! value is read with the tag's own format override (if any) or the default.

use crate::reader::{ByteOrder, Reader};
use crate::tag::ExtractedTag;
use crate::value::Value;

#[derive(Clone, Copy, Debug)]
pub enum Fmt {
    U8,
    S8,
    U16,
    S16,
    U32,
    S32,
    /// A fixed-length ASCII string of N bytes.
    Str(usize),
}

impl Fmt {
    pub fn size(self) -> usize {
        match self {
            Fmt::U8 | Fmt::S8 => 1,
            Fmt::U16 | Fmt::S16 => 2,
            Fmt::U32 | Fmt::S32 => 4,
            Fmt::Str(n) => n,
        }
    }
}

/// A PrintConv for a binary/IFD maker-note tag.
#[derive(Clone, Copy)]
pub enum Pc {
    None,
    Enum(&'static [(i64, &'static str)]),
    /// Like [`Pc::Enum`] but the source PrintConv had an `OTHER => sub{…}`
    /// fallback: unlisted values return `None` (so the vendor `special`
    /// converter can take over) instead of "Unknown (N)".
    EnumO(&'static [(i64, &'static str)]),
    /// String-keyed enumeration (e.g. Nikon's "AUTO" -> "Auto"). Also matches a
    /// numeric value against its integer-string key (Canon/Sony/Minolta lens
    /// tables use mixed integer and "N.M" variant keys).
    EnumStr(&'static [(&'static str, &'static str)]),
}

impl Pc {
    pub fn apply(&self, v: &Value) -> Option<String> {
        match self {
            Pc::None => None,
            Pc::Enum(table) => {
                let k = v.as_i64()?;
                match table.iter().find(|(kk, _)| *kk == k) {
                    Some((_, s)) => Some(s.to_string()),
                    // ExifTool prints "Unknown (N)" for an unlisted enum value.
                    None => Some(format!("Unknown ({k})")),
                }
            }
            Pc::EnumO(table) => {
                let k = v.as_i64()?;
                // Unlisted -> None (the OTHER fallback is handled by `special`).
                table.iter().find(|(kk, _)| *kk == k).map(|(_, s)| s.to_string())
            }
            Pc::EnumStr(table) => {
                let lookup = |k: &str| table.iter().find(|(kk, _)| *kk == k).map(|(_, s)| s.to_string());
                match v.as_str() {
                    Some(s) => lookup(s.trim()),
                    // Numeric value: match its integer-string key (lens tables).
                    None => lookup(&v.as_i64()?.to_string()),
                }
            }
        }
    }
}

/// An ExifTool `RawConv => '... undef ...'` n/a-suppression rule: a tag whose
/// raw value matches is dropped entirely (not just printed as "Unknown"). Only
/// the handful of patterns that actually occur are modelled.
#[derive(Clone, Copy)]
pub enum Skip {
    /// Never suppress.
    Never,
    /// Suppress when the value equals this sentinel (e.g. -1 or 0 = "n/a").
    Eq(i64),
    /// Suppress when the value is <= this bound (e.g. out-of-range exposures).
    Le(i64),
}

impl Skip {
    /// Whether `v` should be suppressed under this rule.
    pub fn suppresses(&self, v: &Value) -> bool {
        match self {
            Skip::Never => false,
            Skip::Eq(n) => v.as_i64() == Some(*n),
            Skip::Le(n) => v.as_i64().map(|x| x <= *n).unwrap_or(false),
        }
    }
}

/// How many elements a binary entry reads. Variable counts (ExifTool's
/// `int16s[$val{N}]`) both read N elements *and* shift every later entry's
/// position by the extra bytes consumed — the running `varSize` of
/// `ProcessBinaryData`. Fixed counts read N elements without shifting (matching
/// ExifTool's non-`var_` bracketed formats).
#[derive(Clone, Copy)]
pub enum Count {
    /// A fixed number of elements (1 for ordinary scalars).
    Fixed(usize),
    /// `$val{ix}` elements (e.g. one per AF point); shifts later positions.
    Var(i32),
    /// `int(($val{ix}+15)/16)` elements (a bitmask, one bit per point); shifts.
    VarBits(i32),
}

pub struct BinTag {
    pub index: i32,
    pub name: &'static str,
    pub fmt: Option<Fmt>,
    pub pc: Pc,
    pub skip: Skip,
    pub count: Count,
}

pub struct BinTable {
    pub default_fmt: Fmt,
    pub first_entry: i32,
    pub tags: &'static [BinTag],
}

/// A vendor-specific converter for tags whose PrintConv/ValueConv is a formula
/// rather than a simple enumeration (e.g. Canon's APEX aperture encoding).
/// Returns the print string, or None to fall back to enum/raw rendering.
pub type Special = fn(&str, &Value) -> Option<String>;

/// A no-op special converter.
pub fn no_special(_name: &str, _v: &Value) -> Option<String> {
    None
}

/// Read `count` values of `fmt` starting at byte offset `off` within `r`. A
/// single value yields a 1-element `Value`; multiple values yield a list.
fn read_array(r: &Reader, fmt: Fmt, off: usize, count: usize) -> Option<Value> {
    if let Fmt::Str(n) = fmt {
        let b = r.bytes(off, n)?;
        let end = b.iter().position(|&c| c == 0).unwrap_or(b.len());
        return Some(Value::Text(String::from_utf8_lossy(&b[..end]).trim_end().to_string()));
    }
    let size = fmt.size();
    let signed = matches!(fmt, Fmt::S8 | Fmt::S16 | Fmt::S32);
    if signed {
        let mut v = Vec::with_capacity(count);
        for i in 0..count {
            let o = off + i * size;
            v.push(match fmt {
                Fmt::S8 => r.u8(o)? as i8 as i64,
                Fmt::S16 => r.i16(o)? as i64,
                Fmt::S32 => r.i32(o)? as i64,
                _ => unreachable!(),
            });
        }
        Some(Value::I(v))
    } else {
        let mut v = Vec::with_capacity(count);
        for i in 0..count {
            let o = off + i * size;
            v.push(match fmt {
                Fmt::U8 => r.u8(o)? as u64,
                Fmt::U16 => r.u16(o)? as u64,
                Fmt::U32 => r.u32(o)? as u64,
                _ => unreachable!(),
            });
        }
        Some(Value::U(v))
    }
}

/// Process a binary-data blob against `table`, appending tags to `out`.
/// `special` handles formula-based conversions the enum tables can't express.
pub fn process(
    data: &[u8],
    order: ByteOrder,
    table: &BinTable,
    group1: &str,
    special: Special,
    out: &mut Vec<ExtractedTag>,
) {
    let r = Reader::new(data, order);
    let unit = table.default_fmt.size();
    // Running extra-size accumulator and a map of already-read scalar values
    // (for `int16s[$val{N}]`-style count expressions). Entries must be visited
    // in index order, which the generated tables guarantee.
    let mut var_size: usize = 0;
    let mut vals: Vec<(i32, i64)> = Vec::new();
    let lookup = |vals: &[(i32, i64)], ix: i32| -> i64 {
        vals.iter().find(|(k, _)| *k == ix).map(|(_, v)| *v).unwrap_or(0)
    };

    for t in table.tags {
        let fmt = t.fmt.unwrap_or(table.default_fmt);
        let elsize = fmt.size();
        let (count, shifts) = match t.count {
            Count::Fixed(n) => (n, false),
            Count::Var(ix) => (lookup(&vals, ix).max(0) as usize, true),
            Count::VarBits(ix) => (((lookup(&vals, ix).max(0) + 15) / 16) as usize, true),
        };
        let byte_off = (t.index as usize).wrapping_mul(unit) + var_size;
        let value = match read_array(&r, fmt, byte_off, count) {
            Some(v) => v,
            None => continue, // past end of record
        };
        // Variable-length arrays shift every subsequent entry's position.
        if shifts {
            var_size += count.saturating_mul(elsize).saturating_sub(unit);
        }
        // Remember the (scalar) value so later count expressions can reference it.
        if let Some(n) = value.as_i64() {
            vals.push((t.index, n));
        }
        if t.skip.suppresses(&value) {
            continue; // ExifTool RawConv => undef (n/a sentinel)
        }
        let print = t
            .pc
            .apply(&value)
            .or_else(|| special(t.name, &value))
            .unwrap_or_else(|| value.to_string());
        out.push(ExtractedTag::new("MakerNotes", group1, t.name, value, print));
    }
}
