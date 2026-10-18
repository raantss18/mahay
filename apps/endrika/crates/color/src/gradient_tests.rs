//! A linear gradient's width handle sits square to its axis on screen: the default one's bands run
//! across the shape (they were skewed), and moving an end keeps them so (the fill could vanish
//! when the stale handle ended up on the axis).

use crate::{Color, Gradient, GradientKind};

/// The axis and the width handle's direction in a `w`×`h` box, as screen vectors.
fn axes(g: &Gradient, w: f64, h: f64) -> ((f64, f64), (f64, f64)) {
    let d = |p: (f64, f64)| ((p.0 - g.from.0) * w, (p.1 - g.from.1) * h);
    (d(g.to), d(g.width))
}

fn square(g: &Gradient, w: f64, h: f64) -> bool {
    let (a, k) = axes(g, w, h);
    let dot = a.0 * k.0 + a.1 * k.1;
    let cross = a.0 * k.1 - a.1 * k.0;
    dot.abs() < 1e-9 && cross.abs() > 1e-9
}

#[test]
fn the_default_linear_gradient_runs_top_to_bottom_with_square_bands() {
    let g = Gradient::default_for(GradientKind::Linear, Color::BLACK);
    assert_eq!((g.from, g.to), ((0.5, 0.0), (0.5, 1.0)));
    for (w, h) in [(120.0, 80.0), (80.0, 120.0), (1.0, 1.0)] {
        assert!(square(&g, w, h), "{w}×{h}: {g:?}");
    }
}

#[test]
fn moving_an_end_keeps_the_bands_square_in_the_box() {
    let mut g = Gradient::default_for(GradientKind::Linear, Color::BLACK);
    // Dragging the start to the left edge's middle put the old handle on the axis.
    g.from = (0.0, 0.5);
    g.to = (1.0, 0.5);
    g.square_linear_width(120.0, 80.0);
    assert!(square(&g, 120.0, 80.0), "{g:?}");
    g.to = (1.0, 1.0);
    g.square_linear_width(120.0, 80.0);
    assert!(square(&g, 120.0, 80.0), "a diagonal on a wide box: {g:?}");
    // A degenerate box falls back to the unit box rather than dividing by zero.
    g.square_linear_width(0.0, f64::NAN);
    assert!(g.width.0.is_finite() && g.width.1.is_finite());
}
