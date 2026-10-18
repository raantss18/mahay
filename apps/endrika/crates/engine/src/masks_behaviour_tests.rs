//! Masks behaviour suite: each case is a rule from the public documentation of masks (Use as
//! mask, what a mask covers, alpha / vector / luminance masks, groups as masks, clicks on masked
//! out areas, the mask group's box and exports) with the result the rule implies. The comment on
//! each case is the one-line derivation. Where the documentation is silent the case says so and
//! pins today's behaviour.

use serde_json::{Value, json};

use crate::Session;
use crate::params::id_of;
use figcraft_doc::NodeId;
use figcraft_geom::Point;

fn id(v: Value) -> NodeId {
    id_of(&v["id"]).unwrap()
}

fn ex(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd}: {e}"))
}

fn rect(s: &mut Session, x: f64, y: f64, w: f64, h: f64, fill: &str, parent: Option<NodeId>) -> NodeId {
    let mut p = json!({"x": x, "y": y, "width": w, "height": h, "props": {"fills": [fill]}});
    if let Some(parent) = parent {
        p["parent"] = json!(parent);
    }
    id(ex(s, "shape.rectangle", p))
}

/// A white frame to draw a case in.
fn board(s: &mut Session, w: f64, h: f64) -> NodeId {
    id(ex(s, "frame.create", json!({"x": 0, "y": 0, "width": w, "height": h})))
}

/// The RGB of `node` rendered at 1x, at (`x`, `y`) of its export box.
fn rgb(s: &Session, node: NodeId, x: u32, y: u32) -> [u8; 3] {
    let pix = figcraft_render::render_node(s.doc().unwrap(), node, 1.0).unwrap();
    let i = ((y * pix.width() + x) * 4) as usize;
    let d = pix.data();
    [d[i], d[i + 1], d[i + 2]]
}

const WHITE: [u8; 3] = [255, 255, 255];
const RED: [u8; 3] = [255, 0, 0];
const GREEN: [u8; 3] = [0, 255, 0];
const BLUE: [u8; 3] = [0, 0, 255];

fn wb(s: &Session, n: NodeId) -> (f64, f64, f64, f64) {
    let b = s.doc().unwrap().world_bounds(n).unwrap();
    (b.x0, b.y0, b.width(), b.height())
}

#[test]
fn use_as_mask_groups_the_layers_over_the_bottom_one() {
    // Documented: with several layers selected, Use as mask makes the bottom one the mask and puts
    // them in a "Mask group" (where the topmost was); with one layer it turns it into a mask (or
    // back); one undo step.
    let mut s = Session::new();
    let a = rect(&mut s, 0.0, 0.0, 50.0, 50.0, "#000000", None);
    let b = rect(&mut s, 10.0, 10.0, 50.0, 50.0, "#FF0000", None);
    let c = rect(&mut s, 20.0, 20.0, 50.0, 50.0, "#00FF00", None);
    let top = rect(&mut s, 200.0, 0.0, 10.0, 10.0, "#0000FF", None);
    let g = id(ex(&mut s, "object.mask", json!({"ids": [c, a, b]})));
    let doc = s.doc().unwrap();
    let gn = doc.node(g).unwrap();
    assert_eq!((gn.name.as_str(), gn.children.clone()), ("Mask group", vec![a, b, c]));
    assert!(doc.node(a).unwrap().is_mask && !doc.node(b).unwrap().is_mask);
    let page = s.page().unwrap();
    assert_eq!(doc.node(page).unwrap().children, vec![g, top]);
    assert_eq!(s.selection(), vec![g]);
    s.undo().unwrap();
    assert!(s.doc().unwrap().node(g).is_none() && !s.doc().unwrap().node(a).unwrap().is_mask);
    // One layer: a toggle.
    ex(&mut s, "object.mask", json!({"ids": [a]}));
    assert!(s.doc().unwrap().node(a).unwrap().is_mask);
    ex(&mut s, "object.mask", json!({"ids": [a]}));
    assert!(!s.doc().unwrap().node(a).unwrap().is_mask);
    // Release mask: the layer is a plain layer again; silent on the group: pinned (it stays).
    let g = id(ex(&mut s, "object.mask", json!({"ids": [a, b]})));
    ex(&mut s, "object.releaseMask", json!({"ids": [a]}));
    assert!(!s.doc().unwrap().node(a).unwrap().is_mask);
    assert_eq!(s.doc().unwrap().node(a).unwrap().parent, Some(g));
}

