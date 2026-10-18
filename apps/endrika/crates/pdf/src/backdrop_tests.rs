//! The PDF export draws background blur and glass from what lies under the layer on the page, the
//! way the PNG export does: the images it places match the PNG's pixels.

use figcraft_color::{Color, Paint};
use figcraft_doc::{Document, Effect, EffectKind, Glass, NodeId, NodeKind};
use figcraft_geom::{Affine, Rect};

use crate::PdfOptions;
use crate::draw::Exporter;

fn solid(d: &mut Document, id: NodeId, hex: &str) {
    d.get_mut(id).unwrap().fills = vec![Paint::solid(Color::from_hex(hex).unwrap())];
}

/// A red clipping frame with stripes and a transparent pane (80..200 × 40..120) with `effect`.
pub(crate) fn scene(effect: EffectKind) -> (Document, NodeId, NodeId) {
    let mut d = Document::new("pdf backdrop");
    let p = d.first_page().unwrap();
    let f = d.create(p, NodeKind::Frame, "Frame", 0.0, 0.0, 240.0, 160.0).unwrap();
    solid(&mut d, f, "E03030");
    d.get_mut(f).unwrap().clip = true;
    for (i, x) in [30.0, 90.0, 150.0].into_iter().enumerate() {
        let s = d.create(f, NodeKind::Rectangle, "Stripe", x, 0.0, 30.0, 160.0).unwrap();
        solid(&mut d, s, if i % 2 == 0 { "2040E0" } else { "20C060" });
    }
    let pane = d.create(f, NodeKind::Rectangle, "Pane", 80.0, 40.0, 120.0, 80.0).unwrap();
    let n = d.get_mut(pane).unwrap();
    n.fills = vec![];
    n.effects = vec![Effect { kind: effect, visible: true }];
    (d, f, pane)
}

/// The PDF's backdrop image of `pane` and the PNG export of `frame` at the same scale: the largest
/// channel difference over the pane's inside (2 page units in from its edges).
pub(crate) fn pdf_vs_png(d: &Document, frame: NodeId, pane: NodeId) -> u8 {
    let opts = PdfOptions::default();
    let mut ex = Exporter::new(d, &opts);
    let b = figcraft_render::export_bounds(d, frame).unwrap();
    ex.page_origin = Affine::translate((-b.x0, -b.y0));
    ex.page_rect = b;
    ex.page_ids = vec![frame];
    let (pix, view) = ex.backdrop_pixels(pane).unwrap();
    let png = figcraft_render::render_node(d, frame, view.scale).unwrap();
    let k = view.scale;
    let inside = Rect::new(82.0, 42.0, 198.0, 118.0);
    let mut worst = 0;
    for y in (inside.y0 * k) as u32..(inside.y1 * k) as u32 {
        for x in (inside.x0 * k) as u32..(inside.x1 * k) as u32 {
            // Pixel (x, y) of the PNG (origin at the frame's corner) in the backdrop image.
            let (bx, by) = (x - (view.origin.0 * k).round() as u32, y - (view.origin.1 * k).round() as u32);
            let i = ((y * png.width() + x) * 4) as usize;
            let j = ((by * pix.width() + bx) * 4) as usize;
            for c in 0..4 {
                worst = worst.max(png.data()[i + c].abs_diff(pix.data()[j + c]));
            }
        }
    }
    worst
}

#[test]
fn pdf_background_blur_matches_the_png() {
    let (d, f, pane) = scene(EffectKind::BackgroundBlur(figcraft_doc::Blur::uniform(12.0)));
    let diff = pdf_vs_png(&d, f, pane);
    assert!(diff <= 3, "{diff}");
}

#[test]
fn pdf_glass_matches_the_png() {
    let (d, f, pane) = scene(EffectKind::Glass(Glass::default()));
    let diff = pdf_vs_png(&d, f, pane);
    assert!(diff <= 3, "{diff}");
}

#[test]
fn pdf_places_the_backdrop_image() {
    let images = |d: &Document, f: NodeId| {
        let pdf = crate::export_pdf(d, &[f], &PdfOptions::default()).unwrap();
        String::from_utf8_lossy(&pdf).matches("/Subtype/Image").count()
    };
    let (mut d, f, pane) = scene(EffectKind::BackgroundBlur(figcraft_doc::Blur::uniform(12.0)));
    let with = images(&d, f);
    d.get_mut(pane).unwrap().effects.clear();
    let without = images(&d, f);
    assert!(with > without, "{with} / {without}");
    // Glass with a layer blur on top: the backdrop image goes under the rasterised layer (an image
    // of its own).
    d.get_mut(pane).unwrap().effects = vec![Effect::glass(), Effect::layer_blur()];
    let both = images(&d, f);
    d.get_mut(pane).unwrap().effects = vec![Effect::layer_blur()];
    assert!(both > images(&d, f), "{both}");
}
