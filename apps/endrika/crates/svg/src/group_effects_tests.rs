//! Effects on groups in the SVG export: an inner shadow (cast from the pixels of the group's
//! layers, over them) makes the group an embedded render; a background blur is an image of what
//! lies beneath, through the outlines of the group's layers, under them.

use figcraft_color::{Color, Paint};
use figcraft_doc::{Blur, Document, Effect, EffectKind, NodeId, NodeKind, Shadow};

use super::*;

const FILL: &str = "#3366CC";

/// A backdrop square and, over it, a group of one rectangle (filled when `filled`) with `effect`.
fn scene(effect: Option<EffectKind>, filled: bool) -> (Document, NodeId) {
    let mut d = Document::new("svg group fx");
    let p = d.first_page().unwrap();
    let f = d.create(p, NodeKind::Frame, "F", 0.0, 0.0, 200.0, 120.0).unwrap();
    let back = d.create(f, NodeKind::Rectangle, "Back", 0.0, 0.0, 100.0, 120.0).unwrap();
    d.get_mut(back).unwrap().fills = vec![Paint::solid(Color::BLACK)];
    let g = d.create(f, NodeKind::Group, "G", 0.0, 0.0, 200.0, 120.0).unwrap();
    let r = d.create(g, NodeKind::Rectangle, "R", 60.0, 30.0, 80.0, 60.0).unwrap();
    d.get_mut(r).unwrap().fills = if filled { vec![Paint::solid(Color::from_hex(FILL).unwrap())] } else { vec![] };
    d.get_mut(g).unwrap().effects = effect.into_iter().map(|kind| Effect { kind, visible: true }).collect();
    (d, f)
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
fn a_group_with_an_inner_shadow_is_an_embedded_render() {
    let plain = export(&scene(None, true).0, scene(None, true).1);
    assert!(plain.contains(&format!("fill=\"{FILL}\"")) && !plain.contains("<image"), "without effects the group is vectors: {plain}");
    let shadow = Shadow { color: Color::BLACK, offset: (6.0, 6.0), radius: 4.0, ..Shadow::default() };
    let (d, f) = scene(Some(EffectKind::InnerShadow(shadow)), true);
    let svg = export(&d, f);
    assert_eq!(svg.matches("<image").count(), 1, "{svg}");
    assert!(!svg.contains(&format!("fill=\"{FILL}\"")), "the group's layers are in the render: {svg}");
    assert_eq!(svg, export(&d, f), "deterministic");
}

#[test]
fn a_groups_background_blur_is_an_image_under_its_layers() {
    let (d, f) = scene(Some(EffectKind::BackgroundBlur(Blur::uniform(8.0))), true);
    let svg = export(&d, f);
    let (image, layer) = (svg.find("<image").expect("a backdrop image"), svg.find(&format!("fill=\"{FILL}\"")).expect("the layer stays vector"));
    assert!(image < layer, "under the layers: {svg}");
    // The image covers the layers' outlines: 80 × 60 at two pixels a unit.
    assert!(svg.contains("width=\"160\" height=\"120\""), "{svg}");
    // An empty group has nothing to show a blur through.
    let (mut d, f) = scene(Some(EffectKind::BackgroundBlur(Blur::uniform(8.0))), false);
    let g = d.nodes().find(|n| n.name == "G").unwrap().id;
    let kids = d.node(g).unwrap().children.clone();
    for k in kids {
        d.remove(k).unwrap();
    }
    assert!(!export(&d, f).contains("<image"));
}
