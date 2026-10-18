//! Outline mode's two options, View ▸ Outlines ▸ Include hidden layers and Include object bounds:
//! what the renderer draws with each on and off, that a click picks what is drawn (and nothing
//! that isn't), and the commands themselves (toggles of the view, checked in the menu).

use figcraft_doc::NodeId;
use figcraft_render::{Options, Outline, View, render_view, to_rgba8};
use serde_json::{Value, json};

use crate::Session;
use crate::params::id_of;
use crate::tools::{PointerEvent, PointerKind};

fn ex(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd}: {e}"))
}

fn id(v: &Value) -> NodeId {
    id_of(&v["id"]).unwrap()
}

fn click(s: &mut Session, x: f64, y: f64) {
    s.pointer(&PointerEvent::at(PointerKind::Down, x, y, 1.0)).unwrap();
    s.pointer(&PointerEvent::at(PointerKind::Up, x, y, 1.0)).unwrap();
}

/// A shown rectangle (0,0 100×100), a hidden one (200,0 100×100), an ellipse (0,200 100×100)
/// and a group of two small rectangles whose box is (200,200)–(300,300).
fn scene(s: &mut Session) -> (NodeId, NodeId, NodeId, NodeId) {
    let shown = id(&ex(s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 100, "height": 100})));
    let hidden = id(&ex(s, "shape.rectangle", json!({"x": 200, "y": 0, "width": 100, "height": 100})));
    ex(s, "node.set", json!({"id": hidden, "visible": false}));
    let ellipse = id(&ex(s, "shape.ellipse", json!({"x": 0, "y": 200, "width": 100, "height": 100})));
    let a = id(&ex(s, "shape.rectangle", json!({"x": 200, "y": 200, "width": 20, "height": 20})));
    let b = id(&ex(s, "shape.rectangle", json!({"x": 280, "y": 280, "width": 20, "height": 20})));
    let group = id(&ex(s, "object.group", json!({"ids": [a, b]})));
    ex(s, "select.none", json!({}));
    (shown, hidden, ellipse, group)
}

/// Whether the canvas, drawn as the view's flags say, has an outline pixel (unlike the page) at
/// page point (x, y).
fn drawn(s: &Session, x: f64, y: f64) -> bool {
    let ds = s.ds().unwrap();
    let v = &ds.view;
    let opts = Options {
        outline_mode: v.outlines,
        outline: Outline { hidden: v.outline_hidden, bounds: v.outline_bounds, light: false },
        ..Default::default()
    };
    let view = View { origin: (-20.0, -20.0), scale: 1.0, width: 360, height: 360 };
    let pix = render_view(&ds.doc, ds.page, &view, &opts).unwrap();
    let rgba = to_rgba8(&pix);
    let at = |px: i64, py: i64| {
        let i = ((py + 20) * 360 + (px + 20)) as usize * 4;
        [rgba[i], rgba[i + 1], rgba[i + 2]]
    };
    let page = at(-10, -10);
    // A 1 px line can fall on either side of the coordinate.
    (-1..=1).any(|d| at(x as i64 + d, y as i64) != page || at(x as i64, y as i64 + d) != page)
}

#[test]
fn the_options_are_view_toggles_checked_in_the_menu() {
    let mut s = Session::new();
    for (cmd, read) in [
        ("view.outlinesHiddenLayers", (|s: &Session| s.ds().unwrap().view.outline_hidden) as fn(&Session) -> bool),
        ("view.outlinesObjectBounds", |s: &Session| s.ds().unwrap().view.outline_bounds),
    ] {
        assert!(!read(&s), "{cmd} is off in a new file");
        assert_eq!(crate::cmd::view::checked(&s, cmd), Some(false));
        assert_eq!(ex(&mut s, cmd, json!({}))["on"], true, "{cmd} toggles");
        assert!(read(&s));
        assert_eq!(crate::cmd::view::checked(&s, cmd), Some(true), "{cmd} shows its check mark");
        assert_eq!(ex(&mut s, cmd, json!({"on": true}))["on"], true, "{cmd} {{on}} sets");
        assert_eq!(ex(&mut s, cmd, json!({"on": false}))["on"], false);
        // In the View ▸ Outlines menu, with a Spanish-translatable label of its own.
        let item = crate::catalog::MENU.iter().find(|m| m.id == cmd).unwrap_or_else(|| panic!("{cmd} has no menu item"));
        assert_eq!(item.menu, "View/Outlines");
        // A view setting: nothing to undo, the file isn't changed.
        assert!(s.ds().unwrap().history.undo.is_empty());
        assert!(!s.ds().unwrap().is_dirty());
    }
    assert_eq!(crate::cmd::view::checked(&s, "file.save"), None, "not a toggle");
}

