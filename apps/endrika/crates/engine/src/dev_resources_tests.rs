//! Dev resources: links for developers on layers, added, edited and removed, shown on a main
//! component's instances, checked as web links (never fetched), saved, synced, undoable and in
//! what agents inspect.

use figcraft_collab::{Change, Prop, Replica};
use figcraft_doc::NodeId;
use serde_json::{Value, json};

use crate::Session;
use crate::params::id_of;

fn ok(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd} {p}: {e}"))
}

fn rect(s: &mut Session) -> NodeId {
    id_of(&ok(s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 40, "height": 40}))["id"]).unwrap()
}

fn names(s: &mut Session, id: NodeId) -> Vec<String> {
    ok(s, "devResource.list", json!({"id": id})).as_array().unwrap().iter().map(|r| r["name"].as_str().unwrap().to_string()).collect()
}

#[test]
fn links_are_added_edited_and_removed() {
    let mut s = Session::new();
    let r = rect(&mut s);
    // A link with a name, and one without: named after its URL (no page title is looked up).
    let a = ok(&mut s, "devResource.add", json!({"id": r, "url": "https://storybook.example.com/button", "name": "Button stories"}));
    assert_eq!(a["index"], 0);
    ok(&mut s, "devResource.add", json!({"id": r, "url": "https://www.example.com/docs/"}));
    assert_eq!(names(&mut s, r), ["Button stories", "example.com/docs"]);
    // Edit the name or the URL; one undo step each.
    ok(&mut s, "devResource.edit", json!({"id": r, "index": 1, "name": "Docs"}));
    ok(&mut s, "devResource.edit", json!({"id": r, "index": 1, "url": "https://example.com/v2"}));
    let list = ok(&mut s, "devResource.list", json!({"id": r}));
    assert_eq!((&list[1]["name"], &list[1]["url"], &list[1]["index"]), (&json!("Docs"), &json!("https://example.com/v2"), &json!(1)));
    ok(&mut s, "edit.undo", json!({}));
    assert_eq!(ok(&mut s, "devResource.list", json!({"id": r}))[1]["url"], "https://www.example.com/docs/");
    // Remove one, then the rest.
    ok(&mut s, "devResource.remove", json!({"id": r, "index": 0}));
    assert_eq!(names(&mut s, r), ["Docs"]);
    assert_eq!(ok(&mut s, "devResource.remove", json!({"id": r}))["removed"], 1);
    assert!(names(&mut s, r).is_empty());
    assert!(s.execute("devResource.remove", &json!({"id": r})).is_err(), "nothing left to remove");
    // The selection is the default target.
    ok(&mut s, "select.set", json!({"ids": [r]}));
    ok(&mut s, "devResource.add", json!({"url": "http://localhost:6006"}));
    assert_eq!(names(&mut s, r), ["localhost:6006"]);
}

#[test]
fn only_web_links_are_kept() {
    let mut s = Session::new();
    let r = rect(&mut s);
    let long = format!("https://example.com/{}", "a".repeat(3000));
    for url in ["javascript:alert(1)", "ftp://example.com", "example.com", "https://", "https://exa mple.com", "https://:80/x", "", "  ", &long] {
        assert!(s.execute("devResource.add", &json!({"id": r, "url": url})).is_err(), "{url}");
    }
    for p in [json!({"id": r}), json!({"id": r, "url": 5}), json!({"id": r, "url": "https://a.b", "name": ["x"]})] {
        assert!(s.execute("devResource.add", &p).is_err(), "{p}");
    }
    // Spaces around a link are trimmed; HTTPS in capitals is fine; long names are clipped.
    let name = "n".repeat(500);
    ok(&mut s, "devResource.add", json!({"id": r, "url": "  HTTPS://Example.com/A  ", "name": name}));
    let l = ok(&mut s, "devResource.list", json!({"id": r}));
    assert_eq!(l[0]["url"], "HTTPS://Example.com/A");
    assert_eq!(l[0]["name"].as_str().unwrap().chars().count(), crate::dev_resources::MAX_NAME);
    // Docs silent on a limit: a layer keeps at most 32.
    for i in 1..crate::dev_resources::MAX_PER_LAYER {
        ok(&mut s, "devResource.add", json!({"id": r, "url": format!("https://example.com/{i}")}));
    }
    assert!(s.execute("devResource.add", &json!({"id": r, "url": "https://example.com/more"})).is_err());
    // Pages have none; edits need a link of the layer's own.
    let page = s.page().unwrap();
    assert!(s.execute("devResource.add", &json!({"id": page, "url": "https://example.com"})).is_err());
    assert!(s.execute("devResource.edit", &json!({"id": r, "index": 99, "name": "x"})).is_err());
    assert!(s.execute("devResource.edit", &json!({"id": r, "index": 0})).is_err(), "nothing to change");
}

