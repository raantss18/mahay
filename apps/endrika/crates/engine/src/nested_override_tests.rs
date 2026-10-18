//! Overrides on and inside nested instances across swaps, variant switches, main updates, detach
//! and copy/paste.

use crate::Session;
use crate::components::tests::{button, copy_of, ex, fill, healthy, idv, kids, node};
use crate::components::*;
use figcraft_doc::*;
use serde_json::json;

/// A 24×24 icon component at (`x`, 300) with one 20×20 rectangle named `layer`.
fn icon(s: &mut Session, x: f64, name: &str, layer: &str) -> NodeId {
    let f = idv(&ex(s, "frame.create", json!({"x": x, "y": 300, "width": 24, "height": 24, "name": name})));
    ex(s, "shape.rectangle", json!({"x": 2, "y": 2, "width": 20, "height": 20, "parent": f, "props": {"name": layer, "fill": "#111111"}}));
    idv(&ex(s, "component.create", json!({"ids": [f]})))
}

fn main_of(s: &Session, id: NodeId) -> NodeId {
    match node(s, id).kind {
        NodeKind::Instance { component, .. } => component,
        _ => panic!("{id} is not an instance"),
    }
}

#[test]
fn swapping_a_nested_instance_keeps_overrides_on_matching_layers() {
    let mut s = Session::new();
    let (_icon, shape, button, nested) = button(&mut s);
    let other = icon(&mut s, 200.0, "Other", "Shape");
    let unrelated = icon(&mut s, 300.0, "Unrelated", "Glyph");
    let i = idv(&ex(&mut s, "instance.create", json!({"component": button, "x": 300, "y": 400})));
    let inested = copy_of(&s, i, nested);
    let ishape = kids(&s, inested)[0];
    ex(&mut s, "node.set", json!({"id": ishape, "props": {"fill": "#ff00ff", "opacity": 0.5}}));
    // A main with a same-named layer: the overrides carry over.
    ex(&mut s, "instance.swap", json!({"ids": [inested], "component": other}));
    healthy(&s);
    let ishape = kids(&s, inested)[0];
    assert_eq!(main_of(&s, inested), other);
    assert_eq!(fill(&s, ishape), "FF00FF");
    assert_eq!(node(&s, ishape).opacity, 0.5);
    assert!(node(&s, ishape).overrides.contains("fills"));
    // ... and survive edits of the outer main and of the swapped-in main.
    ex(&mut s, "node.set", json!({"id": button, "props": {"fill": "#00ff00"}}));
    let other_shape = kids(&s, other)[0];
    ex(&mut s, "node.set", json!({"id": other_shape, "props": {"cornerRadius": 5, "fill": "#222222"}}));
    healthy(&s);
    let ishape = kids(&s, copy_of(&s, i, nested))[0];
    assert_eq!(fill(&s, ishape), "FF00FF");
    assert_eq!(node(&s, ishape).corners.radii, [5.0; 4]);
    // No matching layer: the overrides go.
    ex(&mut s, "instance.swap", json!({"ids": [inested], "component": unrelated}));
    healthy(&s);
    assert_eq!(fill(&s, kids(&s, inested)[0]), "111111");
    // Back to the original: it shows its main again.
    ex(&mut s, "component.resetOverride", json!({"id": inested, "group": "component"}));
    healthy(&s);
    assert_eq!(fill(&s, kids(&s, inested)[0]), fill(&s, copy_of(&s, nested, shape)));
}

