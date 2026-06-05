//! Integration tests that read the ExifTool project's own sample images and
//! assert known tag values, locking in parity with the reference tool.

use std::collections::HashMap;
use std::path::PathBuf;

use exiftool_rs::extract_from_path;

/// Path to a sample image in the adjacent exiftool checkout.
fn sample(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../exiftool/t/images")
        .join(name)
}

/// Extract a file into a name -> print-value map (first occurrence wins, as in
/// ExifTool's default duplicate suppression).
fn tags(name: &str) -> HashMap<String, String> {
    let path = sample(name);
    if !path.exists() {
        eprintln!("skipping: sample {name} not found");
        return HashMap::new();
    }
    let mut map = HashMap::new();
    for t in extract_from_path(&path).expect("extract") {
        map.entry(t.name).or_insert(t.print);
    }
    map
}

fn assert_tag(map: &HashMap<String, String>, name: &str, expected: &str) {
    if map.is_empty() {
        return; // sample missing; test is a no-op in that environment
    }
    assert_eq!(
        map.get(name).map(String::as_str),
        Some(expected),
        "tag {name}"
    );
}

#[test]
fn exiftool_jpg_core_tags() {
    let m = tags("ExifTool.jpg");
    assert_tag(&m, "FileType", "JPEG");
    assert_tag(&m, "MIMEType", "image/jpeg");
    assert_tag(&m, "ImageDescription", "A witty caption");
    assert_tag(&m, "Orientation", "Horizontal (normal)");
    assert_tag(&m, "Software", "Adobe Photoshop 7.0");
    assert_tag(&m, "FNumber", "3.5");
    assert_tag(&m, "ExposureProgram", "Program AE");
    assert_tag(&m, "ISO", "100");
    assert_tag(&m, "ColorSpace", "sRGB");
    assert_tag(&m, "ComponentsConfiguration", "Y, Cb, Cr, -");
    assert_tag(&m, "FocalLength", "6.0 mm");
    assert_tag(&m, "Flash", "Fired");
}

#[test]
fn gps_jpg_coordinates() {
    let m = tags("GPS.jpg");
    assert_tag(&m, "GPSLatitude", "54 deg 59' 22.80\" N");
    assert_tag(&m, "GPSLongitude", "1 deg 54' 51.00\" W");
    assert_tag(&m, "GPSLatitudeRef", "North");
    assert_tag(&m, "GPSLongitudeRef", "West");
    assert_tag(&m, "ExposureProgram", "Shutter speed priority AE");
}

#[test]
fn canon_apex_shutter_speed_guard() {
    // APEX shutter speed that decodes to an out-of-range value must print "0".
    let m = tags("Canon.jpg");
    assert_tag(&m, "ShutterSpeedValue", "0");
}

#[test]
fn rational_precision_matches_roundfloat() {
    let m = tags("GPS.jpg");
    // ExifTool rounds rationals to 10 significant figures (RoundFloat).
    assert_tag(&m, "BrightnessValue", "0.26015625");
}
