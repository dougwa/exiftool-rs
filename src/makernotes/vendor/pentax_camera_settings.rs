// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};

static PENTAX_CAMERA_SETTINGS_PC_0: &[(i64, &str)] = &[(12, "Bulb, Off-Auto-Aperture"), (13, "Shutter & Aperture Priority AE"), (11, "Manual, Off-Auto-Aperture"), (15, "Sensitivity Priority AE"), (9, "Bulb"), (10, "Aperture Priority, Off-Auto-Aperture"), (3, "Green Mode"), (6, "Program Tv Shift"), (7, "Program Av Shift"), (2, "Program AE"), (5, "Aperture Priority"), (16, "Flash X-Sync Speed AE"), (1, "Auto PICT"), (0, "Scene Mode"), (8, "Manual"), (4, "Shutter Speed Priority")];
static PENTAX_CAMERA_SETTINGS_PC_1: &[(i64, &str)] = &[(8, "Slow-sync"), (0, "Normal"), (1, "Red-eye reduction"), (6, "Wireless (Control)"), (2, "Auto"), (5, "Wireless (Master)"), (9, "Slow-sync, Red-eye reduction"), (10, "Trailing-curtain Sync"), (3, "Auto, Red-eye reduction")];
static PENTAX_CAMERA_SETTINGS_PC_2: &[(i64, &str)] = &[(0, "Auto")];
static PENTAX_CAMERA_SETTINGS_PC_3: &[(i64, &str)] = &[(0, "Auto")];
static PENTAX_CAMERA_SETTINGS_PC_4: &[(i64, &str)] = &[(0, "Single-frame")];
static PENTAX_CAMERA_SETTINGS_PC_5: &[(i64, &str)] = &[(11, "1.3"), (13, "1.7"), (12, "1.5"), (8, "1.0"), (3, "0.3"), (16, "2.0"), (4, "0.5"), (5, "0.7")];
static PENTAX_CAMERA_SETTINGS_PC_6: &[(i64, &str)] = &[(0, "n/a"), (69, "5 of 5"), (35, "3 of 3"), (21, "2 of 5"), (53, "4 of 5"), (19, "2 of 3"), (3, "1 of 3"), (37, "3 of 5"), (18, "2 of 2"), (5, "1 of 5"), (2, "1 of 2")];
static PENTAX_CAMERA_SETTINGS_PC_7: &[(i64, &str)] = &[(13, "Set Color Temperature 2"), (12, "Set Color Temperature 1"), (2, "Shade"), (7, "Tungsten"), (5, "Day White Fluorescent"), (6, "White Fluorescent"), (14, "Set Color Temperature 3"), (3, "Cloudy"), (9, "Manual"), (8, "Flash"), (0, "Auto"), (1, "Daylight"), (4, "Daylight Fluorescent")];
static PENTAX_CAMERA_SETTINGS_PC_8: &[(i64, &str)] = &[(68, "RAW (PEF, Good)"), (36, "RAW (PEF, Better)"), (8, "RAW (DNG, Best)"), (1, "JPEG (Best)"), (72, "RAW (DNG, Good)"), (73, "RAW+JPEG (DNG, Good)"), (65, "JPEG (Good)"), (4, "RAW (PEF, Best)"), (69, "RAW+JPEG (PEF, Good)"), (33, "JPEG (Better)"), (40, "RAW (DNG, Better)"), (5, "RAW+JPEG (PEF, Best)"), (41, "RAW+JPEG (DNG, Better)"), (9, "RAW+JPEG (DNG, Best)"), (37, "RAW+JPEG (PEF, Better)")];
static PENTAX_CAMERA_SETTINGS_PC_9: &[(i64, &str)] = &[(1, "Red-eye reduction"), (0, "Normal"), (8, "Slow-sync"), (9, "Slow-sync, Red-eye reduction"), (10, "Trailing-curtain Sync"), (3, "Auto, Red-eye reduction"), (6, "Wireless (Control)"), (5, "Wireless (Master)"), (2, "Auto")];

pub static PENTAX_CAMERA_SETTINGS: BinTable = BinTable {
    default_fmt: Fmt::U8,
    first_entry: 0,
    tags: &[
    BinTag { index: 0, name: "PictureMode2", fmt: None, pc: Pc::Enum(PENTAX_CAMERA_SETTINGS_PC_0), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 2, name: "FlashOptions", fmt: None, pc: Pc::Enum(PENTAX_CAMERA_SETTINGS_PC_1), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 3, name: "AFPointMode", fmt: None, pc: Pc::Enum(PENTAX_CAMERA_SETTINGS_PC_2), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 4, name: "AFPointSelected2", fmt: Some(Fmt::U16), pc: Pc::Enum(PENTAX_CAMERA_SETTINGS_PC_3), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 6, name: "ISOFloor", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 7, name: "DriveMode2", fmt: None, pc: Pc::Enum(PENTAX_CAMERA_SETTINGS_PC_4), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 8, name: "ExposureBracketStepSize", fmt: None, pc: Pc::Enum(PENTAX_CAMERA_SETTINGS_PC_5), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 9, name: "BracketShotNumber", fmt: None, pc: Pc::Enum(PENTAX_CAMERA_SETTINGS_PC_6), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 10, name: "WhiteBalanceSet", fmt: None, pc: Pc::Enum(PENTAX_CAMERA_SETTINGS_PC_7), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 13, name: "RawAndJpgRecording", fmt: None, pc: Pc::Enum(PENTAX_CAMERA_SETTINGS_PC_8), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 16, name: "FlashOptions2", fmt: None, pc: Pc::Enum(PENTAX_CAMERA_SETTINGS_PC_9), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 18, name: "TvExposureTimeSetting", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 19, name: "AvApertureSetting", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 20, name: "SvISOSetting", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 21, name: "BaseExposureCompensation", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
