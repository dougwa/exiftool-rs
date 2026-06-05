//! Format-specific parsers. Each takes the whole file buffer and returns the
//! metadata tags it can extract. Dispatch is by detected file type.

pub mod jpeg;
pub mod png;
pub mod tiff;

use crate::tag::ExtractedTag;

/// Run the parser appropriate for the given canonical file type. Returns None
/// when we have no metadata parser for that type (the caller still reports the
/// filesystem/File-type tags).
pub fn parse(typ: &str, buf: &[u8]) -> Option<Vec<ExtractedTag>> {
    match typ {
        "JPEG" => Some(jpeg::parse(buf)),
        "PNG" => Some(png::parse(buf)),
        // All TIFF-based types share the IFD engine.
        "TIFF" | "BigTIFF" | "CR2" | "NEF" | "NRW" | "ARW" | "DNG" | "ORF" | "RW2" | "PEF"
        | "SRW" => Some(tiff::parse(buf)),
        _ => None,
    }
}
