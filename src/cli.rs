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
use crate::{Edit, EditOp};

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
    let mut edits: Vec<Edit> = Vec::new();
    let mut overwrite_original = false;
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
        } else if a == "-overwrite_original" {
            overwrite_original = true;
        } else if let Some(tag) = a.strip_prefix('-') {
            if tag.is_empty() || !tag.chars().next().unwrap().is_ascii_alphabetic() {
                return Err(format!("unknown option: {a}"));
            }
            // `-TAG=VALUE` sets a tag; `-TAG=` deletes it; bare `-TAG` is a read filter.
            match tag.split_once('=') {
                Some((name, value)) => edits.push(Edit {
                    name: name.to_string(),
                    op: if value.is_empty() {
                        EditOp::Delete
                    } else {
                        EditOp::Set(value.to_string())
                    },
                }),
                None => o.filters.push(tag.to_string()),
            }
        } else {
            o.files.push(PathBuf::from(a));
        }
    }
    if o.files.is_empty() {
        return Err("no input files".to_string());
    }
    if !edits.is_empty() {
        return Ok(Action::Write { files: o.files, edits, overwrite_original });
    }
    Ok(Action::Run(o))
}

pub enum Action {
    Run(Options),
    Write { files: Vec<PathBuf>, edits: Vec<Edit>, overwrite_original: bool },
    Version,
    Help,
}

pub const HELP: &str = "\
exiftool-rs — read and write media metadata (a Rust subset of ExifTool)

USAGE:
    exiftool-rs [OPTIONS] FILE...           read metadata
    exiftool-rs -TAG=VALUE [...] FILE...    write metadata (JPEG)

OPTIONS:
    -ver                 print version number and exit
    -j, -json            output in JSON format
    -G, -G0              show family-0 group name for each tag
    -G1                  show family-1 group name for each tag
    -s                   short output: tag name instead of description
    -n                   numeric: disable human-readable conversions
    -a                   allow duplicate tag names
    -TAG                 extract only the named tag (e.g. -FNumber), repeatable
    -TAG=VALUE           set a tag (e.g. -Artist=\"Jane Doe\")
    -TAG=                delete a tag
    -overwrite_original  edit in place without writing a _original backup
    -h, --help           show this help
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

/// Maker-note tags that ExifTool trusts *over* the same-named EXIF tag (its
/// effective table/tag `Priority` is 1, not the 0 it assigns to the unreliable
/// EXIF-duplicate maker tags). Keyed by (family-1 group, tag name). Only entries
/// whose maker-note conversion we already render correctly are listed, so the
/// override never replaces a good EXIF value with a not-yet-converted maker one.
const TRUSTED_MAKER_OVERRIDES: &[(&str, &str)] = &[
    ("Olympus", "MeteringMode"), // EXIF "Multi-segment" -> Olympus "ESP"
    ("Nikon", "LightSource"),    // EXIF "Unknown" -> Nikon "Speedlight"/"Colored"
    ("Nikon", "WhiteBalance"),   // EXIF "Manual" -> Nikon "Preset0"
];

/// ExifTool's tag priority, reduced to the two levels our data exercises.
///
/// ExifTool keeps the highest-priority tag for a name; on a tie the *later*
/// extraction wins, so since EXIF is parsed before maker notes a priority-1
/// maker tag overrides the EXIF one. Most EXIF-duplicate maker tags carry
/// `Priority => 0` (ExifTool deems them unreliable) and so never displace EXIF;
/// we model that by giving maker-note tags priority 0 *unless* they are a
/// trusted override. Thumbnail `IFD1` / `PreviewIFD` tags are also priority 0,
/// so a main-image tag always wins over its thumbnail copy.
fn priority(t: &ExtractedTag) -> i32 {
    if t.group0 == "MakerNotes" {
        let trusted = TRUSTED_MAKER_OVERRIDES
            .iter()
            .any(|(g, n)| *g == t.group1 && n.eq_ignore_ascii_case(&t.name));
        return if trusted { 1 } else { 0 };
    }
    match t.group1.as_str() {
        "IFD1" | "PreviewIFD" => 0,
        _ => 1,
    }
}

/// Apply duplicate-suppression and tag filters to the extracted list.
fn select<'a>(tags: &'a [ExtractedTag], o: &Options) -> Vec<&'a ExtractedTag> {
    let passes = |t: &ExtractedTag| {
        o.filters.is_empty() || o.filters.iter().any(|f| f.eq_ignore_ascii_case(&t.name))
    };
    if o.allow_dup {
        // -a: keep every (filtered) tag, in extraction order.
        return tags.iter().filter(|t| passes(t)).collect();
    }
    // Resolve duplicates by ExifTool's rule, which (with priorities of 0/1 and
    // EXIF parsed before maker notes) reduces to: keep the last priority-1
    // occurrence of a name, or the first occurrence if none is priority 1.
    let mut winners: Vec<(String, usize)> = Vec::new(); // (lowercase name, winning index)
    for (i, t) in tags.iter().enumerate() {
        if !passes(t) {
            continue;
        }
        let key = t.name.to_ascii_lowercase();
        match winners.iter_mut().find(|(n, _)| *n == key) {
            // A later tag only displaces an earlier one if it is priority 1.
            Some(w) => {
                if priority(t) == 1 {
                    w.1 = i;
                }
            }
            None => winners.push((key, i)),
        }
    }
    // Emit the surviving tags in their original extraction order.
    let mut keep: Vec<usize> = winners.iter().map(|w| w.1).collect();
    keep.sort_unstable();
    keep.into_iter().map(|i| &tags[i]).collect()
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

/// Apply `edits` to each file, printing ExifTool-style status lines. Returns
/// true if any file failed.
pub fn run_write(files: &[PathBuf], edits: &[Edit], overwrite_original: bool) -> bool {
    let opts = crate::WriteOptions { overwrite_original };
    let mut updated = 0u32;
    let mut had_error = false;
    for path in files {
        match crate::write_to_path(path, edits, opts) {
            Ok(()) => updated += 1,
            Err(e) => {
                had_error = true;
                eprintln!("Error: {} - {}", path.display(), e);
            }
        }
    }
    if updated > 0 {
        println!("    {updated} image files updated");
    }
    had_error
}
