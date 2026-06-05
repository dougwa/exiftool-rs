//! Filesystem pseudo-tags (the "File" group): name, directory, size, timestamps,
//! permissions, and the detected file type / MIME. These mirror the System and
//! File group tags ExifTool emits for every file.

use std::fs::Metadata;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

use crate::datetime::format_local;
use crate::filetype::FileType;
use crate::tag::ExtractedTag;
use crate::value::Value;

fn text_tag(group1: &str, name: &str, s: String) -> ExtractedTag {
    ExtractedTag::new("File", group1, name, Value::Text(s.clone()), s)
}

/// ExifTool's ConvertFileSize (decimal units: bytes, kB, MB, GB).
pub fn convert_file_size(val: u64) -> String {
    let v = val as f64;
    if val < 2000 {
        format!("{} bytes", val)
    } else if val < 10_000 {
        format!("{:.1} kB", v / 1000.0)
    } else if val < 2_000_000 {
        format!("{:.0} kB", v / 1000.0)
    } else if val < 10_000_000 {
        format!("{:.1} MB", v / 1_000_000.0)
    } else if val < 2_000_000_000 {
        format!("{:.0} MB", v / 1_000_000.0)
    } else if val < 10_000_000_000 {
        format!("{:.1} GB", v / 1_000_000_000.0)
    } else {
        format!("{:.0} GB", v / 1_000_000_000.0)
    }
}

/// Render a Unix mode as a permission string like `-rw-r--r--`.
pub fn permission_string(mode: u32) -> String {
    let type_char = match mode & 0o170000 {
        0o040000 => 'd',
        0o120000 => 'l',
        0o060000 => 'b',
        0o020000 => 'c',
        0o010000 => 'p',
        0o140000 => 's',
        _ => '-',
    };
    let mut s = String::new();
    s.push(type_char);
    let bits = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];
    for (mask, ch) in bits {
        s.push(if mode & mask != 0 { ch } else { '-' });
    }
    s
}

/// Build the File-group tags for `path` given its metadata and detected type.
pub fn file_tags(path: &Path, meta: &Metadata, ft: Option<FileType>) -> Vec<ExtractedTag> {
    let mut out = Vec::new();

    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let dir = path
        .parent()
        .map(|p| {
            let s = p.to_string_lossy();
            if s.is_empty() { ".".to_string() } else { s.into_owned() }
        })
        .unwrap_or_else(|| ".".to_string());

    out.push(text_tag("System", "FileName", file_name));
    out.push(text_tag("System", "Directory", dir));

    let size = meta.len();
    out.push(ExtractedTag::new(
        "File",
        "System",
        "FileSize",
        Value::U(vec![size]),
        convert_file_size(size),
    ));

    if let Ok(mtime) = meta.modified() {
        out.push(text_tag("System", "FileModifyDate", format_local(mtime)));
    }
    if let Ok(atime) = meta.accessed() {
        out.push(text_tag("System", "FileAccessDate", format_local(atime)));
    }
    // Inode change time (ctime) — from raw stat fields.
    let ctime = std::time::UNIX_EPOCH + std::time::Duration::new(meta.ctime() as u64, 0);
    out.push(text_tag("System", "FileInodeChangeDate", format_local(ctime)));

    out.push(text_tag(
        "System",
        "FilePermissions",
        permission_string(meta.permissions().mode()),
    ));

    if let Some(ft) = ft {
        out.push(text_tag("File", "FileType", ft.typ.to_string()));
        out.push(text_tag("File", "FileTypeExtension", ft.ext.to_string()));
        out.push(text_tag("File", "MIMEType", ft.mime.to_string()));
    }

    out
}
