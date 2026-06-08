//! Editable, byte-round-trippable TIFF/EXIF model — the write counterpart to the
//! read engine in [`super`].
//!
//! The read path ([`super::parse_tiff`]) is deliberately lossy: it applies
//! PrintConv, drops unknown tags, and flattens maker notes. None of that can be
//! re-serialized faithfully. This module parses the same TIFF block into a model
//! that keeps **every** entry exactly as stored (`WVal::Data`), recurses the
//! known sub-IFD pointers (`WVal::Sub`), and carries the maker note and IFD1
//! thumbnail as opaque blobs that the serializer relocates with offset fixup.

use crate::error::{Error, Result};
use crate::exif::format::format_size;
use crate::reader::{ByteOrder, Reader};

/// EXIF tag ids that point at a nested IFD we parse structurally.
const TAG_EXIF_OFFSET: u16 = 0x8769;
const TAG_GPS_INFO: u16 = 0x8825;
const TAG_INTEROP_OFFSET: u16 = 0xa005;
const TAG_MAKER_NOTE: u16 = 0x927c;
const TAG_MAKE: u16 = 0x010f;
pub const TAG_THUMB_OFFSET: u16 = 0x0201;
pub const TAG_THUMB_LENGTH: u16 = 0x0202;

/// A full TIFF structure rooted at IFD0 (whose `next` is the IFD1 thumbnail dir).
pub struct WTiff {
    pub order: ByteOrder,
    pub ifd0: WIfd,
}

/// One image file directory.
#[derive(Default)]
pub struct WIfd {
    pub entries: Vec<WEntry>,
    /// IFD1's thumbnail JPEG bytes (referenced by 0x201/0x202), captured so the
    /// serializer can relocate them. `None` for IFDs without a thumbnail.
    pub thumb: Option<Vec<u8>>,
    /// The next IFD in the chain (IFD0 -> IFD1). Only the main chain uses this.
    pub next: Option<Box<WIfd>>,
}

/// One IFD entry: its tag id, EXIF format code, and value.
pub struct WEntry {
    pub tag: u16,
    pub format: u16,
    pub val: WVal,
}

pub enum WVal {
    /// Raw value bytes exactly as stored in the file (in TIFF byte order). The
    /// element count is `bytes.len() / format_size(format)`.
    Data(Vec<u8>),
    /// A nested IFD (ExifOffset / GPSInfo / InteropOffset).
    Sub(WIfd),
    /// The MakerNote blob (tag 0x927c), with the info needed to fix up its
    /// internal offsets when it is relocated.
    Maker(MakerBlob),
}

/// How a relocated maker note's internal offsets must be repaired.
///
/// A maker note is a plain TIFF IFD prefixed by a vendor signature of known
/// length. Its out-of-line value offsets are either absolute within the host
/// TIFF (so they must be shifted when the blob moves) or relative to the
/// maker-note / an embedded TIFF header (self-consistent under a bulk move).
/// This mirrors the per-vendor `ifd_off`/`base` table the reader uses in
/// `makernotes::vendor::detect` (and `nikon`, `canon`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MakerFix {
    /// TIFF-relative offsets; the IFD starts this many bytes into the blob
    /// (after the signature). Shift every out-of-line offset by the move delta.
    Shift { ifd_in_blob: usize },
    /// Self-relative or embedded-TIFF offsets — leave the blob byte-identical.
    None,
    /// Unknown vendor: shift only if the blob is detected to be a self-contained
    /// TIFF-relative IFD at offset 0 (see `wserialize::is_self_contained_tiff_ifd`).
    AutoDetect,
}

pub struct MakerBlob {
    pub bytes: Vec<u8>,
    /// Absolute offset of the blob within the original TIFF (for the fixup delta).
    pub orig_tiff_off: usize,
    pub fix: MakerFix,
}

/// Determine how to repair a maker note's offsets after relocation, from the
/// camera `make` and the blob's leading signature. Mirrors the structural
/// `ifd_off`/`base` decisions in [`crate::makernotes::vendor`] (`detect`),
/// [`crate::makernotes::nikon`] and Canon's host-base IFD.
pub fn maker_fix(make: &str, blob: &[u8]) -> MakerFix {
    let sig = |p: &[u8]| blob.len() >= p.len() && &blob[..p.len()] == p;
    let shift = |ifd_in_blob| MakerFix::Shift { ifd_in_blob };

    // --- Nikon ----------------------------------------------------------------
    if sig(b"Nikon\x00") {
        // Type 3 ("Nikon\0" + 0x02): a self-contained embedded TIFF — no fixup.
        // Type 1/2: signature + version, then a TIFF-relative IFD at offset 8.
        return if blob.get(6) == Some(&0x02) { MakerFix::None } else { shift(8) };
    }

    // --- Signature-prefixed vendors (lengths from vendor::detect) -------------
    if sig(b"SONY DSC ") || sig(b"SONY CAM ") || sig(b"SONY MOBILE") {
        return shift(12);
    }
    if sig(b"OM SYSTEM\0") || sig(b"OLYMPUS\0") || sig(b"FUJIFILM") || sig(b"GENERALE") {
        return MakerFix::None; // base = maker-note start (self-relative)
    }
    if sig(b"OLYMP\0") || sig(b"EPSON\0") {
        return shift(8);
    }
    if sig(b"Panasonic\0") {
        return shift(12);
    }
    if sig(b"SANYO\0") || sig(b"RICOH\0") || sig(b"Ricoh") {
        return shift(8);
    }
    if sig(b"SIGMA\0") || sig(b"FOVEON\0") {
        return shift(10);
    }
    if sig(b"AOC\0") {
        return shift(6); // Pentax/Asahi
    }

    // --- Signature-less vendors (a bare IFD at offset 0) ----------------------
    let mu = make.to_ascii_uppercase();
    if mu.starts_with("CANON")
        || mu.starts_with("CASIO")
        || mu.starts_with("MINOLTA")
        || mu.starts_with("KONICA MINOLTA")
    {
        return shift(0);
    }

    MakerFix::AutoDetect
}

