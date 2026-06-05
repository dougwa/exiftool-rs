//! Composite tags — ExifTool's post-extraction computed tags.
//!
//! A composite tag is derived from other tags that were already extracted. Each
//! definition lists its source tags (ExifTool's `Require` — must be present —
//! and `Desire` — optional) and a formula combining them. Composites can depend
//! on other composites (e.g. `CircleOfConfusion` needs `ScaleFactor35efl`,
//! `DOF` needs `Aperture` and `CircleOfConfusion`), so we evaluate in dependency
//! order and loop until no further composite can be built.
//!
//! ExifTool's formulas operate on each source's *ValueConv* (numeric) value and
//! sometimes its *PrintConv* (`$prt`) string. We carry both on `ExtractedTag`
//! (`value` and `print`); for the standard EXIF sources used here the ValueConv
//! value equals the raw value, so `Value::as_f64` is the right input.
//!
//! Only the common, well-defined composites are ported. The long tail (lens
//! databases, GPS/date merges, format-specific ones) is left for later.

use crate::exif::printconv;
use crate::tag::ExtractedTag;
use crate::value::Value;

/// A resolved source tag (borrowed from the extracted set).
struct Src<'a> {
    value: &'a Value,
    print: &'a str,
}

impl Src<'_> {
    fn f(&self) -> Option<f64> {
        self.value.as_f64()
    }
}

/// `args[i].f()` or None.
fn fv(args: &[Option<Src>], i: usize) -> Option<f64> {
    args.get(i).and_then(|o| o.as_ref()).and_then(Src::f)
}

/// A composite definition: ordered source names (with a required flag) and the
/// builder that turns the resolved sources into a (raw value, print) pair.
struct Def {
    name: &'static str,
    /// Sources in ExifTool index order; `true` = Require, `false` = Desire.
    srcs: &'static [(&'static str, bool)],
    build: fn(&[Option<Src>], make: &str) -> Option<(Value, String)>,
}

/// Compute composite tags and append them to `tags` (ExifTool emits them in the
/// `Composite` group). Evaluated in dependency order with a multi-pass fixpoint
/// so composites that feed other composites resolve correctly.
pub fn compute(tags: &mut Vec<ExtractedTag>) {
    let make = tags
        .iter()
        .find(|t| t.name == "Make")
        .and_then(|t| t.value.as_str())
        .unwrap_or("")
        .to_string();

    let mut done = vec![false; DEFS.len()];
    loop {
        let mut progress = false;
        for (di, def) in DEFS.iter().enumerate() {
            if done[di] {
                continue;
            }
            // Resolve each source by name (first occurrence wins, as in
            // ExifTool's default duplicate suppression). Group-qualified names
            // like "Composite:DigitalZoom" match on the bare tag name.
            let args: Vec<Option<Src>> = def
                .srcs
                .iter()
                .map(|(n, _)| {
                    // A "Group:Name" source matches only that family-1 group
                    // (e.g. "Composite:DigitalZoom" must not pick up Canon's
                    // maker-note DigitalZoom); a bare name matches any group.
                    let (group, bare) = match n.split_once(':') {
                        Some((g, b)) => (Some(g), b),
                        None => (None, *n),
                    };
                    tags.iter()
                        .find(|t| t.name == bare && group.is_none_or(|g| t.group1 == g))
                        .map(|t| Src { value: &t.value, print: &t.print })
                })
                .collect();

            // A composite can only be built once all its Require sources exist.
            let missing_required = def
                .srcs
                .iter()
                .zip(&args)
                .any(|((_, req), a)| *req && a.is_none());
            if missing_required {
                continue; // maybe a later pass (a required composite dep)
            }

            match (def.build)(&args, &make) {
                Some((value, print)) => {
                    drop(args);
                    tags.push(ExtractedTag::new("Composite", "Composite", def.name, value, print));
                    done[di] = true;
                    progress = true;
                }
                None => {
                    // All required sources present but the formula declined
                    // (e.g. a guard failed): it won't succeed later either.
                    done[di] = true;
                }
            }
        }
        if !progress {
            break;
        }
    }
}

/// Number value -> Composite tag (Value::F + print). Convenience for builders.
fn num(value: f64, print: String) -> Option<(Value, String)> {
    Some((Value::F(vec![value]), print))
}

