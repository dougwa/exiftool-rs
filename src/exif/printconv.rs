//! PrintConv: convert raw tag values into the human-readable strings ExifTool
//! displays by default. These are ported from the PrintConv hashes / helper subs
//! in Exif.pm and GPS.pm. Only a curated set of common tags is covered; anything
//! without a specific conversion falls back to the raw value rendering.

use crate::value::{fmt_float, Value};

/// Return the enumerated PrintConv table for a tag name, if one exists.
fn enum_table(name: &str) -> Option<&'static [(i64, &'static str)]> {
    let table: &[(i64, &str)] = match name {
        "Orientation" => &[
            (1, "Horizontal (normal)"),
            (2, "Mirror horizontal"),
            (3, "Rotate 180"),
            (4, "Mirror vertical"),
            (5, "Mirror horizontal and rotate 270 CW"),
            (6, "Rotate 90 CW"),
            (7, "Mirror horizontal and rotate 90 CW"),
            (8, "Rotate 270 CW"),
        ],
        "ResolutionUnit" | "FocalPlaneResolutionUnit" => &[(1, "None"), (2, "inches"), (3, "cm")],
        "YCbCrPositioning" => &[(1, "Centered"), (2, "Co-sited")],
        "ExposureProgram" => &[
            (0, "Not Defined"),
            (1, "Manual"),
            (2, "Program AE"),
            (3, "Aperture-priority AE"),
            (4, "Shutter speed priority AE"),
            (5, "Creative (Slow speed)"),
            (6, "Action (High speed)"),
            (7, "Portrait"),
            (8, "Landscape"),
            (9, "Bulb"),
        ],
        "MeteringMode" => &[
            (0, "Unknown"),
            (1, "Average"),
            (2, "Center-weighted average"),
            (3, "Spot"),
            (4, "Multi-spot"),
            (5, "Multi-segment"),
            (6, "Partial"),
            (255, "Other"),
        ],
        // CalibrationIlluminant1/2 (DNG) share the LightSource enumeration.
        "LightSource" | "CalibrationIlluminant1" | "CalibrationIlluminant2" => &[
            (0, "Unknown"),
            (1, "Daylight"),
            (2, "Fluorescent"),
            (3, "Tungsten (Incandescent)"),
            (4, "Flash"),
            (9, "Fine Weather"),
            (10, "Cloudy"),
            (11, "Shade"),
            (12, "Daylight Fluorescent"),
            (13, "Day White Fluorescent"),
            (14, "Cool White Fluorescent"),
            (15, "White Fluorescent"),
            (16, "Warm White Fluorescent"),
            (17, "Standard Light A"),
            (18, "Standard Light B"),
            (19, "Standard Light C"),
            (20, "D55"),
            (21, "D65"),
            (22, "D75"),
            (23, "D50"),
            (24, "ISO Studio Tungsten"),
            (255, "Other"),
        ],
        "ColorSpace" => &[
            (1, "sRGB"),
            (2, "Adobe RGB"),
            (0xffff, "Uncalibrated"),
            (0xfffe, "ICC Profile"),
            (0xfffd, "Wide Gamut RGB"),
        ],
        "SensingMethod" => &[
            (1, "Not defined"),
            (2, "One-chip color area"),
            (3, "Two-chip color area"),
            (4, "Three-chip color area"),
            (5, "Color sequential area"),
            (7, "Trilinear"),
            (8, "Color sequential linear"),
        ],
        "FileSource" => &[
            (1, "Film Scanner"),
            (2, "Reflection Print Scanner"),
            (3, "Digital Camera"),
        ],
        "SceneType" => &[(1, "Directly photographed")],
        "SceneCaptureType" => &[
            (0, "Standard"),
            (1, "Landscape"),
            (2, "Portrait"),
            (3, "Night"),
            (4, "Other"),
        ],
        "ExposureMode" => &[(0, "Auto"), (1, "Manual"), (2, "Auto bracket")],
        "WhiteBalance" => &[(0, "Auto"), (1, "Manual")],
        "Contrast" => &[(0, "Normal"), (1, "Low"), (2, "High")],
        "Saturation" => &[(0, "Normal"), (1, "Low"), (2, "High")],
        "Sharpness" => &[(0, "Normal"), (1, "Soft"), (2, "Hard")],
        "GainControl" => &[
            (0, "None"),
            (1, "Low gain up"),
            (2, "High gain up"),
            (3, "Low gain down"),
            (4, "High gain down"),
        ],
        "CustomRendered" => &[(0, "Normal"), (1, "Custom")],
        "CompositeImage" => &[
            (0, "Unknown"),
            (1, "Not a Composite Image"),
            (2, "General Composite Image"),
            (3, "Composite Image Captured While Shooting"),
        ],
        "SubjectDistanceRange" => &[
            (0, "Unknown"),
            (1, "Macro"),
            (2, "Close"),
            (3, "Distant"),
        ],
        "Compression" => &[
            (1, "Uncompressed"),
            (2, "CCITT 1D"),
            (3, "T4/Group 3 Fax"),
            (4, "T6/Group 4 Fax"),
            (5, "LZW"),
            (6, "JPEG (old-style)"),
            (7, "JPEG"),
            (8, "Adobe Deflate"),
            (32773, "PackBits"),
            (34892, "Lossy JPEG"),
        ],
        "PhotometricInterpretation" => &[
            (0, "WhiteIsZero"),
            (1, "BlackIsZero"),
            (2, "RGB"),
            (3, "RGB Palette"),
            (4, "Transparency Mask"),
            (5, "CMYK"),
            (6, "YCbCr"),
            (8, "CIELab"),
            (32803, "Color Filter Array"),
            (34892, "Linear Raw"),
        ],
        "PlanarConfiguration" => &[(1, "Chunky"), (2, "Planar")],
        "Predictor" => &[
            (1, "None"),
            (2, "Horizontal differencing"),
            (3, "Floating point"),
            (34892, "Horizontal difference X2"),
            (34893, "Horizontal difference X4"),
            (34894, "Floating point X2"),
            (34895, "Floating point X4"),
        ],
        "SubfileType" => &[
            (0, "Full-resolution image"),
            (1, "Reduced-resolution image"),
            (2, "Single page of multi-page image"),
            (3, "Single page of multi-page reduced-resolution image"),
            (4, "Transparency mask"),
            (8, "Depth map"),
            (16, "Enhanced image data"),
            (0x10001, "Alternate reduced-resolution image"),
            (0xffffffff, "invalid"),
        ],
        "Flash" => &[
            (0x00, "No Flash"),
            (0x01, "Fired"),
            (0x05, "Fired, Return not detected"),
            (0x07, "Fired, Return detected"),
            (0x08, "On, Did not fire"),
            (0x09, "On, Fired"),
            (0x0d, "On, Return not detected"),
            (0x0f, "On, Return detected"),
            (0x10, "Off, Did not fire"),
            (0x14, "Off, Did not fire, Return not detected"),
            (0x18, "Auto, Did not fire"),
            (0x19, "Auto, Fired"),
            (0x1d, "Auto, Fired, Return not detected"),
            (0x1f, "Auto, Fired, Return detected"),
            (0x20, "No flash function"),
            (0x30, "Off, No flash function"),
            (0x41, "Fired, Red-eye reduction"),
            (0x45, "Fired, Red-eye reduction, Return not detected"),
            (0x47, "Fired, Red-eye reduction, Return detected"),
            (0x49, "On, Red-eye reduction"),
            (0x4d, "On, Red-eye reduction, Return not detected"),
            (0x4f, "On, Red-eye reduction, Return detected"),
            (0x50, "Off, Red-eye reduction"),
            (0x58, "Auto, Did not fire, Red-eye reduction"),
            (0x59, "Auto, Fired, Red-eye reduction"),
            (0x5d, "Auto, Fired, Red-eye reduction, Return not detected"),
            (0x5f, "Auto, Fired, Red-eye reduction, Return detected"),
        ],
        // GPS reference enums (string keys handled separately below)
        "GPSAltitudeRef" => &[
            (0, "Above Sea Level"),
            (1, "Below Sea Level"),
            (2, "Positive Sea Level (sea-level ref)"),
            (3, "Negative Sea Level (sea-level ref)"),
        ],
        "GPSDifferential" => &[
            (0, "No Correction"),
            (1, "Differential Corrected"),
        ],
        _ => return None,
    };
    Some(table)
}

