//! Nikon maker notes (`Image::ExifTool::Nikon::Main`).
//!
//! Modern Nikon maker notes (Type 3) begin with `Nikon\0` + a 4-byte version,
//! followed at offset 10 by a **complete, self-contained TIFF header** with its
//! own byte order and IFD-offset. Internal value offsets are relative to that
//! embedded header, so we parse it as an independent sub-TIFF. Older "headerless"
//! Nikon notes (no signature) are a bare IFD relative to the host TIFF base.

mod nikon_afinfo;
mod nikon_main;

use super::walk_ifd;
use crate::reader::{ByteOrder, Reader};
use crate::tag::ExtractedTag;
use crate::value::Value;

use nikon_main::NIKON_MAIN;

/// Nikon Main's table-wide default PrintConv (`FormatString` in Nikon.pm):
/// title-case each word that contains a vowel, leaving vowel-less words (e.g.
/// "VR", "MM") upper-case, then patch the special cases "AF" and "RAW".
fn format_string(input: &str) -> String {
    let s = input.trim_end();
    let has_vowel = |w: &str| w.bytes().any(|b| matches!(b.to_ascii_uppercase(), b'A' | b'E' | b'I' | b'O' | b'U' | b'Y'));
    if !has_vowel(s) {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut word = String::new();
    let flush = |word: &mut String, out: &mut String| {
        if word.is_empty() {
            return;
        }
        if has_vowel(word) {
            let mut cs = word.chars();
            let first = cs.next().unwrap();
            let mut w: String = first.to_string();
            w.extend(cs.flat_map(|c| c.to_lowercase()));
            // Patches for words ExifTool keeps upper-case.
            match w.as_str() {
                "Af" => w = "AF".to_string(),
                "Raw" => w = "RAW".to_string(),
                _ => {}
            }
            out.push_str(&w);
        } else {
            out.push_str(word);
        }
        word.clear();
    };
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            word.push(c);
        } else {
            flush(&mut word, &mut out);
            out.push(c);
        }
    }
    flush(&mut word, &mut out);
    out
}

/// Port of Nikon `LensType`'s DecodeBits PrintConv: a bit field plus the
/// abbreviation cleanups ExifTool applies (e.g. "D G" -> "G", "E" moved first).
fn nikon_lens_type(val: i64) -> String {
    if val == 0 {
        return "AF".into();
    }
    let labels = [
        (0, "MF"),
        (1, "D"),
        (2, "G"),
        (3, "VR"),
        (4, "1"),
        (5, "FT-1"),
        (6, "E"),
        (7, "AF-P"),
    ];
    // DecodeBits joins set-bit labels with ", "; ExifTool then strips the commas.
    let mut s = labels
        .iter()
        .filter(|(b, _)| val & (1 << b) != 0)
        .map(|(_, l)| *l)
        .collect::<Vec<_>>()
        .join(" ");
    s = s.replacen("D G", "G", 1);
    if let Some(p) = s.find(" E") {
        s.replace_range(p..p + 2, "");
        s = match s.strip_prefix("G ") {
            Some(rest) => format!("E {rest}"),
            None => format!("E {s}"),
        };
    }
    if let Some(p) = s.find(" 1") {
        s.replace_range(p..p + 2, "");
        s = format!("1 {s}");
    }
    if let Some(rest) = s.strip_prefix("FT-1 ") {
        s = format!("{rest} FT-1");
    }
    s
}

/// Port of Nikon `ShootingMode`'s DecodeBits PrintConv. Bit 5 is model-specific
/// (D70 = "Unused LE-NR Slowdown"); we use the common "Auto ISO" label.
fn nikon_shooting_mode(val: i64) -> String {
    let mut out = String::new();
    // Without any of the drive/bracketing bits (0x87) it is single-frame.
    if val & 0x87 == 0 {
        if val == 0 {
            return "Single-Frame".into();
        }
        out.push_str("Single-Frame, ");
    }
    let labels = [
        (0, "Continuous"),
        (1, "Delay"),
        (2, "PC Control"),
        (3, "Self-timer"),
        (4, "Exposure Bracketing"),
        (5, "Auto ISO"),
        (6, "White-Balance Bracketing"),
        (7, "IR Control"),
        (8, "D-Lighting Bracketing"),
        (11, "Pre-capture"),
    ];
    let bits = labels
        .iter()
        .filter(|(b, _)| val & (1 << b) != 0)
        .map(|(_, l)| *l)
        .collect::<Vec<_>>()
        .join(", ");
    out.push_str(&bits);
    out
}

