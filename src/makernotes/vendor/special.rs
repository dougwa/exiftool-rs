//! Per-vendor formula converters (ValueConv/PrintConv that the enum tables
//! can't express). Each returns the print string, or `None` to fall back to the
//! table's PrintConv / shared EXIF PrintConv / raw rendering. Ported by hand
//! from the matching converters in the ExifTool vendor modules.

use crate::exif::printconv;
use crate::value::{format_g, Value};

pub fn sony(_name: &str, _v: &Value) -> Option<String> {
    None
}

/// An `undef` value's raw bytes, if any.
fn bytes(v: &Value) -> Option<&[u8]> {
    match v {
        Value::Bytes(b) => Some(b),
        _ => None,
    }
}

/// A numeric value's elements as i64 (empty for non-numeric values).
fn ints(v: &Value) -> Vec<i64> {
    match v {
        Value::U(a) => a.iter().map(|&x| x as i64).collect(),
        Value::I(a) => a.clone(),
        _ => Vec::new(),
    }
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
        // FocusDistance is a rational whose denominator is mm/cm depending on
        // model; ExifTool ignores the denominator (uses the numerator a) and
        // returns a/1000 metres (0xffffffff or 0 -> "inf").
        "FocusDistance" => {
            let a = match v {
                Value::R(r) => r.first().map(|&(n, _)| n)?,
                _ => v.as_i64()?,
            };
            if a as u64 == 0xffff_ffff || a == 0 {
                Some("inf".into())
            } else {
                Some(format!("{} m", format_g(a as f64 / 1000.0, 10)))
            }
        }
        // 3-number setting records: "value (min X, max Y)" (the E-1's CS-relative
        // variant is not modelled here).
        "CustomSaturation" | "ContrastSetting" | "SharpnessSetting" => {
            let n = ints(v);
            (n.len() == 3).then(|| format!("{} (min {}, max {})", n[0], n[1], n[2]))
        }
        // Underwater-housing manometer: pressure raw/10 kPa; reading is two
        // values /10 rendered "X m, Y ft".
        "ManometerPressure" => Some(format!("{} kPa", format_g(v.as_f64()? / 10.0, 10))),
        "ManometerReading" => {
            let n = ints(v);
            (n.len() == 2).then(|| {
                format!("{} m, {} ft", format_g(n[0] as f64 / 10.0, 10), format_g(n[1] as f64 / 10.0, 10))
            })
        }
        // ColorMatrix is stored int16u but ExifTool reads (and shows) it int16s.
        "ColorMatrix" => match v {
            Value::U(a) => Some(
                a.iter().map(|&x| (x as u16 as i16).to_string()).collect::<Vec<_>>().join(" "),
            ),
            _ => None,
        },
        // WhiteBalanceTemperature: 0 -> "Auto".
        "WhiteBalanceTemperature" => {
            let t = v.as_i64()?;
            Some(if t == 0 { "Auto".into() } else { t.to_string() })
        }
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

pub fn minolta(name: &str, v: &Value) -> Option<String> {
    match name {
        // FocusDistance: millimetres -> "N m" (0 -> "inf").
        "FocusDistance" => {
            let d = v.as_f64()? / 1000.0;
            Some(if d == 0.0 { "inf".into() } else { format!("{} m", format_g(d, 10)) })
        }
        // Date/Time packed into an int: yyyy<<16 | mm<<8 | dd (and hh/mm/ss).
        "MinoltaDate" => {
            let n = v.as_i64()?;
            Some(format!("{}:{:02}:{:02}", n >> 16, (n & 0xff00) >> 8, n & 0xff))
        }
        "MinoltaTime" => {
            let n = v.as_i64()?;
            Some(format!("{:02}:{:02}:{:02}", n >> 16, (n & 0xff00) >> 8, n & 0xff))
        }
        // APEX max aperture: 2^((v-8)/16), printed "%.1f".
        "MaxAperture" => Some(format!("{:.1}", 2f64.powf((v.as_f64()? - 8.0) / 16.0))),
        // White-balance gains stored *256.
        "ColorBalanceRed" | "ColorBalanceGreen" | "ColorBalanceBlue" => {
            Some(format_g(v.as_f64()? / 256.0, 10))
        }
        // Brightness: v/8 - 6.
        "Brightness" => Some(format_g(v.as_f64()? / 8.0 - 6.0, 10)),
        // Flash exposure comp: (v-6)/3 EV, printed as a fraction.
        "FlashExposureComp" => Some(printconv::print_fraction((v.as_f64()? - 6.0) / 3.0)),
        // ColorFilter: v - 3 (the DiMAGE A2's -5 variant is not modelled here).
        "ColorFilter" => Some(format_g(v.as_f64()? - 3.0, 10)),
        _ => None,
    }
}

/// ExifTool's `Image::ExifTool::Pentax::PentaxEv` — decode Pentax's APEX-style
/// 1/8-EV integer encoding. The odd 1/3-EV codes (low octal digit 3 or 5) are
/// nudged to true thirds; note Perl's `8/3`/`16/3` are float, so the result is
/// fractional even though the input is an integer.
fn pentax_ev(val: i64) -> f64 {
    let mut x = val as f64;
    if val & 0x01 != 0 {
        let sign = if val < 0 { -1.0 } else { 1.0 };
        let frac = (val.unsigned_abs() & 0x07) as f64; // ($val * $sign) & 0x07
        if frac == 3.0 {
            x += sign * (8.0 / 3.0 - frac);
        } else if frac == 5.0 {
            x += sign * (16.0 / 3.0 - frac);
        }
    }
    x / 8.0
}

pub fn pentax(name: &str, v: &Value) -> Option<String> {
    match name {
        // "3 0 0 0" -> "3.0.0.0" (tr/ /./).
        "PentaxVersion" | "PentaxFirmwareVersion" => Some(v.to_string().replace(' ', ".")),
        // "640 480" -> "640x480" (tr/ /x/).
        "PreviewImageSize" => Some(v.to_string().replace(' ', "x")),
        "CameraTemperature" => Some(format!("{} C", v)),

        // Date/Time are stored as packed `undef` bytes: Date = big-endian
        // uint16 year + month + day; Time = hour, minute, second.
        "Date" => {
            let b = bytes(v)?;
            (b.len() == 4).then(|| {
                format!("{:04}:{:02}:{:02}", (b[0] as u16) << 8 | b[1] as u16, b[2], b[3])
            })
        }
        "Time" => {
            let b = bytes(v)?;
            (b.len() >= 3).then(|| format!("{:02}:{:02}:{:02}", b[0], b[1], b[2]))
        }
        // Firmware versions: 4 bytes "encrypted" by toggling all bits, then
        // formatted "d.dd.dd.dd".
        "DSPFirmwareVersion" | "CPUFirmwareVersion" => {
            let b = bytes(v)?;
            (b.len() == 4).then(|| {
                let a: Vec<u8> = b.iter().map(|x| x ^ 0xff).collect();
                format!("{}.{:02}.{:02}.{:02}", a[0], a[1], a[2], a[3])
            })
        }
        // ImageEditing: 4 int8u (stored undef) looked up as a space-joined key.
        "ImageEditing" => {
            let key =
                bytes(v)?.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
            Some(
                match key.as_str() {
                    "0 0" | "0 0 0 0" => "None",
                    "0 0 0 4" => "Digital Filter",
                    "1 0 0 0" => "Resized",
                    "2 0 0 0" => "Cropped",
                    "4 0 0 0" => "Digital Filter 4",
                    "6 0 0 0" => "Digital Filter 6",
                    "8 0 0 0" => "Red-eye Correction",
                    "16 0 0 0" => "Frame Synthesis?",
                    _ => return Some(key),
                }
                .to_string(),
            )
        }

        // --- AEInfo / CameraSettings exposure-value conversions (Pentax.pm). ---
        // Each tag stores a raw integer that ExifTool turns into a real exposure
        // quantity via an APEX-style ValueConv, then formats with PrintConv. We
        // produce the final print string directly (the stored value stays raw).
        //
        // Shutter speeds: `exp(-PentaxEv(...)*log2)` / `24*exp(-(v-32)*log2/8)`
        // are `2^x`; printed with the shared PrintExposureTime ("1/203").
        "TvExposureTimeSetting" => {
            let raw = v.as_i64()?;
            Some(printconv::print_exposure_time(2f64.powf(-pentax_ev(raw - 68))))
        }
        "AEExposureTime" | "AEMinExposureTime" => {
            let raw = v.as_f64()?;
            Some(printconv::print_exposure_time(24.0 * 2f64.powf(-(raw - 32.0) / 8.0)))
        }
        // Apertures: `2^((v-68)/16)` (or PentaxEv variant), printed "%.1f" — but
        // AEMinAperture rounds to "%.0f".
        "AvApertureSetting" => Some(format!("{:.1}", 2f64.powf(pentax_ev(v.as_i64()? - 68) / 2.0))),
        "AEAperture" | "AEMaxAperture" | "AEMaxAperture2" => {
            Some(format!("{:.1}", 2f64.powf((v.as_f64()? - 68.0) / 16.0)))
        }
        "AEMinAperture" => Some(format!("{:.0}", 2f64.powf((v.as_f64()? - 68.0) / 16.0))),
        // ISO: `int(100*2^...+0.5)` (already integral) or `int($val+0.5)`.
        "SvISOSetting" | "ISOFloor" => {
            let raw = v.as_i64()?;
            Some(format!("{}", (100.0 * 2f64.powf(pentax_ev(raw - 32)) + 0.5).floor() as i64))
        }
        "AE_ISO" => {
            let raw = v.as_f64()?;
            Some(format!("{}", (100.0 * 2f64.powf((raw - 32.0) / 8.0) + 0.5).floor() as i64))
        }
        // Exposure compensation: `PentaxEv(64-v)`, printed "%+.1f" (0 -> "0").
        "BaseExposureCompensation" => {
            let ev = pentax_ev(64 - v.as_i64()?);
            Some(if ev != 0.0 { format!("{ev:+.1}") } else { "0".into() })
        }
        // Plain ValueConv, no PrintConv (numeric value shown as-is).
        "AEXv" => Some(format_g((v.as_f64()? - 64.0) / 8.0, 10)),
        "AEBXv" => Some(format_g(v.as_f64()? / 8.0, 10)),
        "SensitivityAdjust" => Some(format_g((v.as_f64()? - 50.0) / 10.0, 10)),
        "EffectiveLV" => Some(format!("{:.1}", v.as_f64()? / 1024.0)),

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
