//! Progressive blurs: the radius ramps from the start radius at the start point to the end radius
//! at the end point (layer and background blur), matching a uniform blur where the ramp is flat;
//! levels, degenerate lines, bands and hostile values.

use figcraft_color::{Color, Paint};
use figcraft_doc::{Blur, BlurType, Document, Effect, EffectKind, NodeId, NodeKind};

use super::*;

fn progressive(start_radius: f64, radius: f64, start: (f64, f64), end: (f64, f64)) -> Blur {
    Blur { radius, blur_type: BlurType::Progressive, start_radius, start, end }
}

/// A white 200 × 200 frame (not clipping) with black vertical stripes 4 wide every 8.
fn stripes(d: &mut Document, parent: NodeId) -> NodeId {
    let f = d.create(parent, NodeKind::Frame, "Stripes", 0.0, 0.0, 200.0, 200.0).unwrap();
    d.get_mut(f).unwrap().fills = vec![Paint::solid(Color::WHITE)];
    for i in 0..25 {
        let s = d.create(f, NodeKind::Rectangle, "S", f64::from(i) * 8.0, 0.0, 4.0, 200.0).unwrap();
        d.get_mut(s).unwrap().fills = vec![Paint::solid(Color::BLACK)];
    }
    f
}

/// The stripes with a layer blur `kind` on the frame.
fn blurred(kind: Option<Blur>) -> (Document, NodeId) {
    let mut d = Document::new("progressive");
    let p = d.first_page().unwrap();
    let f = stripes(&mut d, p);
    if let Some(b) = kind {
        d.get_mut(f).unwrap().effects = vec![Effect { kind: EffectKind::LayerBlur(b), visible: true }];
    }
    (d, f)
}

fn render(d: &Document, id: NodeId) -> Pixmap {
    let mut pix = Pixmap::new(200, 200).unwrap();
    render_nodes_into(d, &[id], &View { origin: (0.0, 0.0), scale: 1.0, width: 200, height: 200 }, &mut pix);
    pix
}

fn red(p: &Pixmap, x: u32, y: u32) -> u8 {
    p.data()[((y * p.width() + x) * 4) as usize]
}

/// Brightest minus darkest red across the middle of row `y` (columns 40..160).
fn contrast(p: &Pixmap, y: u32) -> u8 {
    let row: Vec<u8> = (40..160).map(|x| red(p, x, y)).collect();
    row.iter().max().unwrap() - row.iter().min().unwrap()
}

fn row(p: &Pixmap, y: u32) -> Vec<u8> {
    let w = p.width() as usize;
    p.data()[y as usize * w * 4..(y as usize + 1) * w * 4].to_vec()
}

#[test]
fn layer_blur_ramps_from_sharp_to_the_end_radius() {
    // Sharp above a quarter of the way down, radius 16 below three quarters.
    let (d, f) = blurred(Some(progressive(0.0, 16.0, (0.5, 0.25), (0.5, 0.75))));
    let pix = render(&d, f);
    let (sharp_d, sharp_f) = blurred(None);
    let sharp = render(&sharp_d, sharp_f);
    let (uni_d, uni_f) = blurred(Some(Blur::uniform(16.0)));
    let uniform = render(&uni_d, uni_f);
    // Before the start: untouched; past the end: exactly the uniform blur.
    for y in [10, 30, 45] {
        assert_eq!(row(&pix, y)[40 * 4..160 * 4], row(&sharp, y)[40 * 4..160 * 4], "row {y}");
    }
    for y in [155, 175, 190] {
        assert_eq!(row(&pix, y)[40 * 4..160 * 4], row(&uniform, y)[40 * 4..160 * 4], "row {y}");
    }
    // In between the stripes fade steadily.
    let c: Vec<u8> = [45, 70, 90, 110, 130, 155].iter().map(|y| contrast(&pix, *y)).collect();
    assert_eq!(c[0], 255, "{c:?}");
    assert!(c.windows(2).all(|w| w[1] <= w[0]), "{c:?}");
    assert!(c[2] < c[0] && c[4] < c[2] && c[5] < 40, "{c:?}");
}

#[test]
fn the_ramp_follows_the_line_and_runs_either_way() {
    // Left to right, from radius 20 down to 0: blurred on the left, sharp on the right.
    let (d, f) = blurred(Some(progressive(20.0, 0.0, (0.0, 0.5), (1.0, 0.5))));
    let pix = render(&d, f);
    let span = |x0: u32| {
        let v: Vec<u8> = (x0..x0 + 16).map(|x| red(&pix, x, 100)).collect();
        v.iter().max().unwrap() - v.iter().min().unwrap()
    };
    assert!(span(10) < 60, "{}", span(10));
    assert_eq!(span(176), 255);
}