#[test]
fn a_mask_covers_every_layer_above_it_in_its_parent() {
    // Documented: a mask shows the layers above it (in its group or frame) only where it is, and
    // isn't drawn itself; layers below it aren't affected. A 200×100 white board: a blue band
    // under the mask, the mask at x 50..100, red and green bands above it.
    let mut s = Session::new();
    let f = board(&mut s, 200.0, 100.0);
    rect(&mut s, 0.0, 0.0, 200.0, 20.0, "#0000FF", Some(f));
    let m = rect(&mut s, 50.0, 0.0, 50.0, 100.0, "#888888", Some(f));
    let red = rect(&mut s, 0.0, 40.0, 200.0, 20.0, "#FF0000", Some(f));
    rect(&mut s, 0.0, 70.0, 200.0, 20.0, "#00FF00", Some(f));
    ex(&mut s, "object.mask", json!({"ids": [m]}));
    let cases = [
        // Below the mask: drawn everywhere, and the mask's own fill isn't drawn over it.
        ((10, 10), BLUE),
        ((75, 10), BLUE),
        // Above the mask: inside it, drawn; outside it, hidden.
        ((75, 50), RED),
        ((10, 50), WHITE),
        ((75, 80), GREEN),
        ((150, 80), WHITE),
    ];
    for ((x, y), want) in cases {
        assert_eq!(rgb(&s, f, x, y), want, "({x}, {y})");
    }
    // A hidden mask masks nothing (silent on whether it's drawn: it isn't).
    ex(&mut s, "node.set", json!({"id": m, "props": {"visible": false}}));
    assert_eq!(rgb(&s, f, 10, 50), RED);
    ex(&mut s, "node.set", json!({"id": m, "props": {"visible": true}}));
    // Silent on a second mask higher up: pinned, it starts its own masked run (the green band is
    // masked by it alone).
    let m2 = rect(&mut s, 120.0, 0.0, 50.0, 100.0, "#888888", Some(f));
    ex(&mut s, "node.reorder", json!({"id": m2, "index": 3}));
    ex(&mut s, "object.mask", json!({"ids": [m2]}));
    assert_eq!(s.doc().unwrap().node(f).unwrap().children.get(3), Some(&m2));
    assert_eq!((rgb(&s, f, 75, 50), rgb(&s, f, 75, 80), rgb(&s, f, 150, 80), rgb(&s, f, 150, 50)), (RED, WHITE, GREEN, WHITE));
    let _ = red;
}

#[test]
fn alpha_vector_and_luminance_masks() {
    // Documented: an alpha mask uses the mask's opacity, a vector mask its outline (fully), a
    // luminance mask its brightness. A red layer over a white board, masked by a full-size layer.
    let cases: [(&str, &str, f64, std::ops::RangeInclusive<u8>); 6] = [
        // Alpha, half opacity: red at 50% over white (G and B ≈ 128).
        ("alpha", "#000000", 0.5, 126..=129),
        // Alpha, opaque (the colour doesn't matter): red.
        ("alpha", "#000000", 1.0, 0..=0),
        // Vector: the outline counts fully, whatever the opacity.
        ("vector", "#000000", 0.5, 0..=0),
        // Luminance: white shows the layer, black hides it, mid grey half shows it.
        ("luminance", "#FFFFFF", 1.0, 0..=0),
        ("luminance", "#000000", 1.0, 255..=255),
        ("luminance", "#808080", 1.0, 100..=160),
    ];
    for (kind, fill, opacity, gb) in cases {
        let mut s = Session::new();
        let f = board(&mut s, 100.0, 20.0);
        let m = rect(&mut s, 0.0, 0.0, 100.0, 20.0, fill, Some(f));
        rect(&mut s, 0.0, 0.0, 100.0, 20.0, "#FF0000", Some(f));
        ex(&mut s, "node.set", json!({"id": m, "props": {"opacity": opacity, "maskType": kind}}));
        ex(&mut s, "object.mask", json!({"ids": [m]}));
        let [r, g, b] = rgb(&s, f, 50, 10);
        assert!(r == 255 && gb.contains(&g) && gb.contains(&b), "{kind} {fill} {opacity}: {r} {g} {b}");
    }
}

#[test]
fn a_group_can_be_the_mask() {
    // Documented: any layer can be a mask, a group too: what it draws is the mask. Two squares at
    // 0..20 and 60..80 mask a red band.
    let mut s = Session::new();
    let f = board(&mut s, 100.0, 20.0);
    let a = rect(&mut s, 0.0, 0.0, 20.0, 20.0, "#000000", Some(f));
    let b = rect(&mut s, 60.0, 0.0, 20.0, 20.0, "#000000", Some(f));
    let g = id(ex(&mut s, "object.group", json!({"ids": [a, b]})));
    let red = rect(&mut s, 0.0, 0.0, 100.0, 20.0, "#FF0000", Some(f));
    ex(&mut s, "object.mask", json!({"ids": [g, red]}));
    assert!(s.doc().unwrap().node(g).unwrap().is_mask);
    assert_eq!([10, 40, 70, 90].map(|x| rgb(&s, f, x, 10)), [RED, WHITE, RED, WHITE]);
}

