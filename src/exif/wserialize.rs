//! Serialize a [`WTiff`] back into TIFF bytes.
//!
//! Layout per IFD: the 12-byte entry array (entries sorted ascending by tag,
//! per the TIFF spec), a 4-byte next-IFD pointer, then this IFD's out-of-line
//! data, sub-IFDs, maker note and thumbnail. Offsets are assigned as data is
//! appended; everything is word-aligned so offsets stay even. The maker-note
//! blob is relocated with per-vendor offset fixup (see [`fixup_maker`]).

use crate::reader::ByteOrder;
use super::wtiff::{MakerBlob, MakerFix, WEntry, WIfd, WTiff, WVal, TAG_THUMB_LENGTH, TAG_THUMB_OFFSET};
use crate::exif::format::format_size;

pub fn serialize(tiff: &WTiff) -> Vec<u8> {
    let order = tiff.order;
    let mut buf = Vec::with_capacity(4096);
    // Reserve the 8-byte TIFF header; patched once IFD0 is placed.
    buf.extend_from_slice(&[0u8; 8]);
    let ifd0_off = write_ifd(&mut buf, order, &tiff.ifd0);

    buf[0..2].copy_from_slice(match order {
        ByteOrder::Little => b"II",
        ByteOrder::Big => b"MM",
    });
    buf[2..4].copy_from_slice(&enc_u16(order, 42));
    buf[4..8].copy_from_slice(&enc_u32(order, ifd0_off as u32));
    buf
}

/// Append `ifd` (and its sub-tree) to `buf`; returns the offset it was written at.
fn write_ifd(buf: &mut Vec<u8>, order: ByteOrder, ifd: &WIfd) -> usize {
    // Emit entries in ascending tag order (TIFF requirement).
    let mut entries: Vec<&WEntry> = ifd.entries.iter().collect();
    entries.sort_by_key(|e| e.tag);
    let n = entries.len();

    let dir_off = buf.len();
    let dir_size = 2 + 12 * n + 4;
    buf.resize(dir_off + dir_size, 0);
    buf[dir_off..dir_off + 2].copy_from_slice(&enc_u16(order, n as u16));

    // The thumbnail bytes are shared by the 0x201/0x202 entries; append once.
    let mut thumb_off = 0u32;
    let mut thumb_len = 0u32;
    if let Some(thumb) = &ifd.thumb {
        align(buf);
        thumb_off = buf.len() as u32;
        thumb_len = thumb.len() as u32;
        buf.extend_from_slice(thumb);
    }

    for (j, e) in entries.iter().enumerate() {
        let rec = dir_off + 2 + j * 12;
        match &e.val {
            WVal::Sub(sub) => {
                let off = write_ifd(buf, order, sub) as u32;
                write_entry_head(buf, rec, order, e.tag, e.format, 1);
                buf[rec + 8..rec + 12].copy_from_slice(&enc_u32(order, off));
            }
            WVal::Maker(blob) => {
                align(buf);
                let off = buf.len();
                let fixed = fixup_maker(blob, off, order);
                let count = fixed.len() as u32;
                buf.extend_from_slice(&fixed);
                write_entry_head(buf, rec, order, e.tag, 7, count); // undef
                buf[rec + 8..rec + 12].copy_from_slice(&enc_u32(order, off as u32));
            }
            // Thumbnail pointer/length: rewrite to the relocated blob.
            WVal::Data(_) if ifd.thumb.is_some() && e.tag == TAG_THUMB_OFFSET => {
                write_entry_head(buf, rec, order, e.tag, 4, 1);
                buf[rec + 8..rec + 12].copy_from_slice(&enc_u32(order, thumb_off));
            }
            WVal::Data(_) if ifd.thumb.is_some() && e.tag == TAG_THUMB_LENGTH => {
                write_entry_head(buf, rec, order, e.tag, 4, 1);
                buf[rec + 8..rec + 12].copy_from_slice(&enc_u32(order, thumb_len));
            }
            WVal::Data(bytes) => {
                let esize = format_size(e.format).unwrap_or(1).max(1);
                let count = (bytes.len() / esize) as u32;
                write_entry_head(buf, rec, order, e.tag, e.format, count);
                if bytes.len() <= 4 {
                    // Inline: left-aligned in the value field, zero-padded.
                    buf[rec + 8..rec + 8 + bytes.len()].copy_from_slice(bytes);
                } else {
                    align(buf);
                    let off = buf.len() as u32;
                    buf.extend_from_slice(bytes);
                    buf[rec + 8..rec + 12].copy_from_slice(&enc_u32(order, off));
                }
            }
        }
    }

    // Next-IFD pointer (IFD0 -> IFD1); 0 when absent.
    let next_field = dir_off + 2 + 12 * n;
    if let Some(next) = &ifd.next {
        let off = write_ifd(buf, order, next) as u32;
        buf[next_field..next_field + 4].copy_from_slice(&enc_u32(order, off));
    }

    dir_off
}

