use super::*;

fn close(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

#[test]
fn profile_ids_parse_and_serde() {
    for p in ColorProfile::ALL {
        assert_eq!(ColorProfile::parse(p.id()), Some(p));
        assert_eq!(ColorProfile::parse(p.label()), Some(p));
        assert_eq!(serde_json::to_value(p).unwrap(), serde_json::json!(p.id()));
    }
    assert_eq!(ColorProfile::parse("display-p3"), Some(ColorProfile::DisplayP3));
    assert_eq!(ColorProfile::parse("P3"), Some(ColorProfile::DisplayP3));
    assert_eq!(ColorProfile::parse("adobe rgb"), None);
    assert_eq!(ColorProfile::default(), ColorProfile::Srgb);
}

#[test]
fn srgb_red_in_display_p3() {
    // sRGB #FF0000 is color(display-p3 0.9175 0.2003 0.1386).
    let p3 = ColorProfile::Srgb.conversion_to(ColorProfile::DisplayP3).apply(Color::new(1.0, 0.0, 0.0, 0.5));
    assert!(close(p3.r, 0.9175, 0.001) && close(p3.g, 0.2003, 0.001) && close(p3.b, 0.1386, 0.001), "{p3:?}");
    assert_eq!(p3.a, 0.5);
}

#[test]
fn conversions_round_trip_and_keep_neutrals() {
    let to_p3 = ColorProfile::Srgb.conversion_to(ColorProfile::DisplayP3);
    let back = ColorProfile::DisplayP3.conversion_to(ColorProfile::Srgb);
    assert!(ColorProfile::Srgb.conversion_to(ColorProfile::Srgb).is_identity());
    for hex in ["0D99FF", "F24822", "14AE5C", "000000", "FFFFFF", "808080", "9747FF", "FF0000", "00FF00"] {
        let c = Color::from_hex(hex).unwrap();
        assert_eq!(back.apply(to_p3.apply(c)).to_hex(), hex);
    }
    // Shared white point: greys stay grey.
    let g = to_p3.apply(Color::from_hex("808080").unwrap());
    assert!(close(g.r, g.g, 1e-4) && close(g.g, g.b, 1e-4), "{g:?}");
    // Display P3's pure green is outside sRGB: it clips into range.
    let c = back.apply(Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(c.r == 0.0 && c.g == 1.0 && c.b == 0.0, "{c:?}");
}

#[test]
fn conversion_survives_junk() {
    let c = ColorProfile::DisplayP3.conversion_to(ColorProfile::Srgb).apply(Color { r: f32::NAN, g: f32::INFINITY, b: -3.0, a: 2.0 });
    for v in [c.r, c.g, c.b, c.a] {
        assert!((0.0..=1.0).contains(&v));
    }
}

fn be_u32(b: &[u8], at: usize) -> u32 {
    u32::from_be_bytes(b[at..at + 4].try_into().unwrap())
}

fn s15(b: &[u8], at: usize) -> f64 {
    f64::from(i32::from_be_bytes(b[at..at + 4].try_into().unwrap())) / 65536.0
}

/// The tag `sig`'s data in an ICC profile.
fn tag<'a>(icc: &'a [u8], sig: &[u8; 4]) -> &'a [u8] {
    let n = be_u32(icc, 128) as usize;
    (0..n)
        .map(|i| 132 + i * 12)
        .find(|at| &icc[*at..*at + 4] == sig)
        .map(|at| {
            let (off, len) = (be_u32(icc, at + 4) as usize, be_u32(icc, at + 8) as usize);
            &icc[off..off + len]
        })
        .unwrap()
}

