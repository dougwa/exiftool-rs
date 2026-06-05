// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};

static CANON_SHOT_INFO_PC_0: &[(i64, &str)] = &[(15, "Custom 1"), (3, "Tungsten"), (19, "Custom 4"), (10, "PC Set1"), (20, "PC Set4"), (18, "Custom 3"), (4, "Fluorescent"), (5, "Flash"), (1, "Daylight"), (11, "PC Set2"), (9, "Manual Temperature (Kelvin)"), (21, "PC Set5"), (2, "Cloudy"), (14, "Daylight Fluorescent"), (23, "Auto (ambience priority)"), (16, "Custom 2"), (6, "Custom"), (7, "Black & White"), (8, "Shade"), (12, "PC Set3"), (17, "Underwater"), (0, "Auto")];
static CANON_SHOT_INFO_PC_1: &[(i64, &str)] = &[(2, "On"), (3, "None"), (0, "Off"), (1, "Night Scene"), (-1, "n/a")];
static CANON_SHOT_INFO_PC_2: &[(i64, &str)] = &[(12291, "Center+Right"), (12295, "All"), (12292, "Left"), (12290, "Center"), (12293, "Left+Right"), (12288, "None (MF)"), (12289, "Right"), (12294, "Left+Center")];
static CANON_SHOT_INFO_PC_3: &[(i64, &str)] = &[(1, "On (shot 1)"), (-1, "On"), (2, "On (shot 2)"), (3, "On (shot 3)"), (0, "Off")];
static CANON_SHOT_INFO_PC_4: &[(i64, &str)] = &[(0, "n/a"), (3, "Computer Remote Control"), (1, "Camera Local Control")];
static CANON_SHOT_INFO_PC_5: &[(i64, &str)] = &[(250, "Compact"), (248, "EOS High-end"), (255, "DV Camera"), (252, "EOS Mid-range"), (0, "n/a")];
static CANON_SHOT_INFO_PC_6: &[(i64, &str)] = &[(2, "Rotate 180"), (0, "None"), (3, "Rotate 270 CW"), (1, "Rotate 90 CW"), (-1, "n/a")];
static CANON_SHOT_INFO_PC_7: &[(i64, &str)] = &[(0, "Off"), (-1, "n/a"), (1, "On")];

pub static CANON_SHOT_INFO: BinTable = BinTable {
    default_fmt: Fmt::S16,
    first_entry: 1,
    tags: &[
    BinTag { index: 1, name: "AutoISO", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 2, name: "BaseISO", fmt: None, pc: Pc::None, skip: Skip::Eq(0), count: Count::Fixed(1) },
    BinTag { index: 3, name: "MeasuredEV", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 4, name: "TargetAperture", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 5, name: "TargetExposureTime", fmt: None, pc: Pc::None, skip: Skip::Le(-1001), count: Count::Fixed(1) },
    BinTag { index: 6, name: "ExposureCompensation", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 7, name: "WhiteBalance", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_0), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 8, name: "SlowShutter", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_1), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 9, name: "SequenceNumber", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 10, name: "OpticalZoomCode", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 12, name: "CameraTemperature", fmt: None, pc: Pc::None, skip: Skip::Eq(0), count: Count::Fixed(1) },
    BinTag { index: 13, name: "FlashGuideNumber", fmt: None, pc: Pc::None, skip: Skip::Eq(-1), count: Count::Fixed(1) },
    BinTag { index: 14, name: "AFPointsInFocus", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_2), skip: Skip::Eq(0), count: Count::Fixed(1) },
    BinTag { index: 15, name: "FlashExposureComp", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 16, name: "AutoExposureBracketing", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_3), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 17, name: "AEBBracketValue", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 18, name: "ControlMode", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_4), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 19, name: "FocusDistanceUpper", fmt: Some(Fmt::U16), pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 20, name: "FocusDistanceLower", fmt: Some(Fmt::U16), pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 21, name: "FNumber", fmt: None, pc: Pc::None, skip: Skip::Eq(0), count: Count::Fixed(1) },
    BinTag { index: 22, name: "ExposureTime", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 23, name: "MeasuredEV2", fmt: None, pc: Pc::None, skip: Skip::Eq(0), count: Count::Fixed(1) },
    BinTag { index: 24, name: "BulbDuration", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 26, name: "CameraType", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_5), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 27, name: "AutoRotate", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_6), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 28, name: "NDFilter", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_7), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 29, name: "SelfTimer2", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 33, name: "FlashOutput", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
