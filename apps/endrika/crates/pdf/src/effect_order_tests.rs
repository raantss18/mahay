//! Effects in the PDF: inner shadow images go over the fills and under the children and strokes
//! (the renderer's order); progressive blurs are drawn by the renderer like the PNG export.

use super::*;
use crate::backdrop_tests::{pdf_vs_png, scene};
use crate::tests::inflated;
use figcraft_color::{Color, Paint};
use figcraft_doc::{Blur, BlurType, Effect, EffectKind, NodeKind, Shadow, StrokeAlign};

const FILL: &str = "0.2 0.4 0.8 rg";
const CHILD: &str = "1 0 0 rg";
const STROKE: &str = "0 1 0 rg";

/// A blue 100 × 100 `kind` with a green inside stroke and an inner shadow (and a red child when a
/// frame); the page content stream.
fn content(kind: NodeKind) -> String {
    let mut d = Document::new("pdf order");
    let p = d.first_page().unwrap();
    let r = d.create(p, kind, "R", 0.0, 0.0, 100.0, 100.0).unwrap();
    let n = d.get_mut(r).unwrap();
    n.fills = vec![Paint::solid(Color::from_hex("3366CC").unwrap())];
    n.strokes = vec![Paint::solid(Color::from_hex("00FF00").unwrap())];
    n.stroke.weight = 10.0;
    n.stroke.align = StrokeAlign::Inside;
    n.effects = vec![Effect { kind: EffectKind::InnerShadow(Shadow::default()), visible: true }];
    if n.kind.is_frame_like() {
        let c = d.create(r, NodeKind::Rectangle, "C", 20.0, 20.0, 40.0, 40.0).unwrap();
        d.get_mut(c).unwrap().fills = vec![Paint::solid(Color::from_hex("FF0000").unwrap())];
    }
    inflated(&export_pdf(&d, &[r], &PdfOptions::default()).unwrap())
}

#[test]
fn inner_shadows_go_over_fills_and_under_strokes() {
    let c = content(NodeKind::Rectangle);
    let (fill, shadow, stroke) = (c.find(FILL).unwrap(), c.find(" Do").unwrap(), c.find(STROKE).unwrap());
    assert!(fill < shadow && shadow < stroke, "{c}");
}

#[test]
fn a_frames_children_go_over_its_inner_shadow() {
    let c = content(NodeKind::Frame);
    let (fill, shadow, child, stroke) = (c.find(FILL).unwrap(), c.find(" Do").unwrap(), c.find(CHILD).unwrap(), c.find(STROKE).unwrap());
    assert!(fill < shadow && shadow < child && child < stroke, "{c}");
}

#[test]
fn pdf_progressive_background_blur_matches_the_png() {
    let b = Blur { radius: 16.0, blur_type: BlurType::Progressive, start_radius: 2.0, start: (0.0, 0.5), end: (1.0, 0.5) };
    let (d, f, pane) = scene(EffectKind::BackgroundBlur(b));
    let diff = pdf_vs_png(&d, f, pane);
    assert!(diff <= 3, "{diff}");
}

#[test]
fn progressive_layer_blur_is_rasterised() {
    let images = |d: &Document, id| count_images(&export_pdf(d, &[id], &PdfOptions::default()).unwrap());
    let (mut d, f, pane) = scene(EffectKind::LayerBlur(Blur::uniform(0.0)));
    d.get_mut(pane).unwrap().fills = vec![Paint::solid(Color::WHITE)];
    let plain = images(&d, f);
    let b = Blur { radius: 12.0, blur_type: BlurType::Progressive, ..Blur::default() };
    d.get_mut(pane).unwrap().effects = vec![Effect { kind: EffectKind::LayerBlur(b), visible: true }];
    assert!(images(&d, f) > plain);
}

fn count_images(pdf: &[u8]) -> usize {
    String::from_utf8_lossy(pdf).matches("/Subtype/Image").count()
}
