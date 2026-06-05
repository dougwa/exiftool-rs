// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};

static NIKON_AFINFO_PC_0: &[(i64, &str)] = &[(1, "Dynamic Area"), (4, "Single Area (wide)"), (3, "Group Dynamic"), (2, "Dynamic Area (closest subject)"), (5, "Dynamic Area (wide)"), (0, "Single Area")];
static NIKON_AFINFO_PC_1: &[(i64, &str)] = &[(5, "Upper-left"), (2, "Bottom"), (4, "Mid-right"), (10, "Far Right"), (6, "Upper-right"), (7, "Lower-left"), (8, "Lower-right"), (9, "Far Left"), (0, "Center"), (3, "Mid-left"), (1, "Top")];

pub static NIKON_AFINFO: BinTable = BinTable {
    default_fmt: Fmt::U8,
    first_entry: 0,
    tags: &[
    BinTag { index: 0, name: "AFAreaMode", fmt: None, pc: Pc::Enum(NIKON_AFINFO_PC_0), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 1, name: "AFPoint", fmt: None, pc: Pc::Enum(NIKON_AFINFO_PC_1), skip: Skip::Never, count: Count::Fixed(1) },
    // AFPointsInFocus is a BITMASK PrintConv (afPoints11) the codegen can't
    // express as a flat enum; handled in nikon::special instead (Pc::None).
    BinTag { index: 2, name: "AFPointsInFocus", fmt: Some(Fmt::U16), pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
