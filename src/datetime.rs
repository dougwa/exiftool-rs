//! Local-time formatting without external crates.
//!
//! ExifTool prints filesystem timestamps as `YYYY:MM:DD HH:MM:SS±HH:MM` in the
//! local time zone. We convert a UNIX timestamp to local broken-down time via a
//! tiny FFI to `localtime_r`, which is available on macOS and Linux and exposes
//! the GMT offset in `tm_gmtoff`.

use std::time::SystemTime;

#[repr(C)]
struct Tm {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    tm_wday: i32,
    tm_yday: i32,
    tm_isdst: i32,
    tm_gmtoff: i64,
    tm_zone: *const i8,
}

extern "C" {
    fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
}

/// Format a `SystemTime` as ExifTool's date/time string in local time.
pub fn format_local(t: SystemTime) -> String {
    let secs = match t.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(e) => -(e.duration().as_secs() as i64),
    };
    // SAFETY: we pass valid pointers; localtime_r fills `tm` or returns null.
    let mut tm: Tm = unsafe { std::mem::zeroed() };
    let ok = unsafe { localtime_r(&secs as *const i64, &mut tm as *mut Tm) };
    if ok.is_null() {
        return String::new();
    }
    let off = tm.tm_gmtoff;
    let sign = if off < 0 { '-' } else { '+' };
    let off_abs = off.abs();
    let oh = off_abs / 3600;
    let om = (off_abs % 3600) / 60;
    format!(
        "{:04}:{:02}:{:02} {:02}:{:02}:{:02}{}{:02}:{:02}",
        tm.tm_year + 1900,
        tm.tm_mon + 1,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec,
        sign,
        oh,
        om
    )
}