// ---------------------------------------------------------------------------
// Helper computations ported from Image::ExifTool::Exif.
// ---------------------------------------------------------------------------

const DIAG_35MM: f64 = 43.266615305567875; // sqrt(36^2 + 24^2)

/// Canon's `CalcSensorDiag`: the sensor diagonal (mm) recovered from the
/// *denominators* of the FocalPlaneX/YResolution rationals (Canon stores image
/// pixels in the numerator and sensor size in 1/1000-inch in the denominator).
fn canon_sensor_diag(xres: Option<&Value>, yres: Option<&Value>) -> Option<f64> {
    let den = |v: Option<&Value>| match v {
        Some(Value::R(r)) => r.first().map(|&(n, d)| (n, d)),
        _ => None,
    };
    let (xn, xd) = den(xres)?;
    let (yn, yd) = den(yres)?;
    // Verify Canon's assumptions before trusting the trick.
    if xn % 1000 == 0
        && yn % 1000 == 0
        && xn >= 640_000
        && yn >= 480_000
        && xn < 10_000_000
        && yn < 10_000_000
        && (61..1500).contains(&xd)
        && (61..1000).contains(&yd)
        && xd != yd
    {
        Some(((xd * xd + yd * yd) as f64).sqrt() * 0.0254)
    } else {
        None
    }
}

/// ExifTool's `CalcScaleFactor35efl` — the 35 mm-equivalent focal-length scale
/// factor (crop factor). Source indices match the Desire list.
fn calc_scale_factor(args: &[Option<Src>], make: &str) -> Option<(Value, String)> {
    let focal = fv(args, 0).unwrap_or(0.0);
    let foc35 = fv(args, 1).unwrap_or(0.0);
    if focal != 0.0 && foc35 != 0.0 {
        return num(foc35 / focal, String::new()); // print filled below
    }
    let digz = fv(args, 2).filter(|&z| z != 0.0).unwrap_or(1.0);
    let mut diag = fv(args, 3); // FocalPlaneDiagonal

    if make.starts_with("Canon") {
        let xr = args.get(8).and_then(|o| o.as_ref()).map(|s| s.value);
        let yr = args.get(9).and_then(|o| o.as_ref()).map(|s| s.value);
        if let Some(d) = canon_sensor_diag(xr, yr) {
            diag = Some(d);
        }
    }

    if diag.is_none() {
        // Try FocalPlaneX/YSize (mm) with an aspect-ratio sanity check.
        if let (Some(xs), Some(ys)) = (fv(args, 5), fv(args, 6)) {
            let a = xs / ys;
            if (a - 1.3333).abs() < 0.1 || (a - 1.5).abs() < 0.1 {
                diag = Some((xs * xs + ys * ys).sqrt());
            }
        }
    }

    if diag.is_none() {
        // Recover from focal-plane resolution + image dimensions.
        // units: mm per resolution unit (default inches = 25.4).
        let units = match args.get(7).and_then(|o| o.as_ref()).and_then(|s| s.value.as_i64()) {
            Some(3) => 10.0,   // cm
            Some(4) => 1.0,    // mm
            Some(5) => 0.001,  // um
            _ => 25.4,         // inches (2) or unknown
        };
        let x_res = fv(args, 8).filter(|&r| r != 0.0)?;
        let y_res = fv(args, 9).filter(|&r| r != 0.0).unwrap_or(x_res);
        // Find a (width,height) pair with a sane aspect ratio.
        let pairs = [(10usize, 11usize), (12, 13), (14, 15)];
        let mut wh = None;
        for (wi, hi) in pairs {
            if let (Some(w), Some(h)) = (fv(args, wi), fv(args, hi)) {
                if w != 0.0 && h != 0.0 {
                    let a = w / h;
                    if a > 0.5 && a < 2.0 {
                        wh = Some((w, h));
                        break;
                    }
                }
            }
        }
        let (w, h) = wh?;
        let w = w * units / x_res;
        let h = h * units / y_res;
        let d = (w * w + h * h).sqrt();
        if d > 1.0 && d < 100.0 {
            diag = Some(d);
        }
    }

    let diag = diag?;
    let sf = DIAG_35MM * digz / diag;
    num(sf, String::new())
}

// ---------------------------------------------------------------------------
// Composite definitions (dependency order: deps appear before dependents).
// ---------------------------------------------------------------------------

