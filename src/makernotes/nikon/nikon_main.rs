// Auto-generated from ExifTool via /tmp/dump_table.pl. Do not edit by hand.
use crate::makernotes::binary::{Pc, Skip};
use crate::makernotes::{MnKind, MnTag};

static NIKON_MAIN_PC_0: &[(i64, &str)] = &[(9, "Fired, TTL Mode"), (0, "Did Not Fire"), (8, "Fired, Commander Mode"), (1, "Fired, Manual"), (18, "LED Light"), (3, "Not Ready"), (7, "Fired, External")];
static NIKON_MAIN_PC_1: &[(i64, &str)] = &[(2, "Uncompressed"), (14, "High Efficiency*"), (5, "Striped packed 12 bits"), (7, "Unpacked 12 bits"), (4, "Lossy (type 2)"), (8, "Small"), (9, "Packed 12 bits"), (13, "High Efficiency"), (3, "Lossless"), (1, "Lossy (type 1)"), (6, "Uncompressed (reduced to 12 bit)"), (10, "Packed 14 bits")];
static NIKON_MAIN_PC_2: &[(i64, &str)] = &[(1, "Date & Time"), (0, "Off"), (2, "Date"), (3, "Date Counter")];
static NIKON_MAIN_PC_3: &[(i64, &str)] = &[(1, "Minimal"), (4, "Normal"), (0, "Off"), (6, "High"), (5, "Medium High"), (3, "Medium Low"), (2, "Low")];
static NIKON_MAIN_PC_4: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static NIKON_MAIN_PC_5: &[(i64, &str)] = &[(1, "1.3x Crop"), (11, "FX Uncropped"), (6, "16:9 Crop"), (10, "1.3x Movie Crop"), (13, "2.8x Movie Crop"), (3, "5:4 Crop"), (12, "DX Uncropped"), (4, "3:2 Crop"), (8, "2.7x Crop"), (9, "DX Movie 16:9 Crop"), (15, "1.5x Movie Crop"), (0, "Off"), (18, "DX 1:1 Crop"), (14, "1.4x Movie Crop"), (17, "FX 1:1 Crop"), (2, "DX Crop")];
static NIKON_MAIN_PC_6: &[(i64, &str)] = &[(2, "Adobe RGB"), (4, "BT.2100"), (1, "sRGB")];
static NIKON_MAIN_PC_7: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static NIKON_MAIN_PC_8: &[(i64, &str)] = &[(3, "Normal"), (11, "Extra High 4"), (0, "Off"), (1, "Low"), (10, "Extra High 3"), (7, "Extra High"), (9, "Extra High 2"), (65535, "Auto"), (8, "Extra High 1"), (5, "High")];
static NIKON_MAIN_PC_9: &[(&str, &str)] = &[("14 0 0 0", "14"), ("8 8 8 0", "8 x 3"), ("0 0 0 0", "n/a (JPEG)"), ("12 0 0 0", "12"), ("16 16 16 0", "16 x 3")];
static NIKON_MAIN_PC_10: &[(i64, &str)] = &[(3, "Normal"), (5, "High"), (1, "Low"), (0, "Off")];
static NIKON_MAIN_PC_11: &[(i64, &str)] = &[(16, "Electronic"), (0, "Mechanical"), (96, "Electronic (High Speed)"), (80, "Auto (Mechanical)"), (48, "Electronic Front Curtain"), (81, "Auto (Electronic Front Curtain)"), (64, "Electronic (Movie)")];
static NIKON_MAIN_PC_12: &[(i64, &str)] = &[(1, "Large"), (2, "Medium"), (3, "Small")];
static NIKON_MAIN_PC_13: &[(i64, &str)] = &[(1, "Size Priority"), (3, "Optimal Quality")];

