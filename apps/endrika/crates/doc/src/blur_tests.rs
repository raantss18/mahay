//! Blur settings: files from before progressive blur open as uniform blurs, progressive fields
//! round-trip (camel-case names too), hostile values are made safe, and reach covers the largest
//! radius.

use crate::*;

#[test]
fn old_blur_files_open_as_uniform() {
    let e: Effect = serde_json::from_str(r#"{"type":"layerBlur","radius":6.0}"#).unwrap();
    assert_eq!(e.kind, EffectKind::LayerBlur(Blur::uniform(6.0)));
    let e: Effect = serde_json::from_str(r#"{"type":"backgroundBlur","radius":3.0,"visible":false}"#).unwrap();
    assert_eq!((e.blur().map(|b| (b.radius, b.blur_type)), e.visible), (Some((3.0, BlurType::Uniform)), false));
}

#[test]
fn progressive_fields_round_trip() {
    let e: Effect = serde_json::from_str(
        r#"{"type":"layerBlur","radius":20,"blurType":"PROGRESSIVE","startRadius":2,"startOffset":[0.1,0.2],"endOffset":[0.9,0.8]}"#,
    )
    .unwrap();
    let b = e.blur().unwrap();
    assert_eq!((b.blur_type, b.radius, b.start_radius, b.start, b.end), (BlurType::Progressive, 20.0, 2.0, (0.1, 0.2), (0.9, 0.8)));
    let back: Effect = serde_json::from_str(&serde_json::to_string(&e).unwrap()).unwrap();
    assert_eq!(back, e);
    // The snake-case names the file format writes, and "normal" as uniform.
    let e: Effect = serde_json::from_str(r#"{"type":"backgroundBlur","radius":4,"blur_type":"normal","start_radius":9}"#).unwrap();
    assert!(!e.blur().unwrap().is_progressive());
}

#[test]
fn hostile_blur_values_are_made_safe() {
    let b =
        Blur { radius: f64::INFINITY, blur_type: BlurType::Progressive, start_radius: -5.0, start: (f64::NAN, 1e9), end: (f64::NEG_INFINITY, 0.5) }
            .sanitized();
    assert_eq!((b.radius, b.start_radius), (0.0, 0.0));
    assert_eq!((b.start, b.end), ((0.5, 1000.0), (0.5, 0.5)));
    let huge = Blur { radius: 1e300, ..Blur::default() };
    assert_eq!(huge.max_radius(), MAX_BLUR_RADIUS);
}

#[test]
fn reach_and_backdrop_cover_the_largest_radius() {
    let b = Blur { radius: 2.0, blur_type: BlurType::Progressive, start_radius: 30.0, ..Blur::default() };
    assert_eq!(b.max_radius(), 30.0);
    // A uniform blur ignores the progressive start radius.
    assert_eq!(Blur { blur_type: BlurType::Uniform, ..b.clone() }.max_radius(), 2.0);
    assert_eq!(Effect { kind: EffectKind::LayerBlur(b.clone()), visible: true }.reach(), 45.0);
    assert_eq!(Effect { kind: EffectKind::BackgroundBlur(b.clone()), visible: true }.backdrop_radius(), 30.0);
    // Scaling scales both radii, never the unit-box points.
    let s = b.scaled(2.0);
    assert_eq!((s.radius, s.start_radius, s.start, s.end), (4.0, 60.0, b.start, b.end));
}
