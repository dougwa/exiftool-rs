// Auto-generated from ExifTool via /tmp/dump_table.pl. Do not edit by hand.
use crate::makernotes::binary::{Pc, Skip};
use crate::makernotes::{MnKind, MnTag};

static PENTAX_MAIN_PC_0: &[(i64, &str)] = &[(2, "2"), (8, "4"), (6, "3"), (5, "-3"), (1, "Normal"), (7, "-4"), (3, "-1"), (4, "1"), (65535, "None"), (0, "-2")];
static PENTAX_MAIN_PC_1: &[(&str, &str)] = &[("2 0", "2"), ("-1 0", "-1"), ("1 0", "1"), ("-2 0", "-2"), ("-4 0", "-4"), ("3 0", "3"), ("4 0", "4"), ("-3 0", "-3"), ("0 0", "0")];
static PENTAX_MAIN_PC_2: &[(&str, &str)] = &[("3 0", "3"), ("4 0", "4"), ("-3 0", "-3"), ("0 0", "0"), ("2 0", "2"), ("-1 0", "-1"), ("1 0", "1"), ("-2 0", "-2"), ("-4 0", "-4")];
static PENTAX_MAIN_PC_3: &[(&str, &str)] = &[("-3 0", "-3"), ("4 0", "4"), ("3 0", "3"), ("0 0", "0"), ("-1 0", "-1"), ("2 0", "2"), ("-2 0", "-2"), ("-4 0", "-4"), ("1 0", "1")];
static PENTAX_MAIN_PC_4: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static PENTAX_MAIN_PC_5: &[(i64, &str)] = &[(4, "Red"), (3, "Orange"), (65535, "None"), (2, "Yellow"), (7, "Cyan"), (1, "Green"), (8, "Infrared"), (5, "Magenta"), (6, "Blue")];
static PENTAX_MAIN_PC_6: &[(i64, &str)] = &[(3, "-1"), (4, "0"), (65535, "None"), (0, "-4"), (2, "-2"), (1, "-3"), (7, "3"), (8, "4"), (6, "2"), (5, "1")];
static PENTAX_MAIN_PC_7: &[(&str, &str)] = &[("2 4", "Auto"), ("1", "On"), ("1 2", "Normal"), ("2", "Auto 2"), ("1 1", "Weak"), ("1 3", "Strong"), ("0", "Off"), ("0 0", "Off")];
static PENTAX_MAIN_PC_8: &[(i64, &str)] = &[(34, "Favorite 2"), (2, "Preset 1"), (1, "Random"), (4, "Preset 3"), (35, "Favorite 3"), (3, "Preset 2"), (33, "Favorite 1"), (0, "Off")];
static PENTAX_MAIN_PC_9: &[(i64, &str)] = &[(4, "Red"), (3, "Orange"), (0, "Off"), (65535, "n/a"), (2, "Yellow"), (8, "Cyan"), (5, "Magenta"), (6, "Purple"), (7, "Blue"), (1, "Green")];
static PENTAX_MAIN_PC_10: &[(i64, &str)] = &[(2, "16:9"), (3, "1:1"), (0, "4:3"), (1, "3:2")];
static PENTAX_MAIN_PC_11: &[(i64, &str)] = &[(274, "AF-A (Release-priority)"), (18, "AF-A (Focus-priority)"), (6, "Auto-area"), (1, "Macro"), (32771, "Manual (Macro)"), (12, "Snap"), (10, "Tracking"), (8, "Select"), (9, "Pinpoint"), (32777, "Pinpoint (Macro)"), (273, "AF-C (Release-priority)"), (0, "Normal"), (32, "Contrast-detect (Focus-priority)"), (2, "Infinity"), (32778, "Tracking (Macro)"), (5, "Pan Focus"), (32775, "Zone Select (Macro)"), (32774, "Auto-area (Macro)"), (7, "Zone Select"), (16, "AF-S (Focus-priority)"), (11, "Continuous"), (3, "Manual"), (32776, "Select (Macro)"), (17, "AF-C (Focus-priority)"), (288, "Contrast-detect (Release-priority)"), (32779, "Continuous (Macro)"), (272, "AF-S (Release-priority)"), (4, "Super Macro"), (33, "Tracking Contrast-detect (Focus-priority)")];
static PENTAX_MAIN_PC_12: &[(i64, &str)] = &[(0, "Normal"), (1, "Electronic")];
static PENTAX_MAIN_PC_13: &[(&str, &str)] = &[("1 1", "On (On)"), ("0", "Off"), ("0 2", "Off (Auto)"), ("0 0", "Off (Off)"), ("1 2", "On (Auto)"), ("1", "On")];
static PENTAX_MAIN_PC_14: &[(&str, &str)] = &[("0 0", "Off")];
static PENTAX_MAIN_PC_15: &[(&str, &str)] = &[("1 2", "On (type 2)"), ("1 1", "On (type 1)"), ("0 0", "Off")];
static PENTAX_MAIN_PC_16: &[(i64, &str)] = &[(0, "(none)")];
static PENTAX_MAIN_PC_17: &[(&str, &str)] = &[("0 0", "Off")];
static PENTAX_MAIN_PC_18: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static PENTAX_MAIN_PC_19: &[(i64, &str)] = &[(9, "200"), (24, "6400"), (35, "80000"), (273, "9000"), (278, "51200"), (65535, "Auto"), (281, "144000"), (279, "72000"), (8, "160"), (264, "400"), (266, "800"), (45, "819200"), (800, "800"), (19, "2000"), (260, "100"), (12, "400"), (21, "3200"), (265, "560"), (259, "70"), (37, "128000"), (26, "10000"), (267, "1100"), (6, "100"), (258, "50"), (200, "200"), (42, "409600"), (20, "2500"), (272, "6400"), (261, "140"), (23, "5000"), (15, "800"), (285, "576000"), (284, "409600"), (17, "1250"), (286, "819200"), (280, "102400"), (32, "40000"), (3200, "3200"), (39, "204800"), (28, "16000"), (100, "100"), (282, "204800"), (65534, "Auto 2"), (269, "2200"), (13, "500"), (34, "64000"), (10, "250"), (25, "8000"), (41, "320000"), (268, "1600"), (263, "280"), (27, "12800"), (36, "102400"), (277, "36000"), (276, "25600"), (44, "640000"), (270, "3200"), (274, "12800"), (18, "1600"), (31, "32000"), (275, "18000"), (4, "64"), (14, "640"), (33, "51200"), (30, "25600"), (262, "200"), (271, "4500"), (283, "288000"), (50, "50"), (11, "320"), (22, "4000"), (3, "50"), (40, "256000"), (43, "512000"), (38, "160000"), (29, "20000"), (5, "80"), (1600, "1600"), (400, "400"), (7, "125"), (16, "1000")];
static PENTAX_MAIN_PC_20: &[(i64, &str)] = &[(2, "Spot"), (6, "Highlight"), (0, "Multi-segment"), (1, "Center-weighted average")];
static PENTAX_MAIN_PC_21: &[(i64, &str)] = &[(14, "Multi Auto"), (0, "Auto"), (65535, "User-Selected"), (65534, "Unknown"), (9, "Flash"), (4, "Tungsten"), (8, "White Fluorescent"), (15, "Color Temperature Enhancement"), (10, "Cloudy"), (17, "Kelvin"), (11, "Warm White Fluorescent"), (3, "Fluorescent"), (6, "Daylight Fluorescent"), (5, "Manual"), (7, "Day White Fluorescent"), (1, "Daylight"), (2, "Shade")];
static PENTAX_MAIN_PC_22: &[(i64, &str)] = &[(8, "Auto (White Fluorescent)"), (6, "Auto (Daylight Fluorescent)"), (1, "Auto (Daylight)"), (7, "Auto (Day White Fluorescent)"), (10, "Auto (Cloudy)"), (2, "Auto (Shade)"), (65534, "Unknown"), (65535, "User-Selected"), (3, "Auto (Flash)"), (4, "Auto (Tungsten)")];
static PENTAX_MAIN_PC_23: &[(i64, &str)] = &[(0, "Hometown"), (1, "Destination")];
static PENTAX_MAIN_PC_24: &[(i64, &str)] = &[(72, "Warsaw"), (17, "Sao Paulo"), (52, "Manila"), (59, "Noumea"), (61, "Auckland"), (66, "Athens"), (39, "Dacca"), (28, "Jerusalem"), (32, "Dubai"), (2, "Anchorage"), (64, "Algiers"), (20, "London"), (23, "Rome"), (49, "Perth"), (15, "Halifax"), (42, "Kuala Lumpur"), (12, "New York"), (21, "Paris"), (57, "Guam"), (19, "Madrid"), (45, "Phnom Penh"), (6, "Calgary"), (68, "Amsterdam"), (37, "Colombo"), (26, "Istanbul"), (9, "Chicago"), (24, "Berlin"), (35, "Male"), (55, "Adelaide"), (8, "Mexico City"), (47, "Jakarta"), (63, "Dakar"), (60, "Wellington"), (40, "Yangon"), (43, "Vientiane"), (67, "Nairobi"), (38, "Kathmandu"), (29, "Moscow"), (11, "Toronto"), (22, "Milan"), (3, "Vancouver"), (5, "Los Angeles"), (7, "Denver"), (16, "Buenos Aires"), (58, "Sydney"), (14, "Caracus"), (33, "Karachi"), (30, "Jeddah"), (48, "Hong Kong"), (4, "San Francisco"), (65, "Helsinki"), (70, "Lisbon"), (73, "Prague"), (53, "Taipei"), (50, "Beijing"), (62, "Lima"), (27, "Cairo"), (69, "Stockholm"), (71, "Copenhagen"), (51, "Shanghai"), (36, "Delhi"), (31, "Tehran"), (56, "Tokyo"), (1, "Honolulu"), (44, "Singapore"), (18, "Rio de Janeiro"), (74, "Budapest"), (0, "Pago Pago"), (54, "Seoul"), (46, "Ho Chi Minh"), (41, "Bangkok"), (13, "Santiago"), (34, "Kabul"), (10, "Miami"), (25, "Johannesburg")];
static PENTAX_MAIN_PC_25: &[(i64, &str)] = &[(72, "Warsaw"), (17, "Sao Paulo"), (52, "Manila"), (59, "Noumea"), (61, "Auckland"), (66, "Athens"), (39, "Dacca"), (28, "Jerusalem"), (32, "Dubai"), (2, "Anchorage"), (64, "Algiers"), (20, "London"), (23, "Rome"), (49, "Perth"), (15, "Halifax"), (42, "Kuala Lumpur"), (12, "New York"), (21, "Paris"), (57, "Guam"), (19, "Madrid"), (45, "Phnom Penh"), (6, "Calgary"), (68, "Amsterdam"), (37, "Colombo"), (26, "Istanbul"), (9, "Chicago"), (24, "Berlin"), (35, "Male"), (55, "Adelaide"), (8, "Mexico City"), (47, "Jakarta"), (63, "Dakar"), (60, "Wellington"), (40, "Yangon"), (43, "Vientiane"), (67, "Nairobi"), (38, "Kathmandu"), (29, "Moscow"), (11, "Toronto"), (22, "Milan"), (3, "Vancouver"), (5, "Los Angeles"), (7, "Denver"), (16, "Buenos Aires"), (58, "Sydney"), (14, "Caracus"), (33, "Karachi"), (30, "Jeddah"), (48, "Hong Kong"), (4, "San Francisco"), (65, "Helsinki"), (70, "Lisbon"), (73, "Prague"), (53, "Taipei"), (50, "Beijing"), (62, "Lima"), (27, "Cairo"), (69, "Stockholm"), (71, "Copenhagen"), (51, "Shanghai"), (36, "Delhi"), (31, "Tehran"), (56, "Tokyo"), (1, "Honolulu"), (44, "Singapore"), (18, "Rio de Janeiro"), (74, "Budapest"), (0, "Pago Pago"), (54, "Seoul"), (46, "Ho Chi Minh"), (41, "Bangkok"), (13, "Santiago"), (34, "Kabul"), (10, "Miami"), (25, "Johannesburg")];
static PENTAX_MAIN_PC_26: &[(i64, &str)] = &[(1, "Yes"), (0, "No")];
static PENTAX_MAIN_PC_27: &[(i64, &str)] = &[(0, "No"), (1, "Yes")];
static PENTAX_MAIN_PC_28: &[(i64, &str)] = &[(77430, "K-5"), (77300, "Optio WS80"), (78490, "GR IIIx"), (78380, "KP"), (76400, "Optio SV"), (78350, "GR III"), (77160, "Optio E60/M90"), (77120, "Optio W60"), (78320, "WG-M2"), (76980, "Optio E40"), (77640, "Optio LS465"), (77760, "K-3"), (77650, "K-30"), (77470, "Optio LS1000"), (77040, "Optio V10"), (77050, "K200D"), (77720, "WG-3"), (76590, "Optio S45"), (77910, "WG-30"), (77540, "Q"), (77560, "K-01"), (77020, "Optio S10"), (77060, "Optio S12"), (77681, "K-5 II s"), (77200, "Optio L70"), (77660, "X-5"), (77750, "K-50"), (76790, "Optio M10"), (77520, "Optio S1"), (76960, "Optio A30"), (76920, "Optio E30"), (76850, "Optio M20"), (76840, "Optio S7"), (76370, "Optio S30"), (76230, "Optio 33WR/43WR/555"), (76860, "Optio W20"), (77580, "Optio RZ18"), (76950, "Optio W30"), (76940, "Optio M30"), (77080, "Optio M50"), (77680, "K-5 II"), (78420, "K-3 Mark III"), (76180, "*ist D"), (77012, "Samsung GX20"), (76430, "Optio S5i"), (76300, "Optio S40"), (78480, "WG-70"), (77190, "Optio P70"), (76642, "Samsung GX-1S"), (76140, "Optio S"), (77400, "X90"), (77330, "Optio E80"), (76120, "Optio 330GS"), (76620, "Optio WPi"), (76245, "Optio S4"), (76560, "Optio S5z"), (77790, "WG-4 GPS"), (76470, "Optio MX4"), (76640, "*ist DS2"), (76650, "Optio A10"), (77690, "Q7"), (77980, "K-3 II"), (77090, "Optio L50"), (76540, "Optio S55"), (76720, "Optio T10/T20"), (77590, "Optio VS20"), (77610, "Optio WG-2 GPS"), (77860, "K-S2"), (76580, "Optio S60"), (77010, "K20D"), (77230, "X70"), (76990, "Optio M40"), (76997, "Optio L36"), (77950, "WG-30W"), (77960, "WG-5 GPS"), (77370, "Optio I-10"), (77850, "K-S1"), (77710, "WG-3 GPS"), (77840, "645Z"), (77210, "Optio E70"), (77800, "WG-4"), (77030, "Optio A40"), (77730, "WG-10"), (76995, "Optio L40"), (77171, "K-m"), (76701, "K110D"), (76830, "K10D"), (76600, "Optio S6"), (78370, "K-70"), (76220, "Optio 33LF"), (76930, "Optio T30"), (76700, "K100D"), (77290, "Optio P80"), (77170, "K2000"), (77450, "Optio RS1000/RS1500"), (77670, "Q10"), (77070, "Optio E50"), (76490, "Optio WP"), (76145, "Optio S V1.01"), (77420, "K-r"), (77310, "K-x"), (77770, "K-500"), (77460, "Optio RZ10"), (76832, "Samsung GX10"), (78400, "K-1 Mark II"), (76340, "Optio 30"), (77390, "Optio E90"), (76870, "Optio A20"), (77130, "Optio M60"), (76410, "Optio X"), (78520, "KF"), (76945, "Optio L30"), (78560, "GR IV"), (78550, "K-3 Mark III Monochrome"), (76845, "Optio L20"), (13, "Optio 330/430"), (76925, "Optio E35"), (78640, "GR IV Monochrome"), (76630, "BenQ DC X600"), (76210, "Optio 33L"), (76290, "Optio MX"), (76672, "Samsung GX-1L"), (77240, "K-7"), (77700, "MX-1"), (77260, "Optio W80"), (77000, "Optio Z10"), (77830, "WG-20"), (77500, "Optio WG-1 GPS"), (77380, "Optio H90"), (76770, "Optio W10"), (76310, "Optio S4i"), (77100, "Optio V20"), (76450, "*ist DS"), (76440, "Optio S50"), (76670, "*ist DL2"), (76070, "Optio 230"), (76570, "*ist DL"), (76405, "Optio SVi"), (76706, "K100D Super"), (77360, "Optio W90"), (76130, "Optio 450/550"), (77970, "K-1"), (77320, "645D"), (76480, "Optio S5n"), (77870, "Q-S1"), (76390, "Optio 750Z")];
static PENTAX_MAIN_PC_29: &[(&str, &str)] = &[("4 0 0 0", "Digital Filter 4"), ("0 0 0 4", "Digital Filter"), ("16 0 0 0", "Frame Synthesis?"), ("0 0 0 0", "None"), ("0 0", "None"), ("2 0 0 0", "Cropped"), ("1 0 0 0", "Resized"), ("6 0 0 0", "Digital Filter 6"), ("8 0 0 0", "Red-eye Correction")];
static PENTAX_MAIN_PC_30: &[(i64, &str)] = &[(0, "sRGB"), (1, "Adobe RGB")];
static PENTAX_MAIN_PC_31: &[(i64, &str)] = &[(0, "(none)")];
static PENTAX_MAIN_PC_32: &[(i64, &str)] = &[(1, "On"), (0, "Off")];
static PENTAX_MAIN_PC_33: &[(i64, &str)] = &[(0, "Off"), (1, "On")];
static PENTAX_MAIN_PC_34: &[(i64, &str)] = &[(263, "Bleach Bypass 2"), (8, "Bleach Bypass"), (257, "Vivid"), (10, "Cross Processing"), (0, "Natural"), (256, "Standard"), (9, "Radiant"), (32770, "Soft"), (1, "Bright"), (258, "Monotone"), (267, "Negative Film"), (6, "Muted"), (259, "Soft Monotone"), (265, "HDR Tone"), (260, "Hard Monotone"), (266, "Cross Processing 2"), (264, "Retro"), (261, "Hi-contrast B&W"), (33024, "Monochrome"), (262, "Positive Film"), (4, "Vibrant"), (32769, "Hard"), (7, "Reversal Film"), (5, "Monochrome"), (2, "Portrait"), (3, "Landscape"), (11, "Flat"), (32768, "Standard")];
static PENTAX_MAIN_PC_35: &[(i64, &str)] = &[(2, "Best"), (7, "RAW (pixel shift enabled)"), (1, "Better"), (8, "Dynamic Pixel Shift"), (5, "Premium"), (4, "RAW"), (3, "TIFF"), (9, "Monochrome"), (0, "Good"), (65535, "n/a")];
static PENTAX_MAIN_PC_36: &[(&str, &str)] = &[("4", "1600x1200"), ("33 2", "1152x768"), ("36 0", "3008x2008 or 3040x2024"), ("30", "4288x3216"), ("5 0", "2048x1536"), ("37 0", "3008x2000"), ("20", "2288x1712"), ("23", "3056x2296"), ("3", "1280x960"), ("22", "2304x1728 or 2592x1944"), ("4 0", "1600x1200"), ("29", "4000x3000"), ("2", "1024x768"), ("32 2", "960x640"), ("5", "2048x1536"), ("9", "3072x2304"), ("0", "640x480"), ("135", "4608x2592"), ("25", "2816x2212 or 2816x2112"), ("129", "1920x1080"), ("10", "3264x2448"), ("34 2", "1536x1024"), ("257", "3216x3216"), ("8", "2560x1920 or 2304x1728"), ("19", "320x240"), ("0 0", "2304x1728"), ("27", "3648x2736"), ("21", "2592x1944"), ("8 0", "2560x1920"), ("1", "Full"), ("35 1", "2400x1600"), ("31", "4608x3456")];
static PENTAX_MAIN_PC_37: &[(i64, &str)] = &[(12, "12 (MX-1,Q-S1,Q7)"), (21, "21 (K-3IIIMonochrome)"), (17, "17 (K-70)"), (11, "11 (Q10)"), (19, "19 (GR III)"), (3, "3 (K20D)"), (5, "5 (K-x)"), (6, "6 (645D)"), (7, "7 (K-r)"), (16, "16 (K-1)"), (1, "1 (K10D,K200D,K2000,K-m)"), (18, "18 (KP)"), (14, "14 (645Z)"), (9, "9 (Q)"), (4, "4 (K-7)"), (20, "20 (K-3III)"), (8, "8 (K-5,K-5II,K-5IIs)"), (15, "15 (K-S1,K-S2)"), (13, "13 (K-3,K-3II)"), (10, "10 (K-01,K-30,K-50,K-500)")];

