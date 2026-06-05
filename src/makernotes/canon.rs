//! Canon maker notes (`Image::ExifTool::Canon::Main`).
//!
//! The Canon maker note is a plain IFD. Most of the interesting tags live in
//! ProcessBinaryData sub-records (CameraSettings, ShotInfo, FocalLength) pointed
//! to by the IFD's first few tags; the rest are scalar strings/numbers.

mod canon_afinfo;
mod canon_camera_settings;
mod canon_focal_length;
mod canon_model_id;
mod canon_shot_info;

use super::binary::{Pc, Skip};
use super::{MnKind, MnTag};
use crate::value::{format_g, Value};

use canon_afinfo::CANON_AFINFO;
use canon_camera_settings::CANON_CAMERA_SETTINGS;
use canon_focal_length::CANON_FOCAL_LENGTH;
use canon_model_id::CANON_MODEL_ID;
use canon_shot_info::CANON_SHOT_INFO;

/// Canon serial-number format (0x15) enumeration.
static SERIAL_FORMAT: &[(i64, &str)] = &[
    (0x90000000, "Format 1"),
    (0xa0000000, "Format 2"),
];

/// Canon's APEX-style EV decoder (`CanonEv` in Canon.pm): decodes the 1/3-stop
/// fraction codes 0x0c and 0x14 and divides by 32.
fn canon_ev(val: f64) -> f64 {
    let (sign, a) = if val < 0.0 { (-1.0, -val) } else { (1.0, val) };
    let ai = a as i64;
    let frac_code = ai & 0x1f;
    let base = (ai - frac_code) as f64;
    let frac = match frac_code {
        0x0c => 32.0 / 3.0,
        0x14 => 64.0 / 3.0,
        f => f as f64,
    };
    sign * (base + frac) / 32.0
}

