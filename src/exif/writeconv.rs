//! Inverse value conversions: turn user-supplied text into the bytes stored in
//! an IFD entry. This is the write-direction counterpart to `printconv`.

use crate::error::{Error, Result};
use crate::reader::ByteOrder;
use super::writable::{Conv, Writable};

/// The result of encoding one assignment: the primary entry plus any sibling
/// entries it implies (e.g. setting GPSLatitude also sets GPSLatitudeRef).
pub struct Encoded {
    pub fmt: u16,
    pub bytes: Vec<u8>,
    pub extra: Vec<ExtraTag>,
}

pub struct ExtraTag {
    pub id: u16,
    pub fmt: u16,
    pub bytes: Vec<u8>,
}

fn err(msg: impl Into<String>) -> Error {
    Error::Write(msg.into())
}

/// Encode `input` for writable tag `w` in the given byte `order`.
pub fn encode(w: &Writable, input: &str, order: ByteOrder) -> Result<Encoded> {
    let plain = |fmt, bytes| Encoded { fmt, bytes, extra: Vec::new() };
    match w.conv {
        Conv::Ascii | Conv::Date => Ok(plain(2, ascii_z(input))),
        Conv::GpsRef => Ok(plain(2, ascii_z(&input.trim().to_uppercase()))),
        Conv::Int => Ok(plain(w.fmt, encode_int(w.name, input, w.fmt, order)?)),
        Conv::Orientation => {
            let n = orientation_value(input)?;
            Ok(plain(3, enc_u16(order, n).to_vec()))
        }
        Conv::URational => {
            let (n, d) = to_rational(input)?;
            Ok(plain(5, rational_bytes(order, &[(n, d)])))
        }
        Conv::FNumber => {
            let f = input.trim().trim_start_matches(['f', 'F', '/']).trim();
            let (n, d) = to_rational(f)?;
            Ok(plain(5, rational_bytes(order, &[(n, d)])))
        }
        Conv::Exposure => {
            let (n, d) = to_rational(input)?;
            Ok(plain(5, rational_bytes(order, &[(n, d)])))
        }
        Conv::UserComment => {
            // EXIF UserComment: an 8-byte character-code prefix then the text.
            let mut bytes = b"ASCII\0\0\0".to_vec();
            bytes.extend_from_slice(input.as_bytes());
            Ok(plain(7, bytes))
        }
        Conv::GpsCoord => {
            let deg = input
                .trim()
                .parse::<f64>()
                .map_err(|_| err(format!("invalid GPS coordinate: {input:?} (expected decimal degrees)")))?;
            let (refs, mag) = if w.id == 0x0002 {
                // latitude
                (if deg < 0.0 { "S" } else { "N" }, deg.abs())
            } else {
                // longitude
                (if deg < 0.0 { "W" } else { "E" }, deg.abs())
            };
            let d = mag.trunc();
            let m_full = (mag - d) * 60.0;
            let m = m_full.trunc();
            let s = (m_full - m) * 60.0;
            let (sn, sd) = decimal_to_rational(s);
            let parts = [(d as u64, 1u64), (m as u64, 1u64), (sn, sd)];
            let ref_id = if w.id == 0x0002 { 0x0001 } else { 0x0003 };
            Ok(Encoded {
                fmt: 5,
                bytes: rational_bytes(order, &parts),
                extra: vec![ExtraTag { id: ref_id, fmt: 2, bytes: ascii_z(refs) }],
            })
        }
    }
}

/// NUL-terminated ASCII bytes (EXIF strings store the trailing NUL).
fn ascii_z(s: &str) -> Vec<u8> {
    let mut b = s.as_bytes().to_vec();
    b.push(0);
    b
}

fn encode_int(name: &str, input: &str, fmt: u16, order: ByteOrder) -> Result<Vec<u8>> {
    let v: i64 = input
        .trim()
        .parse()
        .map_err(|_| err(format!("{name}: expected an integer, got {input:?}")))?;
    Ok(match fmt {
        1 => vec![v as u8],                      // int8u
        3 => enc_u16(order, v as u16).to_vec(),  // int16u
        4 => enc_u32(order, v as u32).to_vec(),  // int32u
        _ => return Err(err(format!("{name}: unsupported integer format {fmt}"))),
    })
}

/// Parse a rational from `n/d`, an integer, or a decimal.
fn to_rational(input: &str) -> Result<(u64, u64)> {
    let s = input.trim();
    if let Some((n, d)) = s.split_once('/') {
        let n: u64 = n.trim().parse().map_err(|_| err(format!("invalid rational: {input:?}")))?;
        let d: u64 = d.trim().parse().map_err(|_| err(format!("invalid rational: {input:?}")))?;
        return Ok((n, d.max(1)));
    }
    let f: f64 = s.parse().map_err(|_| err(format!("invalid number: {input:?}")))?;
    if f < 0.0 {
        return Err(err(format!("value must be non-negative: {input:?}")));
    }
    Ok(decimal_to_rational(f))
}

/// Approximate a non-negative float as an unsigned rational. Exact integers map
/// to `(n, 1)`; otherwise scale by 1e6 and reduce, which reproduces the common
/// EXIF rationals (e.g. 2.8 -> 14/5, 0.005 -> 1/200) exactly.
fn decimal_to_rational(f: f64) -> (u64, u64) {
    if f == f.trunc() && f < u64::MAX as f64 {
        return (f as u64, 1);
    }
    let den: u64 = 1_000_000;
    let num = (f * den as f64).round() as u64;
    let g = gcd(num, den).max(1);
    let (mut n, mut d) = (num / g, den / g);
    // Fall back to a coarse integer ratio if reduction still overflows u32
    // (rational fields are 32-bit each).
    if n > u32::MAX as u64 || d > u32::MAX as u64 {
        n = f.round() as u64;
        d = 1;
    }
    (n, d)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Serialize rationals as pairs of 32-bit num/den in `order`.
fn rational_bytes(order: ByteOrder, parts: &[(u64, u64)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(parts.len() * 8);
    for &(n, d) in parts {
        out.extend_from_slice(&enc_u32(order, n as u32));
        out.extend_from_slice(&enc_u32(order, d as u32));
    }
    out
}

/// Accept an Orientation as a number (1-8) or a common PrintConv name.
fn orientation_value(input: &str) -> Result<u16> {
    let s = input.trim();
    if let Ok(n) = s.parse::<u16>() {
        if (1..=8).contains(&n) {
            return Ok(n);
        }
    }
    let key = s.to_ascii_lowercase();
    Ok(match key.as_str() {
        "horizontal" | "horizontal (normal)" => 1,
        "mirror horizontal" => 2,
        "rotate 180" => 3,
        "mirror vertical" => 4,
        "mirror horizontal and rotate 270 cw" => 5,
        "rotate 90 cw" => 6,
        "mirror horizontal and rotate 90 cw" => 7,
        "rotate 270 cw" => 8,
        _ => return Err(err(format!("invalid Orientation: {input:?}"))),
    })
}

fn enc_u16(order: ByteOrder, v: u16) -> [u8; 2] {
    match order {
        ByteOrder::Little => v.to_le_bytes(),
        ByteOrder::Big => v.to_be_bytes(),
    }
}

fn enc_u32(order: ByteOrder, v: u32) -> [u8; 4] {
    match order {
        ByteOrder::Little => v.to_le_bytes(),
        ByteOrder::Big => v.to_be_bytes(),
    }
}
