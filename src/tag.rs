//! The unit of output: one extracted metadata tag.

use crate::value::Value;

#[derive(Clone, Debug)]
pub struct ExtractedTag {
    /// Family-0 group (information type): "File", "EXIF", "JFIF", "PNG"...
    pub group0: String,
    /// Family-1 group (specific location): "IFD0", "ExifIFD", "GPS", "System"...
    pub group1: String,
    /// Canonical tag name (no spaces), e.g. "FNumber".
    pub name: String,
    /// Raw value.
    pub value: Value,
    /// Human-readable (PrintConv) rendering.
    pub print: String,
}

impl ExtractedTag {
    pub fn new(
        group0: &str,
        group1: &str,
        name: impl Into<String>,
        value: Value,
        print: String,
    ) -> Self {
        ExtractedTag {
            group0: group0.to_string(),
            group1: group1.to_string(),
            name: name.into(),
            value,
            print,
        }
    }
}