/// Nikon special converter: the table-default FormatString for strings, plus a
/// couple of formula tags.
pub fn special(name: &str, v: &Value) -> Option<String> {
    match name {
        // MakerNoteVersion: ExifTool inserts a "." after the first two digits
        // and strips a leading zero. The 4 bytes are either raw digit *values*
        // (e.g. 00 01 00 00 -> "1.00") or ASCII digits (e.g. "0210" -> "2.10").
        "MakerNoteVersion" => {
            if let Value::Bytes(b) = v {
                if b.len() == 4 && b.iter().all(|&c| c < 10) {
                    return Some(format!("{}.{}{}", b[0] as u32 * 10 + b[1] as u32, b[2], b[3]));
                }
            }
            let s = v.to_string();
            let d = s.as_bytes();
            if d.len() == 4 && d.iter().all(|c| c.is_ascii_digit()) {
                let major: u32 = s[0..2].parse().ok()?;
                return Some(format!("{}.{}", major, &s[2..4]));
            }
            None
        }
        // Lens: 4 rationals (min/max focal, min/max aperture) -> "18-70mm f/3.5-4.5".
        "Lens" => match v {
            Value::R(r) => crate::exif::printconv::print_lens_info(r),
            _ => None,
        },
        // LensType is a bit field (ExifTool's DecodeBits) with a few cleanups.
        "LensType" => Some(nikon_lens_type(v.as_i64()?)),
        // ShootingMode bit field; no drive bits set -> "Single-Frame".
        "ShootingMode" => Some(nikon_shooting_mode(v.as_i64()?)),
        // SensorPixelSize: two rationals -> "X x Y um".
        "SensorPixelSize" => Some(format!("{} um", v.to_string().replacen(' ', " x ", 1))),
        // LensFStops: 3 *unsigned* bytes a,b,c -> a*(b/c), printed "%.2f".
        "LensFStops" => {
            let b = match v {
                Value::Bytes(b) if b.len() >= 3 => b,
                _ => return None,
            };
            let (a, bb, c) = (b[0] as f64, b[1] as f64, b[2] as f64);
            Some(format!("{:.2}", if c != 0.0 { a * (bb / c) } else { 0.0 }))
        }
        // EV fields encoded as 3 *signed* bytes a,b,c -> a*(b/c); per-tag print.
        "ProgramShift"
        | "ExposureDifference"
        | "FlashExposureComp"
        | "ExternalFlashExposureComp"
        | "FlashExposureBracketValue" => {
            let b = match v {
                Value::Bytes(b) if b.len() >= 3 => b,
                _ => return None,
            };
            let (a, bb, c) = (b[0] as i8 as f64, b[1] as i8 as f64, b[2] as i8 as f64);
            let val = if c != 0.0 { a * (bb / c) } else { 0.0 };
            Some(match name {
                "ExposureDifference" => {
                    if val != 0.0 {
                        format!("{val:+.1}")
                    } else {
                        "0".into()
                    }
                }
                "FlashExposureBracketValue" => format!("{val:.1}"),
                // ProgramShift / FlashExposureComp / ExternalFlashExposureComp
                _ => crate::exif::printconv::print_fraction(val),
            })
        }
        // Nikon ISO is stored as [0, value]; show the actual ISO (last element).
        "ISO" | "ISOSetting" => match v {
            Value::U(a) => a.last().map(|x| x.to_string()),
            Value::I(a) => a.last().map(|x| x.to_string()),
            _ => None,
        },
        _ => {
            if let Value::Text(s) = v {
                Some(format_string(s))
            } else {
                None
            }
        }
    }
}

pub fn parse(r: &Reader, mn_off: usize, mn_len: usize, out: &mut Vec<ExtractedTag>) {
    let mn = match r.bytes(mn_off, mn_len) {
        Some(b) => b,
        None => return,
    };

    // Type 3: "Nikon\0" + version byte 0x02, then a TIFF header at offset 10.
    if mn.len() > 18 && &mn[0..6] == b"Nikon\x00" && mn[6] == 0x02 {
        let tiff = &mn[10..];
        let order = match &tiff[0..2] {
            b"II" => ByteOrder::Little,
            b"MM" => ByteOrder::Big,
            _ => return,
        };
        let sub = Reader::new(tiff, order);
        // magic (42) at offset 2, first-IFD offset at 4 (relative to this header).
        if sub.u16(2) != Some(42) {
            return;
        }
        if let Some(ifd) = sub.u32(4) {
            walk_ifd(&sub, ifd as usize, NIKON_MAIN, "Nikon", special, out);
        }
        return;
    }

    // Headerless Nikon (Type 3 without signature): a bare IFD at the maker-note
    // offset, value offsets relative to the host TIFF base.
    if mn.len() >= 2 && !mn.starts_with(b"Nikon") {
        walk_ifd(r, mn_off, NIKON_MAIN, "Nikon", special, out);
    }
}
