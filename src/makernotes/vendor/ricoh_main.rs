// Auto-generated from ExifTool via /tmp/dump_table.pl. Do not edit by hand.
use crate::makernotes::binary::{Pc, Skip};
use crate::makernotes::{MnKind, MnTag};

static RICOH_MAIN_PC_0: &[(i64, &str)] = &[(2, "JPEG"), (3, "DNG")];
static RICOH_MAIN_PC_1: &[(i64, &str)] = &[(8, "AF-priority Continuous"), (0, "Single-frame"), (1, "Continuous")];
static RICOH_MAIN_PC_2: &[(i64, &str)] = &[(0, "Sharp"), (2, "Soft"), (1, "Normal")];
static RICOH_MAIN_PC_3: &[(i64, &str)] = &[(5, "Infinity"), (9, "Pinpoint AF"), (7, "Face Detect"), (1, "Manual"), (2, "Multi AF"), (8, "Subject Tracking"), (4, "Snap"), (10, "Movie"), (3, "Spot AF")];
static RICOH_MAIN_PC_4: &[(i64, &str)] = &[(17, "Contrast"), (18, "WB2"), (9, "AE"), (19, "Effect"), (16, "DR"), (11, "WB"), (0, "Off")];
static RICOH_MAIN_PC_5: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static RICOH_MAIN_PC_6: &[(i64, &str)] = &[(4, "Slow Sync"), (2, "On"), (8, "Auto, Did not fire"), (0, "Off"), (6, "On, Red-eye reduction"), (3, "Auto, Fired, Red-eye reduction"), (5, "Manual"), (7, "Synchro, Red-eye reduction"), (1, "Auto, Fired")];
static RICOH_MAIN_PC_7: &[(i64, &str)] = &[(-192, "1/16"), (-216, "1/22"), (0, "Full"), (-144, "1/8"), (-288, "1/64"), (-72, "1/2.8"), (-48, "1/2"), (-120, "1/5.6"), (-24, "1/1.4"), (-96, "1/4"), (-168, "1/11"), (-240, "1/32")];
static RICOH_MAIN_PC_8: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static RICOH_MAIN_PC_9: &[(i64, &str)] = &[(5, "Strong"), (0, "Off"), (4, "Medium"), (3, "Weak")];
static RICOH_MAIN_PC_10: &[(i64, &str)] = &[(0, "Off"), (2, "Medium"), (3, "Strong"), (1, "Weak")];
static RICOH_MAIN_PC_11: &[(i64, &str)] = &[(10, "Cross Process"), (6, "Setting 1"), (0, "Standard"), (11, "Positive Film"), (7, "Setting 2"), (17, "High Key"), (15, "Miniature"), (13, "Retro"), (12, "Bleach Bypass"), (3, "Black & White"), (9, "High-contrast B&W"), (5, "B&W Toning Effect"), (1, "Vivid")];
static RICOH_MAIN_PC_12: &[(i64, &str)] = &[(0, "Off"), (2, "Medium"), (3, "High"), (1, "Low")];
static RICOH_MAIN_PC_13: &[(i64, &str)] = &[(2147483647, "MAX")];
static RICOH_MAIN_PC_14: &[(i64, &str)] = &[(6, "B&W"), (0, "Off"), (4, "Blue"), (2, "Red"), (3, "Green"), (5, "Purple"), (1, "Sepia"), (7, "Color")];
static RICOH_MAIN_PC_15: &[(i64, &str)] = &[(3, "Yellow"), (6, "Cool"), (0, "Off"), (4, "Normal"), (2, "Magenta"), (1, "Basic"), (5, "Warm")];
static RICOH_MAIN_PC_16: &[(i64, &str)] = &[(2, "Attached"), (0, "Not Attached")];
static RICOH_MAIN_PC_17: &[(i64, &str)] = &[(2, "On (47mm)"), (0, "Off"), (1, "On (35mm)")];
static RICOH_MAIN_PC_18: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static RICOH_MAIN_PC_19: &[(i64, &str)] = &[(0, "Out of Focus"), (1, "In Focus")];
static RICOH_MAIN_PC_20: &[(i64, &str)] = &[(2, "Manual"), (0, "Auto")];

