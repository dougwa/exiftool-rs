// Auto-generated from ExifTool (binary-data table). Do not edit by hand.
use crate::makernotes::binary::{BinTable, BinTag, Count, Fmt, Pc, Skip};

static PENTAX_FLASH_INFO_PC_0: &[(i64, &str)] = &[(0, "Off"), (2, "External, Did not fire"), (1, "Off (1)"), (8, "Internal, Did not fire (0x08)"), (6, "External, Fired"), (13, "Internal, Fired"), (9, "Internal, Did not fire")];
static PENTAX_FLASH_INFO_PC_1: &[(i64, &str)] = &[(241, "Did not fire, Red-eye reduction"), (201, "Fired, Slow-sync, Red-eye reduction"), (244, "Did not fire, (Unknown 0xf4)"), (0, "n/a - Off-Auto-Aperture"), (149, "Fired, Wireless (Master)"), (194, "Fired, Auto"), (198, "Fired, Wireless (Control), Fired normally not as control"), (193, "Fired, Red-eye reduction"), (243, "Did not fire, Auto, Red-eye reduction"), (248, "Did not fire, Slow-sync"), (200, "Fired, Slow-sync"), (202, "Fired, Trailing-curtain Sync"), (250, "Did not fire, Trailing-curtain Sync"), (246, "Did not fire, Wireless (Control)"), (242, "Did not fire, Auto"), (195, "Fired, Auto, Red-eye reduction"), (240, "Did not fire, Normal"), (134, "Fired, Wireless (Control)"), (192, "Fired"), (249, "Did not fire, Slow-sync, Red-eye reduction"), (245, "Did not fire, Wireless (Master)")];
static PENTAX_FLASH_INFO_PC_2: &[(i64, &str)] = &[(197, "On, Contrast-control Sync"), (63, "Off"), (64, "On, Auto"), (240, "Not Connected"), (0, "n/a - Off-Auto-Aperture"), (192, "On, Manual"), (198, "On, High-speed Sync"), (191, "On, Flash Problem"), (204, "On, Wireless"), (196, "On, P-TTL Auto"), (205, "On, Wireless, High-speed Sync")];
static PENTAX_FLASH_INFO_PC_3: &[(i64, &str)] = &[(0, "n/a"), (180, "1.0"), (168, "-2.0"), (171, "-1.5"), (172, "-1.0"), (179, "0.5"), (175, "-0.5"), (167, "-2.5"), (144, "n/a (Manual Mode)"), (164, "-3.0"), (176, "0.0")];
static PENTAX_FLASH_INFO_PC_4: &[(i64, &str)] = &[(16, "Direct"), (0, "n/a"), (48, "Bounce")];

pub static PENTAX_FLASH_INFO: BinTable = BinTable {
    default_fmt: Fmt::U8,
    first_entry: 0,
    tags: &[
    BinTag { index: 0, name: "FlashStatus", fmt: None, pc: Pc::Enum(PENTAX_FLASH_INFO_PC_0), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 1, name: "InternalFlashMode", fmt: None, pc: Pc::Enum(PENTAX_FLASH_INFO_PC_1), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 2, name: "ExternalFlashMode", fmt: None, pc: Pc::Enum(PENTAX_FLASH_INFO_PC_2), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 3, name: "InternalFlashStrength", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 4, name: "TTL_DA_AUp", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 5, name: "TTL_DA_ADown", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 6, name: "TTL_DA_BUp", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 7, name: "TTL_DA_BDown", fmt: None, pc: Pc::None, skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 25, name: "ExternalFlashExposureComp", fmt: None, pc: Pc::Enum(PENTAX_FLASH_INFO_PC_3), skip: Skip::Never, count: Count::Fixed(1) },
    BinTag { index: 26, name: "ExternalFlashBounce", fmt: None, pc: Pc::Enum(PENTAX_FLASH_INFO_PC_4), skip: Skip::Never, count: Count::Fixed(1) },
    ],
};