#[test]
fn variant_switch_keeps_nested_overrides_and_swaps() {
    let mut s = Session::new();
    let ic = icon(&mut s, 0.0, "Icon", "Shape");
    let star = icon(&mut s, 100.0, "Star", "Shape");
    let mut variants = Vec::new();
    for (k, state) in ["Default", "Hover"].iter().enumerate() {
        let f =
            idv(&ex(&mut s, "frame.create", json!({"x": k as f64 * 200.0, "y": 0, "width": 120, "height": 40, "name": format!("State={state}")})));
        ex(
            &mut s,
            "shape.rectangle",
            json!({"x": 0, "y": 0, "width": 120, "height": 40, "parent": f, "props": {"name": "Bg", "fill": if k == 0 { "#ffffff" } else { "#eeeeee" }}}),
        );
        ex(&mut s, "instance.create", json!({"component": ic, "x": 8, "y": 8, "parent": f}));
        variants.push(idv(&ex(&mut s, "component.create", json!({"ids": [f]}))));
    }
    let set = idv(&ex(&mut s, "component.combineVariants", json!({"ids": variants.clone()})));
    let i = idv(&ex(&mut s, "instance.create", json!({"component": set, "x": 0, "y": 200})));
    let inested = kids(&s, i)[1];
    let ishape = kids(&s, inested)[0];
    ex(&mut s, "node.set", json!({"id": ishape, "props": {"fill": "#ff8800"}}));
    ex(&mut s, "node.set", json!({"id": inested, "props": {"opacity": 0.4}}));
    ex(&mut s, "instance.setProperties", json!({"ids": [i], "properties": {"State": "Hover"}}));
    healthy(&s);
    let inested = kids(&s, i)[1];
    assert_eq!(fill(&s, kids(&s, i)[0]), "EEEEEE", "the new variant shows");
    assert_eq!(fill(&s, kids(&s, inested)[0]), "FF8800", "override inside the nested instance kept");
    assert_eq!(node(&s, inested).opacity, 0.4, "override on the nested instance kept");
    // A swap override on the nested instance survives switching back.
    ex(&mut s, "instance.swap", json!({"ids": [inested], "component": star}));
    ex(&mut s, "instance.setProperties", json!({"ids": [i], "properties": {"State": "Default"}}));
    healthy(&s);
    let inested = kids(&s, i)[1];
    assert_eq!(main_of(&s, inested), star);
    assert_eq!(fill(&s, kids(&s, inested)[0]), "FF8800");
}

#[test]
fn nested_overrides_survive_main_restructuring() {
    let mut s = Session::new();
    let (_icon, shape, button, nested) = button(&mut s);
    let other = icon(&mut s, 200.0, "Other", "Shape");
    let i = idv(&ex(&mut s, "instance.create", json!({"component": button, "x": 300, "y": 400})));
    let ishape = copy_of(&s, i, copy_of(&s, nested, shape));
    ex(&mut s, "node.set", json!({"id": ishape, "props": {"fill": "#ff00ff"}}));
    // The main wraps its nested instance in a frame, then reorders it.
    let wrap = idv(&ex(&mut s, "object.frameSelection", json!({"ids": [nested]})));
    healthy(&s);
    let ishape = copy_of(&s, i, copy_of(&s, nested, shape));
    assert_eq!(fill(&s, ishape), "FF00FF", "kept when the main moves the nested instance into a frame");
    ex(&mut s, "node.reorder", json!({"id": wrap, "index": 5}));
    healthy(&s);
    assert_eq!(fill(&s, copy_of(&s, i, copy_of(&s, nested, shape))), "FF00FF");
    // The main swaps its nested instance to a main with the same layer.
    ex(&mut s, "instance.swap", json!({"ids": [nested], "component": other}));
    healthy(&s);
    let inested = copy_of(&s, i, nested);
    assert_eq!(main_of(&s, inested), other);
    assert_eq!(fill(&s, kids(&s, inested)[0]), "FF00FF");
}

#[test]
fn detach_keeps_swapped_nested_instances_and_their_overrides() {
    let mut s = Session::new();
    let (_icon, _shape, button, nested) = button(&mut s);
    let other = icon(&mut s, 200.0, "Other", "Glyph");
    let i = idv(&ex(&mut s, "instance.create", json!({"component": button, "x": 300, "y": 400})));
    let inested = copy_of(&s, i, nested);
    ex(&mut s, "instance.swap", json!({"ids": [inested], "component": other}));
    let iglyph = kids(&s, inested)[0];
    ex(&mut s, "node.set", json!({"id": iglyph, "props": {"fill": "#00aaff"}}));
    ex(&mut s, "component.detach", json!({"ids": [i]}));
    healthy(&s);
    assert_eq!(main_of(&s, inested), other);
    assert_eq!(node(&s, inested).source, Some(other));
    assert!(!node(&s, inested).overrides.contains("component"));
    let glyph = kids(&s, other)[0];
    assert_eq!(node(&s, iglyph).source, Some(glyph));
    assert_eq!(fill(&s, iglyph), "00AAFF");
    ex(&mut s, "node.set", json!({"id": glyph, "props": {"fill": "#000000", "cornerRadius": 3}}));
    assert_eq!(fill(&s, iglyph), "00AAFF");
    assert_eq!(node(&s, iglyph).corners.radii, [3.0; 4]);
    healthy(&s);
}

