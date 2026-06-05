// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};

static PENTAX_BATTERY_INFO_PC_0: &[(i64, &str)] = &[(5, "Full"), (2, "Running Low"), (4, "Close to Full"), (3, "Half Full"), (0, "Empty or Missing"), (1, "Almost Empty")];

pub static PENTAX_BATTERY_INFO: BinTable = BinTable {
    default_fmt: Fmt::U8,
    first_entry: 0,
    tags: &[
    BinTag { index: 2, name: "BodyBatteryADNoLoad", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 3, name: "BodyBatteryADLoad", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 4, name: "GripBatteryADNoLoad", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 5, name: "GripBatteryADLoad", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 6, name: "BodyBatteryVoltage3", fmt: Some(Fmt::U16), pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 8, name: "BodyBatteryVoltage4", fmt: Some(Fmt::U16), pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 16, name: "GripBatteryState", fmt: None, pc: Pc::Enum(PENTAX_BATTERY_INFO_PC_0), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 17, name: "GripBatteryPercent", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 18, name: "GripBatteryVoltage", fmt: Some(Fmt::U32), pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
