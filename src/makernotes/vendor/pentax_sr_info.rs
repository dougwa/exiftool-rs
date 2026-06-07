// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};

// SRResult uses a BITMASK PrintConv (handled in pentax special, Pc::None).
static PENTAX_SR_INFO_PC_1: &[(i64, &str)] = &[(5, "On but Disabled"), (4, "Off (4)"), (135, "On (135)"), (7, "On (7)"), (0, "Off"), (15, "On (15)"), (6, "On (Video)"), (39, "On (mode 2)"), (167, "On (mode 1)"), (1, "On")];

pub static PENTAX_SR_INFO: BinTable = BinTable {
    default_fmt: Fmt::U8,
    first_entry: 0,
    tags: &[
    BinTag { index: 0, name: "SRResult", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 1, name: "ShakeReduction", fmt: None, pc: Pc::Enum(PENTAX_SR_INFO_PC_1), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 2, name: "SRHalfPressTime", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 3, name: "SRFocalLength", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
