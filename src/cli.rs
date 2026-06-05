//! Command-line interface and output formatting.
//!
//! Supports a useful subset of ExifTool's options:
//!   FILE...            files to read
//!   -ver               print version and exit
//!   -j / -json         JSON output
//!   -G / -G0 / -G1     prefix each line with the family-0 / family-1 group
//!   -s                 short output (tag name instead of description)
//!   -n                 numeric: disable PrintConv (show raw values)
//!   -a                 allow duplicate tag names
//!   -TAG               only output the named tag(s) (repeatable)
//!   -h / --help        usage

use std::path::{Path, PathBuf};

use crate::exif::tags::make_description;
use crate::tag::ExtractedTag;

#[derive(Default)]
pub struct Options {
    pub json: bool,
    pub group: Option<u8>, // Some(0) or Some(1)
    pub short: bool,
    pub numeric: bool,
    pub allow_dup: bool,
    pub filters: Vec<String>, // tag names to keep (case-insensitive); empty = all
    pub files: Vec<PathBuf>,
}

pub fn parse_args(args: &[String]) -> std::result::Result<Action, String> {
    let mut o = Options::default();
    for a in args {
        if a == "-ver" {
            return Ok(Action::Version);
        } else if a == "-h" || a == "--help" {
            return Ok(Action::Help);
        } else if a == "-j" || a == "-json" {
            o.json = true;
        } else if a == "-G" || a == "-G0" {
            o.group = Some(0);
        } else if a == "-G1" {
            o.group = Some(1);
        } else if a == "-s" {
            o.short = true;
        } else if a == "-n" {
            o.numeric = true;
        } else if a == "-a" {
            o.allow_dup = true;
        } else if let Some(tag) = a.strip_prefix('-') {
            // Treat any other -Word as a tag filter (e.g. -FNumber).
            if !tag.is_empty() && tag.chars().next().unwrap().is_ascii_alphabetic() {
                o.filters.push(tag.to_string());
            } else {
                return Err(format!("unknown option: {a}"));
            }
        } else {
            o.files.push(PathBuf::from(a));
        }
    }
    if o.files.is_empty() {
        return Err("no input files".to_string());
    }
    Ok(Action::Run(o))
}

pub enum Action {
    Run(Options),
    Version,
    Help,
}

pub const HELP: &str = "\
exiftool-rs — read media metadata (a Rust subset of ExifTool)

USAGE:
    exiftool-rs [OPTIONS] FILE...

OPTIONS:
    -ver           print version number and exit
    -j, -json      output in JSON format
    -G, -G0        show family-0 group name for each tag
    -G1            show family-1 group name for each tag
    -s             short output: tag name instead of description
    -n             numeric: disable human-readable conversions
    -a             allow duplicate tag names
    -TAG           extract only the named tag (e.g. -FNumber), repeatable
    -h, --help     show this help
";

/// Description overrides that don't follow the name-spacing algorithm.
fn description(name: &str) -> String {
    match name {
        "ExifToolVersion" => "ExifTool Version Number",
        "FileModifyDate" => "File Modification Date/Time",
        "FileAccessDate" => "File Access Date/Time",
        "FileInodeChangeDate" => "File Inode Change Date/Time",
        "DateTimeOriginal" => "Date/Time Original",
        "GPSDateStamp" => "GPS Date Stamp",
        _ => return make_description(name),
    }
    .to_string()
}

/// Apply duplicate-suppression and tag filters to the extracted list.
fn select<'a>(tags: &'a [ExtractedTag], o: &Options) -> Vec<&'a ExtractedTag> {
    let mut seen: Vec<&str> = Vec::new();
    let mut out = Vec::new();
    for t in tags {
        if !o.filters.is_empty()
            && !o.filters.iter().any(|f| f.eq_ignore_ascii_case(&t.name))
        {
            continue;
        }
        if !o.allow_dup && seen.iter().any(|n| n.eq_ignore_ascii_case(&t.name)) {
            continue;
        }
        seen.push(&t.name);
        out.push(t);
    }
    out
}

fn value_string(t: &ExtractedTag, o: &Options) -> String {
    if o.numeric {
        t.value.to_string()
    } else {
        t.print.clone()
    }
}

fn label(t: &ExtractedTag, o: &Options) -> String {
    let base = if o.short {
        t.name.clone()
    } else {
        description(&t.name)
    };
    match o.group {
        Some(0) => format!("{}:{}", t.group0, base),
        Some(1) => format!("{}:{}", t.group1, base),
        _ => base,
    }
}

/// Render the aligned, human-readable output for one file.
pub fn print_human(tags: &[ExtractedTag], o: &Options) {
    for t in select(tags, o) {
        let lbl = label(t, o);
        // ExifTool pads the label to 32 columns, then ": ".
        println!("{:<32}: {}", lbl, value_string(t, o));
    }
}

/// Minimal JSON serialiser (no external deps). Emits one object per file.
pub fn print_json(files: &[(PathBuf, Vec<ExtractedTag>)], o: &Options) {
    let mut out = String::from("[");
    for (i, (path, tags)) in files.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\n");
        out.push_str(&format!("  \"SourceFile\": {},\n", json_str(&path.to_string_lossy())));
        let selected = select(tags, o);
        for (j, t) in selected.iter().enumerate() {
            let key = match o.group {
                Some(0) => format!("{}:{}", t.group0, t.name),
                Some(1) => format!("{}:{}", t.group1, t.name),
                _ => t.name.clone(),
            };
            let val = value_string(t, o);
            let comma = if j + 1 < selected.len() { "," } else { "" };
            out.push_str(&format!("  {}: {}{}\n", json_str(&key), json_value(&val, o.numeric), comma));
        }
        out.push('}');
    }
    out.push(']');
    println!("{out}");
}

/// Emit a JSON value: an unquoted number when it parses cleanly (and -n was
/// given), otherwise a quoted string — matching ExifTool's -j behaviour closely.
fn json_value(s: &str, numeric: bool) -> String {
    if numeric {
        if let Ok(i) = s.parse::<i64>() {
            return i.to_string();
        }
        if let Ok(f) = s.parse::<f64>() {
            if f.is_finite() {
                return crate::value::fmt_float(f);
            }
        }
    }
    json_str(s)
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Print a header line before a file's tags when multiple files are given
/// (ExifTool prints `======== FILE`).
pub fn print_file_header(path: &Path) {
    println!("======== {}", path.display());
}
