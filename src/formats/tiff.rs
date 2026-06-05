//! TIFF (and TIFF-based RAW) entry point. The whole file *is* the TIFF buffer,
//! so we hand it straight to the EXIF/IFD engine.

use crate::exif;
use crate::tag::ExtractedTag;
use crate::value::Value;

pub fn parse(buf: &[u8]) -> Vec<ExtractedTag> {
    let mut out = Vec::new();
    // Byte-order pseudo tag, like ExifTool reports for TIFF files.
    if let Some(order) = buf.get(0..2) {
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
    if let Ok(tags) = exif::parse_tiff(buf) {
        out.extend(tags);
    }
    out
}
