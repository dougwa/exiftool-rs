// Auto-generated from ExifTool (IFD maker-note table). Do not edit by hand.
use crate::makernotes::binary::Pc;
use crate::makernotes::{MnKind, MnTag};

static NIKON_MAIN_PC_0: &[(i64, &str)] = &[(0, "Off"), (1, "1.3x Crop"), (2, "DX Crop"), (3, "5:4 Crop"), (4, "3:2 Crop"), (6, "16:9 Crop"), (8, "2.7x Crop"), (9, "DX Movie 16:9 Crop"), (10, "1.3x Movie Crop"), (11, "FX Uncropped"), (12, "DX Uncropped"), (13, "2.8x Movie Crop"), (14, "1.4x Movie Crop"), (15, "1.5x Movie Crop"), (17, "FX 1:1 Crop"), (18, "DX 1:1 Crop")];
static NIKON_MAIN_PC_1: &[(i64, &str)] = &[(1, "sRGB"), (2, "Adobe RGB"), (4, "BT.2100")];
static NIKON_MAIN_PC_2: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static NIKON_MAIN_PC_3: &[(i64, &str)] = &[(0, "Off"), (1, "Low"), (3, "Normal"), (5, "High"), (7, "Extra High"), (8, "Extra High 1"), (9, "Extra High 2"), (10, "Extra High 3"), (11, "Extra High 4"), (65535, "Auto")];
static NIKON_MAIN_PC_4: &[(i64, &str)] = &[(0, "Off"), (1, "Low"), (3, "Normal"), (5, "High")];
static NIKON_MAIN_PC_5: &[(i64, &str)] = &[(0, "Mechanical"), (16, "Electronic"), (48, "Electronic Front Curtain"), (64, "Electronic (Movie)"), (80, "Auto (Mechanical)"), (81, "Auto (Electronic Front Curtain)"), (96, "Electronic (High Speed)")];
static NIKON_MAIN_PC_6: &[(i64, &str)] = &[(1, "Large"), (2, "Medium"), (3, "Small")];
static NIKON_MAIN_PC_7: &[(i64, &str)] = &[(1, "Size Priority"), (3, "Optimal Quality")];
static NIKON_MAIN_PC_8: &[(i64, &str)] = &[(0, "Did Not Fire"), (1, "Fired, Manual"), (3, "Not Ready"), (7, "Fired, External"), (8, "Fired, Commander Mode"), (9, "Fired, TTL Mode"), (18, "LED Light")];
static NIKON_MAIN_PC_9: &[(i64, &str)] = &[(1, "Lossy (type 1)"), (2, "Uncompressed"), (3, "Lossless"), (4, "Lossy (type 2)"), (5, "Striped packed 12 bits"), (6, "Uncompressed (reduced to 12 bit)"), (7, "Unpacked 12 bits"), (8, "Small"), (9, "Packed 12 bits"), (10, "Packed 14 bits"), (13, "High Efficiency"), (14, "High Efficiency*")];
static NIKON_MAIN_PC_10: &[(i64, &str)] = &[(0, "Off"), (1, "Date & Time"), (2, "Date"), (3, "Date Counter")];
static NIKON_MAIN_PC_11: &[(i64, &str)] = &[(0, "Off"), (1, "Minimal"), (2, "Low"), (3, "Medium Low"), (4, "Normal"), (5, "Medium High"), (6, "High")];
static NIKON_MAIN_PC_12: &[(i64, &str)] = &[(0, "Off"), (1, "On")];