/// Parse a TIFF block (offset 0 = the `II`/`MM` header) into the editable model.
pub fn parse(data: &[u8]) -> Result<WTiff> {
    let order = match data.get(0..2) {
        Some(b"II") => ByteOrder::Little,
        Some(b"MM") => ByteOrder::Big,
        _ => return Err(Error::Format("not a TIFF header".into())),
    };
    let r = Reader::new(data, order);
    let magic = r.u16(2).ok_or(Error::Truncated("tiff magic"))?;
    if magic != 42 {
        return Err(Error::Unsupported(format!("TIFF magic {magic} (BigTIFF?)")));
    }
    let ifd0_off = r.u32(4).ok_or(Error::Truncated("ifd0 offset"))? as usize;

    let mut make = String::new();
    let mut seen = Vec::new();
    let (mut ifd0, next) = parse_one(&r, ifd0_off, &mut make, &mut seen)?;
    // Follow the one next-pointer to IFD1 (the thumbnail directory).
    if next != 0 && !seen.contains(&next) {
        let (ifd1, _) = parse_one(&r, next, &mut make, &mut seen)?;
        ifd0.next = Some(Box::new(ifd1));
    }
    Ok(WTiff { order, ifd0 })
}

/// Parse a single IFD at `off`; returns the directory and the next-IFD offset.
fn parse_one(
    r: &Reader,
    off: usize,
    make: &mut String,
    seen: &mut Vec<usize>,
) -> Result<(WIfd, usize)> {
    seen.push(off);
    let count = r.u16(off).ok_or(Error::Truncated("ifd count"))? as usize;
    let entries_start = off + 2;
    let mut ifd = WIfd::default();
    let mut thumb_off: Option<usize> = None;
    let mut thumb_len: Option<usize> = None;

    for i in 0..count {
        let e = entries_start + i * 12;
        let (tag, fmt, cnt) = match (r.u16(e), r.u16(e + 2), r.u32(e + 4)) {
            (Some(t), Some(f), Some(c)) => (t, f, c as usize),
            _ => break,
        };
        let esize = match format_size(fmt) {
            Some(s) => s,
            None => continue, // unknown format type — drop (cannot size it)
        };
        let total = esize.saturating_mul(cnt);
        let val_field = e + 8;
        let voff = if total <= 4 {
            val_field
        } else {
            match r.u32(val_field) {
                Some(p) => p as usize,
                None => continue,
            }
        };

        // Nested IFDs we understand structurally.
        if matches!(tag, TAG_EXIF_OFFSET | TAG_GPS_INFO | TAG_INTEROP_OFFSET) {
            if let Some(ptr) = r.u32(val_field) {
                let ptr = ptr as usize;
                if !seen.contains(&ptr) {
                    let (sub, _) = parse_one(r, ptr, make, seen)?;
                    ifd.entries.push(WEntry { tag, format: fmt, val: WVal::Sub(sub) });
                }
            }
            continue;
        }

        // MakerNote: opaque blob + offset scheme for later fixup.
        if tag == TAG_MAKER_NOTE {
            if let Some(bytes) = r.bytes(voff, total) {
                ifd.entries.push(WEntry {
                    tag,
                    format: fmt,
                    val: WVal::Maker(MakerBlob {
                        fix: maker_fix(make, bytes),
                        bytes: bytes.to_vec(),
                        orig_tiff_off: voff,
                    }),
                });
            }
            continue;
        }

        let bytes = match r.bytes(voff, total) {
            Some(b) => b.to_vec(),
            None => continue,
        };

        // Capture Make so the maker note (which lives later, in ExifIFD) can be
        // classified. Strings are NUL-terminated in EXIF.
        if tag == TAG_MAKE {
            let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
            *make = String::from_utf8_lossy(&bytes[..end]).trim().to_string();
        }

        // Remember the thumbnail pointer/length (IFD1) — captured below.
        if tag == TAG_THUMB_OFFSET {
            thumb_off = r.u32(val_field).map(|v| v as usize);
        } else if tag == TAG_THUMB_LENGTH {
            thumb_len = read_int_u32(r, fmt, voff).map(|v| v as usize);
        }

        ifd.entries.push(WEntry { tag, format: fmt, val: WVal::Data(bytes) });
    }

    // Capture the thumbnail JPEG bytes if both pointer and length are valid. The
    // 0x201/0x202 entries stay in `entries`; the serializer rewrites their values.
    if let (Some(o), Some(l)) = (thumb_off, thumb_len) {
        if l > 0 {
            if let Some(b) = r.bytes(o, l) {
                ifd.thumb = Some(b.to_vec());
            }
        }
    }

    let next_off = r.u32(entries_start + count * 12).map(|v| v as usize).unwrap_or(0);
    Ok((ifd, next_off))
}

/// Read the first element of an int-typed entry at `voff` as u32 (for the
/// thumbnail length, which may be stored as int16u or int32u).
fn read_int_u32(r: &Reader, fmt: u16, voff: usize) -> Option<u32> {
    match fmt {
        3 => r.u16(voff).map(|v| v as u32), // int16u
        4 | 13 => r.u32(voff),              // int32u / ifd
        1 => r.u8(voff).map(|v| v as u32),  // int8u
        _ => None,
    }
}
