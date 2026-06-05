// Auto-generated from ExifTool via /tmp/dump_table.pl. Do not edit by hand.
use crate::makernotes::binary::{Pc, Skip};
use crate::makernotes::{MnKind, MnTag};

static CASIO_MAIN_PC_0: &[(i64, &str)] = &[(3, "Night Scene"), (4, "Portrait"), (5, "Landscape"), (15, "Portrait"), (2, "Panorama"), (1, "Single Shutter"), (7, "Panorama"), (16, "Landscape"), (10, "Night Scene")];
static CASIO_MAIN_PC_1: &[(i64, &str)] = &[(65537, "2x"), (209715, "3.2x"), (131072, "2x"), (262144, "4x"), (104857, "1.6x"), (65536, "Off")];
static CASIO_MAIN_PC_2: &[(i64, &str)] = &[(1, "Soft"), (2, "Hard"), (17, "+1"), (0, "Normal"), (18, "-1"), (16, "Normal")];
static CASIO_MAIN_PC_3: &[(i64, &str)] = &[(16, "Normal"), (18, "-1"), (0, "Normal"), (17, "+1"), (2, "High"), (1, "Low")];
static CASIO_MAIN_PC_4: &[(i64, &str)] = &[(1, "Low"), (2, "High"), (18, "-1"), (17, "+1"), (0, "Normal"), (16, "Normal")];
static CASIO_MAIN_PC_5: &[(i64, &str)] = &[(1, "Economy"), (2, "Normal"), (3, "Fine")];
static CASIO_MAIN_PC_6: &[(i64, &str)] = &[(3, "Green"), (4, "Blue"), (5, "Flesh Tones"), (1, "Off"), (2, "Red")];
static CASIO_MAIN_PC_7: &[(i64, &str)] = &[(1, "Off"), (7, "Yellow"), (3, "Sepia"), (5, "Green"), (4, "Red"), (2, "Black & White"), (6, "Blue"), (9, "Purple"), (8, "Pink")];
static CASIO_MAIN_PC_8: &[(i64, &str)] = &[(2, "Upper Left"), (5, "Far Left/Right of Center"), (8, "Near Upper/Left"), (6, "Far Left/Right of Center/Bottom"), (11, "Top Center"), (15, "Bottom Left"), (12, "Top Right"), (17, "Bottom Right"), (7, "Top Near-left"), (4, "Near Left/Right of Center"), (3, "Upper Right"), (10, "Top Left"), (9, "Top Near-right"), (14, "Center Right"), (1, "Center"), (13, "Center Left"), (16, "Bottom Center")];
static CASIO_MAIN_PC_9: &[(i64, &str)] = &[(3, "Strong"), (2, "Weak"), (1, "Normal")];
static CASIO_MAIN_PC_10: &[(i64, &str)] = &[(7, "Spot AF"), (3, "Auto"), (4, "Manual"), (5, "Infinity"), (2, "Macro")];
static CASIO_MAIN_PC_11: &[(i64, &str)] = &[(2, "On"), (1, "Auto"), (5, "Red-eye Reduction"), (4, "Off"), (3, "Off")];
static CASIO_MAIN_PC_12: &[(i64, &str)] = &[(15, "Strong"), (11, "Weak"), (12, "Low"), (14, "High"), (13, "Normal")];
static CASIO_MAIN_PC_13: &[(i64, &str)] = &[(129, "Manual"), (3, "Daylight"), (4, "Fluorescent"), (5, "Shade"), (1, "Auto"), (2, "Tungsten")];

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
