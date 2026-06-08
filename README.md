# exiftool-rs

A Rust reimplementation of the **core** of [ExifTool](https://exiftool.org/) —
Phil Harvey's Perl tool for reading and writing media-file metadata. This is a
from-scratch reverse-engineering of ExifTool's read/write paths, not a binding
or a port of the Perl. It reads the standard metadata that lives in the vast
majority of camera and phone files, prints it in ExifTool's format, and can
write common EXIF tags back into JPEG files.

ExifTool is ~327k lines of Perl across 193 format modules. No single project can
reproduce all of it at once. This crate faithfully implements the **shared
engine** that almost every format funnels through — the TIFF/EXIF IFD parser —
plus the most common container formats, and is structured so more formats and
maker-note modules can be layered on.

## What works

* **CLI** compatible with common ExifTool invocations:
  `exiftool-rs FILE`, `-json`/`-j`, `-G`/`-G1` (groups), `-s` (short names),
  `-n` (numeric/no PrintConv), `-a` (allow duplicates), `-TagName` filters,
  `-ver`; and for writing, `-TAG=VALUE` (set), `-TAG=` (delete), and
  `-overwrite_original`.
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
* **Composite tags** (the `Composite` group, `composite.rs`): post-extraction
  tags computed from other tags via a multi-pass dependency-ordered engine that
  resolves each composite's `Require`/`Desire` sources by name (respecting
  group-qualified sources like `Composite:DigitalZoom`). Ported formulas:
  `Aperture`, `ShutterSpeed` (incl. APEX `ShutterSpeedValue`/`ApertureValue`
  conversion), `ImageSize`, `Megapixels`, `ScaleFactor35efl` (with Canon's
  rational-denominator sensor-diagonal algorithm and the general
  focal-plane-resolution path), `CircleOfConfusion`, `FocalLength35efl`, `FOV`,
  `HyperfocalDistance`, `DOF`, and `LightValue`.
* **Writing EXIF tags to JPEG** (`exif/wtiff.rs`, `wserialize.rs`, `writable.rs`,
  `writeconv.rs`). Because the read path is lossy (PrintConv strings, dropped
  unknown tags, flattened maker notes), writing uses its own **faithful,
  byte-round-trippable IFD model**: the EXIF block is parsed keeping every entry,
  edited, and re-serialized, then spliced back into the JPEG (an APP1 is created
  if the file had no EXIF). Maker notes, the IFD1 thumbnail, and unknown tags are
  carried through untouched — the relocated maker-note blob has its internal
  offsets **fixed up per vendor** (reusing the same signature/base classification
  the reader uses), so files validate cleanly. A curated writable-tag table
  covers the common IFD0/ExifIFD/GPS tags with **inverse value conversions**
  (dates, GPS decimal degrees → DMS rationals with an auto-set N/S/E/W reference,
  f-number, exposure time, orientation names). Set with `-TAG=VALUE`, delete with
  `-TAG=`; the original is preserved as `FILE_original` unless
  `-overwrite_original` is given. Verified against the reference tool: for an
  identical edit across the test-suite JPEGs, **36 of 41** produce output that
  validates exactly as `exiftool`'s own write does (the rest need ExifTool's deep
  per-vendor maker-note *rewriting* — see below).

## Parity

Measured against the reference `exiftool` across the JPEG images in ExifTool's
own test suite, comparing every shared tag's printed value:

```
all test JPEGs   2672 exact tag-value matches
Canon.jpg         133 / 162 shared tags (AFInfo AF-point arrays + composites)
Olympus2.jpg      116 / 145 (Equipment / CameraSettings / FocusInfo sub-IFDs)
Pentax.jpg        128 / 205 (CameraSettings / AEInfo / LensInfo / … records)
```

With maker-note support for thirteen vendors plus composite tags, exiftool-rs
now extracts several hundred more correct tags across the suite than the
EXIF-only foundation did (e.g. Sanyo 93, Pentax 99, Panasonic 87, FujiFilm 77
shared-tag matches). The remaining differences are things still not implemented
(see below): vendor binary **sub-records** (which is where the `FocusDistance`
that would complete `FOV`/`DOF` lives), the full lens databases, and ExifTool's
cross-group **priority/duplicate** system (the maker-note vs EXIF `MeteringMode`
case).

## Not implemented (by design, for now)

* **Some vendor binary sub-records.** The maker-note `SubDirectory` mechanism is
  implemented — both nested IFDs (`MnKind::SubIfd`, e.g. Olympus
  Equipment/CameraSettings/FocusInfo/ImageProcessing) and ProcessBinaryData
  blocks (Canon AFInfo, Minolta CameraSettings, Pentax
  CameraSettings/AEInfo/LensInfo/FlashInfo/…), including **variable-format**
  records whose element count depends on a sibling tag (Canon AFInfo's
  `int16s[$val{NumAFPoints}]` AF-point arrays, via a running `varSize`). Still
  to do: Canon ColorData (version-keyed offsets), Nikon AFInfo/LensData, and
  Sony's Tag9xxx records (where Sony keeps nearly everything). A number of
  sub-record tags are extracted but not yet fully PrintConv-formatted (the same
  long tail of per-vendor ValueConv/PrintConv formulas as the main IFDs).
* **Writing — scope and gaps.** Writing covers **EXIF tags in JPEG** (set and
  delete); the following are not yet done:
  * **Containers** — only JPEG. Standalone TIFF/RAW writing (the serializer is
    container-agnostic; mostly needs container wiring + StripOffsets relocation)
    and non-EXIF blocks (XMP, IPTC, ICC) are not implemented.
  * **Writable tags** — a curated set across IFD0/ExifIFD/GPS (`writable.rs`);
    trivially extensible but not exhaustive.
  * **Maker-note internals** — maker notes are preserved as opaque blobs
    (relocated with per-vendor offset fixup); individual maker-note *tags* cannot
    be edited.
  * **Per-vendor maker-note rewriting** — ExifTool fully rewrites maker notes; we
    preserve them. The 5 of 41 suite JPEGs that don't match ExifTool's write are
    all this: **Canon 1D-series** footer + `OriginalDecisionData` pointer,
    **AFCP/MIE trailer** offset fixups (`AFCP.jpg`, `ExifTool.jpg`), **GE**'s
    embedded big-endian TIFF (left untouched — safe, 2 minor warnings), and
    **Olympus2**'s preview pointer that was already broken in the original. Each
    is a single/double *minor* `-validate` warning with the data intact.
  * **Other** — EXIF APP1 is capped at one 64 KB segment; no `+=`/`-=` list
    operators; inverse PrintConv only for the common tags.
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
