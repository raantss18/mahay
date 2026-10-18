//! Effects SVG filters can't match, embedded as renders: noise grains over the vector layer,
//! texture and progressive layer blur whole, background blur from what lies beneath; inner shadows
//! over the fills and under the strokes. Every export stays valid SVG and is deterministic.

use figcraft_color::{BlendMode, Color, Paint};
use figcraft_doc::{Blur, BlurType, Document, Effect, EffectKind, NodeId, NodeKind, Noise, Shadow, StrokeAlign, Texture};

use super::*;

const FILL: &str = "#3366CC";
const STROKE: &str = "#00FF00";

fn layer(effects: Vec<EffectKind>) -> (Document, NodeId) {
    let mut d = Document::new("svg raster fx");
    let p = d.first_page().unwrap();
    let n = d.create(p, NodeKind::Rectangle, "S", 10.0, 10.0, 100.0, 80.0).unwrap();
    let node = d.get_mut(n).unwrap();
    node.fills = vec![Paint::solid(Color::from_hex(FILL).unwrap())];
    node.effects = effects.into_iter().map(|kind| Effect { kind, visible: true }).collect();
    (d, n)
}

/// Export and check the result parses as SVG.
fn export(d: &Document, ids: &[NodeId]) -> String {
    let svg = export_svg(d, ids, &SvgOptions::default()).unwrap();
    let mut d2 = Document::new("back");
    let p2 = d2.first_page().unwrap();
    import_svg(&mut d2, svg.as_bytes(), p2, 0.0, 0.0, "Imported").unwrap();
    svg
}

fn progressive() -> Blur {
    Blur { radius: 12.0, blur_type: BlurType::Progressive, ..Blur::default() }
}

#[test]
fn noise_is_a_crisp_grain_image_over_the_vector_layer() {
    let (d, n) = layer(vec![EffectKind::Noise(Noise { density: 0.6, blend: BlendMode::Multiply, ..Noise::default() })]);
    let svg = export(&d, &[n]);
    assert!(svg.contains(&format!("fill=\"{FILL}\"")), "the layer stays vector: {svg}");
    assert_eq!(svg.matches("<image").count(), 1, "{svg}");
    assert!(svg.contains("style=\"image-rendering:pixelated;mix-blend-mode:multiply\""), "{svg}");
    assert!(svg.contains("isolation:isolate"), "grains blend with the layer only: {svg}");
    // Two pixels a grain: the image is twice the layer's size.
    assert!(svg.contains("width=\"200\" height=\"160\""), "{svg}");
    // The grains are the layer's own (stable across exports).
    assert_eq!(svg, export(&d, &[n]));
}

#[test]
fn texture_and_progressive_layer_blur_embed_the_layer_whole() {
    for kind in [EffectKind::Texture(Texture { radius: 3.0, ..Texture::default() }), EffectKind::LayerBlur(progressive())] {
        let (d, n) = layer(vec![kind.clone()]);
        let svg = export(&d, &[n]);
        assert!(!svg.contains("<path"), "{kind:?}: {svg}");
        assert!(!svg.contains("feGaussianBlur"), "{kind:?}: {svg}");
        assert_eq!(svg.matches("<image").count(), 1, "{kind:?}: {svg}");
        assert_eq!(svg, export(&d, &[n]), "deterministic");
    }
    // Noise over a uniform blur depends on their order: whole too. A uniform blur alone stays a filter.
    let (d, n) = layer(vec![EffectKind::LayerBlur(Blur::uniform(6.0)), EffectKind::Noise(Noise::default())]);
    assert!(!export(&d, &[n]).contains("<path"));
    let (d, n) = layer(vec![EffectKind::LayerBlur(Blur::uniform(6.0))]);
    let svg = export(&d, &[n]);
    assert!(svg.contains("<feGaussianBlur in=\"SourceGraphic\" stdDeviation=\"3\"") && !svg.contains("<image"), "{svg}");
}

#[test]
fn background_blur_is_drawn_from_what_lies_beneath() {
    for b in [Blur::uniform(10.0), progressive()] {
        let (mut d, under) = layer(vec![]);
        let p = d.first_page().unwrap();
        let pane = d.create(p, NodeKind::Rectangle, "Pane", 40.0, 30.0, 100.0, 60.0).unwrap();
        let n = d.get_mut(pane).unwrap();
        n.fills = vec![Paint::solid(Color::from_hex("FFFFFF40").unwrap())];
        n.effects = vec![Effect { kind: EffectKind::BackgroundBlur(b.clone()), visible: true }];
        let svg = export(&d, &[under, pane]);
        let img = svg.find("<image").unwrap_or_else(|| panic!("{b:?}: {svg}"));
        // Under the pane's own fill.
        let fill = svg.find("fill=\"#FFFFFF\"").unwrap();
        assert!(img < fill, "{svg}");
        // Exported alone, nothing lies beneath: no image.
        assert!(!export(&d, &[pane]).contains("<image"));
    }
}

#[test]
fn inner_shadows_go_over_fills_and_under_strokes() {
    let shadow = Shadow { color: Color::from_hex("FF0000").unwrap(), offset: (0.0, 0.0), radius: 8.0, spread: 4.0, ..Shadow::default() };
    let (mut d, n) = layer(vec![EffectKind::InnerShadow(shadow)]);
    let node = d.get_mut(n).unwrap();
    node.strokes = vec![Paint::solid(Color::from_hex(STROKE).unwrap())];
    node.stroke.weight = 10.0;
    node.stroke.align = StrokeAlign::Inside;
    let svg = export(&d, &[n]);
    let fill = svg.find(&format!("fill=\"{FILL}\"")).unwrap();
    let shadow = svg.find("<g clip-path=").unwrap();
    // A simplified stroke (an inside stroke on a box is a stroke attribute).
    let stroke = svg.find(&format!("stroke=\"{STROKE}\"")).unwrap();
    assert!(fill < shadow && shadow < stroke, "{svg}");
}

#[test]
fn hostile_raster_effects_still_export() {
    for v in [f64::NAN, f64::INFINITY, -1e12, 1e12] {
        let (d, n) = layer(vec![
            EffectKind::Noise(Noise { size: v, density: v, ..Noise::default() }),
            EffectKind::BackgroundBlur(Blur { radius: v, start_radius: v, start: (v, v), ..progressive() }),
        ]);
        assert!(export_svg(&d, &[n], &SvgOptions::default()).is_some());
        let (d, n) = layer(vec![
            EffectKind::Texture(Texture { radius: v, size: v, ..Texture::default() }),
            EffectKind::LayerBlur(Blur { radius: v, end: (v, 0.0), ..progressive() }),
        ]);
        assert!(export_svg(&d, &[n], &SvgOptions::default()).is_some());
    }
}
