// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};

static PENTAX_AF_INFO_PC_0: &[(i64, &str)] = &[(2, "Bottom"), (3, "Lower-right, Bottom"), (4, "Mid-left, Center"), (20, "Mid-right"), (19, "Center (vertical)"), (12, "Upper-left, Mid-left"), (0, "None"), (9, "Upper-right, Top"), (14, "Top, Center"), (8, "Top"), (7, "Upper-left, Top"), (6, "Mid-right, Center"), (16, "Upper-right, Mid-right"), (18, "Mid-left"), (11, "Lower-left, Mid-left"), (10, "Right"), (17, "Left"), (15, "Lower-right, Mid-right"), (1, "Lower-left, Bottom"), (5, "Center (horizontal)"), (13, "Bottom, Center")];
static PENTAX_AF_INFO_PC_1: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static PENTAX_AF_INFO_PC_2: &[(i64, &str)] = &[(3, "Long"), (2, "Medium"), (0, "Off"), (1, "Short")];
static PENTAX_AF_INFO_PC_3: &[(i64, &str)] = &[(2, "Focus Priority"), (0, "Auto"), (1, "Release Priority")];
static PENTAX_AF_INFO_PC_4: &[(i64, &str)] = &[(1, "Focus Priority"), (0, "Auto"), (2, "FPS Priority")];
static PENTAX_AF_INFO_PC_5: &[(i64, &str)] = &[(1, "Medium"), (0, "Low"), (2, "High"), (3, "Off")];
static PENTAX_AF_INFO_PC_6: &[(i64, &str)] = &[(0, "Off"), (1, "On")];

pub static PENTAX_AF_INFO: BinTable = BinTable {
    default_fmt: Fmt::U8,
    first_entry: 0,
    tags: &[
    BinTag { index: 4, name: "AFPredictor", fmt: Some(Fmt::S16), pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 6, name: "AFDefocus", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 7, name: "AFIntegrationTime", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 11, name: "AFPointsInFocus", fmt: None, pc: Pc::Enum(PENTAX_AF_INFO_PC_0), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 298, name: "AFPointsSelected", fmt: Some(Fmt::U8), pc: Pc::None, skip: Skip::Never, count: Count::Fixed(101) },
    BinTag { index: 506, name: "LiveView", fmt: None, pc: Pc::Enum(PENTAX_AF_INFO_PC_1), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 509, name: "AFHold", fmt: None, pc: Pc::Enum(PENTAX_AF_INFO_PC_2), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 543, name: "FirstFrameActionInAFC", fmt: None, pc: Pc::Enum(PENTAX_AF_INFO_PC_3), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 544, name: "ActionInAFCCont", fmt: None, pc: Pc::Enum(PENTAX_AF_INFO_PC_4), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 545, name: "AFCHold", fmt: None, pc: Pc::Enum(PENTAX_AF_INFO_PC_5), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 2400, name: "SubjectRecognition", fmt: None, pc: Pc::Enum(PENTAX_AF_INFO_PC_6), skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
