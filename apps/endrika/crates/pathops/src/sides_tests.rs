//! One-sided variable widths in stroke outlines: each side offset by its own width, lopsided
//! caps around the middle of the end, dashes and closed paths.

use figcraft_doc::{StrokeAlign, StrokeCap, StrokeProps, WidthPoint, WidthProfile};
use figcraft_geom::kurbo::{BezPath, Shape};

use crate::stroke_outline;

/// A horizontal line from (0, 0) to (100, 0): looking along it, its right side is +y (y down).
fn line() -> BezPath {
    let mut p = BezPath::new();
    p.move_to((0.0, 0.0));
    p.line_to((100.0, 0.0));
    p
}

fn props(points: Vec<WidthPoint>) -> StrokeProps {
    StrokeProps { weight: 10.0, align: StrokeAlign::Center, width_profile: Some(WidthProfile { points }.sanitized()), ..Default::default() }
}

#[test]
fn each_side_is_offset_by_its_own_width() {
    // All on the right: 0 to the left, 10 to the right (side scale 2 × weight 10 / 2).
    let p = props(vec![WidthPoint::from_sides(0.0, 0.0, 2.0), WidthPoint::from_sides(1.0, 0.0, 2.0)]);
    let b = stroke_outline(&line(), &p, false).bounding_box();
    assert!(b.y0.abs() < 1e-6 && (b.y1 - 10.0).abs() < 1e-6 && b.x0.abs() < 1e-6 && (b.x1 - 100.0).abs() < 1e-6, "{b:?}");
    assert!((stroke_outline(&line(), &p, false).area().abs() - 1000.0).abs() < 1.0);
    // Lopsided: 2 on the left, 8 on the right.
    let p = props(vec![WidthPoint::from_sides(0.5, 0.4, 1.6)]);
    let b = stroke_outline(&line(), &p, false).bounding_box();
    assert!((b.y0 + 2.0).abs() < 1e-6 && (b.y1 - 8.0).abs() < 1e-6, "{b:?}");
    // The right side swells in the middle while the left stays straight.
    let p = props(vec![WidthPoint::new(0.0, 1.0), WidthPoint::from_sides(0.5, 1.0, 4.0), WidthPoint::new(1.0, 1.0)]);
    let b = stroke_outline(&line(), &p, false).bounding_box();
    assert!((b.y0 + 5.0).abs() < 1e-6 && (b.y1 - 20.0).abs() < 1e-6, "{b:?}");
}

#[test]
fn caps_wrap_the_middle_of_a_lopsided_end() {
    // 0 on the left, 10 on the right: a round cap is a half circle of radius 5 around (100, 5).
    let mut p = props(vec![WidthPoint::from_sides(0.0, 0.0, 2.0), WidthPoint::from_sides(1.0, 0.0, 2.0)]);
    p.cap_start = StrokeCap::Round;
    p.cap_end = StrokeCap::Round;
    let b = stroke_outline(&line(), &p, false).bounding_box();
    assert!((b.x1 - 105.0).abs() < 0.05 && (b.x0 + 5.0).abs() < 0.05, "{b:?}");
    assert!(b.y0 > -0.05 && (b.y1 - 10.0).abs() < 0.05, "{b:?}");
    p.cap_end = StrokeCap::Square;
    let b = stroke_outline(&line(), &p, false).bounding_box();
    assert!((b.x1 - 105.0).abs() < 1e-6 && b.y0 > -1e-6, "{b:?}");
}

#[test]
fn dashes_and_closed_paths_keep_their_sides() {
    let mut p = props(vec![WidthPoint::from_sides(0.0, 0.0, 2.0), WidthPoint::from_sides(1.0, 0.0, 2.0)]);
    p.dashes = vec![10.0, 10.0];
    let s = stroke_outline(&line(), &p, false);
    let b = s.bounding_box();
    assert!(b.y0.abs() < 1e-6 && (b.y1 - 10.0).abs() < 1e-6, "{b:?}");
    assert!((s.area().abs() - 500.0).abs() < 5.0, "{}", s.area());
    // A closed square drawn clockwise on screen: its right side is the inside.
    let mut sq = BezPath::new();
    sq.move_to((0.0, 0.0));
    sq.line_to((100.0, 0.0));
    sq.line_to((100.0, 100.0));
    sq.line_to((0.0, 100.0));
    sq.close_path();
    let p = props(vec![WidthPoint::from_sides(0.0, 0.0, 2.0), WidthPoint::from_sides(1.0, 0.0, 2.0)]);
    let b = stroke_outline(&sq, &p, true).bounding_box();
    assert!(b.x0 > -1e-6 && b.y0 > -1e-6 && b.x1 < 100.0 + 1e-6 && b.y1 < 100.0 + 1e-6, "all inside: {b:?}");
}
