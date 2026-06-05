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

pub struct BinTag {
    pub index: i32,
    pub name: &'static str,
    pub fmt: Option<Fmt>,
    pub pc: Pc,
    pub skip: Skip,
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

/// Read one value of `fmt` at byte offset `off` within `r`.
fn read_one(r: &Reader, fmt: Fmt, off: usize) -> Option<Value> {
    Some(match fmt {
        Fmt::U8 => Value::U(vec![r.u8(off)? as u64]),
        Fmt::S8 => Value::I(vec![r.u8(off)? as i8 as i64]),
        Fmt::U16 => Value::U(vec![r.u16(off)? as u64]),
        Fmt::S16 => Value::I(vec![r.i16(off)? as i64]),
        Fmt::U32 => Value::U(vec![r.u32(off)? as u64]),
        Fmt::S32 => Value::I(vec![r.i32(off)? as i64]),
        Fmt::Str(n) => {
            let b = r.bytes(off, n)?;
            let end = b.iter().position(|&c| c == 0).unwrap_or(b.len());
            Value::Text(String::from_utf8_lossy(&b[..end]).trim_end().to_string())
        }
    })
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
    for t in table.tags {
        let byte_off = t.index as usize * unit;
        let fmt = t.fmt.unwrap_or(table.default_fmt);
        let value = match read_one(&r, fmt, byte_off) {
            Some(v) => v,
            None => continue, // past end of record
        };
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