#[test]
fn a_degenerate_line_blurs_with_the_end_radius() {
    let (d, f) = blurred(Some(progressive(0.0, 10.0, (0.5, 0.5), (0.5, 0.5))));
    let (u, uf) = blurred(Some(Blur::uniform(10.0)));
    assert_eq!(render(&d, f).data(), render(&u, uf).data());
    // A flat ramp is a uniform blur.
    let (d, f) = blurred(Some(progressive(10.0, 10.0, (0.0, 0.0), (1.0, 1.0))));
    assert_eq!(render(&d, f).data(), render(&u, uf).data());
}

#[test]
fn levels_span_the_ramp_and_stay_few() {
    for (lo, hi) in [(0.0, 0.5), (0.0, 8.0), (0.2, 3.0), (2.0, 500.0), (0.0, 500.0), (5.0, 5.5)] {
        let l = crate::blur::levels(lo, hi);
        assert!(l.len() >= 2 && l.len() <= 8, "{lo}..{hi}: {l:?}");
        assert_eq!((l[0], *l.last().unwrap()), (lo, hi), "{l:?}");
        assert!(l.windows(2).all(|w| w[1] > w[0]), "{l:?}");
    }
}

#[test]
fn background_blur_ramps_across_the_backdrop() {
    let mut d = Document::new("progressive backdrop");
    let p = d.first_page().unwrap();
    let _ = stripes(&mut d, p);
    let pane = d.create(p, NodeKind::Rectangle, "Pane", 20.0, 40.0, 160.0, 120.0).unwrap();
    let n = d.get_mut(pane).unwrap();
    n.fills = vec![];
    // Sharp at the top of the pane, radius 16 at its bottom.
    n.effects = vec![Effect { kind: EffectKind::BackgroundBlur(progressive(0.0, 16.0, (0.5, 0.0), (0.5, 1.0))), visible: true }];
    let pix = render_view(&d, p, &View { origin: (0.0, 0.0), scale: 1.0, width: 200, height: 200 }, &Options::default()).unwrap();
    assert_eq!(contrast(&pix, 42), 255, "the top of the pane is sharp");
    assert!(contrast(&pix, 100) < 200, "{}", contrast(&pix, 100));
    assert!(contrast(&pix, 157) < 40, "{}", contrast(&pix, 157));
    // Outside the pane the stripes stay sharp.
    assert_eq!(contrast(&pix, 20), 255);
    assert_eq!(contrast(&pix, 180), 255);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn bands_match_one_piece() {
    let mut d = Document::new("progressive bands");
    let p = d.first_page().unwrap();
    for (i, x) in [0.0, 220.0, 440.0].into_iter().enumerate() {
        let f = stripes(&mut d, p);
        d.get_mut(f).unwrap().transform = Affine::translate((x, 20.0));
        let b = progressive(0.0, 12.0, (0.0, 0.0), (1.0, 1.0));
        let kind = if i == 1 { EffectKind::BackgroundBlur(b) } else { EffectKind::LayerBlur(b) };
        d.get_mut(f).unwrap().effects = vec![Effect { kind, visible: true }];
    }
    let v = View { origin: (-10.0, 0.0), scale: 1.5, width: 1000, height: 400 };
    let opts = Options::default();
    let one = render_view_single(&d, p, &v, &opts).unwrap();
    let banded = render_view(&d, p, &v, &opts).unwrap();
    assert_eq!(one.data(), banded.data());
}

#[test]
fn hostile_progressive_values_render() {
    // Absurd radii make the largest layers there are, so layer and background blur take turns.
    let bad = [f64::NAN, f64::INFINITY, -1e12, 1e12, 0.0, 1e12];
    for (i, v) in bad.into_iter().enumerate() {
        let kind = if i % 2 == 0 { EffectKind::LayerBlur as fn(Blur) -> EffectKind } else { EffectKind::BackgroundBlur };
        let mut d = Document::new("hostile");
        let p = d.first_page().unwrap();
        let f = stripes(&mut d, p);
        d.get_mut(f).unwrap().effects = vec![Effect { kind: kind(progressive(v, -v, (v, 0.5), (0.5, v))), visible: true }];
        let pix = render_view(&d, p, &View { origin: (0.0, 0.0), scale: 1.0, width: 120, height: 120 }, &Options::default());
        assert!(pix.is_some(), "{v}");
        // Too large to export at this scale is refused, not drawn.
        assert!(render_node(&d, f, 1.0).is_none_or(|p| p.width() <= 16_384));
    }
}