/// Look up an enumerated value. Returns:
///   - `Some(Some(s))` when the tag has a table and the key is present,
///   - `Some(None)` when the tag has a table but the key is missing
///     (ExifTool prints `Unknown (N)` in this case),
///   - `None` when the tag has no enumerated table.
fn enum_lookup_opt(name: &str, key: i64) -> Option<Option<&'static str>> {
    let table = enum_table(name)?;
    Some(table.iter().find(|(k, _)| *k == key).map(|(_, v)| *v))
}

/// String-keyed GPS PrintConvs (e.g. N->North).
fn enum_str(name: &str, val: &str) -> Option<&'static str> {
    let v = val.trim();
    Some(match (name, v) {
        ("GPSLatitudeRef", "N") => "North",
        ("GPSLatitudeRef", "S") => "South",
        ("GPSLongitudeRef", "E") => "East",
        ("GPSLongitudeRef", "W") => "West",
        ("GPSStatus", "A") => "Measurement Active",
        ("GPSStatus", "V") => "Measurement Void",
        ("GPSMeasureMode", "2") => "2-Dimensional Measurement",
        ("GPSMeasureMode", "3") => "3-Dimensional Measurement",
        ("GPSSpeedRef", "K") => "km/h",
        ("GPSSpeedRef", "M") => "mph",
        ("GPSSpeedRef", "N") => "knots",
        ("GPSImgDirectionRef", "M") | ("GPSTrackRef", "M") | ("GPSDestBearingRef", "M") => {
            "Magnetic North"
        }
        ("GPSImgDirectionRef", "T") | ("GPSTrackRef", "T") | ("GPSDestBearingRef", "T") => {
            "True North"
        }
        ("GPSDestDistanceRef", "K") => "Kilometers",
        ("GPSDestDistanceRef", "M") => "Miles",
        ("GPSDestDistanceRef", "N") => "Nautical Miles",
        _ => return None,
    })
}