pub static PENTAX_MAIN: &[MnTag] = &[
    MnTag { id: 0, kind: MnKind::Scalar { name: "PentaxVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 1, kind: MnKind::Scalar { name: "PentaxModelType", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 1022, kind: MnKind::Scalar { name: "DataDump", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 1026, kind: MnKind::Scalar { name: "ToneCurve", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 1027, kind: MnKind::Scalar { name: "ToneCurves", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 1029, kind: MnKind::Scalar { name: "UnknownBlock", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 103, kind: MnKind::Scalar { name: "Hue", pc: Pc::Enum(PENTAX_MAIN_PC_0), bin: false, skip: Skip::Never } },
    MnTag { id: 105, kind: MnKind::Scalar { name: "DynamicRangeExpansion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 108, kind: MnKind::Scalar { name: "HighLowKeyAdj", pc: Pc::EnumStr(PENTAX_MAIN_PC_1), bin: false, skip: Skip::Never } },
    MnTag { id: 109, kind: MnKind::Scalar { name: "ContrastHighlight", pc: Pc::EnumStr(PENTAX_MAIN_PC_2), bin: false, skip: Skip::Never } },
    MnTag { id: 11, kind: MnKind::Scalar { name: "PictureMode", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 110, kind: MnKind::Scalar { name: "ContrastShadow", pc: Pc::EnumStr(PENTAX_MAIN_PC_3), bin: false, skip: Skip::Never } },
    MnTag { id: 111, kind: MnKind::Scalar { name: "ContrastHighlightShadowAdj", pc: Pc::Enum(PENTAX_MAIN_PC_4), bin: false, skip: Skip::Never } },
    MnTag { id: 112, kind: MnKind::Scalar { name: "FineSharpness", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 113, kind: MnKind::Scalar { name: "HighISONoiseReduction", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 114, kind: MnKind::Scalar { name: "AFAdjustment", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 115, kind: MnKind::Scalar { name: "MonochromeFilterEffect", pc: Pc::Enum(PENTAX_MAIN_PC_5), bin: false, skip: Skip::Never } },
    MnTag { id: 116, kind: MnKind::Scalar { name: "MonochromeToning", pc: Pc::Enum(PENTAX_MAIN_PC_6), bin: false, skip: Skip::Never } },
    MnTag { id: 118, kind: MnKind::Scalar { name: "FaceDetect", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 119, kind: MnKind::Scalar { name: "FaceDetectFrameSize", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 12, kind: MnKind::Scalar { name: "FlashMode", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 121, kind: MnKind::Scalar { name: "ShadowCorrection", pc: Pc::EnumStr(PENTAX_MAIN_PC_7), bin: false, skip: Skip::Never } },
    MnTag { id: 122, kind: MnKind::Scalar { name: "ISOAutoMinSpeed", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 123, kind: MnKind::Scalar { name: "CrossProcess", pc: Pc::Enum(PENTAX_MAIN_PC_8), bin: false, skip: Skip::Never } },
    MnTag { id: 126, kind: MnKind::Scalar { name: "WhiteLevel", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 127, kind: MnKind::Scalar { name: "BleachBypassToning", pc: Pc::Enum(PENTAX_MAIN_PC_9), bin: false, skip: Skip::Never } },
    MnTag { id: 128, kind: MnKind::Scalar { name: "AspectRatio", pc: Pc::Enum(PENTAX_MAIN_PC_10), bin: false, skip: Skip::Never } },
    MnTag { id: 13, kind: MnKind::Scalar { name: "FocusMode", pc: Pc::Enum(PENTAX_MAIN_PC_11), bin: false, skip: Skip::Never } },
    MnTag { id: 130, kind: MnKind::Scalar { name: "BlurControl", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 133, kind: MnKind::Scalar { name: "HDR", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 135, kind: MnKind::Scalar { name: "ShutterType", pc: Pc::Enum(PENTAX_MAIN_PC_12), bin: false, skip: Skip::Never } },
    MnTag { id: 136, kind: MnKind::Scalar { name: "NeutralDensityFilter", pc: Pc::EnumStr(PENTAX_MAIN_PC_13), bin: false, skip: Skip::Never } },
    MnTag { id: 139, kind: MnKind::Scalar { name: "ISO", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 14, kind: MnKind::Scalar { name: "AFPointSelected", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 146, kind: MnKind::Scalar { name: "IntervalShooting", pc: Pc::EnumStr(PENTAX_MAIN_PC_14), bin: false, skip: Skip::Never } },
    MnTag { id: 149, kind: MnKind::Scalar { name: "SkinToneCorrection", pc: Pc::EnumStr(PENTAX_MAIN_PC_15), bin: false, skip: Skip::Never } },
    MnTag { id: 15, kind: MnKind::Scalar { name: "AFPointsInFocus", pc: Pc::Enum(PENTAX_MAIN_PC_16), bin: false, skip: Skip::Never } },
    MnTag { id: 150, kind: MnKind::Scalar { name: "ClarityControl", pc: Pc::EnumStr(PENTAX_MAIN_PC_17), bin: false, skip: Skip::Never } },
    MnTag { id: 158, kind: MnKind::Scalar { name: "HDF", pc: Pc::Enum(PENTAX_MAIN_PC_18), bin: false, skip: Skip::Never } },
    MnTag { id: 16, kind: MnKind::Scalar { name: "FocusPosition", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 18, kind: MnKind::Scalar { name: "ExposureTime", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 19, kind: MnKind::Scalar { name: "FNumber", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 2, kind: MnKind::Scalar { name: "PreviewImageSize", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 20, kind: MnKind::Scalar { name: "ISO", pc: Pc::Enum(PENTAX_MAIN_PC_19), bin: false, skip: Skip::Never } },
    MnTag { id: 21, kind: MnKind::Scalar { name: "LightReading", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 22, kind: MnKind::Scalar { name: "ExposureCompensation", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 23, kind: MnKind::Scalar { name: "MeteringMode", pc: Pc::Enum(PENTAX_MAIN_PC_20), bin: false, skip: Skip::Never } },
    MnTag { id: 24, kind: MnKind::Scalar { name: "AutoBracketing", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 25, kind: MnKind::Scalar { name: "WhiteBalance", pc: Pc::Enum(PENTAX_MAIN_PC_21), bin: false, skip: Skip::Never } },
    MnTag { id: 26, kind: MnKind::Scalar { name: "WhiteBalanceMode", pc: Pc::Enum(PENTAX_MAIN_PC_22), bin: false, skip: Skip::Never } },
    MnTag { id: 27, kind: MnKind::Scalar { name: "BlueBalance", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 28, kind: MnKind::Scalar { name: "RedBalance", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 29, kind: MnKind::Scalar { name: "FocalLength", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 3, kind: MnKind::Scalar { name: "PreviewImageLength", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 30, kind: MnKind::Scalar { name: "DigitalZoom", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 31, kind: MnKind::Scalar { name: "Saturation", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 32, kind: MnKind::Scalar { name: "Contrast", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 33, kind: MnKind::Scalar { name: "Sharpness", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 34, kind: MnKind::Scalar { name: "WorldTimeLocation", pc: Pc::Enum(PENTAX_MAIN_PC_23), bin: false, skip: Skip::Never } },
    MnTag { id: 35, kind: MnKind::Scalar { name: "HometownCity", pc: Pc::Enum(PENTAX_MAIN_PC_24), bin: false, skip: Skip::Never } },
    MnTag { id: 36, kind: MnKind::Scalar { name: "DestinationCity", pc: Pc::Enum(PENTAX_MAIN_PC_25), bin: false, skip: Skip::Never } },
    MnTag { id: 37, kind: MnKind::Scalar { name: "HometownDST", pc: Pc::Enum(PENTAX_MAIN_PC_26), bin: false, skip: Skip::Never } },
    MnTag { id: 38, kind: MnKind::Scalar { name: "DestinationDST", pc: Pc::Enum(PENTAX_MAIN_PC_27), bin: false, skip: Skip::Never } },
    MnTag { id: 39, kind: MnKind::Scalar { name: "DSPFirmwareVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 4, kind: MnKind::Scalar { name: "PreviewImageStart", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 40, kind: MnKind::Scalar { name: "CPUFirmwareVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 41, kind: MnKind::Scalar { name: "FrameNumber", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 45, kind: MnKind::Scalar { name: "EffectiveLV", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 5, kind: MnKind::Scalar { name: "PentaxModelID", pc: Pc::Enum(PENTAX_MAIN_PC_28), bin: false, skip: Skip::Never } },
    MnTag { id: 50, kind: MnKind::Scalar { name: "ImageEditing", pc: Pc::EnumStr(PENTAX_MAIN_PC_29), bin: false, skip: Skip::Never } },
    MnTag { id: 51, kind: MnKind::Scalar { name: "PictureMode", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 512, kind: MnKind::Scalar { name: "BlackPoint", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 513, kind: MnKind::Scalar { name: "WhitePoint", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 515, kind: MnKind::Scalar { name: "ColorMatrixA", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 516, kind: MnKind::Scalar { name: "ColorMatrixB", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 52, kind: MnKind::Scalar { name: "DriveMode", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 521, kind: MnKind::Scalar { name: "AEMeteringSegments", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 522, kind: MnKind::Scalar { name: "FlashMeteringSegments", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 523, kind: MnKind::Scalar { name: "SlaveFlashMeteringSegments", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 525, kind: MnKind::Scalar { name: "WB_RGGBLevelsDaylight", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 526, kind: MnKind::Scalar { name: "WB_RGGBLevelsShade", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 527, kind: MnKind::Scalar { name: "WB_RGGBLevelsCloudy", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 528, kind: MnKind::Scalar { name: "WB_RGGBLevelsTungsten", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 529, kind: MnKind::Scalar { name: "WB_RGGBLevelsFluorescentD", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 53, kind: MnKind::Scalar { name: "SensorSize", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 530, kind: MnKind::Scalar { name: "WB_RGGBLevelsFluorescentN", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 531, kind: MnKind::Scalar { name: "WB_RGGBLevelsFluorescentW", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 532, kind: MnKind::Scalar { name: "WB_RGGBLevelsFlash", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 539, kind: MnKind::Scalar { name: "SaturationInfo", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 540, kind: MnKind::Scalar { name: "ColorMatrixA2", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 541, kind: MnKind::Scalar { name: "ColorMatrixB2", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 544, kind: MnKind::Scalar { name: "HuffmanTable", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 55, kind: MnKind::Scalar { name: "ColorSpace", pc: Pc::Enum(PENTAX_MAIN_PC_30), bin: false, skip: Skip::Never } },
    MnTag { id: 553, kind: MnKind::Scalar { name: "SerialNumber", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 558, kind: MnKind::Scalar { name: "Artist", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 559, kind: MnKind::Scalar { name: "Copyright", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 56, kind: MnKind::Scalar { name: "ImageAreaOffset", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 560, kind: MnKind::Scalar { name: "FirmwareVersion", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 561, kind: MnKind::Scalar { name: "ContrastDetectAFArea", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 565, kind: MnKind::Scalar { name: "CrossProcessParams", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 57, kind: MnKind::Scalar { name: "RawImageSize", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 575, kind: MnKind::Scalar { name: "Model", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 6, kind: MnKind::Scalar { name: "Date", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 60, kind: MnKind::Scalar { name: "AFPointsInFocus", pc: Pc::Enum(PENTAX_MAIN_PC_31), bin: false, skip: Skip::Never } },
    MnTag { id: 61, kind: MnKind::Scalar { name: "DataScaling", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 62, kind: MnKind::Scalar { name: "PreviewImageBorders", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 64, kind: MnKind::Scalar { name: "SensitivityAdjust", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 65, kind: MnKind::Scalar { name: "ImageEditCount", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 7, kind: MnKind::Scalar { name: "Time", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 71, kind: MnKind::Scalar { name: "CameraTemperature", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 72, kind: MnKind::Scalar { name: "AELock", pc: Pc::Enum(PENTAX_MAIN_PC_32), bin: false, skip: Skip::Never } },
    MnTag { id: 73, kind: MnKind::Scalar { name: "NoiseReduction", pc: Pc::Enum(PENTAX_MAIN_PC_33), bin: false, skip: Skip::Never } },
    MnTag { id: 77, kind: MnKind::Scalar { name: "FlashExposureComp", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 79, kind: MnKind::Scalar { name: "ImageTone", pc: Pc::Enum(PENTAX_MAIN_PC_34), bin: false, skip: Skip::Never } },
    MnTag { id: 8, kind: MnKind::Scalar { name: "Quality", pc: Pc::Enum(PENTAX_MAIN_PC_35), bin: false, skip: Skip::Never } },
    MnTag { id: 80, kind: MnKind::Scalar { name: "ColorTemperature", pc: Pc::None, bin: false, skip: Skip::Eq(0) } },
    MnTag { id: 83, kind: MnKind::Scalar { name: "ColorTempDaylight", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 84, kind: MnKind::Scalar { name: "ColorTempShade", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 85, kind: MnKind::Scalar { name: "ColorTempCloudy", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 86, kind: MnKind::Scalar { name: "ColorTempTungsten", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 87, kind: MnKind::Scalar { name: "ColorTempFluorescentD", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 88, kind: MnKind::Scalar { name: "ColorTempFluorescentN", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 89, kind: MnKind::Scalar { name: "ColorTempFluorescentW", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 9, kind: MnKind::Scalar { name: "PentaxImageSize", pc: Pc::EnumStr(PENTAX_MAIN_PC_36), bin: false, skip: Skip::Never } },
    MnTag { id: 90, kind: MnKind::Scalar { name: "ColorTempFlash", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 93, kind: MnKind::Scalar { name: "ShutterCount", pc: Pc::None, bin: false, skip: Skip::Never } },
    MnTag { id: 98, kind: MnKind::Scalar { name: "RawDevelopmentProcess", pc: Pc::Enum(PENTAX_MAIN_PC_37), bin: false, skip: Skip::Never } },
];
