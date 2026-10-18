//! Recognising primitives in path data, so imported shapes stay editable: rectangles (with a
//! uniform corner radius), ellipses and straight lines. `<rect>`, `<circle>`, `<ellipse>` and
//! `<line>` reach us as paths; so do hand-written paths that trace the same outlines, and they're
//! recognised the same way. The test is geometric: every sampled point of the outline lies on the
//! candidate's outline and the lengths agree.

use figcraft_geom::kurbo::{ParamCurve, PathEl};
use figcraft_geom::{Affine, BezPath, Point, Rect, Shape, Vec2};

/// A primitive found in a path, in the path's own coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Prim {
    Rect { rect: Rect, radius: f64 },
    Ellipse(Rect),
    Line(Point, Point),
}

/// Relative tolerance of the geometric tests.
const TOL: f64 = 2e-3;
/// Most path elements a primitive is looked for in (they take a handful; this keeps big paths
/// cheap).
const MAX_ELEMENTS: usize = 40;

/// The primitive `path` traces. An unclosed path ending where it starts counts as closed when
/// `unstroked` (with a stroke its ends would show as caps, not a corner).
pub(crate) fn recognize(path: &BezPath, unstroked: bool) -> Option<Prim> {
    let els = path.elements();
    if els.len() > MAX_ELEMENTS || els.iter().skip(1).any(|e| matches!(e, PathEl::MoveTo(_))) {
        return None;
    }
    if let [PathEl::MoveTo(a), PathEl::LineTo(b)] = els {
        return (a.distance(*b) > 1e-9).then_some(Prim::Line(*a, *b));
    }
    let bb = path.bounding_box();
    let (w, h) = (bb.width(), bb.height());
    if !(w > 1e-6 && h > 1e-6 && w.is_finite() && h.is_finite()) {
        return None;
    }
    let tol = TOL * w.max(h);
    let ends = (els.first().and_then(PathEl::end_point), els.iter().rev().find_map(PathEl::end_point));
    let closed = matches!(els.last(), Some(PathEl::ClosePath)) || (unstroked && matches!(ends, (Some(a), Some(b)) if a.distance(b) <= tol * 1e-3));
    if !closed {
        return None;
    }
    let samples = samples(path);
    let length = || path.perimeter(1e-4);
    // An ellipse first: a circle also passes as a fully rounded square.
    let (c, rx, ry) = (bb.center(), w / 2.0, h / 2.0);
    let on_ellipse = samples.iter().all(|p| {
        let (dx, dy) = ((p.x - c.x) / rx, (p.y - c.y) / ry);
        ((dx * dx + dy * dy).sqrt() - 1.0).abs() * rx.min(ry) <= tol
    });
    if on_ellipse && close(length(), ellipse_perimeter(rx, ry)) {
        return Some(Prim::Ellipse(bb));
    }
    // A rounded rectangle's area is w·h − (4 − π)·r²; where its straight edges start (the end
    // points nearest that) gives r exactly.
    let area = path.area().abs();
    let estimate = ((w * h - area) / (4.0 - std::f64::consts::PI)).max(0.0).sqrt();
    let mut radius = els
        .iter()
        .filter_map(PathEl::end_point)
        .flat_map(|p| [p.x - bb.x0, bb.x1 - p.x, p.y - bb.y0, bb.y1 - p.y])
        .filter(|r| (r - estimate).abs() <= 0.05 * estimate)
        .min_by(|a, b| (a - estimate).abs().total_cmp(&(b - estimate).abs()))
        .unwrap_or(estimate)
        .min(w.min(h) / 2.0);
    if radius < tol {
        radius = 0.0;
    }
    // Rounding noise off (7.999999999999998 → 8).
    radius = (radius * 1e6).round() / 1e6;
    let on_rect = samples.iter().all(|p| round_box_distance(*p, bb, radius).abs() <= tol);
    let perimeter = 2.0 * (w + h) - (8.0 - 2.0 * std::f64::consts::PI) * radius;
    (on_rect && close(length(), perimeter)).then_some(Prim::Rect { rect: bb, radius })
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 5e-3 * b.max(1e-9)
}

/// Points along every segment (ends and between).
fn samples(path: &BezPath) -> Vec<Point> {
    let mut out = Vec::new();
    for seg in path.segments() {
        for i in 0..8 {
            out.push(seg.eval(f64::from(i) / 8.0));
        }
    }
    out
}

/// Ramanujan's approximation of an ellipse's perimeter.
fn ellipse_perimeter(a: f64, b: f64) -> f64 {
    let h = ((a - b) / (a + b)).powi(2);
    std::f64::consts::PI * (a + b) * (1.0 + 3.0 * h / (10.0 + (4.0 - 3.0 * h).sqrt()))
}

/// Signed distance from `p` to the outline of `rect` with corner `radius`.
fn round_box_distance(p: Point, rect: Rect, radius: f64) -> f64 {
    let c = rect.center();
    let half = Vec2::new(rect.width() / 2.0 - radius, rect.height() / 2.0 - radius);
    let q = Vec2::new((p.x - c.x).abs() - half.x, (p.y - c.y).abs() - half.y);
    let outside = Vec2::new(q.x.max(0.0), q.y.max(0.0)).hypot();
    outside + q.x.max(q.y).min(0.0) - radius
}

/// A `w × h` box at `origin` in user space, placed by `world`: the layer's transform (rotation and
/// flips only) and its size. `None` when `world` skews the box (or squashes it to nothing).
pub(crate) fn place(world: Affine, origin: Point, w: f64, h: f64) -> Option<(Affine, f64, f64)> {
    let m = world * Affine::translate(origin.to_vec2());
    let [a, b, c, d, e, f] = m.as_coeffs();
    let (sx, sy) = ((a * a + b * b).sqrt(), (c * c + d * d).sqrt());
    if !(sx > 1e-9 && sy > 1e-9 && [a, b, c, d, e, f].iter().all(|v| v.is_finite())) || (a * c + b * d).abs() > 1e-6 * sx * sy {
        return None;
    }
    Some((Affine::new([a / sx, b / sx, c / sy, d / sy, e, f]), w * sx, h * sy))
}

/// How much `world` scales lengths (stroke widths, blur radii): the square root of its area scale.
pub(crate) fn length_scale(world: Affine) -> f64 {
    let s = world.determinant().abs().sqrt();
    if s.is_finite() { s } else { 1.0 }
}
