//! The curated set of EXIF tags `exiftool-rs` can write.
//!
//! This is deliberately separate from the generated read table (`table_exif.rs`,
//! which carries no format/writability info). Each entry routes a tag name to
//! the IFD it belongs in, its tag id and storage format, and the conversion used
//! to turn user input into stored bytes (see [`super::writeconv`]). The set is
//! easily extended; it intentionally covers the tags people actually write.

/// Which IFD a writable tag lives in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Loc {
    /// IFD0 (the main image directory).
    Ifd0,
    /// The EXIF sub-IFD (ExifOffset 0x8769).
    Exif,
    /// The GPS sub-IFD (GPSInfo 0x8825).
    Gps,
}

/// How a tag's input string is converted to stored bytes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Conv {
    /// ASCII string, NUL-terminated.
    Ascii,
    /// Date/time string `YYYY:MM:DD HH:MM:SS`, stored as ASCII.
    Date,
    /// Integer stored in this entry's `fmt` (int8u/int16u/int32u).
    Int,
    /// Unsigned rational (decimal or `n/d`).
    URational,
    /// Aperture (`f/2.8` or `2.8`) -> rational.
    FNumber,
    /// Exposure time (`1/200` or `0.005`) -> rational.
    Exposure,
    /// GPS coordinate (signed decimal degrees) -> 3 rationals + sibling Ref tag.
    GpsCoord,
    /// GPS coordinate reference (`N`/`S`/`E`/`W`), stored as ASCII.
    GpsRef,
    /// Orientation, accepting the number or common PrintConv names.
    Orientation,
    /// EXIF UserComment (8-byte charset prefix + text), stored as undef.
    UserComment,
}

/// A writable tag definition.
#[derive(Clone, Copy, Debug)]
pub struct Writable {
    pub name: &'static str,
    pub ifd: Loc,
    pub id: u16,
    /// EXIF format code (used for `Conv::Int`; other convs fix their own format).
    pub fmt: u16,
    pub conv: Conv,
}

use Conv::*;
use Loc::*;

/// The writable-tag table.
pub static WRITABLE: &[Writable] = &[
    // ---- IFD0 ----
    Writable { name: "ImageDescription", ifd: Ifd0, id: 0x010e, fmt: 2, conv: Ascii },
    Writable { name: "Make",             ifd: Ifd0, id: 0x010f, fmt: 2, conv: Ascii },
    Writable { name: "Model",            ifd: Ifd0, id: 0x0110, fmt: 2, conv: Ascii },
    Writable { name: "Orientation",      ifd: Ifd0, id: 0x0112, fmt: 3, conv: Orientation },
    Writable { name: "XResolution",      ifd: Ifd0, id: 0x011a, fmt: 5, conv: URational },
    Writable { name: "YResolution",      ifd: Ifd0, id: 0x011b, fmt: 5, conv: URational },
    Writable { name: "Software",         ifd: Ifd0, id: 0x0131, fmt: 2, conv: Ascii },
    Writable { name: "ModifyDate",       ifd: Ifd0, id: 0x0132, fmt: 2, conv: Date },
    Writable { name: "Artist",           ifd: Ifd0, id: 0x013b, fmt: 2, conv: Ascii },
    Writable { name: "Copyright",        ifd: Ifd0, id: 0x8298, fmt: 2, conv: Ascii },
    // ---- ExifIFD ----
    Writable { name: "ExposureTime",     ifd: Exif, id: 0x829a, fmt: 5, conv: Exposure },
    Writable { name: "FNumber",          ifd: Exif, id: 0x829d, fmt: 5, conv: FNumber },
    Writable { name: "ISO",              ifd: Exif, id: 0x8827, fmt: 3, conv: Int },
    Writable { name: "DateTimeOriginal", ifd: Exif, id: 0x9003, fmt: 2, conv: Date },
    Writable { name: "CreateDate",       ifd: Exif, id: 0x9004, fmt: 2, conv: Date },
    Writable { name: "UserComment",      ifd: Exif, id: 0x9286, fmt: 7, conv: UserComment },
    Writable { name: "FocalLength",      ifd: Exif, id: 0x920a, fmt: 5, conv: URational },
    Writable { name: "LensModel",        ifd: Exif, id: 0xa434, fmt: 2, conv: Ascii },
    // ---- GPS ----
    Writable { name: "GPSLatitudeRef",   ifd: Gps, id: 0x0001, fmt: 2, conv: GpsRef },
    Writable { name: "GPSLatitude",      ifd: Gps, id: 0x0002, fmt: 5, conv: GpsCoord },
    Writable { name: "GPSLongitudeRef",  ifd: Gps, id: 0x0003, fmt: 2, conv: GpsRef },
    Writable { name: "GPSLongitude",     ifd: Gps, id: 0x0004, fmt: 5, conv: GpsCoord },
    Writable { name: "GPSAltitudeRef",   ifd: Gps, id: 0x0005, fmt: 1, conv: Int },
    Writable { name: "GPSAltitude",      ifd: Gps, id: 0x0006, fmt: 5, conv: URational },
    Writable { name: "GPSDateStamp",     ifd: Gps, id: 0x001d, fmt: 2, conv: Ascii },
];

/// Look up a writable tag by name (case-insensitive).
pub fn lookup(name: &str) -> Option<&'static Writable> {
    WRITABLE.iter().find(|w| w.name.eq_ignore_ascii_case(name))
}
