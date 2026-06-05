// Auto-generated from ExifTool via /tmp/dump_table.pl. Do not edit by hand.
use crate::makernotes::binary::{Pc, Skip};
use crate::makernotes::{MnKind, MnTag};

static OLYMPUS_FOCUS_INFO_PC_0: &[(&str, &str)] = &[("0 0", "Off"), ("1 0", "On")];
static OLYMPUS_FOCUS_INFO_PC_1: &[(i64, &str)] = &[(1, "Direct"), (0, "Bounce or Off")];
static OLYMPUS_FOCUS_INFO_PC_2: &[(&str, &str)] = &[("0", "Off"), ("1", "On"), ("0 0", "Off"), ("1 0", "On")];
static OLYMPUS_FOCUS_INFO_PC_3: &[(i64, &str)] = &[(1, "On"), (0, "Off")];

pub static OLYMPUS_FOCUS_INFO: &[MnTag] = &[
    MnTag { id: 0, kind: MnKind::Scalar { name: "FocusInfoVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4609, kind: MnKind::Scalar { name: "ExternalFlash", pc: Pc::EnumStr(OLYMPUS_FOCUS_INFO_PC_0), bin: false, skip: Skip::Never } },
    MnTag { id: 4612, kind: MnKind::Scalar { name: "ExternalFlashBounce", pc: Pc::Enum(OLYMPUS_FOCUS_INFO_PC_1), bin: false, skip: Skip::Never } },
    MnTag { id: 4613, kind: MnKind::Scalar { name: "ExternalFlashZoom", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4616, kind: MnKind::Scalar { name: "InternalFlash", pc: Pc::EnumStr(OLYMPUS_FOCUS_INFO_PC_2), bin: false, skip: Skip::Never } },
    MnTag { id: 4617, kind: MnKind::Scalar { name: "ManualFlash", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4618, kind: MnKind::Scalar { name: "MacroLED", pc: Pc::Enum(OLYMPUS_FOCUS_INFO_PC_3), bin: false, skip: Skip::Never } },
    MnTag { id: 528, kind: MnKind::Scalar { name: "SceneDetect", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 5376, kind: MnKind::Scalar { name: "SensorTemperature", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 5632, kind: MnKind::Scalar { name: "ImageStabilization", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 768, kind: MnKind::Scalar { name: "ZoomStepCount", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 769, kind: MnKind::Scalar { name: "FocusStepCount", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 771, kind: MnKind::Scalar { name: "FocusStepInfinity", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 772, kind: MnKind::Scalar { name: "FocusStepNear", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 773, kind: MnKind::Scalar { name: "FocusDistance", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 776, kind: MnKind::Scalar { name: "AFPoint", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 795, kind: MnKind::Scalar { name: "AFPointDetails", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 8448, kind: MnKind::Scalar { name: "AntiShockWaitingTime", pc: Pc::None, bin: false, skip: Skip::Never } },
];
