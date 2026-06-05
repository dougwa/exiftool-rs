// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};

static CANON_FOCAL_LENGTH_PC_0: &[(i64, &str)] = &[(1, "Fixed"), (2, "Zoom")];

pub static CANON_FOCAL_LENGTH: BinTable = BinTable {
    default_fmt: Fmt::U16,
    first_entry: 0,
    tags: &[
    BinTag { index: 0, name: "FocalType", fmt: None, pc: Pc::Enum(CANON_FOCAL_LENGTH_PC_0), skip: Skip::Eq(0), count: Count::Fixed(1) },
    BinTag { index: 1, name: "FocalLength", fmt: None, pc: Pc::None, skip: Skip::Eq(0), count: Count::Fixed(1) },
    BinTag { index: 2, name: "FocalPlaneXSize", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 3, name: "FocalPlaneYSize", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