/// Vendor special converter for Canon binary/IFD tags whose conversion is a
/// formula. Ported from the matching ValueConv/PrintConv in Canon.pm.
pub fn special(name: &str, v: &Value) -> Option<String> {
    let f = v.as_f64();
    match name {
        // APEX aperture: exp(CanonEv(val) * ln2 / 2), printed with 2 sig figs.
        "MaxAperture" | "MinAperture" | "TargetAperture" => {
            let val = f?;
            let ap = (canon_ev(val) * std::f64::consts::LN_2 / 2.0).exp();
            Some(format_g(ap, 2))
        }
        // printParameter: 0 => "Normal", otherwise a signed number.
        "Contrast" | "Saturation" | "ColorTone" => {
            let val = v.as_i64()?;
            Some(if val == 0 { "Normal".to_string() } else { format!("{val:+}") })
        }
        // Sharpness PrintConv: ">0 => +N", else the raw number.
        "Sharpness" => {
            let val = v.as_i64()?;
            Some(if val > 0 { format!("+{val}") } else { val.to_string() })
        }
        // Digital zoom enumeration (comment in source defeated auto-extraction).
        "DigitalZoom" => {
            let val = v.as_i64()?;
            Some(match val {
                0 => "None",
                1 => "2x",
                2 => "4x",
                3 => "Other",
                _ => return Some(format!("Unknown ({val})")),
            }
            .to_string())
        }
        // Self-timer: 0 => Off, else tenths of a second (+ optional Custom flag).
        "SelfTimer" => {
            let val = v.as_i64()?;
            if val == 0 {
                Some("Off".to_string())
            } else {
                let secs = format_g((val & 0xfff) as f64 / 10.0, 10);
                let custom = if val & 0x4000 != 0 { ", Custom" } else { "" };
                Some(format!("{secs} s{custom}"))
            }
        }
        "FocalUnits" => Some(format!("{}/mm", v.as_i64()?)),
        // Canon CameraISO lookup.
        "CameraISO" => {
            let val = v.as_i64()?;
            if val & 0x4000 != 0 {
                return Some((val & 0x3fff).to_string());
            }
            Some(match val {
                0 => "n/a".to_string(),
                14 => "Auto High".to_string(),
                15 => "Auto".to_string(),
                16 => "50".to_string(),
                17 => "100".to_string(),
                18 => "200".to_string(),
                19 => "400".to_string(),
                20 => "800".to_string(),
                _ => format!("Unknown ({val})"),
            })
        }
        // Focus distance: val/100 metres, "inf" beyond ~655 m.
        "FocusDistanceUpper" | "FocusDistanceLower" => {
            let d = f? / 100.0;
            Some(if d > 655.345 { "inf".to_string() } else { format!("{} m", format_g(d, 10)) })
        }
        // Measured EV (ShotInfo variant): val/8 - 6.
        "MeasuredEV2" => Some(format_g(f? / 8.0 - 6.0, 10)),
        // ShotInfo BaseISO: exp(val/32 * ln2) * 100/32, rounded to a whole number.
        "BaseISO" => Some(format!("{:.0}", (f? / 32.0 * std::f64::consts::LN_2).exp() * 100.0 / 32.0)),
        // ShotInfo MeasuredEV: val/32 + 5, two decimals.
        "MeasuredEV" => Some(format!("{:.2}", f? / 32.0 + 5.0)),
        // ShotInfo BulbDuration: tenths of a second.
        "BulbDuration" => Some(format_g(f? / 10.0, 10)),
        // FocalLength FocalPlaneX/YSize: 1/1000-inch units -> mm, 2 decimals.
        "FocalPlaneXSize" | "FocalPlaneYSize" => Some(format!("{:.2} mm", f? * 25.4 / 1000.0)),
        // ShotInfo OpticalZoomCode: 8 means "not applicable".
        "OpticalZoomCode" => {
            let val = v.as_i64()?;
            Some(if val == 8 { "n/a".to_string() } else { val.to_string() })
        }
        // Auto ISO: exp(val/32 * ln2) * 100, rounded to a whole number.
        "AutoISO" => Some(format!("{:.0}", (f? / 32.0 * std::f64::consts::LN_2).exp() * 100.0)),
        // Focal lengths: assume FocalUnits == 1 (the common case) and append mm.
        "MaxFocalLength" | "MinFocalLength" | "FocalLength2" => Some(format!("{} mm", format_g(f?, 10))),
        // File number "118-1861" from a packed integer (s/(\d+)(\d{4})/$1-$2/).
        "FileNumber" => {
            let s = v.as_i64()?.to_string();
            if s.len() > 4 {
                let (a, b) = s.split_at(s.len() - 4);
                Some(format!("{a}-{b}"))
            } else {
                Some(s)
            }
        }
        _ => None,
    }
}

pub static CANON_MAIN: &[MnTag] = &[
    // ProcessBinaryData sub-records.
    MnTag { id: 0x1, kind: MnKind::Binary(&CANON_CAMERA_SETTINGS) },
    MnTag { id: 0x2, kind: MnKind::Binary(&CANON_FOCAL_LENGTH) },
    MnTag { id: 0x4, kind: MnKind::Binary(&CANON_SHOT_INFO) },
    MnTag { id: 0x12, kind: MnKind::Binary(&CANON_AFINFO) },
    // Scalar tags (read with the IFD entry's own format).
    MnTag { id: 0x6, kind: MnKind::Scalar { name: "CanonImageType", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 0x7, kind: MnKind::Scalar { name: "CanonFirmwareVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 0x8, kind: MnKind::Scalar { name: "FileNumber", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 0x9, kind: MnKind::Scalar { name: "OwnerName", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 0xe, kind: MnKind::Scalar { name: "CanonFileLength", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 0x10, kind: MnKind::Scalar { name: "CanonModelID", pc: Pc::Enum(CANON_MODEL_ID), bin: false, skip: Skip::Never } },
    MnTag { id: 0x15, kind: MnKind::Scalar { name: "SerialNumberFormat", pc: Pc::Enum(SERIAL_FORMAT), bin: false, skip: Skip::Never } },
    MnTag { id: 0x1e, kind: MnKind::Scalar { name: "FirmwareRevision", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 0x28, kind: MnKind::Scalar { name: "ImageUniqueID", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 0x38, kind: MnKind::Scalar { name: "BatteryType", pc: Pc::None, bin: false, skip: Skip::Never } },
];
