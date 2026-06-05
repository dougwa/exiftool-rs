//! Tag-table definitions and name/description lookup.
//!
//! The static tables in `table_exif.rs` / `table_gps.rs` are generated directly
//! from ExifTool's Perl source so the tag id -> name mapping stays faithful.

/// What a tag points at, when it is a sub-directory (a nested IFD).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubDir {
    None,
    /// A nested EXIF IFD using the same tag table (ExifOffset, InteropOffset...).
    Ifd,
    /// A GPS IFD (uses the GPS tag table).
    Gps,
    /// A sub-directory we recognise structurally but do not yet parse.
    Skip,
}

#[derive(Clone, Copy, Debug)]
pub struct TagDef {
    pub id: u32,
    pub name: &'static str,
    pub sub: SubDir,
}

/// Which tag table a (sub-)IFD is using.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Table {
    Exif,
    Gps,
}

impl Table {
    fn defs(self) -> &'static [TagDef] {
        match self {
            Table::Exif => super::table_exif::EXIF_MAIN,
            Table::Gps => super::table_gps::GPS_MAIN,
        }
    }

    pub fn lookup(self, id: u16) -> Option<&'static TagDef> {
        let id = id as u32;
        self.defs().iter().find(|t| t.id == id)
    }
}

/// Derive a human description from a tag name, following ExifTool's
/// `MakeDescription` rules (Exif.pm / ExifTool.pm).
pub fn make_description(name: &str) -> String {
    // Force first letter upper-case and translate underscores to spaces.
    let mut s: Vec<char> = name.chars().collect();
    if let Some(c) = s.first_mut() {
        *c = c.to_ascii_uppercase();
    }
    let s: String = s
        .into_iter()
        .map(|c| if c == '_' { ' ' } else { c })
        .collect();
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let is_upper = |c: char| c.is_ascii_uppercase();
    let is_lower = |c: char| c.is_ascii_lowercase();
    let is_digit = |c: char| c.is_ascii_digit();

    for i in 0..chars.len() {
        let c = chars[i];
        let prev = if i > 0 { Some(chars[i - 1]) } else { None };
        let next = chars.get(i + 1).copied();
        let next2 = chars.get(i + 2).copied();

        if let Some(p) = prev {
            // lower/digit followed by Upper or digit:  s/([a-z])([A-Z\d])/$1 $2/g
            if is_lower(p) && (is_upper(c) || is_digit(c)) {
                out.push(' ');
            }
            // acronym then word: s/([A-Z])([A-Z][a-z])/$1 $2/g
            else if is_upper(p) && is_upper(c) && next.map(is_lower).unwrap_or(false) {
                out.push(' ');
            }
            // number then word: s/(\d)([A-Z]\S)/$1 $2/g
            else if is_digit(p)
                && is_upper(c)
                && next.map(|n| !n.is_whitespace()).unwrap_or(false)
                && next2.is_some()
            {
                out.push(' ');
            }
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn descriptions() {
        assert_eq!(make_description("FNumber"), "F Number");
        assert_eq!(make_description("XResolution"), "X Resolution");
        assert_eq!(make_description("ISO"), "ISO");
        assert_eq!(make_description("GPSLatitude"), "GPS Latitude");
        assert_eq!(make_description("ExposureTime"), "Exposure Time");
        assert_eq!(make_description("YCbCrPositioning"), "Y Cb Cr Positioning");
    }
}
