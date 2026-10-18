//! Inner shadows and background blurs on groups. A group is no outline of its own, so these go
//! by what its layers draw together: an inner shadow is cast inwards from around their pixels
//! (over them), a background blur shows through their outlines (under them). Where the group's content is
//! one shape, the pixels must match that shape (or a frame filled like it) carrying the effect.
//! Scenes are small and synthetic, drawn on a white page at 1×.

use figcraft_color::{Color, Paint};
use figcraft_doc::{Blur, Document, Effect, EffectKind, NodeId, NodeKind, Shadow};

use super::paint_behaviour_tests::{hex, near, page, px, rect, render, solid_at};
use super::*;

const BLACK: [u8; 3] = [0, 0, 0];
const RED: [u8; 3] = [255, 0, 0];
const BLUE: [u8; 3] = [0, 0, 255];

fn fx(kind: EffectKind) -> Effect {
    Effect { kind, visible: true }
}

fn inner(offset: (f64, f64), radius: f64, spread: f64) -> Effect {
    fx(EffectKind::InnerShadow(Shadow { color: Color::BLACK, offset, radius, spread, ..Shadow::default() }))
}

fn group(d: &mut Document, parent: NodeId) -> NodeId {
    d.create(parent, NodeKind::Group, "G", 0.0, 0.0, 200.0, 200.0).unwrap()
}

/// How a 40 × 40 square at (50, 50) filled with `fills` carries `effects`.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Holder {
    /// A rectangle with the effects.
    Shape,
    /// A clipping frame of the square's box, filled like it, with the effects.
    Frame,
    /// The rectangle alone in a group that has the effects.
    Group,
    /// The same group without the effects.
    Plain,
}

/// The scene: `backdrop` (drawn first), then the square held as `holder` says.
fn scene(holder: Holder, fills: &[Paint], effects: &[Effect], backdrop: &dyn Fn(&mut Document, NodeId)) -> Pixmap {
    let (mut d, p) = page();
    backdrop(&mut d, p);
    let on = match holder {
        Holder::Shape => rect(&mut d, p, (50.0, 50.0, 40.0, 40.0), fills.to_vec()),
        Holder::Frame => {
            let f = d.create(p, NodeKind::Frame, "F", 50.0, 50.0, 40.0, 40.0).unwrap();
            let n = d.get_mut(f).unwrap();
            (n.fills, n.clip) = (fills.to_vec(), true);
            f
        }
        Holder::Group | Holder::Plain => {
            let g = group(&mut d, p);
            rect(&mut d, g, (50.0, 50.0, 40.0, 40.0), fills.to_vec());
            g
        }
    };
    if holder != Holder::Plain {
        d.get_mut(on).unwrap().effects = effects.to_vec();
    }
    render(&d)
}

/// The largest difference of any channel of any pixel.
fn worst(a: &Pixmap, b: &Pixmap) -> u8 {
    a.data().iter().zip(b.data()).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0)
}

fn nothing(_: &mut Document, _: NodeId) {}

/// A black half under the square's left part: an edge at x = 70 for blurs to smear.
fn half_black(d: &mut Document, p: NodeId) {
    rect(d, p, (0.0, 0.0, 70.0, 200.0), vec![Paint::solid(Color::BLACK)]);
}

#[test]
fn an_inner_shadow_on_a_group_of_one_shape_matches_the_shape() {
    let red = [Paint::solid(hex("FF0000"))];
    // (shadow, most a pixel may differ, why)
    for (effect, tol, why) in [
        (inner((6.0, 6.0), 0.0, 0.0), 0, "sharp, whole-pixel offset: the same pixels"),
        (inner((0.0, 4.0), 8.0, 0.0), 2, "blurred: the same blur of the same edge"),
        (inner((0.0, 0.0), 0.0, 5.0), 0, "spread: 5 px in from every edge"),
    ] {
        let g = scene(Holder::Group, &red, std::slice::from_ref(&effect), &nothing);
        for other in [Holder::Shape, Holder::Frame] {
            let diff = worst(&g, &scene(other, &red, std::slice::from_ref(&effect), &nothing));
            assert!(diff <= tol, "{why}: the group differs from {other:?} by {diff}");
        }
        assert!(worst(&g, &scene(Holder::Plain, &red, &[], &nothing)) > 100, "{why}: and it does draw a shadow");
    }
    // Where: offset (6, 6) shades the top and left 6 px inside the square, nothing outside.
    let g = scene(Holder::Group, &red, &[inner((6.0, 6.0), 0.0, 0.0)], &nothing);
    for ((x, y), want, why) in [
        ((52, 70), BLACK, "inside the left edge"),
        ((70, 52), BLACK, "inside the top edge"),
        ((70, 70), RED, "the middle"),
        ((87, 87), RED, "the far corner"),
        ((48, 70), [255, 255, 255], "outside: an inner shadow stays inside"),
    ] {
        assert!(near(px(&g, x, y), want, 2), "{why} at ({x}, {y}): {:?}", px(&g, x, y));
    }
}

