//! Boolean groups whose operands carry their own operations: the fold bottom to top, its
//! agreement with a uniform operation, and single steps.

use figcraft_doc::BoolOp;
use figcraft_geom::Rect;
use figcraft_geom::kurbo::{BezPath, Shape};

use crate::{boolean, boolean_ops, boolean_step};

fn sq(x: f64, y: f64, s: f64) -> BezPath {
    Rect::new(x, y, x + s, y + s).to_path(0.1)
}

fn area(p: &BezPath) -> f64 {
    p.area().abs()
}

#[test]
fn each_operand_combines_with_the_result_below_it() {
    let (a, b, c) = (sq(0.0, 0.0, 10.0), sq(5.0, 0.0, 10.0), sq(0.0, 0.0, 15.0));
    // (A ∪ B) ∩ C: the union spans x 0..15, y 0..10 (150), all inside C.
    let r = boolean_ops([(&a, BoolOp::Union), (&b, BoolOp::Union), (&c, BoolOp::Intersect)]).unwrap();
    assert!((area(&r) - 150.0).abs() < 0.5, "{}", area(&r));
    // (A ∪ B) − a 5 × 5 square in the middle.
    let hole = sq(5.0, 2.5, 5.0);
    let r = boolean_ops([(&a, BoolOp::Union), (&b, BoolOp::Union), (&hole, BoolOp::Subtract)]).unwrap();
    assert!((area_eo(&r) - 125.0).abs() < 0.5, "{}", area_eo(&r));
    // The bottom operand's own operation is unused.
    let r2 = boolean_ops([(&a, BoolOp::Exclude), (&b, BoolOp::Union), (&hole, BoolOp::Subtract)]).unwrap();
    assert!((area_eo(&r2) - 125.0).abs() < 0.5);
}

/// Even-odd area: contours nested in another count negative (holes).
fn area_eo(p: &BezPath) -> f64 {
    let mut total = 0.0;
    let subs: Vec<BezPath> = split(p);
    for (i, s) in subs.iter().enumerate() {
        let inside = subs.iter().enumerate().filter(|(j, o)| *j != i && o.bounding_box().contains_rect(s.bounding_box())).count();
        total += if inside % 2 == 0 { area(s) } else { -area(s) };
    }
    total
}

fn split(p: &BezPath) -> Vec<BezPath> {
    use figcraft_geom::kurbo::PathEl;
    let mut out: Vec<BezPath> = Vec::new();
    for el in p.elements() {
        if matches!(el, PathEl::MoveTo(_)) {
            out.push(BezPath::new());
        }
        if let Some(last) = out.last_mut() {
            last.push(*el);
        }
    }
    out
}

#[test]
fn a_uniform_fold_matches_the_group_operation() {
    let (a, b, c) = (sq(0.0, 0.0, 10.0), sq(5.0, 5.0, 10.0), sq(8.0, 0.0, 4.0));
    for op in [BoolOp::Union, BoolOp::Subtract, BoolOp::Intersect, BoolOp::Exclude] {
        let all = boolean(&[a.clone(), b.clone(), c.clone()], op).unwrap();
        let folded = boolean_ops([(&a, op), (&b, op), (&c, op)]).unwrap();
        assert!((area_eo(&all) - area_eo(&folded)).abs() < 1e-6, "{op:?}");
    }
    // Subtract removes every later operand from the first.
    let s = boolean(&[a.clone(), b.clone(), c.clone()], BoolOp::Subtract).unwrap();
    assert!((area_eo(&s) - (100.0 - 25.0 - 8.0)).abs() < 0.5, "{}", area_eo(&s));
}

#[test]
fn single_steps_and_edge_cases() {
    let (a, b) = (sq(0.0, 0.0, 10.0), sq(5.0, 5.0, 10.0));
    assert!((area(&boolean_step(&a, &b, BoolOp::Intersect).unwrap()) - 25.0).abs() < 0.5);
    assert!((area(&boolean_step(&a, &b, BoolOp::Subtract).unwrap()) - 75.0).abs() < 0.5);
    // Nothing, and a lone operand (simplified).
    assert!(boolean_ops(std::iter::empty()).unwrap().elements().is_empty());
    assert!((area(&boolean_ops([(&a, BoolOp::Subtract)]).unwrap()) - 100.0).abs() < 1e-6);
    let mut bad = BezPath::new();
    bad.move_to((f64::NAN, 0.0));
    bad.line_to((1.0, 1.0));
    assert!(boolean_ops([(&a, BoolOp::Union), (&bad, BoolOp::Intersect)]).is_err());
}
