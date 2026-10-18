//! Effects on groups in the PDF export: a background blur shows through the outlines of the
//! group's layers and matches the PNG; an inner shadow is an image cast from the layers' pixels.

use figcraft_color::Color;
use figcraft_doc::{Blur, Effect, EffectKind, NodeKind, Shadow};

use crate::backdrop_tests::{pdf_vs_png, scene};
use crate::draw::Exporter;
use crate::{PdfOptions, export_nodes_pdf};

fn on(kind: EffectKind) -> Vec<Effect> {
    vec![Effect { kind, visible: true }]
}

#[test]
fn a_groups_background_blur_matches_the_png() {
    // The striped scene's pane, now alone in a group that carries the blur.
    let (mut d, f, pane) = scene(EffectKind::BackgroundBlur(Blur::uniform(12.0)));
    let g = d.create(f, NodeKind::Group, "G", 0.0, 0.0, 240.0, 160.0).unwrap();
    d.reparent(pane, g, None).unwrap();
    let effects = std::mem::take(&mut d.get_mut(pane).unwrap().effects);
    d.get_mut(g).unwrap().effects = effects;
    let diff = pdf_vs_png(&d, f, g);
    assert!(diff <= 3, "{diff}");
    assert!(export_nodes_pdf(&d, &[f]).unwrap().starts_with(b"%PDF"));
}

#[test]
fn a_groups_inner_shadow_is_an_image_of_its_layers_shade() {
    let (mut d, f, pane) = scene(EffectKind::Noise(Default::default()));
    let g = d.create(f, NodeKind::Group, "G", 0.0, 0.0, 240.0, 160.0).unwrap();
    d.reparent(pane, g, None).unwrap();
    let n = d.get_mut(pane).unwrap();
    n.effects.clear();
    n.fills = vec![figcraft_color::Paint::solid(Color::WHITE)];
    let shadow = Shadow { color: Color::BLACK, offset: (6.0, 6.0), radius: 0.0, ..Shadow::default() };
    d.get_mut(g).unwrap().effects = on(EffectKind::InnerShadow(shadow));
    let opts = PdfOptions::default();
    let mut ex = Exporter::new(&d, &opts);
    let b = figcraft_render::export_bounds(&d, f).unwrap();
    (ex.page_origin, ex.page_rect, ex.page_ids) = (figcraft_geom::Affine::translate((-b.x0, -b.y0)), b, vec![f]);
    let sh = ex.shadows(g).unwrap();
    assert_eq!((sh.inner.len(), sh.drop.len()), (1, 0));
    // With the shadow the file holds one more image than without.
    let with = export_nodes_pdf(&d, &[f]).unwrap();
    d.get_mut(g).unwrap().effects.clear();
    let without = export_nodes_pdf(&d, &[f]).unwrap();
    let images = |pdf: &[u8]| pdf.windows(6).filter(|w| *w == b"/Image").count();
    assert!(images(&with) > images(&without), "{} vs {}", images(&with), images(&without));
}
