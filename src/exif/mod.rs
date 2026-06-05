//! The TIFF/EXIF IFD engine — the shared heart of ExifTool.
//!
//! A TIFF "image file directory" (IFD) is an array of 12-byte entries. The same
//! structure is used inside JPEG APP1, standalone TIFF, and most RAW formats.
//! This module walks IFDs starting from a TIFF header, recurses into the EXIF,
//! GPS and Interop sub-IFDs, and produces fully-converted `ExtractedTag`s.

mod format;
pub mod printconv;
pub mod table_exif;
pub mod table_gps;
pub mod tags;

use crate::error::{Error, Result};
use crate::reader::{ByteOrder, Reader};
use crate::tag::ExtractedTag;
use crate::value::Value;
use tags::{SubDir, Table};

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
    // IFD0 (and its chain to IFD1 = thumbnail).
    walk_chain(&r, ifd0, Table::Exif, "IFD0", &mut out, &mut seen);
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
        let next = parse_ifd(r, off, table, &name, out, seen);
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
) -> usize {
    let count = match r.u16(off) {
        Some(c) => c as usize,
        None => return 0,
    };
    let entries_start = off + 2;

    // First pass: collect raw (TagDef, Value) and remember sub-IFD pointers.
    let mut raw: Vec<(&'static tags::TagDef, Value)> = Vec::new();
    let mut subdirs: Vec<(Table, &'static str, usize)> = Vec::new();

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
        let print = compute_print(table, d.name, value, &raw);
        out.push(ExtractedTag::new("EXIF", group1, d.name, value.clone(), print));
    }

    // Recurse into sub-IFDs (after this IFD's own tags, matching extraction order).
    for (tbl, g1, ptr) in subdirs {
        if !seen.contains(&ptr) {
            seen.push(ptr);
            parse_ifd(r, ptr, tbl, g1, out, seen);
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
