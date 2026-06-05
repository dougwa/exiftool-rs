// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Fmt, Pc};

static CANON_SHOT_INFO_PC_0: &[(i64, &str)] = &[(0, "Auto"), (1, "Daylight"), (2, "Cloudy"), (3, "Tungsten"), (4, "Fluorescent"), (5, "Flash"), (6, "Custom"), (7, "Black & White"), (8, "Shade"), (9, "Manual Temperature (Kelvin)"), (10, "PC Set1"), (11, "PC Set2"), (12, "PC Set3"), (14, "Daylight Fluorescent"), (15, "Custom 1"), (16, "Custom 2"), (17, "Underwater"), (18, "Custom 3"), (19, "Custom 4"), (20, "PC Set4"), (21, "PC Set5"), (23, "Auto (ambience priority)")];
static CANON_SHOT_INFO_PC_1: &[(i64, &str)] = &[(-1, "n/a"), (0, "Off"), (1, "Night Scene"), (2, "On"), (3, "None")];
static CANON_SHOT_INFO_PC_2: &[(i64, &str)] = &[(12288, "None (MF)"), (12289, "Right"), (12290, "Center"), (12291, "Center+Right"), (12292, "Left"), (12293, "Left+Right"), (12294, "Left+Center"), (12295, "All")];
static CANON_SHOT_INFO_PC_3: &[(i64, &str)] = &[(-1, "On"), (0, "Off"), (1, "On (shot 1)"), (2, "On (shot 2)"), (3, "On (shot 3)")];
static CANON_SHOT_INFO_PC_4: &[(i64, &str)] = &[(0, "n/a"), (1, "Camera Local Control"), (3, "Computer Remote Control")];
static CANON_SHOT_INFO_PC_5: &[(i64, &str)] = &[(0, "n/a"), (248, "EOS High-end"), (250, "Compact"), (252, "EOS Mid-range"), (255, "DV Camera")];
static CANON_SHOT_INFO_PC_6: &[(i64, &str)] = &[(-1, "n/a"), (0, "None"), (1, "Rotate 90 CW"), (2, "Rotate 180"), (3, "Rotate 270 CW")];
static CANON_SHOT_INFO_PC_7: &[(i64, &str)] = &[(-1, "n/a"), (0, "Off"), (1, "On")];

pub static CANON_SHOT_INFO: BinTable = BinTable {
    default_fmt: Fmt::S16,
    first_entry: 1,
    tags: &[
    BinTag { index: 1, name: "AutoISO", fmt: None, pc: Pc::None },
    BinTag { index: 2, name: "BaseISO", fmt: None, pc: Pc::None },
    BinTag { index: 3, name: "MeasuredEV", fmt: None, pc: Pc::None },
    BinTag { index: 4, name: "TargetAperture", fmt: None, pc: Pc::None },
    BinTag { index: 5, name: "TargetExposureTime", fmt: None, pc: Pc::None },
    BinTag { index: 6, name: "ExposureCompensation", fmt: None, pc: Pc::None },
    BinTag { index: 7, name: "WhiteBalance", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_0) },
    BinTag { index: 8, name: "SlowShutter", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_1) },
    BinTag { index: 9, name: "SequenceNumber", fmt: None, pc: Pc::None },
    BinTag { index: 10, name: "OpticalZoomCode", fmt: None, pc: Pc::None },
    BinTag { index: 12, name: "CameraTemperature", fmt: None, pc: Pc::None },
    BinTag { index: 13, name: "FlashGuideNumber", fmt: None, pc: Pc::None },
    BinTag { index: 14, name: "AFPointsInFocus", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_2) },
    BinTag { index: 15, name: "FlashExposureComp", fmt: None, pc: Pc::None },
    BinTag { index: 16, name: "AutoExposureBracketing", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_3) },
    BinTag { index: 17, name: "AEBBracketValue", fmt: None, pc: Pc::None },
    BinTag { index: 18, name: "ControlMode", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_4) },
    BinTag { index: 19, name: "FocusDistanceUpper", fmt: Some(Fmt::U16), pc: Pc::None },
    BinTag { index: 20, name: "FocusDistanceLower", fmt: Some(Fmt::U16), pc: Pc::None },
    BinTag { index: 21, name: "FNumber", fmt: None, pc: Pc::None },
    BinTag { index: 23, name: "MeasuredEV2", fmt: None, pc: Pc::None },
    BinTag { index: 24, name: "BulbDuration", fmt: None, pc: Pc::None },
    BinTag { index: 26, name: "CameraType", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_5) },
    BinTag { index: 27, name: "AutoRotate", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_6) },
    BinTag { index: 28, name: "NDFilter", fmt: None, pc: Pc::Enum(CANON_SHOT_INFO_PC_7) },
    BinTag { index: 29, name: "SelfTimer2", fmt: None, pc: Pc::None },
    BinTag { index: 33, name: "FlashOutput", fmt: None, pc: Pc::None },
    ],
};
