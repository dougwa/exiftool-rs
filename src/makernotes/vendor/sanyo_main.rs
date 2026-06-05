// Auto-generated from ExifTool via /tmp/dump_table.pl. Do not edit by hand.
use crate::makernotes::binary::{Pc, Skip};
use crate::makernotes::{MnKind, MnTag};

static SANYO_MAIN_PC_0: &[(i64, &str)] = &[(2, "Normal/Medium Low"), (4, "Normal/Medium High"), (259, "Fine/Medium"), (518, "Super Fine/Very High"), (3, "Normal/Medium"), (512, "Super Fine/Very Low"), (260, "Fine/Medium High"), (1, "Normal/Low"), (5, "Normal/High"), (256, "Fine/Very Low"), (262, "Fine/Very High"), (7, "Normal/Super High"), (0, "Normal/Very Low"), (257, "Fine/Low"), (514, "Super Fine/Medium Low"), (517, "Super Fine/High"), (6, "Normal/Very High"), (515, "Super Fine/Medium"), (516, "Super Fine/Medium High"), (261, "Fine/High"), (263, "Fine/Super High"), (519, "Super Fine/Super High"), (258, "Fine/Medium Low"), (513, "Super Fine/Low")];
static SANYO_MAIN_PC_1: &[(i64, &str)] = &[(0, "Normal"), (1, "Macro"), (2, "View"), (3, "Manual")];
static SANYO_MAIN_PC_2: &[(i64, &str)] = &[(3, "Adjust Exposure"), (1, "Standard"), (0, "None"), (2, "Best")];
static SANYO_MAIN_PC_3: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static SANYO_MAIN_PC_4: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static SANYO_MAIN_PC_5: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static SANYO_MAIN_PC_6: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static SANYO_MAIN_PC_7: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static SANYO_MAIN_PC_8: &[(i64, &str)] = &[(1, "Press start, press stop"), (0, "Record while down")];
static SANYO_MAIN_PC_9: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static SANYO_MAIN_PC_10: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static SANYO_MAIN_PC_11: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static SANYO_MAIN_PC_12: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static SANYO_MAIN_PC_13: &[(i64, &str)] = &[(1, "Yes"), (0, "No")];
static SANYO_MAIN_PC_14: &[(i64, &str)] = &[(6, "Lamp"), (5, "User 2"), (1, "Sport"), (2, "TV"), (0, "Off"), (4, "User 1"), (3, "Night")];
static SANYO_MAIN_PC_15: &[(i64, &str)] = &[(3, "20 frames/s"), (0, "5 frames/s"), (1, "10 frames/s"), (2, "15 frames/s")];
static SANYO_MAIN_PC_16: &[(i64, &str)] = &[(2, "Disabled"), (1, "Force"), (0, "Auto"), (3, "Red eye")];

pub static SANYO_MAIN: &[MnTag] = &[
    MnTag { id: 255, kind: MnKind::Scalar { name: "MakerNoteOffset", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 256, kind: MnKind::Scalar { name: "SanyoThumbnail", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 3840, kind: MnKind::Scalar { name: "DataDump", pc: Pc::None, bin: true, skip: Skip::Never } },
    MnTag { id: 512, kind: MnKind::Scalar { name: "SpecialMode", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 513, kind: MnKind::Scalar { name: "SanyoQuality", pc: Pc::Enum(SANYO_MAIN_PC_0), bin: false, skip: Skip::Never } },
    MnTag { id: 514, kind: MnKind::Scalar { name: "Macro", pc: Pc::Enum(SANYO_MAIN_PC_1), bin: false, skip: Skip::Never } },
    MnTag { id: 516, kind: MnKind::Scalar { name: "DigitalZoom", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 519, kind: MnKind::Scalar { name: "SoftwareVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 520, kind: MnKind::Scalar { name: "PictInfo", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 521, kind: MnKind::Scalar { name: "CameraID", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 526, kind: MnKind::Scalar { name: "SequentialShot", pc: Pc::Enum(SANYO_MAIN_PC_2), bin: false, skip: Skip::Never } },
    MnTag { id: 527, kind: MnKind::Scalar { name: "WideRange", pc: Pc::Enum(SANYO_MAIN_PC_3), bin: false, skip: Skip::Never } },
    MnTag { id: 528, kind: MnKind::Scalar { name: "ColorAdjustmentMode", pc: Pc::Enum(SANYO_MAIN_PC_4), bin: false, skip: Skip::Never } },
    MnTag { id: 531, kind: MnKind::Scalar { name: "QuickShot", pc: Pc::Enum(SANYO_MAIN_PC_5), bin: false, skip: Skip::Never } },
    MnTag { id: 532, kind: MnKind::Scalar { name: "SelfTimer", pc: Pc::Enum(SANYO_MAIN_PC_6), bin: false, skip: Skip::Never } },
    MnTag { id: 534, kind: MnKind::Scalar { name: "VoiceMemo", pc: Pc::Enum(SANYO_MAIN_PC_7), bin: false, skip: Skip::Never } },
    MnTag { id: 535, kind: MnKind::Scalar { name: "RecordShutterRelease", pc: Pc::Enum(SANYO_MAIN_PC_8), bin: false, skip: Skip::Never } },
    MnTag { id: 536, kind: MnKind::Scalar { name: "FlickerReduce", pc: Pc::Enum(SANYO_MAIN_PC_9), bin: false, skip: Skip::Never } },
    MnTag { id: 537, kind: MnKind::Scalar { name: "OpticalZoomOn", pc: Pc::Enum(SANYO_MAIN_PC_10), bin: false, skip: Skip::Never } },
    MnTag { id: 539, kind: MnKind::Scalar { name: "DigitalZoomOn", pc: Pc::Enum(SANYO_MAIN_PC_11), bin: false, skip: Skip::Never } },
    MnTag { id: 541, kind: MnKind::Scalar { name: "LightSourceSpecial", pc: Pc::Enum(SANYO_MAIN_PC_12), bin: false, skip: Skip::Never } },
    MnTag { id: 542, kind: MnKind::Scalar { name: "Resaved", pc: Pc::Enum(SANYO_MAIN_PC_13), bin: false, skip: Skip::Never } },
    MnTag { id: 543, kind: MnKind::Scalar { name: "SceneSelect", pc: Pc::Enum(SANYO_MAIN_PC_14), bin: false, skip: Skip::Never } },
    MnTag { id: 547, kind: MnKind::Scalar { name: "ManualFocusDistance", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 548, kind: MnKind::Scalar { name: "SequenceShotInterval", pc: Pc::Enum(SANYO_MAIN_PC_15), bin: false, skip: Skip::Never } },
    MnTag { id: 549, kind: MnKind::Scalar { name: "FlashMode", pc: Pc::Enum(SANYO_MAIN_PC_16), bin: false, skip: Skip::Never } },
];
