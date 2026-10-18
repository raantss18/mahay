//! Shadows in SVG: spread offsets the outline like the renderer (mitred stroke bands, not a
//! morphology filter), negative spread and inner shadows cut with masks; layers without one outline
//! keep the morphology filter. Every export stays valid SVG.

use figcraft_color::{Color, Paint};
use figcraft_doc::{Document, Effect, EffectKind, NodeId, NodeKind, Shadow};

use super::*;

fn with_effects(kind: NodeKind, effects: Vec<EffectKind>) -> (Document, NodeId) {
    let mut d = Document::new("svg fx");
    let p = d.first_page().unwrap();
    let n = d.create(p, kind, "S", 10.0, 10.0, 100.0, 80.0).unwrap();
    let node = d.get_mut(n).unwrap();
    node.fills = vec![Paint::solid(Color::from_hex("3366CC").unwrap())];
    node.effects = effects.into_iter().map(|kind| Effect { kind, visible: true }).collect();
    (d, n)
}

fn shadow(spread: f64) -> Shadow {
    Shadow { color: Color::from_hex("00000080").unwrap(), offset: (3.0, 5.0), radius: 8.0, spread, ..Default::default() }
}

/// Export and check the result parses as SVG.
fn export(d: &Document, id: NodeId) -> String {
    let svg = export_svg(d, &[id], &SvgOptions::default()).unwrap();
    let mut d2 = Document::new("back");
    let p2 = d2.first_page().unwrap();
    import_svg(&mut d2, svg.as_bytes(), p2, 0.0, 0.0, "Imported").unwrap();
    svg
}

#[test]
fn drop_shadow_spread_offsets_the_outline() {
    let star = NodeKind::Star { count: 5, inner_ratio: 0.4 };
    let (d, n) = with_effects(star, vec![EffectKind::DropShadow(shadow(6.0))]);
    let svg = export(&d, n);
    assert!(!svg.contains("feMorphology"), "{svg}");
    assert!(svg.contains("stroke-width=\"12\"") && svg.contains("stroke-linejoin=\"miter\"") && svg.contains("stroke-miterlimit=\"4\""), "{svg}");
    assert!(svg.contains("<feGaussianBlur stdDeviation=\"4\" result=\"b\"/>"), "{svg}");
    // Hidden behind the layer: cut out by an image of its outline.
    assert!(svg.contains("<feComposite in=\"b\" in2=\"cover\" operator=\"out\"/>"), "{svg}");
    assert!(svg.contains("opacity=\"0.502\""), "{svg}");
}

#[test]
fn negative_spread_and_inner_shadows_cut_with_masks() {
    let ellipse = NodeKind::Ellipse { start: 0.0, sweep: std::f64::consts::TAU, inner: 0.0 };
    let (d, n) = with_effects(ellipse.clone(), vec![EffectKind::DropShadow(shadow(-4.0))]);
    let svg = export(&d, n);
    assert!(svg.contains("<mask") && svg.contains("stroke=\"black\" stroke-width=\"8\""), "{svg}");
    let (d, n) = with_effects(ellipse, vec![EffectKind::InnerShadow(shadow(4.0))]);
    let svg = export(&d, n);
    assert!(svg.contains("<clipPath") && svg.contains("<mask"), "{svg}");
    assert!(svg.contains("stroke=\"black\" stroke-width=\"8\""), "the shrunk hole: {svg}");
}

#[test]
fn spread_without_an_outline_keeps_the_filter() {
    let mut d = Document::new("g");
    let p = d.first_page().unwrap();
    let g = d.create(p, NodeKind::Group, "G", 0.0, 0.0, 50.0, 50.0).unwrap();
    let r = d.create(g, NodeKind::Rectangle, "R", 0.0, 0.0, 50.0, 50.0).unwrap();
    d.get_mut(r).unwrap().fills = vec![Paint::solid(Color::BLACK)];
    d.get_mut(g).unwrap().effects = vec![Effect { kind: EffectKind::DropShadow(shadow(5.0)), visible: true }];
    let svg = export(&d, g);
    assert!(svg.contains("feMorphology") && svg.contains("operator=\"dilate\" radius=\"5\""), "{svg}");
    // No spread: no morphology at all.
    d.get_mut(g).unwrap().effects = vec![Effect { kind: EffectKind::DropShadow(shadow(0.0)), visible: true }];
    assert!(!export(&d, g).contains("feMorphology"));
}

#[test]
fn hostile_shadow_values_still_export() {
    for s in [f64::NAN, f64::INFINITY, -1e12, 1e12] {
        let (d, n) = with_effects(
            NodeKind::Rectangle,
            vec![
                EffectKind::DropShadow(Shadow { spread: s, radius: s, offset: (s, 0.0), ..Shadow::default() }),
                EffectKind::InnerShadow(Shadow { spread: s, radius: s, offset: (0.0, s), ..Shadow::default() }),
            ],
        );
        assert!(export_svg(&d, &[n], &SvgOptions::default()).is_some());
    }
}
