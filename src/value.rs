//! Tag values.
//!
//! ExifTool keeps both a raw value and a print-converted (human) value for each
//! tag. We model the raw value as a small enum and derive the display string via
//! PrintConv. A value may hold one element or many (a "count" in EXIF terms).

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Unsigned integers (int8u/int16u/int32u/int64u).
    U(Vec<u64>),
    /// Signed integers (int8s/int16s/int32s/int64s).
    I(Vec<i64>),
    /// Floating point (float/double).
    F(Vec<f64>),
    /// Rationals: (numerator, denominator), signed-capable.
    R(Vec<(i64, i64)>),
    /// A text string (EXIF `string`/ASCII).
    Text(String),
    /// Raw/undefined bytes (`undef`).
    Bytes(Vec<u8>),
}

impl Value {
    /// Number of elements ("count").
    pub fn count(&self) -> usize {
        match self {
            Value::U(v) => v.len(),
            Value::I(v) => v.len(),
            Value::F(v) => v.len(),
            Value::R(v) => v.len(),
            Value::Text(_) => 1,
            Value::Bytes(b) => b.len(),
        }
    }

    /// First element as f64, if numeric.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::U(v) => v.first().map(|&x| x as f64),
            Value::I(v) => v.first().map(|&x| x as f64),
            Value::F(v) => v.first().copied(),
            Value::R(v) => v.first().map(|&(n, d)| {
                if d == 0 {
                    if n == 0 { f64::NAN } else { f64::INFINITY * n.signum() as f64 }
                } else {
                    n as f64 / d as f64
                }
            }),
            _ => None,
        }
    }

    /// First element as i64, if integral.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::U(v) => v.first().map(|&x| x as i64),
            Value::I(v) => v.first().copied(),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::U(v) => v.first().copied(),
            Value::I(v) => v.first().map(|&x| x as u64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s),
            _ => None,
        }
    }
}

/// Format a single rational the way ExifTool does. ExifTool reads rationals via
/// `RoundFloat(num/denom, 10)` — i.e. `sprintf("%.10g")` — so non-integer
/// rationals are limited to 10 significant figures.
fn fmt_rational(n: i64, d: i64) -> String {
    if d == 0 {
        return if n == 0 { "undef".into() } else { "inf".into() };
    }
    if n % d == 0 {
        return (n / d).to_string();
    }
    format_g(n as f64 / d as f64, 10)
}

/// Emulate C's `%.<sig>g`: `sig` significant figures, trailing zeros stripped,
/// switching to scientific notation when the exponent is < -4 or >= sig.
pub fn format_g(value: f64, sig: usize) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if !value.is_finite() {
        return format!("{value}");
    }
    let sig = sig.max(1);
    let neg = value < 0.0;
    let v = value.abs();
    let exp = v.log10().floor() as i32;

    let body = if exp < -4 || exp >= sig as i32 {
        // Scientific notation with sig-1 fractional digits, zeros trimmed.
        let mantissa = v / 10f64.powi(exp);
        let mut m = format!("{:.*}", sig - 1, mantissa);
        trim_zeros(&mut m);
        let esign = if exp < 0 { '-' } else { '+' };
        format!("{}e{}{:02}", m, esign, exp.abs())
    } else {
        let decimals = (sig as i32 - 1 - exp).max(0) as usize;
        let mut s = format!("{:.*}", decimals, v);
        trim_zeros(&mut s);
        s
    };
    if neg {
        format!("-{body}")
    } else {
        body
    }
}

fn trim_zeros(s: &mut String) {
    if s.contains('.') {
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
    }
}

/// Render a float the way ExifTool/Perl does: integers without a decimal point,
/// everything else as the shortest representation that round-trips (Perl uses
/// `%.15g`; Rust's default float formatting gives the shortest exact form, which
/// matches for the exact binary fractions common in EXIF rationals).
pub fn fmt_float(f: f64) -> String {
    if f == f.trunc() && f.abs() < 1e15 {
        return format!("{}", f as i64);
    }
    format!("{}", f)
}

impl fmt::Display for Value {
    /// The "raw"/converted value rendering (before PrintConv), space-separated
    /// for lists, matching ExifTool's value joining.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::U(v) => write!(f, "{}", join(v.iter().map(|x| x.to_string()))),
            Value::I(v) => write!(f, "{}", join(v.iter().map(|x| x.to_string()))),
            Value::F(v) => write!(f, "{}", join(v.iter().map(|x| fmt_float(*x)))),
            Value::R(v) => write!(f, "{}", join(v.iter().map(|(n, d)| fmt_rational(*n, *d)))),
            Value::Text(s) => write!(f, "{}", s.trim_end_matches('\0').trim_end()),
            Value::Bytes(b) => {
                write!(f, "(Binary data {} bytes, use -b option to extract)", b.len())
            }
        }
    }
}

fn join(it: impl Iterator<Item = String>) -> String {
    it.collect::<Vec<_>>().join(" ")
}
