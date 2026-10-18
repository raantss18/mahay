//! Paint order of a layer's effects: drop shadows under everything, then fills, inner shadows,
//! children and strokes, with layer blur over the composite. Strokes draw over inner shadows.

use figcraft_color::{Color, Paint};
use figcraft_doc::{Document, Effect, EffectKind, NodeId, NodeKind, Shadow, StrokeAlign};

use super::*;

const GREEN: [u8; 4] = [0, 255, 0, 255];
const BLUE: [u8; 4] = [0, 0, 255, 255];

fn hex(h: &str) -> Color {
    Color::from_hex(h).unwrap()
}

/// A solid red inner shadow that reaches well past a 12-unit stroke.
fn red_inner_shadow() -> Effect {
    Effect {
        kind: EffectKind::InnerShadow(Shadow { color: hex("FF0000"), offset: (0.0, 0.0), radius: 8.0, spread: 14.0, ..Shadow::default() }),
        visible: true,
    }
}

/// A white 100 × 100 `kind` at (50, 50) with a 12-unit green inside stroke.
fn stroked(kind: NodeKind, effects: Vec<Effect>) -> (Document, NodeId) {
    let mut d = Document::new("order");
    let p = d.first_page().unwrap();
    let r = d.create(p, kind, "S", 50.0, 50.0, 100.0, 100.0).unwrap();
    let n = d.get_mut(r).unwrap();
    n.fills = vec![Paint::solid(Color::WHITE)];
    n.strokes = vec![Paint::solid(hex("00FF00"))];
    n.stroke.weight = 12.0;
    n.stroke.align = StrokeAlign::Inside;
    n.effects = effects;
    (d, r)
}

fn render(d: &Document, id: NodeId) -> Pixmap {
    let mut pix = Pixmap::new(200, 200).unwrap();
    render_nodes_into(d, &[id], &View { origin: (0.0, 0.0), scale: 1.0, width: 200, height: 200 }, &mut pix);
    pix
}

fn px(p: &Pixmap, x: u32, y: u32) -> [u8; 4] {
    let i = ((y * p.width() + x) * 4) as usize;
    [p.data()[i], p.data()[i + 1], p.data()[i + 2], p.data()[i + 3]]
}

#[test]
fn strokes_draw_over_inner_shadows() {
    for kind in [NodeKind::Rectangle, NodeKind::Ellipse { start: 0.0, sweep: std::f64::consts::TAU, inner: 0.0 }] {
        let (d, r) = stroked(kind.clone(), vec![red_inner_shadow()]);
        let with = render(&d, r);
        let (d0, r0) = stroked(kind, vec![]);
        let without = render(&d0, r0);
        // Inside the stroke band (50..62 from the left edge, at mid-height): the stroke's own
        // green, untouched by the shadow's red.
        for x in 52..60 {
            assert_eq!(px(&with, x, 100), GREEN, "x {x}");
            assert_eq!(px(&with, x, 100), px(&without, x, 100), "x {x}");
        }
        // Just inside the stroke the shadow shows over the white fill.
        let [r_, g, b, _] = px(&with, 64, 100);
        assert!(r_ == 255 && g < 200 && b < 200, "{:?}", [r_, g, b]);
        assert_eq!(px(&without, 64, 100), [255, 255, 255, 255]);
    }
}

#[test]
fn a_frames_children_draw_over_its_inner_shadow() {
    let mut d = Document::new("frame order");
    let p = d.first_page().unwrap();
    let f = d.create(p, NodeKind::Frame, "F", 50.0, 50.0, 100.0, 100.0).unwrap();
    let n = d.get_mut(f).unwrap();
    n.fills = vec![Paint::solid(Color::WHITE)];
    n.clip = true;
    n.effects = vec![red_inner_shadow()];
    let c = d.create(f, NodeKind::Rectangle, "C", 0.0, 30.0, 40.0, 40.0).unwrap();
    d.get_mut(c).unwrap().fills = vec![Paint::solid(hex("0000FF"))];
    let pix = render(&d, f);
    // The child at the frame's left edge, where the shadow is strongest: pure blue.
    assert_eq!(px(&pix, 51, 100), BLUE);
    // The frame's fill beside it: shadowed.
    assert!(px(&pix, 51, 60)[0] > 200 && px(&pix, 51, 60)[1] < 120, "{:?}", px(&pix, 51, 60));
}

#[test]
fn layer_blur_still_blurs_strokes_and_inner_shadows() {
    let blur = Effect { kind: EffectKind::LayerBlur(figcraft_doc::Blur::uniform(10.0)), visible: true };
    let (d, r) = stroked(NodeKind::Rectangle, vec![red_inner_shadow(), blur]);
    let pix = render(&d, r);
    // The blur reaches past the box and mixes the stroke into its neighbours.
    assert!(px(&pix, 46, 100)[3] > 0, "{:?}", px(&pix, 46, 100));
    assert_ne!(px(&pix, 56, 100), GREEN);
}

#[test]
fn canvas_and_export_agree_on_the_order() {
    let (d, r) = stroked(NodeKind::Rectangle, vec![red_inner_shadow()]);
    let page = d.first_page().unwrap();
    let canvas = render_view(&d, page, &View { origin: (50.0, 50.0), scale: 2.0, width: 200, height: 200 }, &Options::default()).unwrap();
    let export = render_node(&d, r, 2.0).unwrap();
    for (x, y) in [(4, 100), (20, 100), (100, 4), (100, 100), (30, 30)] {
        assert_eq!(px(&canvas, x, y), px(&export, x, y), "({x}, {y})");
    }
}
