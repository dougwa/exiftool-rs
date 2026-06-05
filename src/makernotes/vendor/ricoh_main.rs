// Auto-generated from ExifTool via /tmp/dump_table.pl. Do not edit by hand.
use crate::makernotes::binary::{Pc, Skip};
use crate::makernotes::{MnKind, MnTag};

static RICOH_MAIN_PC_0: &[(i64, &str)] = &[(2, "JPEG"), (3, "DNG")];
static RICOH_MAIN_PC_1: &[(i64, &str)] = &[(0, "Single-frame"), (8, "AF-priority Continuous"), (1, "Continuous")];
static RICOH_MAIN_PC_2: &[(i64, &str)] = &[(2, "Soft"), (1, "Normal"), (0, "Sharp")];
static RICOH_MAIN_PC_3: &[(i64, &str)] = &[(2, "Multi AF"), (8, "Subject Tracking"), (5, "Infinity"), (4, "Snap"), (10, "Movie"), (7, "Face Detect"), (3, "Spot AF"), (1, "Manual"), (9, "Pinpoint AF")];
static RICOH_MAIN_PC_4: &[(i64, &str)] = &[(18, "WB2"), (11, "WB"), (16, "DR"), (19, "Effect"), (9, "AE"), (17, "Contrast"), (0, "Off")];
static RICOH_MAIN_PC_5: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static RICOH_MAIN_PC_6: &[(i64, &str)] = &[(3, "Auto, Fired, Red-eye reduction"), (1, "Auto, Fired"), (0, "Off"), (7, "Synchro, Red-eye reduction"), (4, "Slow Sync"), (8, "Auto, Did not fire"), (2, "On"), (6, "On, Red-eye reduction"), (5, "Manual")];
static RICOH_MAIN_PC_7: &[(i64, &str)] = &[(-24, "1/1.4"), (-72, "1/2.8"), (0, "Full"), (-216, "1/22"), (-120, "1/5.6"), (-192, "1/16"), (-96, "1/4"), (-48, "1/2"), (-144, "1/8"), (-288, "1/64"), (-168, "1/11"), (-240, "1/32")];
static RICOH_MAIN_PC_8: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static RICOH_MAIN_PC_9: &[(i64, &str)] = &[(3, "Weak"), (5, "Strong"), (4, "Medium"), (0, "Off")];
static RICOH_MAIN_PC_10: &[(i64, &str)] = &[(0, "Off"), (2, "Medium"), (1, "Weak"), (3, "Strong")];
static RICOH_MAIN_PC_11: &[(i64, &str)] = &[(11, "Positive Film"), (1, "Vivid"), (15, "Miniature"), (17, "High Key"), (0, "Standard"), (12, "Bleach Bypass"), (3, "Black & White"), (9, "High-contrast B&W"), (13, "Retro"), (5, "B&W Toning Effect"), (6, "Setting 1"), (10, "Cross Process"), (7, "Setting 2")];
static RICOH_MAIN_PC_12: &[(i64, &str)] = &[(0, "Off"), (3, "High"), (1, "Low"), (2, "Medium")];
static RICOH_MAIN_PC_13: &[(i64, &str)] = &[(2147483647, "MAX")];
static RICOH_MAIN_PC_14: &[(i64, &str)] = &[(4, "Blue"), (0, "Off"), (7, "Color"), (2, "Red"), (5, "Purple"), (6, "B&W"), (3, "Green"), (1, "Sepia")];
static RICOH_MAIN_PC_15: &[(i64, &str)] = &[(4, "Normal"), (0, "Off"), (2, "Magenta"), (5, "Warm"), (6, "Cool"), (1, "Basic"), (3, "Yellow")];
static RICOH_MAIN_PC_16: &[(i64, &str)] = &[(2, "Attached"), (0, "Not Attached")];
static RICOH_MAIN_PC_17: &[(i64, &str)] = &[(1, "On (35mm)"), (2, "On (47mm)"), (0, "Off")];
static RICOH_MAIN_PC_18: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static RICOH_MAIN_PC_19: &[(i64, &str)] = &[(1, "In Focus"), (0, "Out of Focus")];
static RICOH_MAIN_PC_20: &[(i64, &str)] = &[(0, "Auto"), (2, "Manual")];

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