#[test]
fn outline_mode_draws_hidden_layers_and_object_bounds_when_asked() {
    let mut s = Session::new();
    scene(&mut s);
    // (outlines, hidden layers, object bounds) → drawn at: the shown rectangle's edge, the hidden
    // rectangle's edge, the ellipse's box corner (off its outline), the group's box edge.
    let rows = [
        ((true, false, false), [true, false, false, false], "outlines alone: shapes that show"),
        ((true, true, false), [true, true, false, false], "hidden layers are outlined too"),
        ((true, false, true), [true, false, true, true], "object bounds: the boxes of the ellipse and the group"),
        ((true, true, true), [true, true, true, true], "both"),
        ((false, true, true), [true, false, false, false], "the options do nothing outside outline mode (the rectangle is drawn filled)"),
    ];
    for ((outlines, hidden, bounds), want, why) in rows {
        ex(&mut s, "view.outlines", json!({"on": outlines}));
        ex(&mut s, "view.outlinesHiddenLayers", json!({"on": hidden}));
        ex(&mut s, "view.outlinesObjectBounds", json!({"on": bounds}));
        let got = [drawn(&s, 50.0, 0.0), drawn(&s, 250.0, 0.0), drawn(&s, 1.0, 201.0), drawn(&s, 250.0, 200.0)];
        assert_eq!(got, want, "{why}");
    }
}

#[test]
fn a_click_picks_what_outline_mode_draws() {
    let mut s = Session::new();
    let (shown, hidden, ellipse, group) = scene(&mut s);
    // (hidden layers, object bounds, click, selected, why)
    type Row = (bool, bool, (f64, f64), Vec<NodeId>, &'static str);
    let rows: Vec<Row> = vec![
        (false, false, (50.0, 0.5), vec![shown], "a shape is clicked on its outline"),
        (false, false, (250.0, 0.5), vec![], "a hidden layer isn't drawn: nothing to click"),
        (true, false, (250.0, 0.5), vec![hidden], "a hidden layer that is outlined is clicked on its outline"),
        (true, false, (250.0, 50.0), vec![], "inside it there is nothing, as for any shape in outline mode"),
        (false, false, (0.5, 200.5), vec![], "the corner of an ellipse's box is off its outline"),
        (false, true, (0.5, 200.5), vec![ellipse], "with object bounds the box is drawn and clicked"),
        (false, false, (250.0, 200.5), vec![], "a group's box isn't drawn without object bounds"),
        (false, true, (250.0, 200.5), vec![group], "with object bounds a group is clicked on its box"),
        (false, true, (250.0, 250.0), vec![], "inside the box there is nothing"),
    ];
    ex(&mut s, "view.outlines", json!({"on": true}));
    for (hide, bounds, p, after, why) in rows {
        ex(&mut s, "view.outlinesHiddenLayers", json!({"on": hide}));
        ex(&mut s, "view.outlinesObjectBounds", json!({"on": bounds}));
        ex(&mut s, "select.none", json!({}));
        assert_eq!(drawn(&s, p.0, p.1), !after.is_empty(), "what is clicked is what is drawn: {why}");
        click(&mut s, p.0, p.1);
        assert_eq!(s.selection(), after, "{why}");
    }
    // Outside outline mode the options change nothing: hidden layers aren't clicked.
    ex(&mut s, "view.outlines", json!({"on": false}));
    ex(&mut s, "view.outlinesHiddenLayers", json!({"on": true}));
    ex(&mut s, "select.none", json!({}));
    click(&mut s, 250.0, 50.0);
    assert!(s.selection().is_empty());
}

#[test]
fn outlines_are_light_on_a_dark_page() {
    let mut s = Session::new();
    ex(&mut s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 100, "height": 100}));
    ex(&mut s, "view.outlines", json!({"on": true}));
    for (page, value, light) in [("#F5F5F5", 0xF5_i32, false), ("#1E1E1E", 0x1E, true)] {
        ex(&mut s, "page.setBackground", json!({"color": page}));
        let ds = s.ds().unwrap();
        let view = View { origin: (-4.0, 46.0), scale: 4.0, width: 32, height: 32 };
        let pix = render_view(&ds.doc, ds.page, &view, &Options { outline_mode: true, ..Default::default() }).unwrap();
        let rgba = to_rgba8(&pix);
        // The line (a pixel wide, antialiased) is lighter than a dark page and darker than a light one.
        let brightest = i32::from(rgba.chunks(4).map(|p| p[0]).max().unwrap());
        let darkest = i32::from(rgba.chunks(4).map(|p| p[0]).min().unwrap());
        assert_eq!((brightest > value + 0x30, darkest < value - 0x30), (light, !light), "outline on a {page} page");
    }
}
