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
* **Maker notes** (EXIF tag 0x927c) for **Canon**, **Nikon**, and a further
  **eleven vendors** (Olympus, Panasonic, FujiFilm, Minolta, Casio, Sanyo,
  Sigma, Ricoh, Pentax, and partial Sony):
  * A `ProcessBinaryData` engine for the indexed binary records (Canon
    CameraSettings / ShotInfo / FocalLength), with tag tables generated from
    `Canon.pm`.
  * Canon's `Image::ExifTool::Canon::Main` IFD, including ported formula
    converters (APEX aperture via `CanonEv`, signed `printParameter`, self-timer,
    focus distance, camera ISO, ShotInfo BaseISO/MeasuredEV/BulbDuration), the
    full `%canonModelID` lens-body lookup, and `RawConv => undef` n/a
    suppression.
  * Nikon **Type 3** maker notes (the embedded `Nikon\0` sub-TIFF), with the
    `Nikon::Main` IFD table and the table-wide `FormatString` PrintConv
    (title-casing) plus string-keyed enumerations.
  * A **generic vendor IFD dispatcher** (`makernotes/vendor.rs`) that recognises
    each vendor by its maker-note signature ("OLYMP\0", "Panasonic\0",
    "FUJIFILM", "SIGMA\0", "SANYO\0", "AOC\0", …) and resolves ExifTool's two
    layout knobs: the IFD **start** offset and whether out-of-line value offsets
    are **TIFF-base** or **maker-note-base** relative (ExifTool's
    `Base => '$start - N'`). Per-vendor formula converters port the common
    ValueConv/PrintConv (Olympus `SpecialMode`, Sigma `Label:` strip, Casio
    `ObjectDistance`, Panasonic/Pentax version strings, …).
  * All vendor IFD and binary tables are generated directly from the ExifTool
    Perl modules by **introspecting the loaded `%Main` hashes** (rather than
    regex-scraping the source), capturing enumerations, formats, the `Binary`
    flag, and `RawConv` suppression rules.
* **Filesystem pseudo-tags** (the `File`/`System` group): name, directory, size
  (`ConvertFileSize`), modify/access/inode-change timestamps in local time,
  permissions, file type, MIME type.

## Parity

Measured against the reference `exiftool` across the JPEG images in ExifTool's
own test suite, comparing every shared tag's printed value:

```
all test JPEGs   2304 exact tag-value matches vs  96 mismatches
Canon.jpg         113 / 162 shared tags (only the cross-group MeteringMode left)
Nikon.jpg          65 /  80
```

With maker-note support for thirteen vendors, exiftool-rs now extracts several
hundred more correct tags across the suite than the EXIF-only foundation did
(e.g. Sanyo 83, Pentax 89, Panasonic 77, FujiFilm 69 shared-tag matches, all
from a standing start of zero). The remaining differences are things still not
implemented (see below): the long tail of vendor binary **sub-records**, the
full lens databases, and ExifTool's cross-group **Composite/priority** system
(which accounts for the handful of remaining same-named mismatches such as
maker-note vs EXIF `MeteringMode`).

## Not implemented (by design, for now)

* **Vendor binary sub-records** (`SubDirectory` ProcessBinaryData blocks such as
  Canon AFInfo/ColorData, Nikon AFInfo/LensData, and Sony's Tag9xxx records).
  The main IFD of each of the thirteen supported vendors is parsed; the nested
  binary sub-records — which is where Sony in particular keeps nearly everything
  — are skipped for now. Variable-format records (whose element count depends on
  a sibling tag, e.g. Canon AFInfo) also need a richer binary engine.
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
