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

/// Look up `k` in an integer-keyed enum, falling back to ExifTool's
/// "Unknown (N)" (or "Unknown (0xN)" for `PrintHex` tables).
fn enum_or(table: &[(i64, &str)], k: i64, hex: bool) -> String {
    table.iter().find(|(kk, _)| *kk == k).map(|(_, s)| s.to_string()).unwrap_or_else(|| {
        if hex {
            format!("Unknown (0x{k:x})")
        } else {
            format!("Unknown ({k})")
        }
    })
}

/// Look up a string key in a string-keyed enum (ExifTool `Relist` PrintConvs).
fn strenum_or(table: &[(&str, &str)], k: &str) -> String {
    table.iter().find(|(kk, _)| *kk == k).map(|(_, s)| s.to_string()).unwrap_or_else(|| format!("Unknown ({k})"))
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

// --- Pentax multi-value enum-list PrintConvs (one hash per value, joined). ---
// FlashMode (0x000c): PrintHex, 2 component hashes.
#[rustfmt::skip]
static PENTAX_FLASHMODE_0: &[(i64, &str)] = &[
    (0x000, "Auto, Did not fire"), (0x001, "Off, Did not fire"), (0x002, "On, Did not fire"),
    (0x003, "Auto, Did not fire, Red-eye reduction"), (0x005, "On, Did not fire, Wireless (Master)"),
    (0x100, "Auto, Fired"), (0x102, "On, Fired"), (0x103, "Auto, Fired, Red-eye reduction"),
    (0x104, "On, Red-eye reduction"), (0x105, "On, Wireless (Master)"), (0x106, "On, Wireless (Control)"),
    (0x108, "On, Soft"), (0x109, "On, Slow-sync"), (0x10a, "On, Slow-sync, Red-eye reduction"),
    (0x10b, "On, Trailing-curtain Sync"),
];
#[rustfmt::skip]
static PENTAX_FLASHMODE_1: &[(i64, &str)] = &[
    (0x000, "n/a - Off-Auto-Aperture"), (0x03f, "Internal"), (0x100, "External, Auto"),
    (0x23f, "External, Flash Problem"), (0x300, "External, Manual"), (0x304, "External, P-TTL Auto"),
    (0x305, "External, Contrast-control Sync"), (0x306, "External, High-speed Sync"),
    (0x30c, "External, Wireless"), (0x30d, "External, Wireless, High-speed Sync"),
];
// DriveMode (0x0034): 4 component hashes.
#[rustfmt::skip]
static PENTAX_DRIVE_0: &[(i64, &str)] = &[
    (0, "Single-frame"), (1, "Continuous"), (2, "Continuous (Lo)"), (3, "Burst"),
    (4, "Continuous (Medium)"), (5, "Continuous (Low)"), (255, "Video"),
];
#[rustfmt::skip]
static PENTAX_DRIVE_1: &[(i64, &str)] = &[
    (0, "No Timer"), (1, "Self-timer (12 s)"), (2, "Self-timer (2 s)"), (15, "Video"),
    (16, "Mirror Lock-up"), (255, "n/a"),
];
#[rustfmt::skip]
static PENTAX_DRIVE_2: &[(i64, &str)] = &[
    (0, "Shutter Button"), (1, "Remote Control (3 s delay)"), (2, "Remote Control"),
    (4, "Remote Continuous Shooting"),
];
#[rustfmt::skip]
static PENTAX_DRIVE_3: &[(i64, &str)] = &[
    (0x00, "Single Exposure"), (0x01, "Multiple Exposure"), (0x02, "Composite Average"),
    (0x03, "Composite Additive"), (0x04, "Composite Bright"), (0x08, "Interval Shooting"),
    (0x0a, "Interval Composite Average"), (0x0b, "Interval Composite Additive"),
    (0x0c, "Interval Composite Bright"), (0x0f, "Interval Movie"), (0x10, "HDR"),
    (0x20, "HDR Strong 1"), (0x30, "HDR Strong 2"), (0x40, "HDR Strong 3"), (0x50, "HDR Manual"),
    (0xe0, "HDR Auto"), (0xff, "Video"),
];
// PictureMode (0x0033): Relist [[0,1],2]; first hash keyed by "v0 v1", second by v2.
#[rustfmt::skip]
static PENTAX_PICTUREMODE_0: &[(&str, &str)] = &[
    ("0 0", "Program"), ("0 1", "Hi-speed Program"), ("0 2", "DOF Program"), ("0 3", "MTF Program"),
    ("0 4", "Standard"), ("0 5", "Portrait"), ("0 6", "Landscape"), ("0 7", "Macro"), ("0 8", "Sport"),
    ("0 9", "Night Scene Portrait"), ("0 10", "No Flash"), ("0 11", "Night Scene"), ("0 12", "Surf & Snow"),
    ("0 13", "Text"), ("0 14", "Sunset"), ("0 15", "Kids"), ("0 16", "Pet"), ("0 17", "Candlelight"),
    ("0 18", "Museum"), ("0 19", "Food"), ("0 20", "Stage Lighting"), ("0 21", "Night Snap"),
    ("0 23", "Blue Sky"), ("0 24", "Sunset"), ("0 26", "Night Scene HDR"), ("0 27", "HDR"),
    ("0 28", "Quick Macro"), ("0 29", "Forest"), ("0 30", "Backlight Silhouette"),
    ("0 31", "Max. Aperture Priority"), ("0 32", "DOF"), ("1 4", "Auto PICT (Standard)"),
    ("1 5", "Auto PICT (Portrait)"), ("1 6", "Auto PICT (Landscape)"), ("1 7", "Auto PICT (Macro)"),
    ("1 8", "Auto PICT (Sport)"), ("2 0", "Program (HyP)"), ("2 1", "Hi-speed Program (HyP)"),
    ("2 2", "DOF Program (HyP)"), ("2 3", "MTF Program (HyP)"), ("2 22", "Shallow DOF (HyP)"),
    ("3 0", "Green Mode"), ("4 0", "Shutter Speed Priority"), ("4 2", "Shutter Speed Priority 2"),
    ("4 31", "Shutter Speed Priority 31"), ("5 0", "Aperture Priority"), ("5 2", "Aperture Priority 2"),
    ("5 31", "Aperture Priority 31"), ("6 0", "Program Tv Shift"), ("7 0", "Program Av Shift"),
    ("8 0", "Manual"), ("9 0", "Bulb"), ("10 0", "Aperture Priority, Off-Auto-Aperture"),
    ("11 0", "Manual, Off-Auto-Aperture"), ("12 0", "Bulb, Off-Auto-Aperture"), ("19 0", "Astrotracer"),
    ("13 0", "Shutter & Aperture Priority AE"), ("14 0", "Shutter Priority AE"),
    ("15 0", "Sensitivity Priority AE"), ("16 0", "Flash X-Sync Speed AE"), ("17 0", "Flash X-Sync Speed"),
    ("18 0", "Auto Program (Normal)"), ("18 1", "Auto Program (Hi-speed)"), ("18 2", "Auto Program (DOF)"),
    ("18 3", "Auto Program (MTF)"), ("18 22", "Auto Program (Shallow DOF)"), ("20 22", "Blur Control"),
    ("24 0", "Aperture Priority (Adv.Hyp)"), ("25 0", "Manual Exposure (Adv.Hyp)"),
    ("26 0", "Shutter and Aperture Priority (TAv)"), ("249 0", "Movie (TAv)"),
    ("250 0", "Movie (TAv, Auto Aperture)"), ("251 0", "Movie (Manual)"),
    ("252 0", "Movie (Manual, Auto Aperture)"), ("253 0", "Movie (Av)"),
    ("254 0", "Movie (Av, Auto Aperture)"), ("255 0", "Movie (P, Auto Aperture)"), ("255 4", "Video (4)"),
];
static PENTAX_PICTUREMODE_1: &[(i64, &str)] = &[(0, "1/2 EV steps"), (1, "1/3 EV steps")];

pub fn pentax(name: &str, v: &Value) -> Option<String> {
    match name {
        // "3 0 0 0" -> "3.0.0.0" (tr/ /./).
        "PentaxVersion" | "PentaxFirmwareVersion" => Some(v.to_string().replace(' ', ".")),
        // "640 480" -> "640x480" (tr/ /x/).
        "PreviewImageSize" => Some(v.to_string().replace(' ', "x")),
        "CameraTemperature" => Some(format!("{} C", v)),

        // Multi-value enum-list PrintConvs: map each value through its component
        // hash and join (ExifTool joins these list PrintConvs with "; ").
        "FlashMode" => {
            let n = ints(v);
            let mut parts = Vec::new();
            if let Some(&x) = n.first() {
                parts.push(enum_or(PENTAX_FLASHMODE_0, x, true));
            }
            if let Some(&x) = n.get(1) {
                parts.push(enum_or(PENTAX_FLASHMODE_1, x, true));
            }
            (!parts.is_empty()).then(|| parts.join("; "))
        }
        "DriveMode" => {
            let n = ints(v);
            (n.len() >= 4).then(|| {
                [PENTAX_DRIVE_0, PENTAX_DRIVE_1, PENTAX_DRIVE_2, PENTAX_DRIVE_3]
                    .iter()
                    .enumerate()
                    .map(|(i, t)| enum_or(t, n[i], false))
                    .collect::<Vec<_>>()
                    .join("; ")
            })
        }
        // PictureMode joins values 0+1 (Relist) for the first lookup, value 2 for
        // the second (the 3-value 0x0033 variant; the scalar 0x000b is left raw).
        "PictureMode" => {
            let n = ints(v);
            (n.len() >= 3).then(|| {
                let p0 = strenum_or(PENTAX_PICTUREMODE_0, &format!("{} {}", n[0], n[1]));
                format!("{p0}; {}", enum_or(PENTAX_PICTUREMODE_1, n[2], false))
            })
        }
        // AutoBracketing: bracket step (ValueConv) EV, then extended-bracket type.
        "AutoBracketing" => {
            let n = ints(v);
            if n.is_empty() {
                return None;
            }
            let v0 = n[0];
            let s0 = if v0 < 10 {
                format_g(v0 as f64 / 3.0, 10)
            } else if v0 < 20 {
                format_g(v0 as f64 - 9.5, 10)
            } else if v0 & 0x1000 != 0 {
                format!("{}/2", v0 - 0x1000)
            } else if v0 & 0x2000 != 0 {
                format!("{}/3", v0 - 0x2000)
            } else {
                v0.to_string()
            };
            // PrintConv: "%.1f" unless the step is zero or a fraction.
            let mut parts = vec![match s0.parse::<f64>() {
                Ok(f) if f != 0.0 && !s0.contains('/') => format!("{f:.1}"),
                _ => s0,
            }];
            if let Some(&v1) = n.get(1) {
                parts.push(if v1 != 0 {
                    let t = v1 >> 8;
                    let name = match t {
                        1 => "WB-BA",
                        2 => "WB-GM",
                        3 => "Saturation",
                        4 => "Sharpness",
                        5 => "Contrast",
                        6 => "Hue",
                        7 => "HighLowKey",
                        _ => return Some(format!("{} EV, Unknown({t})+{}", parts[0], v1 & 0xff)),
                    };
                    format!("{name}+{}", v1 & 0xff)
                } else {
                    "No Extended Bracket".into()
                });
            }
            Some(parts.join(" EV, "))
        }

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
