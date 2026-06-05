//! PNG chunk parser.
//!
//! PNG is a signature followed by typed chunks: [len:4 BE][type:4][data][crc:4].
//! We read IHDR (dimensions/colour info), the eXIf chunk (a raw TIFF/EXIF block),
//! and tEXt/iTXt textual metadata. Mirrors ExifTool's PNG.pm at a basic level.

use crate::exif;
use crate::reader::{be_u32, ByteOrder};
use crate::tag::ExtractedTag;
use crate::value::Value;

pub fn parse(buf: &[u8]) -> Vec<ExtractedTag> {
    let mut out = Vec::new();
    let mut pos = 8; // skip the 8-byte signature

    while pos + 8 <= buf.len() {
        let len = match be_u32(&buf[pos..]) {
            Some(l) => l as usize,
            None => break,
        };
        let ctype = &buf[pos + 4..pos + 8];
        let data_start = pos + 8;
        let data_end = match data_start.checked_add(len) {
            Some(e) if e <= buf.len() => e,
            _ => break,
        };
        let data = &buf[data_start..data_end];

        match ctype {
            b"IHDR" => handle_ihdr(data, &mut out),
            b"eXIf" => {
                if let Ok(tags) = exif::parse_tiff(data) {
                    out.extend(tags);
                }
            }
            b"tEXt" => handle_text(data, &mut out),
            b"IEND" => break,
            _ => {}
        }

        pos = data_end + 4; // skip CRC
    }

    out
}

fn handle_ihdr(data: &[u8], out: &mut Vec<ExtractedTag>) {
    if data.len() < 13 {
        return;
    }
    let order = ByteOrder::Big;
    let width = order.u32([data[0], data[1], data[2], data[3]]) as u64;
    let height = order.u32([data[4], data[5], data[6], data[7]]) as u64;
    let bit_depth = data[8] as u64;
    let color_type = data[9];

    let ct = match color_type {
        0 => "Grayscale",
        2 => "RGB",
        3 => "Palette",
        4 => "Grayscale with Alpha",
        6 => "RGB with Alpha",
        _ => "Unknown",
    };

    out.push(ExtractedTag::new("PNG", "PNG", "ImageWidth", Value::U(vec![width]), width.to_string()));
    out.push(ExtractedTag::new("PNG", "PNG", "ImageHeight", Value::U(vec![height]), height.to_string()));
    out.push(ExtractedTag::new(
        "PNG",
        "PNG",
        "BitDepth",
        Value::U(vec![bit_depth]),
        bit_depth.to_string(),
    ));
    out.push(ExtractedTag::new(
        "PNG",
        "PNG",
        "ColorType",
        Value::U(vec![color_type as u64]),
        ct.to_string(),
    ));
}

fn handle_text(data: &[u8], out: &mut Vec<ExtractedTag>) {
    // keyword\0text
    if let Some(nul) = data.iter().position(|&b| b == 0) {
        let keyword = String::from_utf8_lossy(&data[..nul]).into_owned();
        let text = String::from_utf8_lossy(&data[nul + 1..]).into_owned();
        // Use the keyword as the tag name (sanitised), preserving the text.
        let name: String = keyword.chars().filter(|c| !c.is_whitespace()).collect();
        out.push(ExtractedTag::new("PNG", "PNG", name, Value::Text(text.clone()), text));
    }
}