/// True for GPS reference tags that carry a string-keyed enumerated PrintConv.
/// (ExifTool prints `Unknown (...)` for an unrecognised/empty reference.)
fn is_gps_str_enum(name: &str) -> bool {
    matches!(
        name,
        "GPSLatitudeRef"
            | "GPSLongitudeRef"
            | "GPSDestLatitudeRef"
            | "GPSDestLongitudeRef"
            | "GPSStatus"
            | "GPSMeasureMode"
            | "GPSSpeedRef"
            | "GPSImgDirectionRef"
            | "GPSTrackRef"
            | "GPSDestBearingRef"
            | "GPSDestDistanceRef"
    )
}

/// Format one rational element the way ExifTool prints it (integer when exact,
/// otherwise 10 significant figures).
fn rational_str(n: i64, d: i64) -> String {
    if d == 0 {
        return if n == 0 { "undef".into() } else { "inf".into() };
    }
    if n % d == 0 {
        (n / d).to_string()
    } else {
        crate::value::format_g(n as f64 / d as f64, 10)
    }
}

/// ExifTool's PrintLensInfo: 4 values -> "min-maxmm f/min-max" (collapsing equal
/// endpoints), e.g. "18-55mm f/3.5-5.6" or "3.99mm f/1.8".
fn print_lens_info(parts: &[(i64, i64)]) -> Option<String> {
    if parts.len() != 4 {
        return None;
    }
    let v: Vec<String> = parts.iter().map(|&(n, d)| rational_str(n, d)).collect();
    let mut s = v[0].clone();
    if v[1] != "0" && v[1] != v[0] {
        s.push_str(&format!("-{}", v[1]));
    }
    s.push_str(&format!("mm f/{}", v[2]));
    if v[3] != "0" && v[3] != v[2] {
        s.push_str(&format!("-{}", v[3]));
    }
    Some(s)
}

/// ExifTool's PrintExposureTime: short exposures as 1/x, long as decimal seconds.
pub fn print_exposure_time(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    if v < 0.25001 && v > 0.0 {
        format!("1/{}", (0.5 + 1.0 / v) as i64)
    } else {
        let s = fmt_float(v);
        s
    }
}

/// ExifTool's PrintFNumber: round to 1 decimal place, or 2 for values < 1.0.
fn print_fnumber(v: f64) -> String {
    if v > 0.0 {
        if v < 1.0 {
            format!("{:.2}", v)
        } else {
            format!("{:.1}", v)
        }
    } else {
        fmt_float(v)
    }
}

