//! Maker-note parsing.
//!
//! EXIF tag 0x927c (MakerNote) holds vendor-specific binary metadata. ExifTool
//! dispatches on the camera `Make` (and a signature in the data) to a per-vendor
//! module. This implements two of the biggest: **Canon** and **Nikon** (Type 3).
//!
//! * Canon maker notes are a plain IFD located at the maker-note offset, whose
//!   out-of-line value offsets are relative to the *original* TIFF base — so it
//!   parses with the host TIFF reader directly.
//! * Nikon Type 3 maker notes embed their own TIFF header 10 bytes in
//!   (`Nikon\0` + version), so they parse as a self-contained sub-TIFF.

pub mod binary;
pub mod canon;
pub mod nikon;
pub mod vendor;

use crate::exif::format;
use crate::reader::Reader;
use crate::tag::ExtractedTag;
use binary::{BinTable, Pc, Skip, Special};

/// One entry in a maker-note IFD table.
pub struct MnTag {
    pub id: u16,
    pub kind: MnKind,
}

pub enum MnKind {
    /// A directly-stored value (read with the IFD entry's own format). `bin`
    /// marks tags ExifTool flags `Binary => 1` (data dumps, embedded images):
    /// these always print as the "(Binary data N bytes…)" placeholder rather
    /// than their decoded numbers.
    Scalar { name: &'static str, pc: Pc, bin: bool, skip: Skip },
    /// A pointer to a ProcessBinaryData record.
    Binary(&'static BinTable),
}

/// Dispatch the maker note at `mn_off` (length `mn_len`) within the host TIFF
/// reader `r`, given the camera `make`.
pub fn parse(make: &str, r: &Reader, mn_off: usize, mn_len: usize, out: &mut Vec<ExtractedTag>) {
    if make.starts_with("Canon") {
        walk_ifd(r, mn_off, canon::CANON_MAIN, "Canon", canon::special, out);
    } else if make.to_ascii_uppercase().starts_with("NIKON") {
        nikon::parse(r, mn_off, mn_len, out);
    } else {
        // Sony, Olympus, Panasonic, FujiFilm, Sanyo, Sigma, Ricoh, Casio,
        // Minolta — recognised by signature/Make. Unknown vendors fall through.
        vendor::parse(make, r, mn_off, mn_len, out);
    }
}

/// Walk a maker-note IFD using `table`, emitting MakerNotes-group tags. Scalar
/// tags are read with the host reader; Binary tags hand their raw value bytes to
/// the ProcessBinaryData engine. Used directly for Canon and for Nikon's
/// embedded sub-TIFF (with an appropriate reader/offset). Out-of-line value
/// offsets are interpreted relative to the host TIFF base (offset 0 of `r`).
pub fn walk_ifd(
    r: &Reader,
    off: usize,
    table: &[MnTag],
    group1: &str,
    special: Special,
    out: &mut Vec<ExtractedTag>,
) {
    walk_ifd_based(r, off, 0, table, group1, special, out);
}

/// Like [`walk_ifd`], but out-of-line value offsets are interpreted relative to
/// `base` (the host position that maker-note offset 0 maps to). Vendors whose
/// dispatch declares `Base => '$start - N'` (Olympus2/3, FujiFilm, Pentax5/6,
/// some Ricoh) store offsets relative to the maker-note start rather than the
/// TIFF base; for those, `base` is the maker-note offset. Host-base vendors
/// (Canon, Sony, Sanyo, Sigma, Panasonic, …) pass `base = 0`.
#[allow(clippy::too_many_arguments)]
pub fn walk_ifd_based(
    r: &Reader,
    off: usize,
    base: usize,
    table: &[MnTag],
    group1: &str,
    special: Special,
    out: &mut Vec<ExtractedTag>,
) {
    let count = match r.u16(off) {
        Some(c) => c as usize,
        None => return,
    };
    let entries = off + 2;
    for i in 0..count {
        let e = entries + i * 12;
        let (tag, fmt, cnt) = match (r.u16(e), r.u16(e + 2), r.u32(e + 4)) {
            (Some(t), Some(f), Some(c)) => (t, f, c as usize),
            _ => break,
        };
        let esize = match format::format_size(fmt) {
            Some(s) => s,
            None => continue,
        };
        let total = esize.saturating_mul(cnt);
        let voff = if total <= 4 {
            e + 8
        } else {
            match r.u32(e + 8) {
                Some(p) => base + p as usize,
                None => continue,
            }
        };

        let def = match table.iter().find(|t| t.id == tag) {
            Some(d) => d,
            None => continue, // unknown maker-note tag (skipped, like ExifTool without -u)
        };

        match &def.kind {
            MnKind::Binary(bin) => {
                if let Some(raw) = r.bytes(voff, total) {
                    binary::process(raw, r.order, bin, group1, special, out);
                }
            }
            MnKind::Scalar { name, pc, bin, skip } => {
                if let Some(value) = format::read_value(r, fmt, cnt, voff) {
                    if skip.suppresses(&value) {
                        continue; // ExifTool RawConv => undef (n/a sentinel)
                    }
                    let print = if *bin {
                        format!("(Binary data {total} bytes, use -b option to extract)")
                    } else {
                        // Prefer the table's PrintConv, then the vendor special
                        // converter, then the shared EXIF PrintConv, then raw.
                        pc.apply(&value)
                            .or_else(|| special(name, &value))
                            .or_else(|| crate::exif::printconv::apply(name, &value))
                            .unwrap_or_else(|| value.to_string())
                    };
                    out.push(ExtractedTag::new("MakerNotes", group1, *name, value, print));
                }
            }
        }
    }
}
