//! Nikon maker notes (`Image::ExifTool::Nikon::Main`).
//!
//! Modern Nikon maker notes (Type 3) begin with `Nikon\0` + a 4-byte version,
//! followed at offset 10 by a **complete, self-contained TIFF header** with its
//! own byte order and IFD-offset. Internal value offsets are relative to that
//! embedded header, so we parse it as an independent sub-TIFF. Older "headerless"
//! Nikon notes (no signature) are a bare IFD relative to the host TIFF base.

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

/// Nikon special converter: the table-default FormatString for strings, plus a
/// couple of formula tags.
pub fn special(name: &str, v: &Value) -> Option<String> {
    match name {
        // undef[4] of digit *values* (e.g. 00 01 00 00) -> "1.00".
        "MakerNoteVersion" => {
            if let Value::Bytes(b) = v {
                if b.len() == 4 && b.iter().all(|&c| c < 10) {
                    return Some(format!("{}.{}{}", b[0] as u32 * 10 + b[1] as u32, b[2], b[3]));
                }
            }
            None
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
