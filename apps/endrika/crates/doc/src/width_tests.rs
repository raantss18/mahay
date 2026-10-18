//! Variable width profiles: interpolation, editing, presets and loading older files.

use crate::{StrokeProps, WidthPoint, WidthPreset, WidthProfile};

#[test]
fn interpolates_smoothly_without_overshoot() {
    let eye = WidthPreset::Eye.profile().unwrap();
    assert_eq!(eye.scale_at(0.0), 0.0);
    assert_eq!(eye.scale_at(0.5), 1.0);
    assert_eq!(eye.scale_at(1.0), 0.0);
    // Symmetric, rising then falling, never above the middle point.
    let a = eye.scale_at(0.25);
    assert!((a - eye.scale_at(0.75)).abs() < 1e-12);
    assert!(a > 0.5 && a < 1.0, "{a}");
    for i in 0..=100 {
        let s = eye.scale_at(i as f64 / 100.0);
        assert!((0.0..=1.0).contains(&s), "{s}");
    }
    // Two points interpolate linearly; outside the points the ends hold.
    let wedge = WidthPreset::Wedge.profile().unwrap();
    assert!((wedge.scale_at(0.3) - 0.7).abs() < 1e-12);
    let held = WidthProfile::new([(0.25, 2.0), (0.75, 4.0)]);
    assert_eq!(held.scale_at(0.0), 2.0);
    assert_eq!(held.scale_at(1.0), 4.0);
    assert_eq!(held.max_scale(), 4.0);
    // Monotone between points of a rising profile.
    let rise = WidthProfile::new([(0.0, 0.0), (0.1, 3.0), (0.2, 3.1), (1.0, 8.0)]);
    let mut prev = -1.0;
    for i in 0..=200 {
        let s = rise.scale_at(i as f64 / 200.0);
        assert!(s >= prev - 1e-12, "monotone: {s} after {prev}");
        prev = s;
    }
    // Empty is uniform; a single point holds everywhere.
    assert_eq!(WidthProfile::default().scale_at(0.4), 1.0);
    assert_eq!(WidthProfile::new([(0.5, 3.0)]).scale_at(0.1), 3.0);
}

#[test]
fn sanitizes_hostile_points() {
    let p = WidthProfile {
        points: vec![
            WidthPoint::new(f64::NAN, 1.0),
            WidthPoint::new(2.0, -5.0),
            WidthPoint::new(-1.0, 1e9),
            WidthPoint::new(0.5, f64::INFINITY),
            WidthPoint::new(0.5, 0.5),
            WidthPoint::new(0.5, 0.7),
        ],
    }
    .sanitized();
    assert_eq!(
        p.points,
        vec![WidthPoint::new(0.0, WidthProfile::MAX_SCALE), WidthPoint::new(0.5, 0.5), WidthPoint::new(0.5, 0.7), WidthPoint::new(1.0, 0.0)]
    );
    // Coincident points don't divide by zero.
    assert!(p.scale_at(0.5).is_finite());
    assert!(p.scale_at(f64::NAN).is_finite());
    let many = WidthProfile::new((0..500).map(|i| (i as f64 / 500.0, 1.0)));
    assert_eq!(many.points.len(), WidthProfile::MAX_POINTS);
}

#[test]
fn edits_keep_points_in_order() {
    let mut p = WidthProfile::uniform();
    let i = p.insert(0.4, 3.0);
    assert_eq!(i, 1);
    assert_eq!(p.points.len(), 3);
    // Moving is limited by the neighbours, so indices stay put.
    assert!(p.set_point(1, Some(1.5), None));
    assert_eq!(p.points[1].t, 1.0);
    assert!(p.set_point(1, Some(0.3), Some(-2.0)));
    assert_eq!(p.points[1], WidthPoint::new(0.3, 0.0));
    assert!(!p.set_point(9, Some(0.5), None));
    let i = p.insert(f64::NAN, f64::NAN);
    assert_eq!(p.points[i], WidthPoint::new(0.0, 1.0), "junk lands at the start at full width");
}

#[test]
fn presets_round_trip() {
    for p in WidthPreset::ALL {
        assert_eq!(WidthPreset::from_id(p.id()), Some(p));
        assert_eq!(WidthPreset::from_id(p.label()), Some(p));
        if let Some(prof) = p.profile() {
            assert_eq!(prof.preset(), Some(p));
            assert!(prof.points.windows(2).all(|w| w[0].t <= w[1].t));
        }
    }
    assert_eq!(WidthPreset::from_id("Taper both ends"), Some(WidthPreset::Taper));
    assert_eq!(WidthPreset::from_id("nope"), None);
    assert_eq!(WidthProfile::new([(0.0, 1.0), (0.3, 2.0)]).preset(), None);
}

#[test]
fn older_strokes_load_and_uniform_strokes_save_as_before() {
    let old: StrokeProps = serde_json::from_str(r#"{"weight": 3, "align": "center"}"#).unwrap();
    assert!(old.width_profile.is_none());
    assert_eq!(old.max_weight(), 3.0);
    let json = serde_json::to_string(&StrokeProps::default()).unwrap();
    assert!(!json.contains("width_profile"), "{json}");
    let s = StrokeProps { weight: 2.0, width_profile: WidthPreset::Taper.profile(), ..Default::default() };
    let back: StrokeProps = serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
    assert_eq!(back, s);
    assert_eq!(back.max_weight(), 2.0);
    // A profile without points is no profile.
    let empty = StrokeProps { width_profile: Some(WidthProfile::default()), ..Default::default() };
    assert!(empty.profile().is_none());
}
