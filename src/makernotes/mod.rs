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

use crate::exif::format;
use crate::reader::Reader;
use crate::tag::ExtractedTag;
use binary::{BinTable, Pc, Special};

/// One entry in a maker-note IFD table.
pub struct MnTag {
    pub id: u16,
    pub kind: MnKind,
}

pub enum MnKind {
    /// A directly-stored value (read with the IFD entry's own format).
    Scalar { name: &'static str, pc: Pc },
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
    }
    // Other vendors fall through (left unparsed, as before).
}

/// Walk a maker-note IFD using `table`, emitting MakerNotes-group tags. Scalar
/// tags are read with the host reader; Binary tags hand their raw value bytes to
/// the ProcessBinaryData engine. Used directly for Canon and for Nikon's
/// embedded sub-TIFF (with an appropriate reader/offset).
pub fn walk_ifd(
    r: &Reader,
    off: usize,
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
                Some(p) => p as usize,
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
            MnKind::Scalar { name, pc } => {
                if let Some(value) = format::read_value(r, fmt, cnt, voff) {
                    // Prefer the table's PrintConv, then the vendor special
                    // converter, then the shared EXIF PrintConv, then raw.
                    let print = pc
                        .apply(&value)
                        .or_else(|| special(name, &value))
                        .or_else(|| crate::exif::printconv::apply(name, &value))
                        .unwrap_or_else(|| value.to_string());
                    out.push(ExtractedTag::new("MakerNotes", group1, *name, value, print));
                }
            }
        }
    }
}
