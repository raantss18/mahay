//! Masks in SVG export: vector masks as clip paths, alpha and luminance masks as the mask layer
//! drawn (gradients and opacity kept), alpha ones marked `mask-type:alpha`.

use figcraft_color::{Color, ColorStop, Gradient, GradientKind, Paint};
use figcraft_doc::{Document, MaskType, NodeId, NodeKind};

use super::*;

/// A group holding a mask of `kind` (a white → clear gradient over the left half) over a red square.
fn masked(kind: MaskType) -> (Document, NodeId) {
    let mut d = Document::new("masks");
    let p = d.first_page().unwrap();
    let g = d.create(p, NodeKind::Group, "G", 0.0, 0.0, 100.0, 100.0).unwrap();
    let m = d.create(g, NodeKind::Rectangle, "M", 0.0, 0.0, 50.0, 100.0).unwrap();
    let stops = vec![ColorStop { position: 0.0, color: Color::WHITE }, ColorStop { position: 1.0, color: Color::new(0.0, 0.0, 0.0, 0.0) }];
    let node = d.get_mut(m).unwrap();
    node.fills = vec![Paint::gradient(Gradient { kind: GradientKind::Linear, stops, from: (0.0, 0.5), to: (1.0, 0.5), width: (0.0, 1.0) })];
    node.is_mask = true;
    node.mask_type = kind;
    let r = d.create(g, NodeKind::Rectangle, "R", 0.0, 0.0, 100.0, 100.0).unwrap();
    d.get_mut(r).unwrap().fills = vec![Paint::solid(Color::from_hex("FF0000").unwrap())];
    (d, g)
}

fn mask_def(svg: &str) -> &str {
    let start = svg.find("<mask").unwrap();
    let end = svg[start..].find("</mask>").unwrap();
    &svg[start..start + end]
}

#[test]
fn each_mask_type_exports_as_it_masks() {
    let export = |kind| {
        let (d, g) = masked(kind);
        export_svg(&d, &[g], &SvgOptions::default()).unwrap()
    };
    let vector = export(MaskType::Vector);
    assert!(vector.contains("<clipPath") && vector.contains("clip-path=\"url(#clip") && !vector.contains("<mask"), "{vector}");
    let luminance = export(MaskType::Luminance);
    assert!(luminance.contains("<linearGradient") && mask_def(&luminance).contains("url(#"), "{luminance}");
    assert!(!mask_def(&luminance).contains("mask-type"), "{luminance}");
    let alpha = export(MaskType::Alpha);
    assert!(mask_def(&alpha).contains("style=\"mask-type:alpha\"") && mask_def(&alpha).contains("url(#"), "{alpha}");
    // The mask layer itself isn't drawn outside its mask.
    assert_eq!(alpha.matches("url(#paint").count(), 1, "{alpha}");
}

#[test]
fn masks_survive_export_and_import() {
    for kind in [MaskType::Vector, MaskType::Luminance, MaskType::Alpha] {
        let (d, g) = masked(kind);
        let svg = export_svg(&d, &[g], &SvgOptions::default()).unwrap();
        let mut d2 = Document::new("back");
        let p = d2.first_page().unwrap();
        let f = import_svg(&mut d2, svg.as_bytes(), p, 0.0, 0.0, "x").unwrap();
        let mask = d2.subtree(f).into_iter().filter_map(|i| d2.node(i)).find(|n| n.is_mask).unwrap();
        assert_eq!(mask.mask_type, kind);
    }
}
