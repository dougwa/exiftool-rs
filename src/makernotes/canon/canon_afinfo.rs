// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};


pub static CANON_AFINFO: BinTable = BinTable {
    default_fmt: Fmt::U16,
    first_entry: 0,
    tags: &[
    BinTag { index: 0, name: "NumAFPoints", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 1, name: "ValidAFPoints", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 2, name: "CanonImageWidth", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 3, name: "CanonImageHeight", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 4, name: "AFImageWidth", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 5, name: "AFImageHeight", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 6, name: "AFAreaWidth", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 7, name: "AFAreaHeight", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 8, name: "AFAreaXPositions", fmt: Some(Fmt::S16), pc: Pc::None, skip: Skip::Never, count: Count::Var(0) },
    BinTag { index: 9, name: "AFAreaYPositions", fmt: Some(Fmt::S16), pc: Pc::None, skip: Skip::Never, count: Count::Var(0) },
    BinTag { index: 10, name: "AFPointsInFocus", fmt: Some(Fmt::S16), pc: Pc::None, skip: Skip::Never, count: Count::VarBits(0) },
    BinTag { index: 11, name: "PrimaryAFPoint", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 12, name: "PrimaryAFPoint", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
