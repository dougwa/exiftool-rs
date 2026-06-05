# exiftool-rs

A Rust reimplementation of the **core** of [ExifTool](https://exiftool.org/) —
Phil Harvey's Perl tool for reading media-file metadata. This is a from-scratch
reverse-engineering of ExifTool's reading path, not a binding or a port of the
Perl. It reads the standard metadata that lives in the vast majority of camera
and phone files and prints it in ExifTool's format.

ExifTool is ~327k lines of Perl across 193 format modules. No single project can
reproduce all of it at once. This crate faithfully implements the **shared
engine** that almost every format funnels through — the TIFF/EXIF IFD parser —
plus the most common container formats, and is structured so more formats and
maker-note modules can be layered on.

## What works

* **CLI** compatible with common ExifTool invocations:
  `exiftool-rs FILE`, `-json`/`-j`, `-G`/`-G1` (groups), `-s` (short names),
  `-n` (numeric/no PrintConv), `-a` (allow duplicates), `-TagName` filters,
  `-ver`.
* **File-type detection** from magic numbers + extension (ported from ExifTool's
  `%magicNumber` / `%fileTypeLookup`): JPEG, TIFF (+ TIFF-based RAW: CR2, NEF,
  DNG, ARW, ORF, RW2, …), PNG, GIF, BMP, HEIC/AVIF/MP4/MOV, WebP, PDF, PSD.
* **TIFF/EXIF IFD engine** — the heart of ExifTool. Handles both byte orders,
  all EXIF format types (int/rational/float/undef/unicode…), inline vs.
  out-of-line values, the IFD0→IFD1 (thumbnail) chain, and recursion into the
  ExifIFD, GPS and Interop sub-IFDs.
* **Faithful tag tables**: the EXIF (587 tags) and GPS (32 tags) name tables are
  generated directly from ExifTool's Perl source (`Exif.pm`, `GPS.pm`).
* **PrintConv** value→human conversions for the common tags, ported exactly:
  enumerations (Orientation, ExposureProgram, Flash, MeteringMode, …), APEX
  conversions (aperture/shutter), `PrintExposureTime`, `PrintFNumber`,
  `PrintFraction`, `PrintLensInfo`, GPS coordinate formatting, `RoundFloat`
  (10-significant-figure rational rendering), UserComment charset decoding, and
  the `Unknown (N)` fallback.
* **Container parsers**: JPEG segment walker (EXIF/APP1, JFIF/APP0, comments,
  SOFn frame info), PNG chunks (IHDR, eXIf, tEXt), TIFF (whole-file IFD).
* **Maker notes** (EXIF tag 0x927c) for **Canon** and **Nikon**:
  * A `ProcessBinaryData` engine for the indexed binary records (Canon
    CameraSettings / ShotInfo / FocalLength), with tag tables generated from
    `Canon.pm`.
  * Canon's `Image::ExifTool::Canon::Main` IFD, including ported formula
    converters (APEX aperture via `CanonEv`, signed `printParameter`, self-timer,
    focus distance, camera ISO).
  * Nikon **Type 3** maker notes (the embedded `Nikon\0` sub-TIFF), with the
    `Nikon::Main` IFD table and the table-wide `FormatString` PrintConv
    (title-casing) plus string-keyed enumerations.
* **Filesystem pseudo-tags** (the `File`/`System` group): name, directory, size
  (`ConvertFileSize`), modify/access/inode-change timestamps in local time,
  permissions, file type, MIME type.

## Parity

Measured against the reference `exiftool` across the JPEG images in ExifTool's
own test suite, comparing every shared tag's printed value:

```
all test JPEGs   2139 exact tag-value matches vs  77 mismatches  →  96.5%
Canon.jpg         105 / 114 maker-note + EXIF tags
Nikon.jpg          64 /  66
```

With Canon and Nikon maker-note support, exiftool-rs now extracts ~150 more
correct tags across the suite than the EXIF-only foundation did. The remaining
differences are things still not implemented (see below): the long tail of
vendor binary sub-records, the full lens/model databases, and ExifTool's
cross-group **Composite/priority** system.

## Not implemented (by design, for now)

* **Other maker-note modules** (Sony, Olympus, Panasonic, …). Canon and Nikon
  are implemented; the remaining vendors are each a large per-vendor module.
  Within Canon/Nikon, the long tail of binary sub-records (AFInfo, LensData,
  ColorBalance, the full LensType/ModelID lens databases) is not yet ported.
* **Composite tags** and ExifTool's cross-group **priority/duplicate** system
  (e.g. the Composite `GPSAltitude` that merges altitude + reference, or
  SubSec-augmented dates).
* **Writing** metadata (this is read-only).
* Non-TIFF metadata blocks: XMP, IPTC, ICC profile, Photoshop IRB, and most
  audio/video container internals.
* BigTIFF (64-bit offsets), multi-offset `SubIFDs` lists.

## Architecture

```
src/
  filetype.rs     magic-number / extension identification
  formats/        per-container parsers (locate metadata blocks)
    jpeg.rs         JPEG marker-segment walker
    png.rs          PNG chunk parser
    tiff.rs         TIFF entry point
  exif/           the shared TIFF/EXIF IFD engine
    mod.rs          IFD walking + sub-IFD recursion + maker-note dispatch
    format.rs       EXIF value format types (@formatSize/@formatName)
    tags.rs         tag-table lookup + MakeDescription
    table_exif.rs   generated from Exif.pm
    table_gps.rs    generated from GPS.pm
    printconv.rs    value -> human conversions
  makernotes/     Canon and Nikon maker-note parsing
    binary.rs       ProcessBinaryData engine (+ Pc enums)
    canon.rs        Canon::Main IFD + ported formula converters
    canon/*.rs      generated binary tables (CameraSettings, ShotInfo, ...)
    nikon.rs        Nikon Type 3 sub-TIFF + FormatString PrintConv
    nikon/*.rs      generated Nikon::Main IFD table
  file_meta.rs    filesystem "File" pseudo-tags
  datetime.rs     local-time formatting (libc localtime_r, no deps)
  value.rs        Value enum + ExifTool-compatible number formatting
  tag.rs          ExtractedTag (group/name/value/print)
  cli.rs          arg parsing + aligned/JSON output
```

The whole crate has **zero external dependencies** (std only).

## Usage

```sh
cargo build --release

# Default human-readable output
./target/release/exiftool-rs photo.jpg

# JSON, with group prefixes, or specific tags
./target/release/exiftool-rs -j photo.jpg
./target/release/exiftool-rs -G1 photo.jpg
./target/release/exiftool-rs -FNumber -ISO -FocalLength photo.jpg
```

## Tests

```sh
cargo test
```

Integration tests read ExifTool's own sample images (from the adjacent
`../exiftool/t/images` checkout) and assert known tag values.

## Relationship to ExifTool

ExifTool is copyright Phil Harvey and licensed under the same terms as Perl.
This project is an independent reimplementation that reads the same file
structures; the generated tag tables are derived from ExifTool's published
source.
