// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};

static PENTAX_AE_INFO_PC_0: &[(i64, &str)] = &[(160, "Program"), (0, "M, P or TAv"), (24, "MTF Program"), (67, "Sport"), (19, "DOF Program (P-Shift)"), (184, "Shallow DOF Program"), (59, "Macro"), (27, "MTF Program (P-Shift)"), (3, "Sv or Green Mode"), (144, "SCN"), (123, "Kids"), (91, "Night Scene"), (83, "No Flash"), (147, "Museum"), (2, "Tv"), (75, "Night Scene Portrait"), (51, "Landscape"), (139, "Candlelight"), (115, "Sunset"), (11, "Hi-speed Program (P-Shift)"), (216, "HDR"), (131, "Pet"), (35, "Standard"), (1, "Av, B or X"), (43, "Portrait"), (107, "Text"), (8, "Hi-speed Program"), (99, "Surf & Snow"), (16, "DOF Program"), (104, "Night Snap")];
static PENTAX_AE_INFO_PC_1: &[(i64, &str)] = &[(0, "Multi-segment")];
static PENTAX_AE_INFO_PC_2: &[(i64, &str)] = &[(8, "Unknown"), (7, "Tungsten"), (0, "Standard"), (3, "Cloudy"), (1, "Daylight"), (4, "Daylight Fluorescent"), (2, "Shade"), (6, "White Fluorescent"), (5, "Day White Fluorescent")];

pub static PENTAX_AE_INFO: BinTable = BinTable {
    default_fmt: Fmt::U8,
    first_entry: 0,
    tags: &[
    BinTag { index: 0, name: "AEExposureTime", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 1, name: "AEAperture", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 2, name: "AE_ISO", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 3, name: "AEXv", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 4, name: "AEBXv", fmt: Some(Fmt::S8), pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 5, name: "AEMinExposureTime", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 6, name: "AEProgramMode", fmt: None, pc: Pc::Enum(PENTAX_AE_INFO_PC_0), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 7, name: "AEFlags", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 8, name: "AEApertureSteps", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 9, name: "AEMaxAperture", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 10, name: "AEMaxAperture2", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 11, name: "AEMinAperture", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 12, name: "AEMeteringMode", fmt: None, pc: Pc::Enum(PENTAX_AE_INFO_PC_1), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 13, name: "AEWhiteBalance", fmt: None, pc: Pc::Enum(PENTAX_AE_INFO_PC_2), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 14, name: "FlashExposureCompSet", fmt: Some(Fmt::S8), pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 21, name: "LevelIndicator", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