static DEFS: &[Def] = &[
    Def {
        name: "ScaleFactor35efl",
        srcs: &[
            ("FocalLength", false),
            ("FocalLengthIn35mmFormat", false),
            ("Composite:DigitalZoom", false),
            ("FocalPlaneDiagonal", false),
            ("SensorSize", false),
            ("FocalPlaneXSize", false),
            ("FocalPlaneYSize", false),
            ("FocalPlaneResolutionUnit", false),
            ("FocalPlaneXResolution", false),
            ("FocalPlaneYResolution", false),
            ("ExifImageWidth", false),
            ("ExifImageHeight", false),
            ("CanonImageWidth", false),
            ("CanonImageHeight", false),
            ("ImageWidth", false),
            ("ImageHeight", false),
        ],
        build: |args, make| {
            let (v, _) = calc_scale_factor(args, make)?;
            let sf = v.as_f64()?;
            num(sf, format!("{sf:.1}"))
        },
    },
    Def {
        name: "Aperture",
        srcs: &[("FNumber", false), ("ApertureValue", false)],
        build: |args, _| {
            // FNumber is a plain f-number; ApertureValue is APEX (Av), whose
            // ValueConv is 2^(Av/2). ExifTool uses the converted value.
            let a = fv(args, 0)
                .filter(|&x| x != 0.0)
                .or_else(|| fv(args, 1).filter(|&x| x != 0.0).map(|av| 2f64.powf(av / 2.0)))?;
            num(a, printconv::print_fnumber(a))
        },
    },
    Def {
        name: "ShutterSpeed",
        srcs: &[("ExposureTime", false), ("ShutterSpeedValue", false), ("BulbDuration", false)],
        build: |args, _| {
            // BulbDuration has a ValueConv (raw/10) we don't store separately;
            // its print string already carries the converted seconds, so parse
            // that rather than the raw value.
            let bulb = args
                .get(2)
                .and_then(|o| o.as_ref())
                .and_then(|s| s.print.parse::<f64>().ok());
            // ExposureTime is seconds; ShutterSpeedValue is APEX (Tv), whose
            // ValueConv is 2^(-Tv). ExifTool uses the converted value.
            let v = match bulb {
                Some(b) if b > 0.0 => b,
                _ => match fv(args, 0) {
                    Some(t) => t,
                    None => 2f64.powf(-fv(args, 1)?),
                },
            };
            num(v, printconv::print_exposure_time(v))
        },
    },
    Def {
        name: "ImageSize",
        srcs: &[
            ("ImageWidth", true),
            ("ImageHeight", true),
            ("ExifImageWidth", false),
            ("ExifImageHeight", false),
        ],
        build: |args, _| {
            let w = fv(args, 0)?;
            let h = fv(args, 1)?;
            let raw = format!("{} {}", w as i64, h as i64);
            let print = format!("{}x{}", w as i64, h as i64);
            Some((Value::Text(raw), print))
        },
    },
    Def {
        name: "Megapixels",
        srcs: &[("ImageSize", true)],
        build: |args, _| {
            let s = args[0].as_ref()?.value.as_str()?;
            let dims: Vec<f64> = s.split_whitespace().filter_map(|x| x.parse().ok()).collect();
            if dims.len() < 2 {
                return None;
            }
            let mp = dims[0] * dims[1] / 1_000_000.0;
            let print = if mp >= 1.0 {
                format!("{mp:.1}")
            } else if mp >= 0.001 {
                format!("{mp:.3}")
            } else {
                format!("{mp:.6}")
            };
            num(mp, print)
        },
    },
    Def {
        name: "CircleOfConfusion",
        srcs: &[("ScaleFactor35efl", true)],
        build: |args, _| {
            let sf = fv(args, 0).filter(|&x| x != 0.0)?;
            let coc = DIAG_35MM / (sf * 1440.0);
            num(coc, format!("{coc:.3} mm"))
        },
    },
    Def {
        name: "FocalLength35efl",
        srcs: &[("FocalLength", true), ("ScaleFactor35efl", false)],
        build: |args, _| {
            let focal = fv(args, 0)?;
            let sf = fv(args, 1);
            let efl = focal * sf.unwrap_or(1.0);
            let print = match sf {
                Some(s) if s != 0.0 => {
                    format!("{focal:.1} mm (35 mm equivalent: {efl:.1} mm)")
                }
                _ => format!("{efl:.1} mm"),
            };
            num(efl, print)
        },
    },
    Def {
        name: "FOV",
        srcs: &[("FocalLength", true), ("ScaleFactor35efl", true), ("FocusDistance", false)],
        build: |args, _| {
            let focal = fv(args, 0).filter(|&x| x != 0.0)?;
            let sf = fv(args, 1).filter(|&x| x != 0.0)?;
            let mut corr = 1.0;
            let fd = fv(args, 2);
            if let Some(d) = fd {
                let dd = 1000.0 * d - focal;
                if dd > 0.0 {
                    corr += focal / dd;
                }
            }
            let fd2 = (36.0_f64).atan2(2.0 * focal * sf * corr);
            let deg = fd2 * 360.0 / std::f64::consts::PI;
            let mut print = format!("{deg:.1} deg");
            if let Some(d) = fd {
                if d > 0.0 && d < 10000.0 {
                    let width = 2.0 * d * fd2.sin() / fd2.cos();
                    print = format!("{deg:.1} deg ({width:.2} m)");
                }
            }
            num(deg, print)
        },
    },
    Def {
        name: "HyperfocalDistance",
        srcs: &[("FocalLength", true), ("Aperture", true), ("CircleOfConfusion", true)],
        build: |args, _| {
            let focal = fv(args, 0)?;
            let ap = fv(args, 1)?;
            let coc = fv(args, 2)?;
            if ap == 0.0 || coc == 0.0 {
                return Some((Value::Text("inf".into()), "inf".into()));
            }
            let h = focal * focal / (ap * coc * 1000.0);
            num(h, format!("{h:.2} m"))
        },
    },
    Def {
        name: "DOF",
        srcs: &[
            ("FocalLength", true),
            ("Aperture", true),
            ("CircleOfConfusion", true),
            ("FocusDistance", false),
            ("SubjectDistance", false),
            ("ObjectDistance", false),
            ("ApproximateFocusDistance", false),
            ("FocusDistanceLower", false),
            ("FocusDistanceUpper", false),
        ],
        build: |args, _| {
            let f = fv(args, 0)?;
            let ap = fv(args, 1)?;
            let coc = fv(args, 2)?;
            // Subject distance: FocusDistance (0 -> infinity), else the
            // alternates, else the average of the lower/upper bounds.
            let d = match fv(args, 3) {
                Some(d) => {
                    if d == 0.0 {
                        1e10
                    } else {
                        d
                    }
                }
                None => fv(args, 4)
                    .or_else(|| fv(args, 5))
                    .or_else(|| fv(args, 6))
                    .or_else(|| match (fv(args, 7), fv(args, 8)) {
                        (Some(lo), Some(hi)) => Some((lo + hi) / 2.0),
                        _ => None,
                    })?,
            };
            if f == 0.0 || coc == 0.0 {
                return num(0.0, "0".into());
            }
            let t = ap * coc * (d * 1000.0 - f) / (f * f);
            let near = d / (1.0 + t);
            let mut far = d / (1.0 - t);
            if far < 0.0 {
                far = 0.0; // 0 means infinity
            }
            let print = if far == 0.0 {
                format!("inf ({near:.2} m - inf)")
            } else {
                let dof = far - near;
                if dof > 0.0 && dof < 0.02 {
                    format!("{dof:.3} m ({near:.3} - {far:.3} m)")
                } else {
                    format!("{dof:.2} m ({near:.2} - {far:.2} m)")
                }
            };
            Some((Value::Text(format!("{near} {far}")), print))
        },
    },
    Def {
        name: "LightValue",
        srcs: &[("Aperture", true), ("ShutterSpeed", true), ("ISO", true)],
        build: |args, _| {
            let ap = fv(args, 0).filter(|&x| x > 0.0)?;
            let ss = fv(args, 1).filter(|&x| x > 0.0)?;
            // ExifTool uses the ISO *print* value; parse a leading number.
            let iso_s = args[2].as_ref()?.print;
            let iso: f64 = iso_s.split_whitespace().next()?.parse().ok()?;
            if iso <= 0.0 {
                return None;
            }
            let lv = (ap * ap * 100.0 / (ss * iso)).log2();
            num(lv, format!("{lv:.1}"))
        },
    },
];
