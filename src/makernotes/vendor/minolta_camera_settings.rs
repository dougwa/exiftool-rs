// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};

static MINOLTA_CAMERA_SETTINGS_PC_0: &[(i64, &str)] = &[(2, "Shutter Priority"), (3, "Manual"), (1, "Aperture Priority"), (0, "Program")];
static MINOLTA_CAMERA_SETTINGS_PC_1: &[(i64, &str)] = &[(1, "Red-eye reduction"), (0, "Fill flash"), (2, "Rear flash sync"), (4, "Off?"), (3, "Wireless")];
static MINOLTA_CAMERA_SETTINGS_PC_2: &[(i64, &str)] = &[(2, "1280x960"), (6, "2080x1560"), (7, "2560x1920"), (8, "3264x2176"), (3, "640x480"), (1, "1600x1200"), (0, "Full")];
static MINOLTA_CAMERA_SETTINGS_PC_3: &[(i64, &str)] = &[(3, "Standard"), (4, "Economy"), (1, "Super Fine"), (0, "Raw"), (2, "Fine"), (5, "Extra Fine")];
static MINOLTA_CAMERA_SETTINGS_PC_4: &[(i64, &str)] = &[(1, "Continuous"), (5, "Interval"), (0, "Single"), (2, "Self-timer"), (6, "UHS continuous"), (7, "HS continuous"), (4, "Bracketing")];
static MINOLTA_CAMERA_SETTINGS_PC_5: &[(i64, &str)] = &[(2, "Spot"), (0, "Multi-segment"), (1, "Center-weighted average")];
static MINOLTA_CAMERA_SETTINGS_PC_6: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static MINOLTA_CAMERA_SETTINGS_PC_7: &[(i64, &str)] = &[(0, "Off"), (1, "Electronic magnification"), (2, "2x")];
static MINOLTA_CAMERA_SETTINGS_PC_8: &[(i64, &str)] = &[(2, "1 EV"), (0, "1/3 EV"), (1, "2/3 EV")];
static MINOLTA_CAMERA_SETTINGS_PC_9: &[(i64, &str)] = &[(0, "No"), (1, "Yes")];
static MINOLTA_CAMERA_SETTINGS_PC_10: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static MINOLTA_CAMERA_SETTINGS_PC_11: &[(i64, &str)] = &[(0, "Normal")];
static MINOLTA_CAMERA_SETTINGS_PC_12: &[(i64, &str)] = &[(0, "Normal")];
static MINOLTA_CAMERA_SETTINGS_PC_13: &[(i64, &str)] = &[(2, "Soft"), (1, "Normal"), (0, "Hard")];
static MINOLTA_CAMERA_SETTINGS_PC_14: &[(i64, &str)] = &[(2, "Text"), (5, "Sports action"), (4, "Sunset"), (3, "Night portrait"), (1, "Portrait"), (0, "None")];
static MINOLTA_CAMERA_SETTINGS_PC_15: &[(i64, &str)] = &[(4, "Auto"), (3, "800"), (1, "200"), (0, "100"), (2, "400"), (5, "64")];
static MINOLTA_CAMERA_SETTINGS_PC_16: &[(i64, &str)] = &[(1, "DiMAGE 5"), (0, "DiMAGE 7, X1, X21 or X31"), (4, "DiMAGE 7i"), (3, "DiMAGE S404"), (5, "DiMAGE 7Hi"), (6, "DiMAGE A1"), (2, "DiMAGE S304"), (7, "DiMAGE A2 or S414")];
static MINOLTA_CAMERA_SETTINGS_PC_17: &[(i64, &str)] = &[(0, "Still Image"), (1, "Time-lapse Movie")];
static MINOLTA_CAMERA_SETTINGS_PC_18: &[(i64, &str)] = &[(1, "Data Form"), (0, "Standard Form")];
static MINOLTA_CAMERA_SETTINGS_PC_19: &[(i64, &str)] = &[(0, "Natural color"), (1, "Black & White"), (4, "Adobe RGB"), (3, "Solarization"), (2, "Vivid color")];
static MINOLTA_CAMERA_SETTINGS_PC_20: &[(i64, &str)] = &[(0, "No"), (1, "Fired")];
static MINOLTA_CAMERA_SETTINGS_PC_21: &[(i64, &str)] = &[(1, "Center zone (horizontal orientation)"), (0, "No zone"), (2, "Center zone (vertical orientation)"), (3, "Left zone"), (4, "Right zone")];
static MINOLTA_CAMERA_SETTINGS_PC_22: &[(i64, &str)] = &[(1, "MF"), (0, "AF")];
static MINOLTA_CAMERA_SETTINGS_PC_23: &[(i64, &str)] = &[(0, "Wide Focus (normal)"), (1, "Spot Focus")];
static MINOLTA_CAMERA_SETTINGS_PC_24: &[(i64, &str)] = &[(3, "Filter"), (2, "Saturation"), (0, "Exposure"), (1, "Contrast")];
static MINOLTA_CAMERA_SETTINGS_PC_25: &[(i64, &str)] = &[(0, "Not Embedded"), (1, "Embedded")];
static MINOLTA_CAMERA_SETTINGS_PC_26: &[(i64, &str)] = &[(0, "None"), (1, "YYYY/MM/DD"), (3, "Text"), (4, "Text + ID#"), (2, "MM/DD/HH:MM")];
static MINOLTA_CAMERA_SETTINGS_PC_27: &[(i64, &str)] = &[(0, "ADI (Advanced Distance Integration)"), (1, "Pre-flash TTL"), (2, "Manual flash control")];

