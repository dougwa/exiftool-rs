//! The TIFF/EXIF IFD engine — the shared heart of ExifTool.
//!
//! A TIFF "image file directory" (IFD) is an array of 12-byte entries. The same
//! structure is used inside JPEG APP1, standalone TIFF, and most RAW formats.
//! This module walks IFDs starting from a TIFF header, recurses into the EXIF,
//! GPS and Interop sub-IFDs, and produces fully-converted `ExtractedTag`s.

pub mod format;
pub mod printconv;
pub mod table_exif;
pub mod table_gps;
pub mod tags;
pub mod writable;
pub mod writeconv;
pub mod wserialize;
pub mod wtiff;

use crate::error::{Error, Result};
use crate::reader::{ByteOrder, Reader};
use crate::tag::ExtractedTag;
use crate::value::Value;
use tags::{SubDir, Table};
use wtiff::{WEntry, WIfd, WTiff, WVal};

/// Parse a TIFF structure (EXIF). `data` is the buffer whose offset 0 is the
/// start of the TIFF header (`II\x2a\0` or `MM\0\x2a`). All IFD offsets are
/// relative to this point. Returns extracted tags in IFD order.
pub fn parse_tiff(data: &[u8]) -> Result<Vec<ExtractedTag>> {
    let order = match data.get(0..2) {
        Some(b"II") => ByteOrder::Little,
        Some(b"MM") => ByteOrder::Big,
        _ => return Err(Error::Format("not a TIFF header".into())),
    };
    let r = Reader::new(data, order);
    let magic = r.u16(2).ok_or(Error::Truncated("tiff magic"))?;
    if magic != 42 {
        // 43 == BigTIFF, which uses a different (64-bit) layout we don't handle yet.
        return Err(Error::Unsupported(format!("TIFF magic {magic} (BigTIFF?)")));
    }
    let ifd0 = r.u32(4).ok_or(Error::Truncated("ifd0 offset"))? as usize;

    let mut out = Vec::new();
    let mut seen = Vec::new();
    // Camera Make, captured from IFD0 and used to dispatch maker notes.
    let mut make: Option<String> = None;
    // IFD0 (and its chain to IFD1 = thumbnail).
    walk_chain(&r, ifd0, Table::Exif, "IFD0", &mut out, &mut seen, &mut make);
    Ok(out)
}

/// Walk an IFD and any following IFDs in its next-pointer chain (IFD0 -> IFD1...).
fn walk_chain(
    r: &Reader,
    mut off: usize,
    table: Table,
    group1: &str,
    out: &mut Vec<ExtractedTag>,
    seen: &mut Vec<usize>,
    make: &mut Option<String>,
) {
    let mut idx = 0usize;
    while off != 0 {
        if seen.contains(&off) {
            break; // guard against loops
        }
        seen.push(off);
        // First IFD in the chain uses the supplied name (e.g. "IFD0"); the next
        // is the thumbnail directory "IFD1" (ExifTool's convention).
        let name = if table == Table::Exif && group1 == "IFD0" && idx == 1 {
            "IFD1".to_string()
        } else {
            group1.to_string()
        };
        let next = parse_ifd(r, off, table, &name, out, seen, make);
        off = next;
        idx += 1;
        if table != Table::Exif {
            break; // only the main IFD0 chain follows next-pointers
        }
    }
}

