//! File-type identification.
//!
//! ExifTool first guesses a type from the file extension, then confirms it with
//! a magic-number test on the leading bytes (`%magicNumber` in ExifTool.pm).
//! Here we run the magic test directly and fall back to the extension. Only the
//! types we can meaningfully report are enumerated; the list is easy to extend.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileType {
    /// Canonical type, e.g. "JPEG".
    pub typ: &'static str,
    /// Preferred extension reported as "File Type Extension".
    pub ext: &'static str,
    /// MIME type.
    pub mime: &'static str,
}

const fn ft(typ: &'static str, ext: &'static str, mime: &'static str) -> FileType {
    FileType { typ, ext, mime }
}

/// Identify a file from its leading bytes (and extension as a tiebreaker).
pub fn identify(buf: &[u8], ext: Option<&str>) -> Option<FileType> {
    // Magic-number checks, ordered roughly by specificity.
    if buf.len() >= 3 && &buf[0..3] == b"\xff\xd8\xff" {
        return Some(ft("JPEG", "jpg", "image/jpeg"));
    }
    if buf.len() >= 8 && &buf[0..8] == b"\x89PNG\r\n\x1a\n" {
        return Some(ft("PNG", "png", "image/png"));
    }
    if buf.len() >= 6 && (&buf[0..6] == b"GIF87a" || &buf[0..6] == b"GIF89a") {
        return Some(ft("GIF", "gif", "image/gif"));
    }
    if buf.len() >= 2 && &buf[0..2] == b"BM" {
        return Some(ft("BMP", "bmp", "image/bmp"));
    }
    // TIFF (and many TIFF-based RAWs share this header; refine by extension).
    if buf.len() >= 4 && (&buf[0..4] == b"II\x2a\x00" || &buf[0..4] == b"MM\x00\x2a") {
        return Some(tiff_variant(ext));
    }
    if buf.len() >= 4 && (&buf[0..4] == b"II\x2b\x00" || &buf[0..4] == b"MM\x00\x2b") {
        return Some(ft("BigTIFF", "btf", "image/x-tiff-big"));
    }
    // ISO base media (MP4/MOV/HEIC...) — "ftyp" at offset 4.
    if buf.len() >= 12 && &buf[4..8] == b"ftyp" {
        return Some(iso_bmff(&buf[8..12]));
    }
    if buf.len() >= 12 && &buf[0..4] == b"RIFF" {
        return Some(riff_variant(&buf[8..12]));
    }
    if buf.len() >= 4 && &buf[0..4] == b"%PDF" {
        return Some(ft("PDF", "pdf", "application/pdf"));
    }
    if buf.len() >= 4 && &buf[0..4] == b"8BPS" {
        return Some(ft("PSD", "psd", "application/vnd.adobe.photoshop"));
    }
    // EXIF blob written standalone (".exv"/raw EXIF) starts like TIFF too; handled above.

    // Fall back to extension-only mapping for a best-effort File Type.
    ext.and_then(by_extension)
}

fn tiff_variant(ext: Option<&str>) -> FileType {
    match ext.map(|e| e.to_ascii_lowercase()).as_deref() {
        Some("cr2") => ft("CR2", "cr2", "image/x-canon-cr2"),
        Some("nef") => ft("NEF", "nef", "image/x-nikon-nef"),
        Some("nrw") => ft("NRW", "nrw", "image/x-nikon-nrw"),
        Some("arw") => ft("ARW", "arw", "image/x-sony-arw"),
        Some("dng") => ft("DNG", "dng", "image/x-adobe-dng"),
        Some("orf") => ft("ORF", "orf", "image/x-olympus-orf"),
        Some("rw2") => ft("RW2", "rw2", "image/x-panasonic-rw2"),
        Some("pef") => ft("PEF", "pef", "image/x-pentax-pef"),
        Some("srw") => ft("SRW", "srw", "image/x-samsung-srw"),
        _ => ft("TIFF", "tif", "image/tiff"),
    }
}

fn iso_bmff(brand: &[u8]) -> FileType {
    match brand {
        b"heic" | b"heix" | b"heim" | b"heis" => ft("HEIC", "heic", "image/heic"),
        b"mif1" | b"msf1" => ft("HEIF", "heif", "image/heif"),
        b"avif" => ft("AVIF", "avif", "image/avif"),
        b"crx " => ft("CR3", "cr3", "image/x-canon-cr3"),
        b"qt  " => ft("MOV", "mov", "video/quicktime"),
        _ => ft("MP4", "mp4", "video/mp4"),
    }
}

fn riff_variant(form: &[u8]) -> FileType {
    match form {
        b"WEBP" => ft("WEBP", "webp", "image/webp"),
        b"AVI " => ft("AVI", "avi", "video/x-msvideo"),
        b"WAVE" => ft("WAV", "wav", "audio/x-wav"),
        _ => ft("RIFF", "riff", "application/octet-stream"),
    }
}

fn by_extension(ext: &str) -> Option<FileType> {
    let e = ext.to_ascii_lowercase();
    Some(match e.as_str() {
        "jpg" | "jpeg" | "jpe" => ft("JPEG", "jpg", "image/jpeg"),
        "png" => ft("PNG", "png", "image/png"),
        "gif" => ft("GIF", "gif", "image/gif"),
        "tif" | "tiff" => ft("TIFF", "tif", "image/tiff"),
        "bmp" => ft("BMP", "bmp", "image/bmp"),
        "pdf" => ft("PDF", "pdf", "application/pdf"),
        "mp4" => ft("MP4", "mp4", "video/mp4"),
        "mov" => ft("MOV", "mov", "video/quicktime"),
        "heic" => ft("HEIC", "heic", "image/heic"),
        "webp" => ft("WEBP", "webp", "image/webp"),
        _ => return None,
    })
}
