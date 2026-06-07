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
* **Composite tags** (the `Composite` group, `composite.rs`): post-extraction
  tags computed from other tags via a multi-pass dependency-ordered engine that
  resolves each composite's `Require`/`Desire` sources by name (respecting
  group-qualified sources like `Composite:DigitalZoom`). Ported formulas:
  `Aperture`, `ShutterSpeed` (incl. APEX `ShutterSpeedValue`/`ApertureValue`
  conversion), `ImageSize`, `Megapixels`, `ScaleFactor35efl` (with Canon's
  rational-denominator sensor-diagonal algorithm and the general
  focal-plane-resolution path), `CircleOfConfusion`, `FocalLength35efl`, `FOV`,
  `HyperfocalDistance`, `DOF`, and `LightValue`.

## Parity

Measured against the reference `exiftool` across the JPEG images in ExifTool's
own test suite, comparing every shared tag's printed value:

```
all test JPEGs   2766 exact tag-value matches / 79 mismatches
Pentax.jpg        165 / 205 shared tags (CameraSettings / AEInfo / SR / … records)
Olympus2.jpg      127 / 145 (Equipment / CameraSettings / FocusInfo sub-IFDs)
Canon.jpg         133 / 162 (AFInfo AF-point arrays + composites)
NikonD70.jpg      111 / 155 (Lens / ShotInfo / AFInfo)
```

With maker-note support for thirteen vendors, composite tags, hand-ported
per-tag PrintConv/ValueConv converters, and ExifTool's tag **priority/duplicate**
resolution, the bulk of the EXIF + common maker-note surface now matches the
reference exactly. The remaining differences are catalogued under
[Known gaps](#known-gaps) below.

## Known gaps

The reading core is faithful, but parity is not complete. The remaining
suite-wide differences (~79 mismatched tag values, plus tags not yet extracted)
fall into the categories below, roughly highest-leverage first. Most are bounded
per-tag conversions; a few need new mechanisms.

**Per-tag PrintConv/ValueConv still to port** (the long tail; each goes in a
vendor `special()` converter):

* **Olympus** multi-value enum/bitfield tags in the CameraSettings/FocusInfo
  sub-IFDs (FocusMode, FocusProcess, AFAreas, Gradation, DriveMode, PanoramaMode,
  AFPoint, ExternalFlash). Blocked on the fact that the same tag name appears in
  several Olympus tables with different maps, which the name-keyed `special()`
  can't yet disambiguate.
* **Canon** ShotInfo/ColorData fields (TargetExposureTime, CameraTemperature,
  FlashOutput, FlashModel, LensType lens-database lookup, FocusDistanceUpper/
  Lower) — several need Canon's version-gated ColorData (`Condition` evaluation)
  which is not yet implemented.
* **Nikon** `CFAPattern` (shared EXIF `undef` decode) and `ExposureTuning`.
* Firmware / serial / time string formats (Ricoh `FirmwareVersion`, Panasonic
  `InternalSerialNumber` / `TimeSincePowerOn`).
* GPS altitude rounding, and sub-second / time-zone `DateTimeOriginal` (a
  composite of `SubSecTime` + `OffsetTime`).

**Mechanisms not yet built:**

* **Cross-group priority — partial.** ExifTool's duplicate resolution (keep the
  highest-priority tag per name) is implemented, but maker-note tags that should
  override EXIF are gated behind a small hand-verified allowlist
  (`TRUSTED_MAKER_OVERRIDES`); the general case wants per-tag/table `Priority`
  emitted into the generated tables.
* **BITMASK / `Mask` PrintConvs.** Currently flattened by the code generator to a
  partial enum and hand-patched to `Pc::None` + handled in `special()`; the
  generator should emit these directly.
* **Encrypted maker records:** Nikon ShotInfo/LensData (XOR-encrypted; this is
  where the `FocusDistance` that would complete Nikon `FOV`/`DOF` lives) and
  Pentax `ShutterCount` (obfuscated with Date/Time).
* **Canon ColorData** version-keyed offsets and the full lens databases.
* **Other metadata blocks:** XMP, IPTC, ICC profile, Photoshop IRB, FLIR thermal,
  and most audio/video container internals (the bulk of the not-yet-extracted
  tags).
* **Writing** metadata (this is read-only).
* BigTIFF (64-bit offsets), multi-offset `SubIFDs` lists, and ExifTool's
  `FixBase`/`ProcessUnknown` preview-offset correction (a few `PreviewImageStart`
  values are off by a small constant).

Some test files are intentional edge cases: `ExifTool.jpg` stores its real
metadata in a proprietary block (so Make/Model/dates read from the wrong place),
and a couple of vendor `DataDump` blobs are truncated.

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