#[test]
fn copied_instances_keep_their_overrides() {
    let mut s = Session::new();
    let (icon, shape, button, nested) = button(&mut s);
    ex(&mut s, "component.addProperty", json!({"id": icon, "name": "Filled", "type": "boolean", "default": true, "layer": shape}));
    ex(&mut s, "component.exposeInstance", json!({"id": nested}));
    let i = idv(&ex(&mut s, "instance.create", json!({"component": button, "x": 300, "y": 400})));
    let inested = copy_of(&s, i, nested);
    let ishape = kids(&s, inested)[0];
    let ilabel = kids(&s, i)[1];
    ex(&mut s, "node.set", json!({"id": ishape, "props": {"fill": "#ff00ff"}}));
    ex(&mut s, "text.set", json!({"id": ilabel, "characters": "Copied"}));
    ex(&mut s, "instance.setProperties", json!({"ids": [inested], "properties": {"Filled": false}}));
    // The whole instance: overrides and property values come along, and survive main edits.
    ex(&mut s, "edit.copy", json!({"ids": [i]}));
    s.set_selection(vec![]);
    let j = ex(&mut s, "edit.paste", json!({"x": 300, "y": 800}))["ids"][0].as_u64().map(NodeId).unwrap();
    healthy(&s);
    ex(&mut s, "node.set", json!({"id": shape, "props": {"cornerRadius": 4}}));
    ex(&mut s, "node.set", json!({"id": button, "props": {"fill": "#eeeeee"}}));
    healthy(&s);
    let jnested = copy_of(&s, j, nested);
    let jshape = kids(&s, jnested)[0];
    assert_eq!(fill(&s, jshape), "FF00FF");
    assert!(!node(&s, jshape).visible, "property value of the nested instance kept");
    assert_eq!(node(&s, jshape).corners.radii, [4.0; 4]);
    assert_eq!(node(&s, kids(&s, j)[1]).kind.text().unwrap().characters, "Copied");
    assert_eq!(fill(&s, j), "EEEEEE");
    // A nested instance copied out on its own becomes an instance of its own main, same look.
    let inested = copy_of(&s, i, nested);
    ex(&mut s, "edit.copy", json!({"ids": [inested]}));
    s.set_selection(vec![]);
    let k = ex(&mut s, "edit.paste", json!({"x": 900, "y": 800}))["ids"][0].as_u64().map(NodeId).unwrap();
    healthy(&s);
    assert!(!inside_instance(s.doc().unwrap(), k));
    assert_eq!(node(&s, k).source, Some(icon));
    let kshape = kids(&s, k)[0];
    assert_eq!(node(&s, kshape).source, Some(shape));
    assert_eq!(fill(&s, kshape), "FF00FF");
    assert!(node(&s, kshape).overrides.contains("fills"));
    assert!(!node(&s, kshape).visible);
    assert!(!node(&s, kshape).overrides.contains("visible"), "set by its property, not an override");
    ex(&mut s, "instance.setProperties", json!({"ids": [k], "properties": {"Filled": true}}));
    assert!(node(&s, kids(&s, k)[0]).visible);
    ex(&mut s, "node.set", json!({"id": shape, "props": {"cornerRadius": 7}}));
    assert_eq!(node(&s, kids(&s, k)[0]).corners.radii, [7.0; 4]);
    assert_eq!(fill(&s, kids(&s, k)[0]), "FF00FF");
    // A plain layer copied out of an instance is a plain layer.
    let ilabel = kids(&s, i)[1];
    ex(&mut s, "edit.copy", json!({"ids": [ilabel]}));
    s.set_selection(vec![]);
    let t = ex(&mut s, "edit.paste", json!({"x": 900, "y": 1000}))["ids"][0].as_u64().map(NodeId).unwrap();
    assert!(node(&s, t).source.is_none() && node(&s, t).overrides.is_empty());
    healthy(&s);
}
