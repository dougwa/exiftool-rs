// Auto-generated from ExifTool via /tmp/dump_table.pl. Do not edit by hand.
use crate::makernotes::binary::{Pc, Skip};
use crate::makernotes::{MnKind, MnTag};

static OLYMPUS_RAW_DEVELOPMENT_PC_0: &[(i64, &str)] = &[(1, "Adobe RGB"), (2, "Pro Photo RGB"), (0, "sRGB")];
static OLYMPUS_RAW_DEVELOPMENT_PC_1: &[(i64, &str)] = &[(1, "High Function"), (0, "High Speed"), (3, "Advanced High Function"), (2, "Advanced High Speed")];
static OLYMPUS_RAW_DEVELOPMENT_PC_2: &[(i64, &str)] = &[(0, "(none)")];
static OLYMPUS_RAW_DEVELOPMENT_PC_3: &[(i64, &str)] = &[(8, "Edited (Portrait)"), (6, "Edited (Portrait)"), (1, "Edited (Landscape)"), (0, "Original")];
static OLYMPUS_RAW_DEVELOPMENT_PC_4: &[(i64, &str)] = &[(0, "(none)")];

pub static OLYMPUS_RAW_DEVELOPMENT: &[MnTag] = &[
    MnTag { id: 0, kind: MnKind::Scalar { name: "RawDevVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 256, kind: MnKind::Scalar { name: "RawDevExposureBiasValue", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 257, kind: MnKind::Scalar { name: "RawDevWhiteBalanceValue", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 258, kind: MnKind::Scalar { name: "RawDevWBFineAdjustment", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 259, kind: MnKind::Scalar { name: "RawDevGrayPoint", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 260, kind: MnKind::Scalar { name: "RawDevSaturationEmphasis", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 261, kind: MnKind::Scalar { name: "RawDevMemoryColorEmphasis", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 262, kind: MnKind::Scalar { name: "RawDevContrastValue", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 263, kind: MnKind::Scalar { name: "RawDevSharpnessValue", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 264, kind: MnKind::Scalar { name: "RawDevColorSpace", pc: Pc::Enum(OLYMPUS_RAW_DEVELOPMENT_PC_0), bin: false, skip: Skip::Never } },
    MnTag { id: 265, kind: MnKind::Scalar { name: "RawDevEngine", pc: Pc::Enum(OLYMPUS_RAW_DEVELOPMENT_PC_1), bin: false, skip: Skip::Never } },
    MnTag { id: 266, kind: MnKind::Scalar { name: "RawDevNoiseReduction", pc: Pc::Enum(OLYMPUS_RAW_DEVELOPMENT_PC_2), bin: false, skip: Skip::Never } },
    MnTag { id: 267, kind: MnKind::Scalar { name: "RawDevEditStatus", pc: Pc::Enum(OLYMPUS_RAW_DEVELOPMENT_PC_3), bin: false, skip: Skip::Never } },
    MnTag { id: 268, kind: MnKind::Scalar { name: "RawDevSettings", pc: Pc::Enum(OLYMPUS_RAW_DEVELOPMENT_PC_4), bin: false, skip: Skip::Never } },
];
