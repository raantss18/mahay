//! REST import of progressive blurs (`blurType`, `startRadius`, `startOffset`, `endOffset`).
//! Fixtures are synthetic, written by hand from the published schema.

use super::*;
use figcraft_doc::{BlurType, EffectKind};
use serde_json::json;

fn imported(effects: Value) -> Vec<figcraft_doc::Effect> {
    let file = json!({"name": "Fixture", "schemaVersion": 0, "document": {"id": "0:0", "name": "Document", "type": "DOCUMENT", "children": [
        {"id": "0:1", "name": "Page", "type": "CANVAS", "children": [
            {"id": "1", "name": "S", "type": "RECTANGLE", "relativeTransform": [[1, 0, 0], [0, 1, 0]], "size": {"x": 10, "y": 10}, "effects": effects}
        ]}
    ]}});
    let (doc, warnings) = import(&file, &no_images).unwrap();
    assert!(warnings.is_empty(), "{warnings:?}");
    doc.nodes().find(|n| n.name == "S").unwrap().effects.clone()
}

#[test]
fn progressive_blurs_import() {
    let fx = imported(json!([
        {"type": "LAYER_BLUR", "radius": 24, "blurType": "PROGRESSIVE", "startRadius": 4, "startOffset": {"x": 0.5, "y": 0.2}, "endOffset": {"x": 0.5, "y": 1}},
        {"type": "BACKGROUND_BLUR", "radius": 10, "blurType": "PROGRESSIVE"},
        {"type": "LAYER_BLUR", "radius": 6, "blurType": "NORMAL", "startRadius": 3}
    ]));
    let EffectKind::LayerBlur(a) = &fx[0].kind else { panic!("{fx:?}") };
    assert_eq!((a.blur_type, a.radius, a.start_radius, a.start, a.end), (BlurType::Progressive, 24.0, 4.0, (0.5, 0.2), (0.5, 1.0)));
    // Missing fields take the defaults: from the top (no blur) to the bottom.
    let EffectKind::BackgroundBlur(b) = &fx[1].kind else { panic!("{fx:?}") };
    assert_eq!((b.blur_type, b.start_radius, b.start, b.end), (BlurType::Progressive, 0.0, (0.5, 0.0), (0.5, 1.0)));
    let EffectKind::LayerBlur(c) = &fx[2].kind else { panic!("{fx:?}") };
    assert!(!c.is_progressive());
    assert_eq!(c.radius, 6.0);
}

#[test]
fn hostile_progressive_fields_are_clamped() {
    let fx = imported(json!([
        {"type": "LAYER_BLUR", "radius": 1e300, "blurType": "PROGRESSIVE", "startRadius": -7, "startOffset": {"x": "a", "y": 0}, "endOffset": {"x": 1e300, "y": -1e300}},
        {"type": "BACKGROUND_BLUR", "radius": 5, "blurType": 7, "startOffset": [1, 2]}
    ]));
    let a = fx[0].blur().unwrap();
    assert_eq!((a.radius, a.start_radius, a.start, a.end), (figcraft_doc::MAX_BLUR_RADIUS, 0.0, (0.5, 0.0), (1000.0, -1000.0)));
    let b = fx[1].blur().unwrap();
    assert!(!b.is_progressive());
}