/// ExifTool's PrintFraction: signed, simplified fraction display (e.g. "+0.7").
pub fn print_fraction(val: f64) -> String {
    let v = val * 1.00001; // avoid round-off errors (as ExifTool does)
    if v == 0.0 {
        "0".to_string()
    } else if (v.trunc() / v) > 0.999 {
        format!("{:+}", v.trunc() as i64)
    } else if ((v * 2.0).trunc()) / (v * 2.0) > 0.999 {
        format!("{:+}/2", (v * 2.0).trunc() as i64)
    } else if ((v * 3.0).trunc()) / (v * 3.0) > 0.999 {
        format!("{:+}/3", (v * 3.0).trunc() as i64)
    } else {
        // %+.3g
        let s = crate::value::format_g(v, 3);
        if v > 0.0 {
            format!("+{s}")
        } else {
            s
        }
    }
}

/// GPS PrintTimeStamp: rationals (h, m, s) -> "HH:MM:SS(.ss)".
fn print_gps_timestamp(parts: &[(i64, i64)]) -> Option<String> {
    let to_f = |(n, d): (i64, i64)| if d == 0 { 0.0 } else { n as f64 / d as f64 };
    let h = to_f(*parts.first()?);
    let m = parts.get(1).map(|p| to_f(*p)).unwrap_or(0.0);
    let s = parts.get(2).map(|p| to_f(*p)).unwrap_or(0.0);
    let secs = (s * 1_000_000.0).round() / 1_000_000.0;
    if secs.fract() == 0.0 {
        Some(format!("{:02}:{:02}:{:02}", h as i64, m as i64, secs as i64))
    } else {
        Some(format!("{:02}:{:02}:{}", h as i64, m as i64, fmt_float(secs)))
    }
}

/// Decode an EXIF text field with an 8-byte charset prefix (UserComment etc.).
fn convert_exif_text(b: &[u8]) -> String {
    if b.len() < 8 {
        return String::from_utf8_lossy(b).trim_end_matches('\0').to_string();
    }
    let id = &b[0..8];
    let body = &b[8..];
    if id.starts_with(b"UNICODE") {
        // UTF-16 — assume big-endian per EXIF default; trim trailing NULs.
        let units: Vec<u16> = body
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();
        String::from_utf16_lossy(&units).trim_end_matches('\0').trim_end().to_string()
    } else {
        // ASCII (or blank header): truncate at first NUL, trim trailing spaces.
        let end = body.iter().position(|&c| c == 0).unwrap_or(body.len());
        String::from_utf8_lossy(&body[..end]).trim_end().to_string()
    }
}