#[test]
fn a_main_components_links_show_on_its_instances() {
    let mut s = Session::new();
    let f = id_of(&ok(&mut s, "frame.create", json!({"x": 0, "y": 0, "width": 100, "height": 40}))["id"]).unwrap();
    let r = id_of(&ok(&mut s, "shape.rectangle", json!({"x": 5, "y": 5, "width": 10, "height": 10, "parent": f}))["id"]).unwrap();
    ok(&mut s, "devResource.add", json!({"id": r, "url": "https://example.com/icon", "name": "Icon"}));
    let main = id_of(&ok(&mut s, "component.create", json!({"ids": [f]}))["id"]).unwrap();
    ok(&mut s, "devResource.add", json!({"id": main, "url": "https://github.com/acme/ui/blob/main/Button.tsx", "name": "Button.tsx"}));
    let inst = id_of(&ok(&mut s, "instance.create", json!({"component": main, "x": 200, "y": 0}))["id"]).unwrap();
    // The instance lists the main's link, read-only (no index), and can have links of its own.
    let l = ok(&mut s, "devResource.list", json!({"id": inst}));
    assert_eq!((&l[0]["name"], &l[0]["from"], l[0].get("index")), (&json!("Button.tsx"), &json!(main), None));
    ok(&mut s, "devResource.add", json!({"id": inst, "url": "https://jira.example.com/UI-12", "name": "UI-12"}));
    assert_eq!(names(&mut s, inst), ["UI-12", "Button.tsx"]);
    // Editing the main reaches the instance; the instance keeps its own across that.
    ok(&mut s, "devResource.edit", json!({"id": main, "index": 0, "name": "Button source"}));
    ok(&mut s, "node.set", json!({"id": main, "props": {"opacity": 0.5}}));
    assert_eq!(names(&mut s, inst), ["UI-12", "Button source"]);
    assert_eq!(s.doc().unwrap().node(inst).unwrap().dev_resources.len(), 1, "the main's links aren't copied");
    // Layers inside an instance show their main layer's links and take none of their own.
    let inner = s.doc().unwrap().node(inst).unwrap().children[0];
    let l = ok(&mut s, "devResource.list", json!({"id": inner}));
    assert_eq!((&l[0]["name"], l[0].get("index")), (&json!("Icon"), None));
    assert!(l[0]["from"].is_u64());
    assert!(s.execute("devResource.add", &json!({"id": inner, "url": "https://example.com"})).is_err());
}

#[test]
fn a_component_sets_links_show_on_its_variants_instances() {
    // Docs silent: links on a component set show on instances of any of its variants.
    let mut s = Session::new();
    let a = rect(&mut s);
    let b = rect(&mut s);
    let ca = id_of(&ok(&mut s, "component.create", json!({"ids": [a]}))["id"]).unwrap();
    let cb = id_of(&ok(&mut s, "component.create", json!({"ids": [b]}))["id"]).unwrap();
    let set = id_of(&ok(&mut s, "component.combineVariants", json!({"ids": [ca, cb]}))["id"]).unwrap();
    ok(&mut s, "devResource.add", json!({"id": set, "url": "https://example.com/button", "name": "Button docs"}));
    let inst = id_of(&ok(&mut s, "instance.create", json!({"component": cb, "x": 300, "y": 0}))["id"]).unwrap();
    let l = ok(&mut s, "devResource.list", json!({"id": inst}));
    assert_eq!((&l[0]["name"], &l[0]["from"]), (&json!("Button docs"), &json!(set)));
}

#[test]
fn links_save_sync_and_show_in_inspect() {
    let mut s = Session::new();
    let r = rect(&mut s);
    let before = s.doc().unwrap().clone();
    ok(&mut s, "devResource.add", json!({"id": r, "url": "https://example.com/spec", "name": "Spec"}));
    // A change the collaboration layer syncs.
    let d = figcraft_collab::diff(&Replica::new(before), s.doc().unwrap()).unwrap();
    assert!(d.changes.iter().any(|c| matches!(c, Change::Set { id, prop: Prop::DevResources, .. } if *id == r)), "{:?}", d.changes);
    // What agents inspect (MCP get_node / inspect_document).
    let j = crate::inspect::node_json(s.doc().unwrap(), r, 0);
    assert_eq!(j["devResources"], json!([{"name": "Spec", "url": "https://example.com/spec", "index": 0}]));
    // Saved and reopened.
    let dir = std::env::temp_dir().join(format!("figcraft-devres-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("f.figcraft").to_string_lossy().into_owned();
    ok(&mut s, "file.saveAs", json!({"path": path}));
    let mut t = Session::empty();
    ok(&mut t, "file.open", json!({"path": path}));
    assert_eq!(names(&mut t, r), ["Spec"]);
    std::fs::remove_dir_all(&dir).ok();
}
