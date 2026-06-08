//! JPEG segment walker.
//!
//! A JPEG is a sequence of marker segments. We scan them to locate the EXIF
//! block (APP1 starting with "Exif\0\0"), JFIF (APP0), comments, and the frame
//! header (SOFn) which carries image dimensions. Mirrors ExifTool's JpegInfo.

use crate::error::{Error, Result};
use crate::exif::{self, Edit};
use crate::reader::{be_u16, ByteOrder};
use crate::tag::ExtractedTag;
use crate::value::Value;

/// A minimal little-endian TIFF used to create an EXIF block from scratch when a
/// JPEG has none. Its IFD0 carries the one mandatory tag ExifTool seeds into a
/// newly-created JPEG IFD0, `YCbCrPositioning = 1 (Centered)`, so a created block
/// validates the same way ExifTool's does.
const EMPTY_TIFF: &[u8] = &[
    b'I', b'I', 0x2a, 0x00, // II, magic 42
    0x08, 0x00, 0x00, 0x00, // IFD0 at offset 8
    0x01, 0x00, // 1 entry
    0x13, 0x02, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // YCbCrPositioning=1
    0x00, 0x00, 0x00, 0x00, // next IFD = 0
];

/// The "Exif\0\0" identifier that prefixes the TIFF block in an APP1 segment.
const EXIF_ID: &[u8] = b"Exif\x00\x00";

/// Apply `edits` to the EXIF block of a JPEG, returning the new file bytes. The
/// EXIF APP1 is rebuilt; an absent one is created. All other segments are copied
/// verbatim.
pub fn write(buf: &[u8], edits: &[Edit]) -> Result<Vec<u8>> {
    if buf.len() < 2 || buf[0] != 0xFF || buf[1] != 0xD8 {
        return Err(Error::Format("not a JPEG".into()));
    }

    // Locate the EXIF APP1 segment (and the end of APP0, an insertion point).
    let mut pos = 2usize;
    let mut exif: Option<(usize, usize)> = None; // (segment start at 0xFF, segment end)
    let mut app0_end: Option<usize> = None;
    while pos + 4 <= buf.len() {
        if buf[pos] != 0xFF {
            pos += 1;
            continue;
        }
        let marker = buf[pos + 1];
        if marker == 0xD8 || marker == 0xD9 || (0xD0..=0xD7).contains(&marker) {
            pos += 2;
            continue;
        }
        if marker == 0xDA {
            break; // start of scan — no more metadata segments
        }
        let seg_len = match be_u16(&buf[pos + 2..]) {
            Some(l) if l >= 2 => l as usize,
            _ => break,
        };
        let seg_end = pos + 2 + seg_len;
        if seg_end > buf.len() {
            break;
        }
        let payload = &buf[pos + 4..seg_end];
        if marker == 0xE1 && payload.starts_with(EXIF_ID) {
            exif = Some((pos, seg_end));
            break;
        }
        if marker == 0xE0 {
            app0_end = Some(seg_end);
        }
        pos = seg_end;
    }

    // Build the new TIFF block: edit the existing one, or a fresh empty TIFF.
    let (tiff_in, splice_start, splice_end): (&[u8], usize, usize) = match exif {
        Some((start, end)) => (&buf[start + 4 + EXIF_ID.len()..end], start, end),
        None => {
            let at = app0_end.unwrap_or(2); // after APP0, else right after SOI
            (EMPTY_TIFF, at, at)
        }
    };
    let new_tiff = exif::write_tiff(tiff_in, edits)?;

    let seg_len = 2 + EXIF_ID.len() + new_tiff.len();
    if seg_len > 0xFFFF {
        return Err(Error::Write(format!(
            "EXIF data ({} bytes) exceeds the 64 KB JPEG segment limit",
            new_tiff.len()
        )));
    }

    let mut out = Vec::with_capacity(buf.len() + new_tiff.len());
    out.extend_from_slice(&buf[..splice_start]);
    out.extend_from_slice(&[0xFF, 0xE1]);
    out.extend_from_slice(&(seg_len as u16).to_be_bytes());
    out.extend_from_slice(EXIF_ID);
    out.extend_from_slice(&new_tiff);
    out.extend_from_slice(&buf[splice_end..]);
    Ok(out)
}