pub static MINOLTA_CAMERA_SETTINGS: BinTable = BinTable {
    default_fmt: Fmt::U32,
    first_entry: 0,
    tags: &[
    BinTag { index: 1, name: "ExposureMode", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_0), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 2, name: "FlashMode", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_1), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 3, name: "WhiteBalance", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 4, name: "MinoltaImageSize", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_2), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 5, name: "MinoltaQuality", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_3), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 6, name: "DriveMode", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_4), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 7, name: "MeteringMode", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_5), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 8, name: "ISO", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 9, name: "ExposureTime", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 10, name: "FNumber", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 11, name: "MacroMode", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_6), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 12, name: "DigitalZoom", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_7), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 13, name: "ExposureCompensation", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 14, name: "BracketStep", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_8), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 16, name: "IntervalLength", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 17, name: "IntervalNumber", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 18, name: "FocalLength", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 19, name: "FocusDistance", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 20, name: "FlashFired", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_9), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 21, name: "MinoltaDate", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 22, name: "MinoltaTime", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 23, name: "MaxAperture", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 26, name: "FileNumberMemory", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_10), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 27, name: "LastFileNumber", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 28, name: "ColorBalanceRed", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 29, name: "ColorBalanceGreen", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 30, name: "ColorBalanceBlue", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 31, name: "Saturation", fmt: None, pc: Pc::EnumO(MINOLTA_CAMERA_SETTINGS_PC_11), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 32, name: "Contrast", fmt: None, pc: Pc::EnumO(MINOLTA_CAMERA_SETTINGS_PC_12), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 33, name: "Sharpness", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_13), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 34, name: "SubjectProgram", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_14), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 35, name: "FlashExposureComp", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 36, name: "ISOSetting", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_15), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 37, name: "MinoltaModelID", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_16), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 38, name: "IntervalMode", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_17), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 39, name: "FolderName", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_18), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 40, name: "ColorMode", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_19), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 41, name: "ColorFilter", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 42, name: "BWFilter", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 43, name: "InternalFlash", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_20), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 44, name: "Brightness", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 45, name: "SpotFocusPointX", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 46, name: "SpotFocusPointY", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 47, name: "WideFocusZone", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_21), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 48, name: "FocusMode", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_22), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 49, name: "FocusArea", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_23), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 50, name: "DECPosition", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_24), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 51, name: "ColorProfile", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_25), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 52, name: "DataImprint", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_26), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 63, name: "FlashMetering", fmt: None, pc: Pc::Enum(MINOLTA_CAMERA_SETTINGS_PC_27), skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