/// Write the tag/format/count of an IFD entry record (the first 8 bytes).
fn write_entry_head(buf: &mut [u8], rec: usize, order: ByteOrder, tag: u16, fmt: u16, count: u32) {
    buf[rec..rec + 2].copy_from_slice(&enc_u16(order, tag));
    buf[rec + 2..rec + 4].copy_from_slice(&enc_u16(order, fmt));
    buf[rec + 4..rec + 8].copy_from_slice(&enc_u32(order, count));
}

/// Relocate a maker-note blob to `new_off`, shifting its internal offsets when
/// the vendor anchors them to the TIFF base. Self-relative and embedded-TIFF
/// schemes are left byte-identical (they survive a bulk move unchanged); for
/// unrecognised vendors we shift only when the blob is detected to be a
/// self-contained TIFF-relative IFD (see [`is_self_contained_tiff_ifd`]).
fn fixup_maker(blob: &MakerBlob, new_off: usize, order: ByteOrder) -> Vec<u8> {
    let mut b = blob.bytes.clone();
    let delta = new_off as i64 - blob.orig_tiff_off as i64;
    if delta == 0 {
        return b;
    }
    let ifd_in_blob = match blob.fix {
        MakerFix::Shift { ifd_in_blob } => ifd_in_blob,
        MakerFix::None => return b,
        // Unknown vendor: shift iff the blob looks like a self-contained
        // TIFF-relative IFD at offset 0 (all out-of-line offsets land in its span).
        MakerFix::AutoDetect => {
            if is_self_contained_tiff_ifd(&b, blob.orig_tiff_off, order) {
                0
            } else {
                return b;
            }
        }
    };
    // The maker note is a plain IFD `ifd_in_blob` bytes into the blob, with
    // out-of-line value offsets absolute within the host TIFF; shift each by
    // `delta`. (Top-level entries only — the affected vendors store their
    // sub-records as flat out-of-line arrays, not nested IFDs.)
    for (pos, off) in offset_patches(&b, ifd_in_blob, order) {
        b[pos..pos + 4].copy_from_slice(&enc_u32(order, (off as i64 + delta) as u32));
    }
    b
}

/// The (byte position, current value) of every out-of-line value offset in the
/// plain IFD assumed to start at `ifd_off` of `b`. Empty if that is not a sane IFD.
fn offset_patches(b: &[u8], ifd_off: usize, order: ByteOrder) -> Vec<(usize, u32)> {
    let mut out = Vec::new();
    let count = match read_u16(b, ifd_off, order) {
        Some(c) if c >= 1 && ifd_off + 2 + c as usize * 12 + 4 <= b.len() => c as usize,
        _ => return out,
    };
    for i in 0..count {
        let rec = ifd_off + 2 + i * 12;
        let (fmt, cnt) = match (read_u16(b, rec + 2, order), read_u32(b, rec + 4, order)) {
            (Some(f), Some(c)) => (f, c as usize),
            _ => break,
        };
        let total = format_size(fmt).unwrap_or(0).saturating_mul(cnt);
        if total > 4 {
            if let Some(off) = read_u32(b, rec + 8, order) {
                out.push((rec + 8, off));
            }
        }
    }
    out
}

/// Heuristic: does `b` look like a self-contained maker note whose offsets are
/// absolute within the host TIFF? True when it parses as a sane IFD at offset 0
/// and every out-of-line offset points within the blob's own original span
/// `[orig_off, orig_off + len]`. Self-relative blobs (offsets < orig_off) and
/// non-IFD blobs fail this, so they are correctly left unshifted.
fn is_self_contained_tiff_ifd(b: &[u8], orig_off: usize, order: ByteOrder) -> bool {
    let patches = offset_patches(b, 0, order);
    if patches.is_empty() {
        return false;
    }
    let end = orig_off + b.len();
    patches.iter().all(|&(_, off)| (orig_off..=end).contains(&(off as usize)))
}

/// Pad `buf` to an even length so subsequent out-of-line data is word-aligned.
fn align(buf: &mut Vec<u8>) {
    if buf.len() & 1 == 1 {
        buf.push(0);
    }
}

fn enc_u16(order: ByteOrder, v: u16) -> [u8; 2] {
    match order {
        ByteOrder::Little => v.to_le_bytes(),
        ByteOrder::Big => v.to_be_bytes(),
    }
}

fn enc_u32(order: ByteOrder, v: u32) -> [u8; 4] {
    match order {
        ByteOrder::Little => v.to_le_bytes(),
        ByteOrder::Big => v.to_be_bytes(),
    }
}

fn read_u16(b: &[u8], off: usize, order: ByteOrder) -> Option<u16> {
    let s = b.get(off..off + 2)?;
    Some(order.u16([s[0], s[1]]))
}

fn read_u32(b: &[u8], off: usize, order: ByteOrder) -> Option<u32> {
    let s = b.get(off..off + 4)?;
    Some(order.u32([s[0], s[1], s[2], s[3]]))
}
