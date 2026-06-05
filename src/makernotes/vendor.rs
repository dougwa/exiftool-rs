//! Generic vendor maker-note dispatch.
//!
//! Beyond Canon and Nikon, most vendors store their maker note as a plain TIFF
//! IFD prefixed by a short signature. They differ in two ways, both captured by
//! ExifTool's `MakerNotes.pm` dispatch table:
//!
//! * **Start** — how many bytes of signature precede the IFD (e.g. "OLYMP\0" +
//!   2 bytes ⇒ start 8; "Panasonic\0" + 2 ⇒ start 12).
//! * **Base** — whether out-of-line value offsets are relative to the host TIFF
//!   base (the common case) or to the maker-note start itself (ExifTool's
//!   `Base => '$start - N'`, e.g. Olympus2/3, FujiFilm).
//!
//! Byte order follows the host TIFF for every file in ExifTool's test suite, so
//! we inherit `r.order` rather than re-detecting it. Sub-records (binary-data
//! SubDirectories) are not yet followed — only the main IFD's scalar tags.

use super::binary::{no_special, Special};
use super::{walk_ifd_based, MnTag};
use crate::reader::Reader;
use crate::tag::ExtractedTag;

mod casio_main;
mod fujifilm_main;
mod minolta_main;
mod olympus_main;
mod panasonic_main;
mod pentax_main;
mod ricoh_main;
mod sanyo_main;
mod sigma_main;
mod sony_main;

mod special;

/// A resolved vendor maker-note layout.
struct Layout {
    /// Absolute position (in the host reader) where the IFD begins.
    ifd_off: usize,
    /// Host position that out-of-line offset 0 maps to (0 = TIFF base).
    base: usize,
    group1: &'static str,
    table: &'static [MnTag],
    special: Special,
}

/// Try to dispatch a non-Canon/Nikon maker note. Returns `true` if a vendor was
/// recognised and parsed.
pub fn parse(make: &str, r: &Reader, mn_off: usize, mn_len: usize, out: &mut Vec<ExtractedTag>) -> bool {
    // The signature lives at the start of the maker-note value.
    let sig = r.bytes(mn_off, mn_len.min(16)).unwrap_or(&[]);
    let layout = match detect(make, sig, mn_off, r) {
        Some(l) => l,
        None => return false,
    };
    walk_ifd_based(r, layout.ifd_off, layout.base, layout.table, layout.group1, layout.special, out);
    true
}

/// Map a maker note to its layout, mirroring `MakerNotes.pm`'s condition list.
fn detect(make: &str, sig: &[u8], mn_off: usize, r: &Reader) -> Option<Layout> {
    let starts = |p: &[u8]| sig.len() >= p.len() && &sig[..p.len()] == p;

    // --- Signature-prefixed vendors --------------------------------------
    if starts(b"SONY DSC ") || starts(b"SONY CAM ") || starts(b"SONY MOBILE") {
        return Some(Layout { ifd_off: mn_off + 12, base: 0, group1: "Sony", table: sony_main::SONY_MAIN, special: special::sony });
    }
    if starts(b"OM SYSTEM\0") {
        return Some(Layout { ifd_off: mn_off + 16, base: mn_off, group1: "Olympus", table: olympus_main::OLYMPUS_MAIN, special: special::olympus });
    }
    if starts(b"OLYMPUS\0") {
        return Some(Layout { ifd_off: mn_off + 12, base: mn_off, group1: "Olympus", table: olympus_main::OLYMPUS_MAIN, special: special::olympus });
    }
    if starts(b"OLYMP\0") || starts(b"EPSON\0") {
        return Some(Layout { ifd_off: mn_off + 8, base: 0, group1: "Olympus", table: olympus_main::OLYMPUS_MAIN, special: special::olympus });
    }
    if starts(b"Panasonic\0") {
        return Some(Layout { ifd_off: mn_off + 12, base: 0, group1: "Panasonic", table: panasonic_main::PANASONIC_MAIN, special: special::panasonic });
    }
    if starts(b"FUJIFILM") || starts(b"GENERALE") {
        // A 4-byte little-endian IFD pointer follows the 8-byte signature; the
        // IFD and its offsets are relative to the maker-note start.
        let rel = r.u32(mn_off + 8)? as usize;
        return Some(Layout { ifd_off: mn_off + rel, base: mn_off, group1: "FujiFilm", table: fujifilm_main::FUJIFILM_MAIN, special: special::fujifilm });
    }
    if starts(b"SANYO\0") {
        return Some(Layout { ifd_off: mn_off + 8, base: 0, group1: "Sanyo", table: sanyo_main::SANYO_MAIN, special: no_special });
    }
    if starts(b"SIGMA\0") || starts(b"FOVEON\0") {
        return Some(Layout { ifd_off: mn_off + 10, base: 0, group1: "Sigma", table: sigma_main::SIGMA_MAIN, special: special::sigma });
    }
    if starts(b"RICOH\0") || starts(b"Ricoh") {
        return Some(Layout { ifd_off: mn_off + 8, base: 0, group1: "Ricoh", table: ricoh_main::RICOH_MAIN, special: no_special });
    }
    if starts(b"AOC\0") {
        // Pentax/Asahi: "AOC\0" + a 2-byte byte-order marker ("MM"/"II"), then
        // the IFD. ExifTool processes these as Unknown maker notes with FixBase;
        // for JPEGs the value offsets are relative to the TIFF base. Byte order
        // follows the host (matches the marker in practice).
        return Some(Layout { ifd_off: mn_off + 6, base: 0, group1: "Pentax", table: pentax_main::PENTAX_MAIN, special: special::pentax });
    }
    if starts(b"QVC\0") || starts(b"DCI\0") {
        // Casio Type2 — same Main table is not used; only Type2. We don't have a
        // Type2 table yet, so leave unparsed rather than misread.
        return None;
    }

    // --- Signature-less vendors (dispatch by Make) -----------------------
    let mu = make.to_ascii_uppercase();
    if mu.starts_with("CASIO") {
        return Some(Layout { ifd_off: mn_off, base: 0, group1: "Casio", table: casio_main::CASIO_MAIN, special: special::casio });
    }
    if mu.starts_with("MINOLTA") || mu.starts_with("KONICA MINOLTA") {
        return Some(Layout { ifd_off: mn_off, base: 0, group1: "Minolta", table: minolta_main::MINOLTA_MAIN, special: special::minolta });
    }

    None
}