pub static RICOH_MAIN: &[MnTag] = &[
    MnTag { id: 1, kind: MnKind::Scalar { name: "MakerNoteType", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 2, kind: MnKind::Scalar { name: "FirmwareVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4096, kind: MnKind::Scalar { name: "RecordingFormat", pc: Pc::Enum(RICOH_MAIN_PC_0), bin: false, skip: Skip::Never } },
    MnTag { id: 4098, kind: MnKind::Scalar { name: "DriveMode", pc: Pc::Enum(RICOH_MAIN_PC_1), bin: false, skip: Skip::Never } },
    MnTag { id: 4099, kind: MnKind::Scalar { name: "Sharpness", pc: Pc::Enum(RICOH_MAIN_PC_2), bin: false, skip: Skip::Never } },
    MnTag { id: 4100, kind: MnKind::Scalar { name: "WhiteBalanceFineTune", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4102, kind: MnKind::Scalar { name: "FocusMode", pc: Pc::Enum(RICOH_MAIN_PC_3), bin: false, skip: Skip::Never } },
    MnTag { id: 4103, kind: MnKind::Scalar { name: "AutoBracketing", pc: Pc::Enum(RICOH_MAIN_PC_4), bin: false, skip: Skip::Never } },
    MnTag { id: 4105, kind: MnKind::Scalar { name: "MacroMode", pc: Pc::Enum(RICOH_MAIN_PC_5), bin: false, skip: Skip::Never } },
    MnTag { id: 4106, kind: MnKind::Scalar { name: "FlashMode", pc: Pc::Enum(RICOH_MAIN_PC_6), bin: false, skip: Skip::Never } },
    MnTag { id: 4107, kind: MnKind::Scalar { name: "FlashExposureComp", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4108, kind: MnKind::Scalar { name: "ManualFlashOutput", pc: Pc::Enum(RICOH_MAIN_PC_7), bin: false, skip: Skip::Never } },
    MnTag { id: 4109, kind: MnKind::Scalar { name: "FullPressSnap", pc: Pc::Enum(RICOH_MAIN_PC_8), bin: false, skip: Skip::Never } },
    MnTag { id: 4110, kind: MnKind::Scalar { name: "DynamicRangeExpansion", pc: Pc::Enum(RICOH_MAIN_PC_9), bin: false, skip: Skip::Never } },
    MnTag { id: 4111, kind: MnKind::Scalar { name: "NoiseReduction", pc: Pc::Enum(RICOH_MAIN_PC_10), bin: false, skip: Skip::Never } },
    MnTag { id: 4112, kind: MnKind::Scalar { name: "ImageEffects", pc: Pc::Enum(RICOH_MAIN_PC_11), bin: false, skip: Skip::Never } },
    MnTag { id: 4113, kind: MnKind::Scalar { name: "Vignetting", pc: Pc::Enum(RICOH_MAIN_PC_12), bin: false, skip: Skip::Never } },
    MnTag { id: 4114, kind: MnKind::Scalar { name: "Contrast", pc: Pc::EnumO(RICOH_MAIN_PC_13), bin: false, skip: Skip::Never } },
    MnTag { id: 4115, kind: MnKind::Scalar { name: "Saturation", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4116, kind: MnKind::Scalar { name: "Sharpness", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4117, kind: MnKind::Scalar { name: "ToningEffect", pc: Pc::Enum(RICOH_MAIN_PC_14), bin: false, skip: Skip::Never } },
    MnTag { id: 4118, kind: MnKind::Scalar { name: "HueAdjust", pc: Pc::Enum(RICOH_MAIN_PC_15), bin: false, skip: Skip::Never } },
    MnTag { id: 4119, kind: MnKind::Scalar { name: "WideAdapter", pc: Pc::Enum(RICOH_MAIN_PC_16), bin: false, skip: Skip::Never } },
    MnTag { id: 4120, kind: MnKind::Scalar { name: "CropMode", pc: Pc::Enum(RICOH_MAIN_PC_17), bin: false, skip: Skip::Never } },
    MnTag { id: 4121, kind: MnKind::Scalar { name: "NDFilter", pc: Pc::Enum(RICOH_MAIN_PC_18), bin: false, skip: Skip::Never } },
    MnTag { id: 4122, kind: MnKind::Scalar { name: "WBBracketShotNumber", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4608, kind: MnKind::Scalar { name: "AFStatus", pc: Pc::Enum(RICOH_MAIN_PC_19), bin: false, skip: Skip::Never } },
    MnTag { id: 4609, kind: MnKind::Scalar { name: "AFAreaXPosition1", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4610, kind: MnKind::Scalar { name: "AFAreaYPosition1", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4611, kind: MnKind::Scalar { name: "AFAreaXPosition", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4612, kind: MnKind::Scalar { name: "AFAreaYPosition", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4613, kind: MnKind::Scalar { name: "AFAreaMode", pc: Pc::Enum(RICOH_MAIN_PC_20), bin: false, skip: Skip::Never } },
    MnTag { id: 4871, kind: MnKind::Scalar { name: "ColorTempKelvin", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4872, kind: MnKind::Scalar { name: "ColorTemperature", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 5, kind: MnKind::Scalar { name: "SerialNumber", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 5376, kind: MnKind::Scalar { name: "FocalLength", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 5633, kind: MnKind::Scalar { name: "SensorWidth", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 5634, kind: MnKind::Scalar { name: "SensorHeight", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 5635, kind: MnKind::Scalar { name: "CroppedImageWidth", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 5636, kind: MnKind::Scalar { name: "CroppedImageHeight", pc: Pc::None, bin: false, skip: Skip::Never } },
];
