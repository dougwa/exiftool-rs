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

#[test]
fn canon_maker_notes() {
    let m = tags("Canon.jpg");
    // ProcessBinaryData (CameraSettings) tags.
    assert_tag(&m, "MacroMode", "Unknown (0)");
    assert_tag(&m, "Quality", "RAW");
    assert_tag(&m, "ContinuousDrive", "Continuous");
    assert_tag(&m, "CanonExposureMode", "Manual");
    // Canon APEX aperture conversion (CanonEv).
    assert_tag(&m, "MaxAperture", "4");
    assert_tag(&m, "MinAperture", "27");
    // Signed printParameter and SelfTimer formula.
    assert_tag(&m, "Contrast", "+1");
    assert_tag(&m, "SelfTimer", "Off");
}

#[test]
fn nikon_maker_notes() {
    let m = tags("Nikon.jpg");
    // Nikon Type 3 (embedded TIFF) with the FormatString default PrintConv.
    assert_tag(&m, "Quality", "Fine");
    assert_tag(&m, "WhiteBalance", "Auto");
    assert_tag(&m, "ColorMode", "Color");
    assert_tag(&m, "MakerNoteVersion", "1.00");
}

#[test]
fn vendor_maker_notes() {
    // Signature-dispatched vendor IFDs (Olympus host-base, Olympus2 mn-relative,
    // FujiFilm pointer+mn-relative, Pentax "AOC\0", signatureless Casio/Minolta).
    let oly = tags("Olympus.jpg");
    assert_tag(&oly, "Macro", "Off");
    assert_tag(&oly, "FocalPlaneDiagonal", "7.8 mm"); // PrintConv "$val mm"
    assert_tag(&oly, "SpecialMode", "Normal, Sequence: 0, Panorama: (none)");

    let pana = tags("Panasonic.jpg");
    assert_tag(&pana, "FirmwareVersion", "0.1.0.8"); // undef bytes -> dotted

    let fuji = tags("FujiFilm.jpg");
    assert_tag(&fuji, "Version", "0130"); // printable undef rendered verbatim

    let sigma = tags("Sigma.jpg");
    assert_tag(&sigma, "Quality", "12"); // "Qual:12" label stripped

    let casio = tags("Casio.jpg");
    assert_tag(&casio, "ObjectDistance", "2.5 m"); // mm/1000 + " m"

    let pentax = tags("Pentax.jpg");
    assert_tag(&pentax, "PentaxVersion", "3.0.0.0"); // tr/ /./
    assert_tag(&pentax, "PreviewImageSize", "640x480"); // tr/ /x/

    let minolta = tags("Minolta.jpg");
    assert_tag(&minolta, "MakerNoteVersion", "MLT0");
}

#[test]
fn canon_binary_record_conversions() {
    let m = tags("Canon.jpg");
    // CanonModelID lookup, ShotInfo ValueConv formulas, RawConv n/a suppression.
    assert_tag(&m, "CanonModelID", "EOS Digital Rebel / 300D / Kiss Digital");
    assert_tag(&m, "BaseISO", "100");
    assert_tag(&m, "MeasuredEV", "-1.25");
    assert_tag(&m, "OpticalZoomCode", "n/a");
    // AESetting has RawConv '$val==-1 ? undef' and the value is -1: suppressed.
    assert!(!m.contains_key("AESetting"), "AESetting (-1) should be suppressed");
}
