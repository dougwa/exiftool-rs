//! Per-vendor formula converters (ValueConv/PrintConv that the enum tables
//! can't express). Each returns the print string, or `None` to fall back to the
//! table's PrintConv / shared EXIF PrintConv / raw rendering. Ported by hand
//! from the matching converters in the ExifTool vendor modules.

use crate::value::{format_g, Value};

pub fn sony(_name: &str, _v: &Value) -> Option<String> {
    None
}

pub fn olympus(name: &str, v: &Value) -> Option<String> {
    match name {
        // SpecialMode: "shootmode, Sequence: N, Panorama: dir" from 3 ints.
        "SpecialMode" => {
            let nums: Vec<i64> = match v {
                Value::U(a) => a.iter().map(|&x| x as i64).collect(),
                Value::I(a) => a.clone(),
                _ => return None,
            };
            if nums.len() < 3 {
                return None;
            }
            let shoot = ["Normal", "Unknown (1)", "Fast", "Panorama"]
                .get(nums[0] as usize)
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("Unknown ({})", nums[0]));
            let dir = ["(none)", "Left to Right", "Right to Left", "Bottom to Top", "Top to Bottom"]
                .get(nums[2] as usize)
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("Unknown ({})", nums[2]));
            Some(format!("{shoot}, Sequence: {}, Panorama: {dir}", nums[1]))
        }
        // Quality: CameraType-dependent in ExifTool; use the common (non-SX) map.
        "Quality" => {
            let q = v.as_i64()?;
            Some(
                match q {
                    1 => "SQ (Low)",
                    2 => "HQ (Normal)",
                    3 => "SHQ (Fine)",
                    4 => "RAW",
                    5 => "Medium-Fine",
                    6 => "Small-Fine",
                    33 => "Uncompressed",
                    _ => return Some(format!("Unknown ({q})")),
                }
                .to_string(),
            )
        }
        // DigitalZoom: ensure a decimal point ("0" -> "0.0").
        "DigitalZoom" => {
            let s = v.to_string();
            Some(if s.contains('.') { s } else { format!("{s}.0") })
        }
        "FocalPlaneDiagonal" => Some(format!("{} mm", v)),
        _ => None,
    }
}

pub fn panasonic(name: &str, v: &Value) -> Option<String> {
    match name {
        // FirmwareVersion: 4 bytes -> "a.b.c.d".
        "FirmwareVersion" => {
            let b = match v {
                Value::Bytes(b) => b.clone(),
                Value::U(a) => a.iter().map(|&x| x as u8).collect(),
                _ => return None,
            };
            Some(b.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("."))
        }
        _ => None,
    }
}

pub fn fujifilm(_name: &str, _v: &Value) -> Option<String> {
    None
}

pub fn sigma(_name: &str, v: &Value) -> Option<String> {
    // Many Sigma string tags are stored "Label:value" (e.g. "Cont:+0.0",
    // "Qual:12"); ExifTool strips the "Label:" prefix in a ValueConv.
    if let Value::Text(s) = v {
        if let Some(pos) = s.find(':') {
            let (label, rest) = s.split_at(pos);
            if !label.is_empty() && label.chars().all(|c| c.is_ascii_alphanumeric() || c == ' ') {
                return Some(rest[1..].trim_start().to_string());
            }
        }
    }
    None
}

pub fn minolta(_name: &str, _v: &Value) -> Option<String> {
    None
}

pub fn pentax(name: &str, v: &Value) -> Option<String> {
    match name {
        // "3 0 0 0" -> "3.0.0.0" (tr/ /./).
        "PentaxVersion" | "PentaxFirmwareVersion" => Some(v.to_string().replace(' ', ".")),
        // "640 480" -> "640x480" (tr/ /x/).
        "PreviewImageSize" => Some(v.to_string().replace(' ', "x")),
        "CameraTemperature" => Some(format!("{} C", v)),
        _ => None,
    }
}

pub fn casio(name: &str, v: &Value) -> Option<String> {
    match name {
        // ObjectDistance: millimetres -> "N m".
        "ObjectDistance" => {
            let d = v.as_f64()? / 1000.0;
            Some(format!("{} m", format_g(d, 10)))
        }
        _ => None,
    }
}