pub static NIKON_MAIN: &[MnTag] = &[
    MnTag { id: 1, kind: MnKind::Scalar { name: "MakerNoteVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 11, kind: MnKind::Scalar { name: "WhiteBalanceFineTune", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 12, kind: MnKind::Scalar { name: "WB_RBLevels", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 128, kind: MnKind::Scalar { name: "ImageAdjustment", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 129, kind: MnKind::Scalar { name: "ToneComp", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 13, kind: MnKind::Scalar { name: "ProgramShift", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 130, kind: MnKind::Scalar { name: "AuxiliaryLens", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 131, kind: MnKind::Scalar { name: "LensType", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 132, kind: MnKind::Scalar { name: "Lens", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 133, kind: MnKind::Scalar { name: "ManualFocusDistance", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 134, kind: MnKind::Scalar { name: "DigitalZoom", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 135, kind: MnKind::Scalar { name: "FlashMode", pc: Pc::Enum(NIKON_MAIN_PC_0), bin: false, skip: Skip::Never } },
    MnTag { id: 136, kind: MnKind::Binary(&super::nikon_afinfo::NIKON_AFINFO) },
    MnTag { id: 137, kind: MnKind::Scalar { name: "ShootingMode", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 139, kind: MnKind::Scalar { name: "LensFStops", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 14, kind: MnKind::Scalar { name: "ExposureDifference", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 140, kind: MnKind::Scalar { name: "ContrastCurve", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 141, kind: MnKind::Scalar { name: "ColorHue", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 143, kind: MnKind::Scalar { name: "SceneMode", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 144, kind: MnKind::Scalar { name: "LightSource", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 146, kind: MnKind::Scalar { name: "HueAdjustment", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 147, kind: MnKind::Scalar { name: "NEFCompression", pc: Pc::Enum(NIKON_MAIN_PC_1), bin: false, skip: Skip::Never } },
    MnTag { id: 148, kind: MnKind::Scalar { name: "SaturationAdj", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 149, kind: MnKind::Scalar { name: "NoiseReduction", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 15, kind: MnKind::Scalar { name: "ISOSelection", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 150, kind: MnKind::Scalar { name: "NEFLinearizationTable", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 153, kind: MnKind::Scalar { name: "RawImageCenter", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 154, kind: MnKind::Scalar { name: "SensorPixelSize", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 156, kind: MnKind::Scalar { name: "SceneAssist", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 157, kind: MnKind::Scalar { name: "DateStampMode", pc: Pc::Enum(NIKON_MAIN_PC_2), bin: false, skip: Skip::Never } },
    MnTag { id: 158, kind: MnKind::Scalar { name: "RetouchHistory", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 16, kind: MnKind::Scalar { name: "DataDump", pc: Pc::None, bin: true, skip: Skip::Never } },
    MnTag { id: 160, kind: MnKind::Scalar { name: "SerialNumber", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 162, kind: MnKind::Scalar { name: "ImageDataSize", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 165, kind: MnKind::Scalar { name: "ImageCount", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 166, kind: MnKind::Scalar { name: "DeletedImageCount", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 167, kind: MnKind::Scalar { name: "ShutterCount", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 169, kind: MnKind::Scalar { name: "ImageOptimization", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 170, kind: MnKind::Scalar { name: "Saturation", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 171, kind: MnKind::Scalar { name: "VariProgram", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 172, kind: MnKind::Scalar { name: "ImageStabilization", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 173, kind: MnKind::Scalar { name: "AFResponse", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 177, kind: MnKind::Scalar { name: "HighISONoiseReduction", pc: Pc::Enum(NIKON_MAIN_PC_3), bin: false, skip: Skip::Never } },
    MnTag { id: 179, kind: MnKind::Scalar { name: "ToningEffect", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 18, kind: MnKind::Scalar { name: "FlashExposureComp", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 182, kind: MnKind::Scalar { name: "PowerUpTime", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 19, kind: MnKind::Scalar { name: "ISOSetting", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 191, kind: MnKind::Scalar { name: "SilentPhotography", pc: Pc::Enum(NIKON_MAIN_PC_4), bin: false, skip: Skip::Never } },
    MnTag { id: 2, kind: MnKind::Scalar { name: "ISO", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 22, kind: MnKind::Scalar { name: "ImageBoundary", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 23, kind: MnKind::Scalar { name: "ExternalFlashExposureComp", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 24, kind: MnKind::Scalar { name: "FlashExposureBracketValue", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 25, kind: MnKind::Scalar { name: "ExposureBracketValue", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 26, kind: MnKind::Scalar { name: "ImageProcessing", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 27, kind: MnKind::Scalar { name: "CropHiSpeed", pc: Pc::EnumO(NIKON_MAIN_PC_5), bin: false, skip: Skip::Never } },
    MnTag { id: 28, kind: MnKind::Scalar { name: "ExposureTuning", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 29, kind: MnKind::Scalar { name: "SerialNumber", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 3, kind: MnKind::Scalar { name: "ColorMode", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 30, kind: MnKind::Scalar { name: "ColorSpace", pc: Pc::Enum(NIKON_MAIN_PC_6), bin: false, skip: Skip::Never } },
    MnTag { id: 32, kind: MnKind::Scalar { name: "ImageAuthentication", pc: Pc::Enum(NIKON_MAIN_PC_7), bin: false, skip: Skip::Never } },
    MnTag { id: 34, kind: MnKind::Scalar { name: "ActiveD-Lighting", pc: Pc::Enum(NIKON_MAIN_PC_8), bin: false, skip: Skip::Never } },
    MnTag { id: 3593, kind: MnKind::Scalar { name: "NikonCaptureVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 3618, kind: MnKind::Scalar { name: "NEFBitDepth", pc: Pc::EnumStr(NIKON_MAIN_PC_9), bin: false, skip: Skip::Never } },
    MnTag { id: 4, kind: MnKind::Scalar { name: "Quality", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 42, kind: MnKind::Scalar { name: "VignetteControl", pc: Pc::Enum(NIKON_MAIN_PC_10), bin: false, skip: Skip::Never } },
    MnTag { id: 5, kind: MnKind::Scalar { name: "WhiteBalance", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 52, kind: MnKind::Scalar { name: "ShutterMode", pc: Pc::Enum(NIKON_MAIN_PC_11), bin: false, skip: Skip::Never } },
    MnTag { id: 55, kind: MnKind::Scalar { name: "MechanicalShutterCount", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 6, kind: MnKind::Scalar { name: "Sharpness", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 61, kind: MnKind::Scalar { name: "BlackLevel", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 62, kind: MnKind::Scalar { name: "ImageSizeRAW", pc: Pc::Enum(NIKON_MAIN_PC_12), bin: false, skip: Skip::Never } },
    MnTag { id: 63, kind: MnKind::Scalar { name: "WhiteBalanceFineTune", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 68, kind: MnKind::Scalar { name: "JPGCompression", pc: Pc::Enum(NIKON_MAIN_PC_13), bin: false, skip: Skip::Never } },
    MnTag { id: 69, kind: MnKind::Scalar { name: "CropArea", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 7, kind: MnKind::Scalar { name: "FocusMode", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 79, kind: MnKind::Scalar { name: "ColorTemperatureAuto", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 8, kind: MnKind::Scalar { name: "FlashSetting", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 9, kind: MnKind::Scalar { name: "FlashType", pc: Pc::None, bin: false, skip: Skip::Never } },
];
