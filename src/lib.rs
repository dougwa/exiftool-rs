//! exiftool-rs — a Rust reimplementation of the core of ExifTool.
//!
//! This crate reads media-file metadata. Its architecture mirrors ExifTool:
//!
//! * [`filetype`] identifies the file from magic numbers / extension.
//! * [`formats`] holds per-container parsers (JPEG, TIFF, PNG) that locate the
//!   metadata blocks.
//! * [`exif`] is the shared TIFF/EXIF IFD engine used by all of them, with the
//!   faithful tag tables generated from ExifTool's Perl source.
//! * [`file_meta`] adds the filesystem ("File") pseudo-tags.
//!
//! The output unit is [`tag::ExtractedTag`], carrying both a raw value and the
//! PrintConv (human-readable) rendering.

pub mod cli;
pub mod composite;
pub mod datetime;
pub mod error;
pub mod exif;
pub mod file_meta;
pub mod filetype;
pub mod formats;
pub mod makernotes;
pub mod reader;
pub mod tag;
pub mod value;

use std::path::Path;

use error::{Error, Result};
use tag::ExtractedTag;
use value::Value;

pub use exif::{Edit, EditOp};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Read a file and extract all metadata tags we can, in roughly ExifTool's order.
pub fn extract_from_path(path: &Path) -> Result<Vec<ExtractedTag>> {
    let data = std::fs::read(path)?;
    let meta = std::fs::metadata(path)?;

    let ext = path.extension().and_then(|e| e.to_str());
    let ft = filetype::identify(&data, ext);

    let mut tags = Vec::new();
    // 1. Tool version pseudo-tag (mirrors "ExifTool Version Number").
    tags.push(ExtractedTag::new(
        "ExifTool",
        "ExifTool",
        "ExifToolVersion",
        Value::Text(VERSION.to_string()),
        VERSION.to_string(),
    ));

    // 2. Filesystem + file-type tags.
    tags.extend(file_meta::file_tags(path, &meta, ft));

    // 3. Format-specific metadata (EXIF/IFD, JFIF, PNG chunks...).
    if let Some(ft) = ft {
        if let Some(format_tags) = formats::parse(ft.typ, &data) {
            tags.extend(format_tags);
        }
    }

    // 4. Composite tags computed from the extracted set (ExifTool's Composite
    //    group): Aperture, ShutterSpeed, ImageSize, FOV, DOF, etc.
    composite::compute(&mut tags);

    Ok(tags)
}

/// Options governing how a write is saved.
#[derive(Default, Clone, Copy)]
pub struct WriteOptions {
    /// Overwrite the original file in place. When false (ExifTool's default),
    /// the untouched original is first preserved as `FILE_original`.
    pub overwrite_original: bool,
}

/// Apply `edits` to the metadata of the file at `path`, saving the result.
///
/// Only JPEG is supported today. Unless `overwrite_original` is set, the
/// original bytes are copied to `<path>_original` (never clobbering an existing
/// backup) before the new file is written.
pub fn write_to_path(path: &Path, edits: &[Edit], opts: WriteOptions) -> Result<()> {
    let data = std::fs::read(path)?;
    let ext = path.extension().and_then(|e| e.to_str());
    let ft = filetype::identify(&data, ext);

    let new_data = match ft.map(|f| f.typ) {
        Some("JPEG") => formats::jpeg::write(&data, edits)?,
        Some(other) => {
            return Err(Error::Unsupported(format!("writing not supported for {other} files")))
        }
        None => return Err(Error::Unsupported("unrecognised file type".into())),
    };

    if !opts.overwrite_original {
        let mut backup = path.as_os_str().to_owned();
        backup.push("_original");
        let backup = std::path::PathBuf::from(backup);
        if !backup.exists() {
            std::fs::write(&backup, &data)?;
        }
    }
    std::fs::write(path, &new_data)?;
    Ok(())
}
