//! REST import of noise, texture and glass effects. Fixtures are synthetic, written by hand from
//! the published schema.

use super::*;
use figcraft_doc::{EffectKind, NoiseMode};
use serde_json::json;

fn imported(effects: Value) -> (Vec<figcraft_doc::Effect>, Vec<String>) {
    let file = json!({"name": "Fixture", "schemaVersion": 0, "document": {"id": "0:0", "name": "Document", "type": "DOCUMENT", "children": [
        {"id": "0:1", "name": "Page", "type": "CANVAS", "children": [
            {"id": "1", "name": "S", "type": "RECTANGLE", "relativeTransform": [[1, 0, 0], [0, 1, 0]], "size": {"x": 10, "y": 10}, "effects": effects}
        ]}
    ]}});
    let (doc, warnings) = import(&file, &no_images).unwrap();
    (doc.nodes().find(|n| n.name == "S").unwrap().effects.clone(), warnings)
}

#[test]
fn noise_texture_and_glass_import() {
    let (fx, warnings) = imported(json!([
        {"type": "NOISE", "noiseType": "DUOTONE", "noiseSize": 2.5, "density": 0.7, "color": {"r": 1, "g": 0, "b": 0, "a": 0.5},
         "secondaryColor": {"r": 0, "g": 0, "b": 1, "a": 1}, "blendMode": "MULTIPLY", "visible": true},
        {"type": "NOISE", "noiseType": "MULTITONE", "noiseSizeVector": {"x": 1, "y": 3}, "density": 9, "opacity": 0.4},
        {"type": "TEXTURE", "noiseSize": 1.5, "radius": 6, "clipToShape": true},
        {"type": "GLASS", "lightIntensity": 0.6, "lightAngle": 30, "refraction": 0.9, "depth": 12, "dispersion": 0.2, "radius": 8, "visible": false}
    ]));
    assert!(warnings.is_empty(), "{warnings:?}");
    let EffectKind::Noise(a) = &fx[0].kind else { panic!("{fx:?}") };
    assert_eq!((a.mode, a.size, a.size_y, a.density), (NoiseMode::Duo, 2.5, None, 0.7));
    assert_eq!((a.color.a, a.secondary.b, a.blend), (0.5, 1.0, figcraft_doc::color::BlendMode::Multiply));
    let EffectKind::Noise(b) = &fx[1].kind else { panic!("{fx:?}") };
    assert_eq!((b.mode, b.size, b.size_y, b.density, b.opacity), (NoiseMode::Multi, 1.0, Some(3.0), 1.0, 0.4));
    let EffectKind::Texture(t) = &fx[2].kind else { panic!("{fx:?}") };
    assert_eq!((t.size, t.radius, t.clip_to_shape), (1.5, 6.0, true));
    let EffectKind::Glass(g) = &fx[3].kind else { panic!("{fx:?}") };
    assert_eq!((g.light_intensity, g.light_angle, g.refraction, g.depth, g.dispersion, g.frost), (0.6, 30.0, 0.9, 12.0, 0.2, 8.0));
    assert!(!fx[3].visible);
}

#[test]
fn junk_effect_values_import_sanitised() {
    let (fx, _) = imported(json!([
        {"type": "GLASS", "refraction": "lots", "depth": -4, "dispersion": 1e9},
        {"type": "NOISE", "noiseType": 7, "density": -2, "noiseSize": -1},
        {"type": "TEXTURE", "radius": "far"}
    ]));
    let EffectKind::Glass(g) = &fx[0].kind else { panic!("{fx:?}") };
    assert_eq!((g.refraction, g.depth, g.dispersion), (figcraft_doc::Glass::default().refraction, 1.0, 1.0));
    let EffectKind::Noise(n) = &fx[1].kind else { panic!("{fx:?}") };
    assert_eq!((n.mode, n.density, n.size), (NoiseMode::Mono, 0.0, 0.0));
    let EffectKind::Texture(t) = &fx[2].kind else { panic!("{fx:?}") };
    assert_eq!(t.radius, figcraft_doc::Texture::default().radius);
}