pub static NIKON_MAIN: &[MnTag] = &[
    MnTag { id: 0x0001, kind: MnKind::Scalar { name: "MakerNoteVersion", pc: Pc::None } },
    MnTag { id: 0x0002, kind: MnKind::Scalar { name: "ISO", pc: Pc::None } },
    MnTag { id: 0x0003, kind: MnKind::Scalar { name: "ColorMode", pc: Pc::None } },
    MnTag { id: 0x0004, kind: MnKind::Scalar { name: "Quality", pc: Pc::None } },
    MnTag { id: 0x0005, kind: MnKind::Scalar { name: "WhiteBalance", pc: Pc::None } },
    MnTag { id: 0x0006, kind: MnKind::Scalar { name: "Sharpness", pc: Pc::None } },
    MnTag { id: 0x0007, kind: MnKind::Scalar { name: "FocusMode", pc: Pc::None } },
    MnTag { id: 0x0008, kind: MnKind::Scalar { name: "FlashSetting", pc: Pc::None } },
    MnTag { id: 0x0009, kind: MnKind::Scalar { name: "FlashType", pc: Pc::None } },
    MnTag { id: 0x000b, kind: MnKind::Scalar { name: "WhiteBalanceFineTune", pc: Pc::None } },
    MnTag { id: 0x000c, kind: MnKind::Scalar { name: "WB_RBLevels", pc: Pc::None } },
    MnTag { id: 0x000d, kind: MnKind::Scalar { name: "ProgramShift", pc: Pc::None } },
    MnTag { id: 0x000e, kind: MnKind::Scalar { name: "ExposureDifference", pc: Pc::None } },
    MnTag { id: 0x000f, kind: MnKind::Scalar { name: "ISOSelection", pc: Pc::None } },
    MnTag { id: 0x0010, kind: MnKind::Scalar { name: "DataDump", pc: Pc::None } },
    MnTag { id: 0x0012, kind: MnKind::Scalar { name: "FlashExposureComp", pc: Pc::None } },
    MnTag { id: 0x0013, kind: MnKind::Scalar { name: "ISOSetting", pc: Pc::None } },
    MnTag { id: 0x0016, kind: MnKind::Scalar { name: "ImageBoundary", pc: Pc::None } },
    MnTag { id: 0x0017, kind: MnKind::Scalar { name: "ExternalFlashExposureComp", pc: Pc::None } },
    MnTag { id: 0x0018, kind: MnKind::Scalar { name: "FlashExposureBracketValue", pc: Pc::None } },
    MnTag { id: 0x0019, kind: MnKind::Scalar { name: "ExposureBracketValue", pc: Pc::None } },
    MnTag { id: 0x001a, kind: MnKind::Scalar { name: "ImageProcessing", pc: Pc::None } },
    MnTag { id: 0x001b, kind: MnKind::Scalar { name: "CropHiSpeed", pc: Pc::Enum(NIKON_MAIN_PC_0) } },
    MnTag { id: 0x001c, kind: MnKind::Scalar { name: "ExposureTuning", pc: Pc::None } },
    MnTag { id: 0x001d, kind: MnKind::Scalar { name: "SerialNumber", pc: Pc::None } },
    MnTag { id: 0x001e, kind: MnKind::Scalar { name: "ColorSpace", pc: Pc::Enum(NIKON_MAIN_PC_1) } },
    MnTag { id: 0x0020, kind: MnKind::Scalar { name: "ImageAuthentication", pc: Pc::Enum(NIKON_MAIN_PC_2) } },
    MnTag { id: 0x0022, kind: MnKind::Scalar { name: "ActiveD-Lighting", pc: Pc::Enum(NIKON_MAIN_PC_3) } },
    MnTag { id: 0x002a, kind: MnKind::Scalar { name: "VignetteControl", pc: Pc::Enum(NIKON_MAIN_PC_4) } },
    MnTag { id: 0x0034, kind: MnKind::Scalar { name: "ShutterMode", pc: Pc::Enum(NIKON_MAIN_PC_5) } },
    MnTag { id: 0x0037, kind: MnKind::Scalar { name: "MechanicalShutterCount", pc: Pc::None } },
    MnTag { id: 0x003d, kind: MnKind::Scalar { name: "BlackLevel", pc: Pc::None } },
    MnTag { id: 0x003e, kind: MnKind::Scalar { name: "ImageSizeRAW", pc: Pc::Enum(NIKON_MAIN_PC_6) } },
    MnTag { id: 0x003f, kind: MnKind::Scalar { name: "WhiteBalanceFineTune", pc: Pc::None } },
    MnTag { id: 0x0044, kind: MnKind::Scalar { name: "JPGCompression", pc: Pc::Enum(NIKON_MAIN_PC_7) } },
    MnTag { id: 0x0045, kind: MnKind::Scalar { name: "CropArea", pc: Pc::None } },
    MnTag { id: 0x004f, kind: MnKind::Scalar { name: "ColorTemperatureAuto", pc: Pc::None } },
    MnTag { id: 0x0080, kind: MnKind::Scalar { name: "ImageAdjustment", pc: Pc::None } },
    MnTag { id: 0x0081, kind: MnKind::Scalar { name: "ToneComp", pc: Pc::None } },
    MnTag { id: 0x0082, kind: MnKind::Scalar { name: "AuxiliaryLens", pc: Pc::None } },
    MnTag { id: 0x0083, kind: MnKind::Scalar { name: "LensType", pc: Pc::None } },
    MnTag { id: 0x0085, kind: MnKind::Scalar { name: "ManualFocusDistance", pc: Pc::None } },
    MnTag { id: 0x0086, kind: MnKind::Scalar { name: "DigitalZoom", pc: Pc::None } },
    MnTag { id: 0x0087, kind: MnKind::Scalar { name: "FlashMode", pc: Pc::Enum(NIKON_MAIN_PC_8) } },
    MnTag { id: 0x0089, kind: MnKind::Scalar { name: "ShootingMode", pc: Pc::None } },
    MnTag { id: 0x008b, kind: MnKind::Scalar { name: "LensFStops", pc: Pc::None } },
    MnTag { id: 0x008c, kind: MnKind::Scalar { name: "ContrastCurve", pc: Pc::None } },
    MnTag { id: 0x008d, kind: MnKind::Scalar { name: "ColorHue", pc: Pc::None } },
    MnTag { id: 0x008f, kind: MnKind::Scalar { name: "SceneMode", pc: Pc::None } },
    MnTag { id: 0x0090, kind: MnKind::Scalar { name: "LightSource", pc: Pc::None } },
    MnTag { id: 0x0092, kind: MnKind::Scalar { name: "HueAdjustment", pc: Pc::None } },
    MnTag { id: 0x0093, kind: MnKind::Scalar { name: "NEFCompression", pc: Pc::Enum(NIKON_MAIN_PC_9) } },
    MnTag { id: 0x0094, kind: MnKind::Scalar { name: "SaturationAdj", pc: Pc::None } },
    MnTag { id: 0x0095, kind: MnKind::Scalar { name: "NoiseReduction", pc: Pc::None } },
    MnTag { id: 0x0096, kind: MnKind::Scalar { name: "NEFLinearizationTable", pc: Pc::None } },
    MnTag { id: 0x0099, kind: MnKind::Scalar { name: "RawImageCenter", pc: Pc::None } },
    MnTag { id: 0x009a, kind: MnKind::Scalar { name: "SensorPixelSize", pc: Pc::None } },
    MnTag { id: 0x009c, kind: MnKind::Scalar { name: "SceneAssist", pc: Pc::None } },
    MnTag { id: 0x009d, kind: MnKind::Scalar { name: "DateStampMode", pc: Pc::Enum(NIKON_MAIN_PC_10) } },
    MnTag { id: 0x009e, kind: MnKind::Scalar { name: "RetouchHistory", pc: Pc::None } },
    MnTag { id: 0x00a0, kind: MnKind::Scalar { name: "SerialNumber", pc: Pc::None } },
    MnTag { id: 0x00a2, kind: MnKind::Scalar { name: "ImageDataSize", pc: Pc::None } },
    MnTag { id: 0x00a5, kind: MnKind::Scalar { name: "ImageCount", pc: Pc::None } },
    MnTag { id: 0x00a6, kind: MnKind::Scalar { name: "DeletedImageCount", pc: Pc::None } },
    MnTag { id: 0x00a7, kind: MnKind::Scalar { name: "ShutterCount", pc: Pc::None } },
    MnTag { id: 0x00a9, kind: MnKind::Scalar { name: "ImageOptimization", pc: Pc::None } },
    MnTag { id: 0x00aa, kind: MnKind::Scalar { name: "Saturation", pc: Pc::None } },
    MnTag { id: 0x00ab, kind: MnKind::Scalar { name: "VariProgram", pc: Pc::None } },
    MnTag { id: 0x00ac, kind: MnKind::Scalar { name: "ImageStabilization", pc: Pc::None } },
    MnTag { id: 0x00ad, kind: MnKind::Scalar { name: "AFResponse", pc: Pc::None } },
    MnTag { id: 0x00b1, kind: MnKind::Scalar { name: "HighISONoiseReduction", pc: Pc::Enum(NIKON_MAIN_PC_11) } },
    MnTag { id: 0x00b3, kind: MnKind::Scalar { name: "ToningEffect", pc: Pc::None } },
    MnTag { id: 0x00b6, kind: MnKind::Scalar { name: "PowerUpTime", pc: Pc::None } },
    MnTag { id: 0x00bf, kind: MnKind::Scalar { name: "SilentPhotography", pc: Pc::Enum(NIKON_MAIN_PC_12) } },
    MnTag { id: 0x0e09, kind: MnKind::Scalar { name: "NikonCaptureVersion", pc: Pc::None } },
    MnTag { id: 0x0e22, kind: MnKind::Scalar { name: "NEFBitDepth", pc: Pc::None } },
];
