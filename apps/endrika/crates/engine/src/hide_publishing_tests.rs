//! Hide when publishing (`component.hideWhenPublishing`, the Assets panel's context menu): keeps a
//! component or a set out of the library its file publishes. The model's sign of it is the name
//! (a leading `.` or `_`), so the command writes the name: one undo step, saved and synced like
//! any rename, and honoured by the next publish.

use figcraft_doc::NodeId;
use serde_json::{Value, json};

use crate::library::{hidden_name, with_hidden_name};
use crate::params::id_of;
use crate::{EngineError, Session};

const CMD: &str = "component.hideWhenPublishing";

fn ex(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd} {p}: {e}"))
}

fn component(s: &mut Session, name: &str, x: f64) -> NodeId {
    let f = id_of(&ex(s, "frame.create", json!({"x": x, "y": 0, "width": 80, "height": 40, "name": name}))["id"]).unwrap();
    id_of(&ex(s, "component.create", json!({"ids": [f]}))["id"]).unwrap()
}

fn name(s: &Session, id: NodeId) -> String {
    s.doc().unwrap().node(id).unwrap().name.clone()
}

/// The names of the components the library published last offers.
fn offered(s: &Session) -> Vec<String> {
    let lib = s.doc().unwrap().published.clone().expect("published");
    let mut names: Vec<String> = lib.manifest.components.iter().filter(|a| a.published).map(|a| a.name.clone()).collect();
    names.sort();
    names
}

#[test]
fn names_gain_and_lose_the_mark() {
    // (name, hide, result)
    let rows = [
        ("Button", true, ".Button"),
        (".Button", true, ".Button"),
        ("_Button", true, "_Button"),
        ("Buttons/Primary", true, ".Buttons/Primary"),
        (".Button", false, "Button"),
        ("_Button", false, "Button"),
        ("._ Button", false, "Button"),
        ("Button", false, "Button"),
        ("...", false, "Component"),
        ("", true, "."),
    ];
    for (name, hide, want) in rows {
        let got = with_hidden_name(name, hide);
        assert_eq!(got, want, "{name:?} hidden={hide}");
        assert_eq!(hidden_name(&got), hide, "{got:?} reads back as hidden={hide}");
    }
}

#[test]
fn the_command_toggles_sets_and_is_one_undo_step() {
    let mut s = Session::new();
    let (a, b) = (component(&mut s, "Button", 0.0), component(&mut s, "Card", 100.0));
    // The selection by default; toggling hides.
    ex(&mut s, "select.set", json!({"ids": [a]}));
    let r = ex(&mut s, CMD, json!({}));
    assert_eq!((r["hidden"].as_bool(), name(&s, a).as_str(), name(&s, b).as_str()), (Some(true), ".Button", "Card"));
    // Several at once: hidden unless all of them are already.
    let r = ex(&mut s, CMD, json!({"ids": [a, b]}));
    assert_eq!((r["hidden"].as_bool(), name(&s, a), name(&s, b)), (Some(true), ".Button".to_string(), ".Card".to_string()));
    let undo = s.ds().unwrap().history.undo.len();
    let r = ex(&mut s, CMD, json!({"ids": [a, b]}));
    assert_eq!((r["hidden"].as_bool(), name(&s, a), name(&s, b)), (Some(false), "Button".to_string(), "Card".to_string()));
    assert_eq!(s.ds().unwrap().history.undo.len(), undo + 1, "one undo step for both");
    ex(&mut s, "edit.undo", json!({}));
    assert_eq!((name(&s, a), name(&s, b)), (".Button".to_string(), ".Card".to_string()));
    ex(&mut s, "edit.redo", json!({}));
    assert_eq!(name(&s, a), "Button");
    // `on` sets, whatever it was.
    ex(&mut s, CMD, json!({"ids": [a], "on": false}));
    assert_eq!(name(&s, a), "Button");
    ex(&mut s, CMD, json!({"ids": [a], "on": true}));
    ex(&mut s, CMD, json!({"ids": [a], "on": true}));
    assert_eq!(name(&s, a), ".Button");
}

#[test]
fn a_layer_in_a_component_or_a_variant_hides_the_whole_asset() {
    let mut s = Session::new();
    let a = component(&mut s, "Button", 0.0);
    let inner = id_of(&ex(&mut s, "shape.rectangle", json!({"x": 4, "y": 4, "width": 8, "height": 8, "parent": a}))["id"]).unwrap();
    let r = ex(&mut s, CMD, json!({"ids": [inner]}));
    assert_eq!(r["ids"], json!([a]));
    assert_eq!(name(&s, a), ".Button");
    // A variant: its set is the asset.
    let (v1, v2) = (component(&mut s, "Size=Small", 200.0), component(&mut s, "Size=Large", 300.0));
    let set = id_of(&ex(&mut s, "component.combineVariants", json!({"ids": [v1, v2]}))["id"]).unwrap();
    let before = name(&s, v1);
    ex(&mut s, CMD, json!({"ids": [v1]}));
    assert!(hidden_name(&name(&s, set)), "the set is hidden: {}", name(&s, set));
    assert_eq!(name(&s, v1), before, "the variant keeps its name (its property values)");
}

#[test]
fn what_isnt_a_component_is_refused() {
    let mut s = Session::new();
    let r = id_of(&ex(&mut s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 10, "height": 10}))["id"]).unwrap();
    for p in [json!({"ids": [r]}), json!({"ids": []}), json!({"ids": [999_999]}), json!({"ids": "x"}), json!({"on": "yes"})] {
        ex(&mut s, "select.none", json!({}));
        let e = s.execute(CMD, &p).expect_err(&p.to_string());
        assert!(!matches!(e, EngineError::Internal { .. }), "{p}: {e}");
    }
    assert!(!hidden_name(&name(&s, r)), "nothing renamed");
    assert!(s.ds().unwrap().history.undo.len() == 1, "and nothing to undo but the rectangle");
}

#[test]
fn publishing_leaves_hidden_components_out_and_brings_them_back() {
    let mut s = Session::new();
    let (a, _b) = (component(&mut s, "Button", 0.0), component(&mut s, "Card", 100.0));
    ex(&mut s, "library.publish", json!({}));
    assert_eq!(offered(&s), ["Button", "Card"]);
    ex(&mut s, CMD, json!({"ids": [a]}));
    ex(&mut s, "library.publish", json!({}));
    assert_eq!(offered(&s), ["Card"], "hidden: taken out of the library");
    ex(&mut s, CMD, json!({"ids": [a]}));
    ex(&mut s, "library.publish", json!({}));
    assert_eq!(offered(&s), ["Button", "Card"], "shown again: back in the library");
}

#[test]
fn it_is_synced_like_any_edit() {
    // What a live session sends is the change between two documents: the rename is one.
    let mut s = Session::new();
    let a = component(&mut s, "Button", 0.0);
    let mut replica = figcraft_collab::Replica::new(s.doc().unwrap().clone());
    ex(&mut s, CMD, json!({"ids": [a]}));
    let d = figcraft_collab::diff(&replica, s.doc().unwrap()).unwrap();
    assert!(!d.changes.is_empty(), "the change is one a session sends");
    replica.apply(&d.changes, figcraft_collab::Mode::Strict).unwrap();
    assert_eq!(replica.document().node(a).unwrap().name, ".Button");
}
