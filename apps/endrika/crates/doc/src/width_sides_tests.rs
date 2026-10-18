//! One-sided variable widths: width points whose sides differ, interpolated side by side, edited
//! one side at a time, sanitized, and saved without changing files that don't use them.

use crate::{StrokeProps, WidthPoint, WidthPreset, WidthProfile, WidthSide};

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn a_point_is_built_from_and_reports_its_sides() {
    let p = WidthPoint::from_sides(0.5, 1.0, 3.0);
    assert!(near(p.scale, 2.0) && near(p.bias, 0.5), "{p:?}");
    let (l, r) = p.sides();
    assert!(near(l, 1.0) && near(r, 3.0));
    assert!(!p.symmetric());
    // All on one side, nothing at all, and junk.
    assert_eq!(WidthPoint::from_sides(0.0, 0.0, 2.0).sides(), (0.0, 2.0));
    assert_eq!(WidthPoint::from_sides(0.0, 0.0, 0.0), WidthPoint::new(0.0, 0.0));
    assert_eq!(WidthPoint::from_sides(0.0, f64::NAN, -4.0), WidthPoint::new(0.0, 0.0));
    assert!(WidthPoint::from_sides(0.0, 2.0, 2.0).symmetric());
    assert_eq!(WidthSide::from_id("LEFT"), Some(WidthSide::Left));
    assert_eq!(WidthSide::from_id("up"), None);
    assert_eq!(WidthSide::Right.id(), "right");
}

#[test]
fn sides_interpolate_on_their_own_and_symmetric_profiles_are_unchanged() {
    // Left stays 1 while the right swells to 3 and back.
    let p = WidthProfile { points: vec![WidthPoint::new(0.0, 1.0), WidthPoint::from_sides(0.5, 1.0, 3.0), WidthPoint::new(1.0, 1.0)] }.sanitized();
    assert!(!p.symmetric());
    for i in 0..=20 {
        let t = f64::from(i) / 20.0;
        let (l, r) = p.sides_at(t);
        assert!(near(l, 1.0), "the left side holds at {t}: {l}");
        assert!((1.0..=3.0 + 1e-9).contains(&r), "{t}: {r}");
        assert!(near(p.scale_at(t), (l + r) / 2.0));
    }
    assert!(near(p.sides_at(0.5).1, 3.0));
    // The widest side bounds the stroke.
    assert!(near(p.max_scale(), 3.0));
    // A symmetric profile interpolates exactly as before.
    let eye = WidthPreset::Eye.profile().unwrap();
    assert!(eye.symmetric());
    for i in 0..=10 {
        let t = f64::from(i) / 10.0;
        let (l, r) = eye.sides_at(t);
        assert_eq!(l, r);
        assert_eq!(eye.scale_at(t), l);
    }
}

#[test]
fn one_side_is_edited_and_a_new_width_makes_a_point_symmetric_again() {
    let mut p = WidthProfile::uniform();
    assert!(p.set_side(1, WidthSide::Right, 4.0));
    assert_eq!(p.points[1].sides(), (1.0, 4.0));
    assert!(p.set_side(1, WidthSide::Left, 0.0));
    assert_eq!(p.points[1].sides(), (0.0, 4.0));
    assert!(!p.set_side(9, WidthSide::Left, 1.0), "no such point");
    assert!(p.set_side(1, WidthSide::Left, f64::NAN), "junk leaves it as it was");
    assert_eq!(p.points[1].sides(), (0.0, 4.0));
    // A point added between keeps both sides as wide as the stroke is there.
    let i = p.insert_here(0.5);
    let (l, r) = p.points[i].sides();
    let (wl, wr) = p.sides_at(0.5);
    assert!(near(l, wl) && near(r, wr), "{l} {r} vs {wl} {wr}");
    assert!(l < r);
    // Setting the width (both sides) makes it symmetric.
    assert!(p.set_point(i, None, Some(2.0)));
    assert!(p.points[i].symmetric());
    assert_eq!(p.points[i].scale, 2.0);
}

#[test]
fn hostile_biases_are_sanitized() {
    let p = WidthProfile {
        points: vec![
            WidthPoint { t: 0.2, scale: 1.0, bias: f64::NAN },
            WidthPoint { t: 0.8, scale: 1.0, bias: 7.0 },
            WidthPoint { t: 0.5, scale: 2.0, bias: -1e9 },
        ],
    }
    .sanitized();
    assert_eq!(p.points.iter().map(|q| q.bias).collect::<Vec<_>>(), vec![0.0, -1.0, 1.0]);
    for i in 0..=10 {
        let (l, r) = p.sides_at(f64::from(i) / 10.0);
        assert!(l.is_finite() && r.is_finite() && l >= 0.0 && r >= 0.0);
    }
}

#[test]
fn files_keep_their_shape() {
    // Symmetric points save exactly as before (no bias key); one-sided ones round-trip.
    let sym = serde_json::to_value(WidthPoint::new(0.5, 2.0)).unwrap();
    assert_eq!(sym, serde_json::json!({"t": 0.5, "scale": 2.0}));
    let old: WidthPoint = serde_json::from_value(serde_json::json!({"t": 0.25, "scale": 3.0})).unwrap();
    assert_eq!(old, WidthPoint::new(0.25, 3.0));
    let one = WidthPoint::from_sides(0.5, 1.0, 3.0);
    let back: WidthPoint = serde_json::from_value(serde_json::to_value(one).unwrap()).unwrap();
    assert_eq!(back, one);
    let stroke = StrokeProps { width_profile: Some(WidthProfile { points: vec![one] }), ..Default::default() };
    let back: StrokeProps = serde_json::from_str(&serde_json::to_string(&stroke).unwrap()).unwrap();
    assert_eq!(back, stroke);
    assert!(near(back.max_weight(), 3.0));
}