#[test]
fn masked_out_areas_dont_take_clicks() {
    // Documented: what a mask hides can't be clicked; a click there reaches what's behind. A
    // 50×50 mask over a 100×100 layer, a blue square behind the group at 60..100.
    let mut s = Session::new();
    let page = s.page().unwrap();
    let behind = rect(&mut s, 60.0, 60.0, 40.0, 40.0, "#0000FF", None);
    let m = rect(&mut s, 0.0, 0.0, 50.0, 50.0, "#000000", None);
    let c = rect(&mut s, 0.0, 0.0, 100.0, 100.0, "#FF0000", None);
    let hit = |s: &Session, x: f64, y: f64| s.doc().unwrap().hit_test(page, Point::new(x, y), 0.0, None);
    // Before there's a mask, the layer is hit everywhere (and the next edit makes the mask count).
    assert_eq!(hit(&s, 75.0, 75.0), Some(c));
    let g = id(ex(&mut s, "object.mask", json!({"ids": [m, c]})));
    let cases = [
        // Inside the mask: the masked layer.
        ((25.0, 25.0), Some(c)),
        // Outside the mask, over the blue square: the square.
        ((75.0, 75.0), Some(behind)),
        // Outside the mask, over nothing else: nothing.
        ((75.0, 10.0), None),
    ];
    for ((x, y), want) in cases {
        assert_eq!(hit(&s, x, y), want, "({x}, {y})");
    }
    // Select layer lists only what's there.
    let layers = ex(&mut s, "select.layersAt", json!({"x": 75, "y": 75}));
    let got: Vec<NodeId> = layers["layers"].as_array().unwrap().iter().map(|l| id_of(&l["id"]).unwrap()).collect();
    assert_eq!(got, vec![behind]);
    // A click selects the mask group (a group is picked whole).
    s.pointer(&crate::tools::PointerEvent::at(crate::tools::PointerKind::Down, 25.0, 25.0, 1.0)).unwrap();
    s.pointer(&crate::tools::PointerEvent::at(crate::tools::PointerKind::Up, 25.0, 25.0, 1.0)).unwrap();
    assert_eq!(s.selection(), vec![g]);
}

#[test]
fn a_mask_groups_box_and_exports_are_the_masks() {
    // Silent in the help centre on the box; pinned to what exports show: a mask group is as big as
    // its mask (plus any layers under the mask), and exports crop to it.
    let mut s = Session::new();
    let m = rect(&mut s, 0.0, 0.0, 50.0, 40.0, "#000000", None);
    let c = rect(&mut s, -30.0, -30.0, 200.0, 200.0, "#FF0000", None);
    let g = id(ex(&mut s, "object.mask", json!({"ids": [m, c]})));
    assert_eq!(wb(&s, g), (0.0, 0.0, 50.0, 40.0));
    let px = figcraft_render::render_node(s.doc().unwrap(), g, 1.0).unwrap();
    assert_eq!((px.width(), px.height()), (50, 40));
    assert_eq!(rgb(&s, g, 25, 20), RED);
    let svg = figcraft_svg::export_svg(s.doc().unwrap(), &[g], &figcraft_svg::SvgOptions::default()).unwrap();
    assert!(svg.contains("viewBox=\"0 0 50 40\""), "{svg}");
    assert!(figcraft_pdf::export_nodes_pdf(s.doc().unwrap(), &[g]).is_ok());
    // Moving the masked layer doesn't change the box; moving the mask does.
    ex(&mut s, "transform.move", json!({"ids": [c], "dx": 10, "dy": 0}));
    assert_eq!(wb(&s, g), (0.0, 0.0, 50.0, 40.0));
    ex(&mut s, "transform.move", json!({"ids": [m], "dx": 10, "dy": 0}));
    assert_eq!(wb(&s, g), (10.0, 0.0, 50.0, 40.0));
    // A layer under the mask counts.
    let under = rect(&mut s, -40.0, 0.0, 10.0, 10.0, "#0000FF", None);
    ex(&mut s, "node.reparent", json!({"ids": [under], "parent": g, "index": 0}));
    assert_eq!(wb(&s, g), (-40.0, 0.0, 100.0, 40.0));
    let _ = (GREEN, BLUE);
}