/// Parse a single IFD at `off`. Returns the next-IFD offset (0 if none).
fn parse_ifd(
    r: &Reader,
    off: usize,
    table: Table,
    group1: &str,
    out: &mut Vec<ExtractedTag>,
    seen: &mut Vec<usize>,
    make: &mut Option<String>,
) -> usize {
    let count = match r.u16(off) {
        Some(c) => c as usize,
        None => return 0,
    };
    let entries_start = off + 2;

    // First pass: collect raw (TagDef, Value) and remember sub-IFD pointers.
    let mut raw: Vec<(&'static tags::TagDef, Value)> = Vec::new();
    let mut subdirs: Vec<(Table, &'static str, usize)> = Vec::new();
    // Maker-note (tag 0x927c) location, dispatched after this IFD's own tags.
    let mut maker_note: Option<(usize, usize)> = None;

    for i in 0..count {
        let e = entries_start + i * 12;
        let (tag, fmt, cnt, val_field) = match (r.u16(e), r.u16(e + 2), r.u32(e + 4)) {
            (Some(t), Some(f), Some(c)) => (t, f, c as usize, e + 8),
            _ => break,
        };
        let esize = match format::format_size(fmt) {
            Some(s) => s,
            None => continue, // unknown format type
        };
        let total = esize.saturating_mul(cnt);
        // Where the value lives: inline (<=4 bytes) or at the pointed offset.
        let voff = if total <= 4 {
            val_field
        } else {
            match r.u32(val_field) {
                Some(p) => p as usize,
                None => continue,
            }
        };

        // MakerNote (0x927c): a vendor-specific blob, dispatched after this IFD.
        if table == Table::Exif && tag == 0x927c {
            maker_note = Some((voff, total));
            continue;
        }

        let def = table.lookup(tag);

        // Handle sub-directories (pointers to nested IFDs).
        if let Some(d) = def {
            match d.sub {
                SubDir::Ifd | SubDir::Gps => {
                    // A sub-IFD pointer is always a 4-byte offset in the value field.
                    if let Some(ptr) = r.u32(val_field) {
                        let sub_tbl = if d.sub == SubDir::Gps { Table::Gps } else { Table::Exif };
                        let sub_g1 = sub_group_name(d.name);
                        subdirs.push((sub_tbl, sub_g1, ptr as usize));
                    }
                    continue;
                }
                SubDir::Skip => {
                    // Recognised but unparsed; skip silently.
                    continue;
                }
                SubDir::None => {}
            }
        }

        let value = match format::read_value(r, fmt, cnt, voff) {
            Some(v) => v,
            None => continue,
        };

        match def {
            Some(d) => raw.push((d, value)),
            None => {
                // Unknown tag — emit as Tag-0xXXXX so nothing is silently lost.
                // (ExifTool only shows these with -u; we keep them out by default.)
            }
        }
    }

    // Second pass: build prints (with sibling access for GPS coordinates).
    for (d, value) in &raw {
        // Capture the camera Make (IFD0) so maker notes can be dispatched later.
        if d.name == "Make" {
            if let Some(s) = value.as_str() {
                *make = Some(s.trim().to_string());
            }
        }
        let print = compute_print(table, d.name, value, &raw);
        out.push(ExtractedTag::new("EXIF", group1, d.name, value.clone(), print));
    }

    // Dispatch maker notes (after the ExifIFD's own tags, matching ExifTool order).
    if let Some((mo, ml)) = maker_note {
        let mk = make.as_deref().unwrap_or("");
        crate::makernotes::parse(mk, r, mo, ml, out);
    }

    // Recurse into sub-IFDs (after this IFD's own tags, matching extraction order).
    for (tbl, g1, ptr) in subdirs {
        if !seen.contains(&ptr) {
            seen.push(ptr);
            parse_ifd(r, ptr, tbl, g1, out, seen, make);
        }
    }

    // Return next-IFD pointer (located after the entry array).
    let next_off = entries_start + count * 12;
    r.u32(next_off).map(|v| v as usize).unwrap_or(0)
}

fn sub_group_name(tag_name: &str) -> &'static str {
    match tag_name {
        "ExifOffset" => "ExifIFD",
        "GPSInfo" => "GPS",
        "InteropOffset" => "InteropIFD",
        "GlobalParametersIFD" => "GlobalParameters",
        _ => "SubIFD",
    }
}

/// Compute the human-readable print value for one tag, consulting siblings when
/// needed (GPS coordinates pull in their N/S/E/W reference tag).
fn compute_print(
    table: Table,
    name: &str,
    value: &Value,
    siblings: &[(&'static tags::TagDef, Value)],
) -> String {
    if table == Table::Gps {
        let ref_name = match name {
            "GPSLatitude" => Some("GPSLatitudeRef"),
            "GPSLongitude" => Some("GPSLongitudeRef"),
            "GPSDestLatitude" => Some("GPSDestLatitudeRef"),
            "GPSDestLongitude" => Some("GPSDestLongitudeRef"),
            _ => None,
        };
        if let (Some(rn), Value::R(parts)) = (ref_name, value) {
            let reference = siblings
                .iter()
                .find(|(d, _)| d.name == rn)
                .and_then(|(_, v)| v.as_str());
            if let Some(s) = printconv::gps_coordinate(parts, reference) {
                return s;
            }
        }

        // GPSAltitude: ExifTool's default output appends the altitude reference
        // ("Above Sea Level" / "Below Sea Level") from the sibling GPSAltitudeRef.
        if name == "GPSAltitude" {
            if let Some(base) = printconv::apply(name, value) {
                let altref = siblings
                    .iter()
                    .find(|(d, _)| d.name == "GPSAltitudeRef")
                    .and_then(|(_, v)| v.as_u64());
                let suffix = match altref {
                    Some(0) => " Above Sea Level",
                    Some(1) => " Below Sea Level",
                    _ => "",
                };
                return format!("{base}{suffix}");
            }
        }
    }

    printconv::apply(name, value).unwrap_or_else(|| value.to_string())
}

// ---------------------------------------------------------------------------
// Writing
// ---------------------------------------------------------------------------

/// One requested change to a file's metadata.
pub struct Edit {
    /// Tag name (matched case-insensitively against the writable table).
    pub name: String,
    pub op: EditOp,
}

pub enum EditOp {
    /// Set the tag to this (human) value.
    Set(String),
    /// Remove the tag.
    Delete,
}

/// Parse a TIFF block, apply `edits`, and re-serialize it. `data` starts at the
/// `II`/`MM` header (the same view `parse_tiff` takes).
pub fn write_tiff(data: &[u8], edits: &[Edit]) -> Result<Vec<u8>> {
    let mut tiff = wtiff::parse(data)?;
    for edit in edits {
        apply_edit(&mut tiff, edit)?;
    }
    Ok(wserialize::serialize(&tiff))
}

fn apply_edit(tiff: &mut WTiff, edit: &Edit) -> Result<()> {
    let w = writable::lookup(&edit.name)
        .ok_or_else(|| Error::Write(format!("tag not writable: {}", edit.name)))?;
    let order = tiff.order;
    let ifd = target_ifd_mut(&mut tiff.ifd0, w.ifd);
    match &edit.op {
        EditOp::Delete => {
            ifd.entries.retain(|e| e.tag != w.id);
        }
        EditOp::Set(input) => {
            let enc = writeconv::encode(w, input, order)?;
            set_entry(ifd, w.id, enc.fmt, enc.bytes);
            for ex in enc.extra {
                set_entry(ifd, ex.id, ex.fmt, ex.bytes);
            }
        }
    }
    Ok(())
}

/// Replace the entry with id `tag` in `ifd`, or append it if absent.
fn set_entry(ifd: &mut WIfd, tag: u16, fmt: u16, bytes: Vec<u8>) {
    let entry = WEntry { tag, format: fmt, val: WVal::Data(bytes) };
    match ifd.entries.iter_mut().find(|e| e.tag == tag) {
        Some(slot) => *slot = entry,
        None => ifd.entries.push(entry),
    }
}

/// Get a mutable reference to the IFD a writable tag belongs in, creating the
/// ExifIFD / GPS sub-IFD (and its pointer entry in IFD0) if it does not exist.
fn target_ifd_mut(ifd0: &mut WIfd, loc: writable::Loc) -> &mut WIfd {
    let sub_tag = match loc {
        writable::Loc::Ifd0 => return ifd0,
        writable::Loc::Exif => 0x8769u16, // ExifOffset
        writable::Loc::Gps => 0x8825u16,  // GPSInfo
    };
    if let Some(i) = ifd0
        .entries
        .iter()
        .position(|e| e.tag == sub_tag && matches!(e.val, WVal::Sub(_)))
    {
        if let WVal::Sub(s) = &mut ifd0.entries[i].val {
            return s;
        }
        unreachable!("matched Sub above");
    }
    ifd0.entries.push(WEntry { tag: sub_tag, format: 4, val: WVal::Sub(WIfd::default()) });
    let last = ifd0.entries.len() - 1;
    match &mut ifd0.entries[last].val {
        WVal::Sub(s) => s,
        _ => unreachable!("just pushed Sub"),
    }
}
