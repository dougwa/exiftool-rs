//! Write-path integration tests: round-trip set/delete, faithful preservation of
//! maker notes & thumbnails across a rewrite, and GPS auto-reference handling.
//!
//! Each test copies a sample from the adjacent ExifTool checkout into a unique
//! temp file so the originals are never touched. Tests no-op when the sample is
//! absent (mirroring `integration.rs`).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use exiftool_rs::{extract_from_path, write_to_path, Edit, EditOp, WriteOptions};

fn sample(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../exiftool/t/images").join(name)
}

/// Copy `name` to a unique temp path; returns None if the sample is missing.
fn temp_copy(name: &str, tag: &str) -> Option<PathBuf> {
    let src = sample(name);
    if !src.exists() {
        eprintln!("skipping: sample {name} not found");
        return None;
    }
    let dst = std::env::temp_dir().join(format!(
        "exiftool-rs-{}-{}-{}",
        std::process::id(),
        tag,
        name
    ));
    std::fs::copy(&src, &dst).expect("copy sample");
    Some(dst)
}

fn set(name: &str, value: &str) -> Edit {
    Edit { name: name.into(), op: EditOp::Set(value.into()) }
}

const OVERWRITE: WriteOptions = WriteOptions { overwrite_original: true };

/// Map of tag name -> print value for a file.
fn read_map(path: &Path) -> HashMap<String, String> {
    let mut m = HashMap::new();
    for t in extract_from_path(path).expect("extract") {
        m.entry(t.name).or_insert(t.print);
    }
    m
}

/// Map restricted to a family-0 group (e.g. "MakerNotes"), keyed `group1:name`.
fn read_group(path: &Path, group0: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    for t in extract_from_path(path).expect("extract") {
        if t.group0 == group0 {
            m.insert(format!("{}:{}", t.group1, t.name), t.print);
        }
    }
    m
}

#[test]
fn round_trip_set_string_and_date() {
    let Some(p) = temp_copy("Canon.jpg", "set") else { return };
    write_to_path(
        &p,
        &[set("Artist", "Jane Doe"), set("DateTimeOriginal", "2026:06:07 12:34:56")],
        OVERWRITE,
    )
    .expect("write");
    let m = read_map(&p);
    assert_eq!(m.get("Artist").map(String::as_str), Some("Jane Doe"));
    assert_eq!(m.get("DateTimeOriginal").map(String::as_str), Some("2026:06:07 12:34:56"));
}

#[test]
fn round_trip_numeric_conversions() {
    let Some(p) = temp_copy("Canon.jpg", "num") else { return };
    write_to_path(
        &p,
        &[set("FNumber", "2.8"), set("ExposureTime", "1/250"), set("ISO", "400")],
        OVERWRITE,
    )
    .expect("write");
    let m = read_map(&p);
    assert_eq!(m.get("FNumber").map(String::as_str), Some("2.8"));
    assert_eq!(m.get("ExposureTime").map(String::as_str), Some("1/250"));
    assert_eq!(m.get("ISO").map(String::as_str), Some("400"));
}

#[test]
fn delete_removes_tag() {
    let Some(p) = temp_copy("Canon.jpg", "del") else { return };
    write_to_path(&p, &[set("Artist", "Temp")], OVERWRITE).expect("write");
    assert!(read_map(&p).contains_key("Artist"));
    write_to_path(&p, &[Edit { name: "Artist".into(), op: EditOp::Delete }], OVERWRITE)
        .expect("delete");
    assert!(!read_map(&p).contains_key("Artist"), "Artist should be gone");
}

#[test]
fn gps_sets_coordinates_and_auto_reference() {
    let Some(p) = temp_copy("Canon.jpg", "gps") else { return };
    write_to_path(
        &p,
        &[set("GPSLatitude", "37.7749"), set("GPSLongitude", "-122.4194"), set("GPSAltitude", "100.5")],
        OVERWRITE,
    )
    .expect("write");
    let m = read_map(&p);
    // Sign of the input determines the reference automatically.
    assert_eq!(m.get("GPSLatitudeRef").map(String::as_str), Some("North"));
    assert_eq!(m.get("GPSLongitudeRef").map(String::as_str), Some("West"));
    assert_eq!(m.get("GPSLatitude").map(String::as_str), Some("37 deg 46' 29.64\" N"));
    assert_eq!(m.get("GPSLongitude").map(String::as_str), Some("122 deg 25' 9.84\" W"));
    assert_eq!(m.get("GPSAltitude").map(String::as_str), Some("100.5 m"));
}

