// Auto-generated from ExifTool via /tmp/dump_table.pl. Do not edit by hand.
use crate::makernotes::binary::{Pc, Skip};
use crate::makernotes::{MnKind, MnTag};

static CASIO_MAIN_PC_0: &[(i64, &str)] = &[(3, "Night Scene"), (1, "Single Shutter"), (4, "Portrait"), (16, "Landscape"), (5, "Landscape"), (15, "Portrait"), (10, "Night Scene"), (7, "Panorama"), (2, "Panorama")];
static CASIO_MAIN_PC_1: &[(i64, &str)] = &[(209715, "3.2x"), (104857, "1.6x"), (131072, "2x"), (65536, "Off"), (65537, "2x"), (262144, "4x")];
static CASIO_MAIN_PC_2: &[(i64, &str)] = &[(2, "Hard"), (0, "Normal"), (18, "-1"), (1, "Soft"), (16, "Normal"), (17, "+1")];
static CASIO_MAIN_PC_3: &[(i64, &str)] = &[(16, "Normal"), (1, "Low"), (17, "+1"), (0, "Normal"), (2, "High"), (18, "-1")];
static CASIO_MAIN_PC_4: &[(i64, &str)] = &[(1, "Low"), (16, "Normal"), (17, "+1"), (2, "High"), (0, "Normal"), (18, "-1")];
static CASIO_MAIN_PC_5: &[(i64, &str)] = &[(3, "Fine"), (1, "Economy"), (2, "Normal")];
static CASIO_MAIN_PC_6: &[(i64, &str)] = &[(2, "Red"), (4, "Blue"), (1, "Off"), (5, "Flesh Tones"), (3, "Green")];
static CASIO_MAIN_PC_7: &[(i64, &str)] = &[(9, "Purple"), (7, "Yellow"), (6, "Blue"), (2, "Black & White"), (8, "Pink"), (5, "Green"), (4, "Red"), (3, "Sepia"), (1, "Off")];
static CASIO_MAIN_PC_8: &[(i64, &str)] = &[(17, "Bottom Right"), (3, "Upper Right"), (5, "Far Left/Right of Center"), (12, "Top Right"), (11, "Top Center"), (14, "Center Right"), (13, "Center Left"), (6, "Far Left/Right of Center/Bottom"), (2, "Upper Left"), (7, "Top Near-left"), (9, "Top Near-right"), (1, "Center"), (4, "Near Left/Right of Center"), (8, "Near Upper/Left"), (16, "Bottom Center"), (10, "Top Left"), (15, "Bottom Left")];
static CASIO_MAIN_PC_9: &[(i64, &str)] = &[(1, "Normal"), (3, "Strong"), (2, "Weak")];
static CASIO_MAIN_PC_10: &[(i64, &str)] = &[(7, "Spot AF"), (4, "Manual"), (2, "Macro"), (3, "Auto"), (5, "Infinity")];
static CASIO_MAIN_PC_11: &[(i64, &str)] = &[(2, "On"), (4, "Off"), (5, "Red-eye Reduction"), (1, "Auto"), (3, "Off")];
static CASIO_MAIN_PC_12: &[(i64, &str)] = &[(13, "Normal"), (12, "Low"), (11, "Weak"), (15, "Strong"), (14, "High")];
static CASIO_MAIN_PC_13: &[(i64, &str)] = &[(129, "Manual"), (4, "Fluorescent"), (2, "Tungsten"), (3, "Daylight"), (5, "Shade"), (1, "Auto")];

pub static CASIO_MAIN: &[MnTag] = &[
    MnTag { id: 1, kind: MnKind::Scalar { name: "RecordingMode", pc: Pc::Enum(CASIO_MAIN_PC_0), bin: false, skip: Skip::Never } },
    MnTag { id: 10, kind: MnKind::Scalar { name: "DigitalZoom", pc: Pc::Enum(CASIO_MAIN_PC_1), bin: false, skip: Skip::Never } },
    MnTag { id: 11, kind: MnKind::Scalar { name: "Sharpness", pc: Pc::Enum(CASIO_MAIN_PC_2), bin: false, skip: Skip::Never } },
    MnTag { id: 12, kind: MnKind::Scalar { name: "Contrast", pc: Pc::Enum(CASIO_MAIN_PC_3), bin: false, skip: Skip::Never } },
    MnTag { id: 13, kind: MnKind::Scalar { name: "Saturation", pc: Pc::Enum(CASIO_MAIN_PC_4), bin: false, skip: Skip::Never } },
    MnTag { id: 2, kind: MnKind::Scalar { name: "Quality", pc: Pc::Enum(CASIO_MAIN_PC_5), bin: false, skip: Skip::Never } },
    MnTag { id: 20, kind: MnKind::Scalar { name: "ISO", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 21, kind: MnKind::Scalar { name: "FirmwareDate", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 22, kind: MnKind::Scalar { name: "Enhancement", pc: Pc::Enum(CASIO_MAIN_PC_6), bin: false, skip: Skip::Never } },
    MnTag { id: 23, kind: MnKind::Scalar { name: "ColorFilter", pc: Pc::Enum(CASIO_MAIN_PC_7), bin: false, skip: Skip::Never } },
    MnTag { id: 24, kind: MnKind::Scalar { name: "AFPoint", pc: Pc::Enum(CASIO_MAIN_PC_8), bin: false, skip: Skip::Never } },
    MnTag { id: 25, kind: MnKind::Scalar { name: "FlashIntensity", pc: Pc::Enum(CASIO_MAIN_PC_9), bin: false, skip: Skip::Never } },
    MnTag { id: 3, kind: MnKind::Scalar { name: "FocusMode", pc: Pc::Enum(CASIO_MAIN_PC_10), bin: false, skip: Skip::Never } },
    MnTag { id: 4, kind: MnKind::Scalar { name: "FlashMode", pc: Pc::Enum(CASIO_MAIN_PC_11), bin: false, skip: Skip::Never } },
    MnTag { id: 5, kind: MnKind::Scalar { name: "FlashIntensity", pc: Pc::Enum(CASIO_MAIN_PC_12), bin: false, skip: Skip::Never } },
    MnTag { id: 6, kind: MnKind::Scalar { name: "ObjectDistance", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 7, kind: MnKind::Scalar { name: "WhiteBalance", pc: Pc::Enum(CASIO_MAIN_PC_13), bin: false, skip: Skip::Never } },
];
