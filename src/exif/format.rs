//! EXIF/TIFF value format types (the numbers stored in each IFD entry).
//!
//! Ported from `@formatSize` / `@formatName` in ExifTool's Exif.pm.

use crate::reader::Reader;
use crate::value::Value;

/// Size in bytes of one element of the given EXIF format code, or None if unknown.
pub fn format_size(fmt: u16) -> Option<usize> {
    Some(match fmt {
        1 => 1,  // int8u
        2 => 1,  // string
        3 => 2,  // int16u
        4 => 4,  // int32u
        5 => 8,  // rational64u
        6 => 1,  // int8s
        7 => 1,  // undef
        8 => 2,  // int16s
        9 => 4,  // int32s
        10 => 8, // rational64s
        11 => 4, // float
        12 => 8, // double
        13 => 4, // ifd (offset)
        14 => 2, // unicode
        15 => 8, // complex
        16 => 8, // int64u
        17 => 8, // int64s
        18 => 8, // ifd64
        129 => 1, // utf8 (Exif 3.0)
        _ => return None,
    })
}

/// Decode `count` elements of `fmt` starting at `off` within `r`.
pub fn read_value(r: &Reader, fmt: u16, count: usize, off: usize) -> Option<Value> {
    let esize = format_size(fmt)?;
    let total = esize.checked_mul(count)?;
    // Make sure the whole region is present.
    r.bytes(off, total)?;

    Some(match fmt {
        // ASCII string / UTF-8 string
        2 | 129 => {
            let raw = r.bytes(off, total)?;
            // Trim at first NUL (EXIF strings are NUL-terminated), keep the rest as text.
            let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
            Value::Text(String::from_utf8_lossy(&raw[..end]).into_owned())
        }
        // undef / complex: keep raw bytes
        7 | 15 => Value::Bytes(r.bytes(off, total)?.to_vec()),
        // unsigned ints
        1 => Value::U((0..count).map(|i| r.u8(off + i).unwrap() as u64).collect()),
        3 => Value::U((0..count).map(|i| r.u16(off + i * 2).unwrap() as u64).collect()),
        4 | 13 => Value::U((0..count).map(|i| r.u32(off + i * 4).unwrap() as u64).collect()),
        16 | 18 => Value::U((0..count).map(|i| r.u64(off + i * 8).unwrap()).collect()),
        // signed ints
        6 => Value::I((0..count).map(|i| r.u8(off + i).unwrap() as i8 as i64).collect()),
        8 => Value::I((0..count).map(|i| r.i16(off + i * 2).unwrap() as i64).collect()),
        9 => Value::I((0..count).map(|i| r.i32(off + i * 4).unwrap() as i64).collect()),
        17 => Value::I((0..count).map(|i| r.i64(off + i * 8).unwrap()).collect()),
        // rationals
        5 => Value::R((0..count)
            .map(|i| {
                let n = r.u32(off + i * 8).unwrap() as i64;
                let d = r.u32(off + i * 8 + 4).unwrap() as i64;
                (n, d)
            })
            .collect()),
        10 => Value::R((0..count)
            .map(|i| {
                let n = r.i32(off + i * 8).unwrap() as i64;
                let d = r.i32(off + i * 8 + 4).unwrap() as i64;
                (n, d)
            })
            .collect()),
        // floats
        11 => Value::F((0..count)
            .map(|i| f32::from_bits(r.u32(off + i * 4).unwrap()) as f64)
            .collect()),
        12 => Value::F((0..count)
            .map(|i| f64::from_bits(r.u64(off + i * 8).unwrap()))
            .collect()),
        14 => {
            // unicode (UTF-16) — decode using the reader's byte order
            let units: Vec<u16> = (0..count).map(|i| r.u16(off + i * 2).unwrap()).collect();
            Value::Text(String::from_utf16_lossy(&units).trim_end_matches('\0').to_string())
        }
        _ => return None,
    })
}