/// Writing an IFD0 tag must leave every maker-note tag byte-identical (the
/// faithful-rebuild + offset-fixup guarantee), across vendors with different
/// offset schemes (Canon = TIFF-relative, Nikon = embedded TIFF).
#[test]
fn maker_notes_preserved_across_rewrite() {
    // OlympusE1 exercises nested sub-IFD offset shifting (CameraSettings etc.).
    for name in ["Canon.jpg", "NikonD70.jpg", "Olympus2.jpg", "OlympusE1.jpg", "Sanyo.jpg"] {
        let Some(p) = temp_copy(name, "mn") else { continue };
        let before = read_group(&p, "MakerNotes");
        if before.is_empty() {
            continue; // no maker notes parsed for this sample
        }
        write_to_path(&p, &[set("Artist", "Rewrite")], OVERWRITE).expect("write");
        let after = read_group(&p, "MakerNotes");
        assert_eq!(before, after, "maker notes changed for {name}");
    }
}

/// Re-editing a file we already wrote must stay stable: our serializer output
/// must round-trip back through our parser, preserving maker notes each time.
#[test]
fn re_editing_own_output_is_stable() {
    let Some(p) = temp_copy("Nikon.jpg", "reedit") else { return };
    let maker_before = read_group(&p, "MakerNotes");
    write_to_path(&p, &[set("Artist", "First")], OVERWRITE).expect("write 1");
    write_to_path(&p, &[set("Copyright", "2026 Me"), set("ISO", "800")], OVERWRITE).expect("write 2");
    write_to_path(&p, &[Edit { name: "Artist".into(), op: EditOp::Delete }], OVERWRITE).expect("write 3");

    let m = read_map(&p);
    assert!(!m.contains_key("Artist"));
    assert_eq!(m.get("Copyright").map(String::as_str), Some("2026 Me"));
    assert_eq!(m.get("ISO").map(String::as_str), Some("800"));
    if !maker_before.is_empty() {
        assert_eq!(maker_before, read_group(&p, "MakerNotes"), "maker notes drifted across re-edits");
    }
}

/// The IFD1 thumbnail image must survive a rewrite intact.
#[test]
fn thumbnail_preserved_across_rewrite() {
    // FLIR.jpg carries a real (2.3 KB) embedded thumbnail.
    let Some(p) = temp_copy("FLIR.jpg", "thumb") else { return };
    let before = read_map(&p);
    let Some(len_before) = before.get("ThumbnailLength").cloned() else { return };
    write_to_path(&p, &[set("Artist", "Rewrite")], OVERWRITE).expect("write");
    let after = read_map(&p);
    assert_eq!(after.get("ThumbnailLength"), Some(&len_before), "thumbnail length changed");
}

/// Creating EXIF from scratch in a JPEG that has none.
#[test]
fn creates_exif_when_absent() {
    let Some(p) = temp_copy("Writer.jpg", "create") else { return };
    // Precondition: no EXIF Artist to begin with.
    assert!(!read_map(&p).contains_key("Artist"));
    write_to_path(&p, &[set("Artist", "Fresh")], OVERWRITE).expect("write");
    assert_eq!(read_map(&p).get("Artist").map(String::as_str), Some("Fresh"));
}

/// Without `overwrite_original`, the untouched original is preserved as
/// `<file>_original`.
#[test]
fn writes_backup_by_default() {
    let Some(p) = temp_copy("Canon.jpg", "backup") else { return };
    let original = std::fs::read(&p).unwrap();
    write_to_path(&p, &[set("Artist", "Backed Up")], WriteOptions::default()).expect("write");
    let backup = PathBuf::from(format!("{}_original", p.display()));
    assert!(backup.exists(), "backup file should exist");
    assert_eq!(std::fs::read(&backup).unwrap(), original, "backup must match the original bytes");
    let _ = std::fs::remove_file(&backup);
}