#[test]
fn icc_profiles_are_well_formed() {
    // Published D50-adapted colorants of sRGB and Display P3.
    let expect = [
        (ColorProfile::Srgb, [[0.4361, 0.2225, 0.0139], [0.3851, 0.7169, 0.0971], [0.1431, 0.0606, 0.7142]]),
        (ColorProfile::DisplayP3, [[0.5151, 0.2412, -0.0010], [0.2920, 0.6922, 0.0419], [0.1571, 0.0666, 0.7844]]),
    ];
    for (p, cols) in expect {
        let icc = p.icc();
        assert_eq!(be_u32(&icc, 0) as usize, icc.len());
        assert_eq!(icc.len() % 4, 0);
        assert_eq!(&icc[36..40], b"acsp");
        assert_eq!(&icc[12..24], b"mntrRGB XYZ ");
        assert_eq!(be_u32(&icc, 8), 0x0430_0000);
        for (sig, col) in [(b"rXYZ", cols[0]), (b"gXYZ", cols[1]), (b"bXYZ", cols[2])] {
            let t = tag(&icc, sig);
            assert_eq!(&t[..4], b"XYZ ");
            for (k, v) in col.iter().enumerate() {
                assert!((s15(t, 8 + k * 4) - v).abs() < 0.0006, "{p:?} {} {k}: {}", String::from_utf8_lossy(sig), s15(t, 8 + k * 4));
            }
        }
        // The colorants add up to the D50 white point.
        let sum: f64 = [b"rXYZ", b"gXYZ", b"bXYZ"].iter().map(|s| s15(tag(&icc, s), 8)).sum();
        assert!((sum - 0.9642).abs() < 0.0005, "{sum}");
        let trc = tag(&icc, b"gTRC");
        assert_eq!(&trc[..4], b"para");
        assert!((s15(trc, 12) - 2.4).abs() < 1e-4);
        let desc = tag(&icc, b"desc");
        assert_eq!(&desc[..4], b"mluc");
        let text: Vec<u16> = desc[28..].chunks(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
        assert_eq!(String::from_utf16(&text).unwrap(), p.label());
    }
}

#[test]
fn pixel_conversion_matches_colour_conversion() {
    let conv = ColorProfile::DisplayP3.conversion_to(ColorProfile::Srgb);
    let samples = [[234u8, 51, 35, 255], [10, 200, 90, 255], [0, 0, 0, 255], [255, 255, 255, 255], [60, 120, 30, 128], [0, 0, 0, 0]];
    let mut px: Vec<u8> = samples.iter().flatten().copied().collect();
    conv.apply_pixels(&mut px);
    for (s, got) in samples.iter().zip(px.chunks(4)) {
        assert_eq!(got[3], s[3], "alpha stays");
        if s[3] == 0 {
            assert_eq!(got, s, "transparent pixels stay");
            continue;
        }
        // Premultiplied in, premultiplied out.
        let a = f32::from(s[3]) / 255.0;
        let un = Color::new(f32::from(s[0]) / 255.0 / a, f32::from(s[1]) / 255.0 / a, f32::from(s[2]) / 255.0 / a, 1.0);
        let want = conv.apply(un).to_rgba8();
        for c in 0..3 {
            let w = (f32::from(want[c]) * a).round();
            assert!((f32::from(got[c]) - w).abs() <= 2.0, "{s:?} → {got:?}, want {want:?}");
        }
    }
    // sRGB #FF0000 kept by a Display P3 file comes back to the screen as itself.
    let mut red = [234, 51, 35, 255];
    conv.apply_pixels(&mut red);
    assert_eq!(red, [255, 0, 0, 255]);
    let mut same = px.clone();
    ColorProfile::Srgb.conversion_to(ColorProfile::Srgb).apply_pixels(&mut same);
    assert_eq!(same, px);
}

#[test]
fn image_presets() {
    assert_eq!(ImageFilters::preset("Black & white"), ImageFilters::preset("blackAndWhite"));
    assert_eq!(ImageFilters::preset("black-and-white").map(|f| f.saturation), Some(-1.0));
    assert_eq!(ImageFilters::preset("reset"), Some(ImageFilters::default()));
    assert!(ImageFilters::preset("sepia-ish").is_none());
    for (id, label, f) in ImageFilters::PRESETS {
        assert_eq!(ImageFilters::preset(id), Some(f));
        assert_eq!(ImageFilters::preset(label), Some(f));
        assert_eq!(f.sanitized(), f, "{id} stays in range");
    }
    let junk = ImageFilters { exposure: f32::NAN, contrast: 9.0, ..ImageFilters::NONE }.sanitized();
    assert_eq!((junk.exposure, junk.contrast), (0.0, 1.0));
}

#[test]
fn paint_colors_visit_solids_and_stops() {
    let mut p = Paint::gradient(Gradient::default_for(GradientKind::Linear, Color::WHITE));
    let mut n = 0;
    p.for_each_color_mut(&mut |c| {
        *c = Color::BLACK.with_alpha(c.a);
        n += 1;
    });
    assert_eq!(n, 2);
    assert_eq!(p.preview_color(), Color::BLACK);
    let mut img = Paint::image("h", ImageScaleMode::Fill);
    img.for_each_color_mut(&mut |_| panic!("images have no colours"));
}

#[test]
fn gamut_checks_find_p3_colours_outside_srgb() {
    let to_srgb = ColorProfile::DisplayP3.conversion_to(ColorProfile::Srgb);
    assert!(to_srgb.clips(Color::new(0.0, 1.0, 0.0, 1.0)), "P3 green is outside sRGB");
    assert!(!to_srgb.clips(Color::new(0.5, 0.5, 0.5, 1.0)), "greys fit");
    // sRGB red written as 8-bit P3 values still fits (rounding isn't clipping).
    let red = ColorProfile::Srgb.conversion_to(ColorProfile::DisplayP3).apply(Color::new(1.0, 0.0, 0.0, 1.0));
    let [r, g, b, _] = red.to_rgba8();
    assert!(!to_srgb.clips(Color::from_rgba8(r, g, b, 255)));
    assert!(!ColorProfile::Srgb.conversion_to(ColorProfile::Srgb).clips(Color::new(0.0, 1.0, 0.0, 1.0)));
    // Pixels: premultiplied, transparent ones never clip.
    let px = [0, 255, 0, 255, r, g, b, 255, 0, 128, 0, 128, 0, 255, 0, 0];
    assert_eq!(to_srgb.clipped_pixels(&px), [true, false, true, false]);
}