pub fn parse(buf: &[u8]) -> Vec<ExtractedTag> {
    let mut out = Vec::new();
    // Skip the SOI marker (FF D8).
    let mut pos = 2usize;

    while pos + 4 <= buf.len() {
        // Markers may be preceded by fill bytes (0xFF); find the marker byte.
        if buf[pos] != 0xFF {
            pos += 1;
            continue;
        }
        let marker = buf[pos + 1];
        pos += 2;
        // Standalone markers without a length payload.
        if marker == 0xD8 || marker == 0xD9 || (0xD0..=0xD7).contains(&marker) {
            continue;
        }
        // Start of scan — image data follows, stop scanning metadata.
        if marker == 0xDA {
            break;
        }
        let seg_len = match be_u16(&buf[pos..]) {
            Some(l) if l >= 2 => l as usize,
            _ => break,
        };
        let data_start = pos + 2;
        let data_end = pos + seg_len;
        if data_end > buf.len() {
            break;
        }
        let seg = &buf[data_start..data_end];

        match marker {
            0xE1 => handle_app1(seg, &mut out),
            0xE0 => handle_app0(seg, &mut out),
            0xFE => {
                if let Ok(s) = std::str::from_utf8(seg) {
                    out.push(ExtractedTag::new(
                        "File",
                        "File",
                        "Comment",
                        Value::Text(s.trim_end_matches('\0').to_string()),
                        s.trim_end_matches('\0').to_string(),
                    ));
                }
            }
            // SOF0..SOF15 (excluding DHT=0xC4, JPG=0xC8, DAC=0xCC) carry frame info.
            0xC0..=0xCF if marker != 0xC4 && marker != 0xC8 && marker != 0xCC => {
                handle_sof(marker, seg, &mut out);
            }
            _ => {}
        }

        pos = data_end;
    }

    out
}

fn handle_app1(seg: &[u8], out: &mut Vec<ExtractedTag>) {
    // EXIF: "Exif\0\0" then a TIFF header.
    if seg.len() > 6 && &seg[0..6] == b"Exif\x00\x00" {
        let tiff = &seg[6..];
        // Report the byte order as a File pseudo-tag (ExifTool does this).
        if let Some(order) = tiff.get(0..2) {
            let label = match order {
                b"II" => Some("Little-endian (Intel, II)"),
                b"MM" => Some("Big-endian (Motorola, MM)"),
                _ => None,
            };
            if let Some(label) = label {
                out.push(ExtractedTag::new(
                    "File",
                    "File",
                    "ExifByteOrder",
                    Value::Text(label.to_string()),
                    label.to_string(),
                ));
            }
        }
        if let Ok(tags) = exif::parse_tiff(tiff) {
            out.extend(tags);
        }
    }
    // (XMP — "http://ns.adobe.com/xap/1.0/\0" — is recognised but not parsed yet.)
}

fn handle_app0(seg: &[u8], out: &mut Vec<ExtractedTag>) {
    // JFIF: "JFIF\0" then version major/minor.
    if seg.len() >= 7 && &seg[0..5] == b"JFIF\x00" {
        let major = seg[5];
        let minor = seg[6];
        let ver = format!("{}.{:02}", major, minor);
        out.push(ExtractedTag::new(
            "JFIF",
            "JFIF",
            "JFIFVersion",
            Value::U(vec![major as u64, minor as u64]),
            ver,
        ));
    }
}

fn handle_sof(marker: u8, seg: &[u8], out: &mut Vec<ExtractedTag>) {
    // SOF layout: [precision:1][height:2][width:2][components:1]...
    if seg.len() < 6 {
        return;
    }
    let order = ByteOrder::Big;
    let bits = seg[0] as u64;
    let height = order.u16([seg[1], seg[2]]) as u64;
    let width = order.u16([seg[3], seg[4]]) as u64;
    let comps = seg[5] as u64;

    let process = marker - 0xC0;
    let enc = encoding_process(process);

    out.push(ExtractedTag::new("File", "File", "ImageWidth", Value::U(vec![width]), width.to_string()));
    out.push(ExtractedTag::new("File", "File", "ImageHeight", Value::U(vec![height]), height.to_string()));
    out.push(ExtractedTag::new(
        "File",
        "File",
        "EncodingProcess",
        Value::U(vec![process as u64]),
        enc.to_string(),
    ));
    out.push(ExtractedTag::new("File", "File", "BitsPerSample", Value::U(vec![bits]), bits.to_string()));
    out.push(ExtractedTag::new(
        "File",
        "File",
        "ColorComponents",
        Value::U(vec![comps]),
        comps.to_string(),
    ));
}

fn encoding_process(p: u8) -> &'static str {
    match p {
        0x0 => "Baseline DCT, Huffman coding",
        0x1 => "Extended sequential DCT, Huffman coding",
        0x2 => "Progressive DCT, Huffman coding",
        0x3 => "Lossless, Huffman coding",
        0x5 => "Sequential DCT, differential Huffman coding",
        0x6 => "Progressive DCT, differential Huffman coding",
        0x7 => "Lossless, Differential Huffman coding",
        0x9 => "Extended sequential DCT, arithmetic coding",
        0xa => "Progressive DCT, arithmetic coding",
        0xb => "Lossless, arithmetic coding",
        0xd => "Sequential DCT, differential arithmetic coding",
        0xe => "Progressive DCT, differential arithmetic coding",
        0xf => "Lossless, differential arithmetic coding",
        _ => "Unknown",
    }
}