/// Apply the PrintConv for a tag. Returns the human string, or None to fall back
/// to the default value rendering.
pub fn apply(name: &str, value: &Value) -> Option<String> {
    // String-keyed (GPS refs etc.)
    if let Value::Text(s) = value {
        if let Some(m) = enum_str(name, s) {
            return Some(m.to_string());
        }
        // A GPS reference tag with an unrecognised value -> "Unknown (...)".
        if is_gps_str_enum(name) {
            return Some(format!("Unknown ({})", s.trim()));
        }
    }
    // InteropIndex (string-keyed).
    if name == "InteropIndex" {
        if let Some(s) = value.as_str() {
            return Some(match s.trim() {
                "R98" => "R98 - DCF basic file (sRGB)",
                "R03" => "R03 - DCF option file (Adobe RGB)",
                "THM" => "THM - DCF thumbnail file",
                other => return Some(other.to_string()),
            }
            .to_string());
        }
    }

    // Special numeric/format conversions first.
    match name {
        "UserComment" => {
            if let Value::Bytes(b) = value {
                return Some(convert_exif_text(b));
            }
        }
        "GPSTimeStamp" => {
            if let Value::R(parts) = value {
                return print_gps_timestamp(parts);
            }
        }
        "ExposureCompensation" | "ExposureBiasValue" => {
            return value.as_f64().map(print_fraction);
        }
        "ExposureTime" => {
            return value.as_f64().map(print_exposure_time);
        }
        // APEX shutter speed: ValueConv `abs(val) < 100 ? 2**(-val) : 0`, then
        // print as an exposure time (guards against undef/huge APEX values).
        "ShutterSpeedValue" => {
            return value.as_f64().map(|v| {
                let secs = if v.abs() < 100.0 { 2f64.powf(-v) } else { 0.0 };
                print_exposure_time(secs)
            });
        }
        "FNumber" => {
            return value.as_f64().map(print_fnumber);
        }
        // APEX aperture: ValueConv 2**(val/2) gives the F number.
        "ApertureValue" | "MaxApertureValue" => {
            return value.as_f64().map(|v| print_fnumber(2f64.powf(v / 2.0)));
        }
        "FocalLength" => {
            return value.as_f64().map(|v| format!("{:.1} mm", v));
        }
        "FocalLengthIn35mmFormat" => {
            return value.as_f64().map(|v| format!("{} mm", fmt_float(v)));
        }
        "GPSAltitude" => {
            return value.as_f64().map(|v| format!("{} m", crate::value::format_g(v, 10)));
        }
        "GPSHPositioningError" | "GPSDOP" => {
            // GPSHPositioningError prints "<val> m"; GPSDOP is plain (kept simple).
            if name == "GPSHPositioningError" {
                return value.as_f64().map(|v| format!("{} m", crate::value::format_g(v, 10)));
            }
        }
        "LensInfo" | "LensSpecification" => {
            if let Value::R(parts) = value {
                return print_lens_info(parts);
            }
        }
        "SubjectDistance" => {
            return value.as_f64().map(|v| format!("{} m", crate::value::format_g(v, 10)));
        }
        "ExifVersion" | "FlashpixVersion" | "InteropVersion" => {
            // undef[4] ascii like "0210"
            if let Value::Bytes(b) = value {
                return Some(String::from_utf8_lossy(b).trim_end_matches('\0').to_string());
            }
        }
        "ComponentsConfiguration" => {
            if let Value::Bytes(b) = value {
                let map = |x: u8| match x {
                    0 => "-",
                    1 => "Y",
                    2 => "Cb",
                    3 => "Cr",
                    4 => "R",
                    5 => "G",
                    6 => "B",
                    _ => "?",
                };
                return Some(b.iter().map(|&x| map(x)).collect::<Vec<_>>().join(", "));
            }
        }
        "GPSVersionID" | "DNGVersion" | "DNGBackwardVersion" => {
            // join integer components with dots (e.g. "1.1.0.0")
            if let Value::U(v) = value {
                return Some(v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("."));
            }
        }
        _ => {}
    }

    // Enumerated integer conversions. Some enumerated tags (FileSource,
    // SceneType) are stored as a single `undef` byte; use that byte as the key.
    let key = value.as_i64().or_else(|| match value {
        Value::Bytes(b) if b.len() == 1 => Some(b[0] as i64),
        _ => None,
    });
    if let Some(k) = key {
        match enum_lookup_opt(name, k) {
            Some(Some(m)) => return Some(m.to_string()),
            // Table exists but value isn't listed: ExifTool prints "Unknown (N)",
            // or "Unknown (0xN)" for tags whose PrintConv uses hex keys.
            Some(None) => {
                return Some(if uses_hex(name) {
                    format!("Unknown (0x{k:x})")
                } else {
                    format!("Unknown ({k})")
                });
            }
            None => {}
        }
    }
    None
}

/// Tags whose PrintConv carries `PrintHex` (unknown values shown in hex).
fn uses_hex(name: &str) -> bool {
    matches!(name, "ColorSpace")
}

/// Format GPS latitude/longitude rationals (deg, min, sec) plus an N/S/E/W ref
/// into ExifTool's `54 deg 59' 22.80" N` form.
pub fn gps_coordinate(parts: &[(i64, i64)], reference: Option<&str>) -> Option<String> {
    let to_f = |(n, d): (i64, i64)| if d == 0 { 0.0 } else { n as f64 / d as f64 };
    let deg = to_f(*parts.first()?);
    let min = parts.get(1).map(|p| to_f(*p)).unwrap_or(0.0);
    let sec = parts.get(2).map(|p| to_f(*p)).unwrap_or(0.0);
    // Normalise any fractional degrees/minutes down (matches ExifTool's ToDMS).
    let total = deg + min / 60.0 + sec / 3600.0;
    let d = total.trunc();
    let mrem = (total - d) * 60.0;
    let m = mrem.trunc();
    let s = (mrem - m) * 60.0;
    let r = reference.unwrap_or("");
    Some(format!("{} deg {}' {:.2}\" {}", d as i64, m as i64, s, r).trim().to_string())
}