#[test]
fn a_groups_inner_shadow_is_cast_by_its_layers_together() {
    // Red (50, 50) and blue (90, 50), 40 × 40, side by side; shadow offset (6, 0). On the group the
    // pair is one silhouette: only its left edge shades. On each layer, each left edge does.
    for (on_group, seam, why) in [(true, BLUE, "on the group: no shadow at the seam"), (false, BLACK, "on each layer: blue is shaded too")] {
        let (mut d, p) = page();
        let g = group(&mut d, p);
        let a = rect(&mut d, g, (50.0, 50.0, 40.0, 40.0), vec![Paint::solid(hex("FF0000"))]);
        let b = rect(&mut d, g, (90.0, 50.0, 40.0, 40.0), vec![Paint::solid(hex("0000FF"))]);
        for id in if on_group { vec![g] } else { vec![a, b] } {
            d.get_mut(id).unwrap().effects = vec![inner((6.0, 0.0), 0.0, 0.0)];
        }
        let pix = render(&d);
        assert!(near(px(&pix, 93, 70), seam, 2), "{why}: {:?}", px(&pix, 93, 70));
        assert!(near(px(&pix, 53, 70), BLACK, 2), "{why}: the pair's left edge is shaded");
    }
    // Translucent content shades in proportion: a 50% red square's shadow at 50%, over the page.
    let half = scene(Holder::Group, &[solid_at("FF0000", 0.5)], &[inner((6.0, 6.0), 0.0, 0.0)], &nothing);
    assert!(near(px(&half, 52, 70), [128, 64, 64], 3), "{:?}", px(&half, 52, 70));
}

#[test]
fn a_background_blur_on_a_group_of_one_shape_matches_the_shape() {
    let glassy = [solid_at("FFFFFF", 0.5)];
    let blur = [fx(EffectKind::BackgroundBlur(Blur::uniform(16.0)))];
    let g = scene(Holder::Group, &glassy, &blur, &half_black);
    for other in [Holder::Shape, Holder::Frame] {
        let diff = worst(&g, &scene(other, &glassy, &blur, &half_black));
        assert!(diff <= 2, "the group differs from {other:?} by {diff}");
    }
    let plain = scene(Holder::Plain, &glassy, &[], &half_black);
    // Without the blur the edge at x = 70 is sharp under the square; with it, smeared.
    assert!(near(px(&plain, 68, 70), [128, 128, 128], 2) && near(px(&plain, 72, 70), [255, 255, 255], 2));
    let (l, r) = (px(&g, 68, 70), px(&g, 72, 70));
    assert!(l[0] > 140 && r[0] < 245, "blurred under the square: {l:?} {r:?}");
    // Only what the group's layers cover: around the square the backdrop is as it was.
    for (x, y) in [(68, 45), (72, 45), (68, 95), (45, 70), (95, 70)] {
        assert_eq!(px(&g, x, y), px(&plain, x, y), "({x}, {y}) is outside the square");
    }
}

#[test]
fn hidden_empty_and_junk_effects_on_groups_draw_nothing_odd() {
    let red = [Paint::solid(hex("FF0000"))];
    let plain = scene(Holder::Plain, &red, &[], &half_black);
    let hidden =
        [Effect { visible: false, ..inner((6.0, 6.0), 0.0, 0.0) }, Effect { kind: EffectKind::BackgroundBlur(Blur::uniform(16.0)), visible: false }];
    assert_eq!(worst(&scene(Holder::Group, &red, &hidden, &half_black), &plain), 0, "hidden effects");
    assert_eq!(worst(&scene(Holder::Group, &red, &[fx(EffectKind::BackgroundBlur(Blur::uniform(0.0)))], &half_black), &plain), 0, "a blur of 0");
    // An opaque square hides a background blur whole.
    assert_eq!(worst(&scene(Holder::Group, &red, &[fx(EffectKind::BackgroundBlur(Blur::uniform(16.0)))], &half_black), &plain), 0);
    for v in [f64::NAN, f64::INFINITY] {
        for e in [inner((v, 0.0), 4.0, 0.0), inner((0.0, 0.0), v, 0.0), inner((2.0, 2.0), 2.0, v), fx(EffectKind::BackgroundBlur(Blur::uniform(v)))] {
            let pix = scene(Holder::Group, &red, &[e], &half_black);
            assert_eq!((pix.width(), pix.height()), (200, 200), "{v}");
        }
    }
    // An empty group with effects draws nothing.
    let (mut d, p) = page();
    let g = group(&mut d, p);
    d.get_mut(g).unwrap().effects = vec![inner((6.0, 6.0), 4.0, 0.0), fx(EffectKind::BackgroundBlur(Blur::uniform(8.0)))];
    assert!(render(&d).data().chunks_exact(4).all(|c| c[..3] == [255, 255, 255]));
}
