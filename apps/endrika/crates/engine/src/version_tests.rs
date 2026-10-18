//! Version history: save, list, restore (one undo step), rename, delete, duplicate, persistence in
//! the `.figcraft` file and bad parameters.

use serde_json::{Value, json};

use crate::params::id_of;
use crate::{EngineError, Session};

fn rect(s: &mut Session, x: f64) -> figcraft_doc::NodeId {
    id_of(&s.execute("shape.rectangle", &json!({"x": x, "y": 0, "width": 10, "height": 10})).unwrap()["id"]).unwrap()
}

fn count(s: &Session) -> usize {
    s.doc().unwrap().node_count()
}

fn list(s: &mut Session) -> Vec<Value> {
    s.execute("version.list", &json!({})).unwrap().as_array().unwrap().clone()
}

#[test]
fn save_list_restore_and_undo() {
    let mut s = Session::new();
    s.execute("file.rename", &json!({"name": "Shop"})).unwrap();
    rect(&mut s, 0.0);
    let one = count(&s);
    let v = s.execute("file.saveVersion", &json!({"name": "  First  ", "description": "one box"})).unwrap();
    assert_eq!(v["name"], "First");
    assert_eq!(v["description"], "one box");
    assert_eq!(v["index"], 0);
    assert!(s.ds().unwrap().is_dirty(), "a new version is an unsaved change");
    rect(&mut s, 20.0);
    let v2 = s.execute("file.saveVersion", &json!({})).unwrap();
    assert_eq!(v2["name"], "Version 2", "untitled versions are numbered");
    rect(&mut s, 40.0);
    let three = count(&s);
    let l = list(&mut s);
    assert_eq!(l.len(), 2);
    assert_eq!(l[0]["name"], "First");
    assert!(l[0]["time"].as_u64().unwrap() > 0);
    // The panel request comes with the same list.
    assert_eq!(s.execute("file.versionHistory", &json!({})).unwrap(), Value::Array(l.clone()));
    assert!(s.ui_requests.iter().any(|r| r.kind == "panel.versionHistory"));

    // Restore by id: the content goes back, the file name stays; one undo step brings it back.
    s.execute("file.rename", &json!({"name": "Shop v2"})).unwrap();
    let r = s.execute("version.restore", &json!({"id": l[0]["id"]})).unwrap();
    assert_eq!(r["name"], "First");
    assert_eq!(count(&s), one);
    assert_eq!(s.doc().unwrap().name, "Shop v2");
    assert!(s.selection().is_empty());
    s.execute("edit.undo", &json!({})).unwrap();
    assert_eq!(count(&s), three);
    s.execute("edit.redo", &json!({})).unwrap();
    assert_eq!(count(&s), one);
    // By index.
    s.execute("version.restore", &json!({"index": 1})).unwrap();
    assert_eq!(count(&s), one + 1);
    assert_eq!(list(&mut s).len(), 3, "restoring keeps the history (plus a checkpoint of what the first restore replaced)");
}

#[test]
fn rename_delete_duplicate() {
    let mut s = Session::new();
    s.execute("file.rename", &json!({"name": "App"})).unwrap();
    rect(&mut s, 0.0);
    let a = s.execute("file.saveVersion", &json!({"name": "A", "description": "first"})).unwrap();
    rect(&mut s, 20.0);
    s.execute("file.saveVersion", &json!({"name": "B"})).unwrap();
    // Rename keeps the description unless one is given.
    let r = s.execute("version.rename", &json!({"id": a["id"], "name": "Alpha"})).unwrap();
    assert_eq!((r["name"].as_str(), r["description"].as_str()), (Some("Alpha"), Some("first")));
    s.execute("version.rename", &json!({"index": 0, "name": "Alpha", "description": "changed"})).unwrap();
    assert_eq!(list(&mut s)[0]["description"], "changed");
    assert!(s.execute("version.rename", &json!({"index": 0, "name": "   "})).is_err());
    // Long names are clipped.
    s.execute("version.rename", &json!({"index": 1, "name": "x".repeat(5000)})).unwrap();
    assert_eq!(list(&mut s)[1]["name"].as_str().unwrap().chars().count(), figcraft_format::MAX_VERSION_NAME);

    // Duplicate opens the version in a new tab without touching this one.
    let before = count(&s);
    let d = s.execute("version.duplicate", &json!({"id": a["id"]})).unwrap();
    assert_eq!(d["index"], 1);
    assert_eq!(s.active, 1);
    assert_eq!(s.doc().unwrap().name, "App (Alpha)");
    assert_eq!(count(&s), before - 1);
    assert!(s.ds().unwrap().versions.is_empty());
    s.execute("doc.switch", &json!({"index": 0})).unwrap();
    assert_eq!(count(&s), before);

    // Delete.
    s.execute("version.delete", &json!({"id": a["id"]})).unwrap();
    let l = list(&mut s);
    assert_eq!(l.len(), 1);
    assert_eq!(l[0]["index"], 0);
    assert!(s.execute("version.restore", &json!({"id": a["id"]})).is_err());
}

#[test]
fn bad_indexes_and_ids_are_rejected() {
    let mut s = Session::new();
    rect(&mut s, 0.0);
    s.execute("file.saveVersion", &json!({"name": "Only"})).unwrap();
    rect(&mut s, 20.0);
    let n = count(&s);
    for p in [
        json!({"index": -1}),
        json!({"index": 1}),
        json!({"index": 0.5}),
        json!({"index": "NaN"}),
        json!({"index": 1e300}),
        json!({"id": "nope"}),
        json!({}),
        json!(null),
    ] {
        for cmd in ["version.restore", "version.delete", "version.duplicate"] {
            assert!(matches!(s.execute(cmd, &p), Err(EngineError::BadParams { .. })), "{cmd} {p}");
        }
        let mut q = p.as_object().cloned().unwrap_or_default();
        q.insert("name".into(), json!("x"));
        assert!(s.execute("version.rename", &Value::Object(q)).is_err(), "rename {p}");
    }
    // Nothing changed: a negative index never restores version 0.
    assert_eq!(count(&s), n);
    assert_eq!(list(&mut s).len(), 1);
    assert_eq!(s.docs.len(), 1);
}

#[test]
fn history_is_capped() {
    let mut s = Session::new();
    for i in 0..figcraft_format::MAX_VERSIONS {
        s.execute("file.saveVersion", &json!({"name": format!("v{i}")})).unwrap();
    }
    assert!(matches!(s.execute("file.saveVersion", &json!({})), Err(EngineError::Disabled(..))));
    let l = list(&mut s);
    assert_eq!(l.len(), figcraft_format::MAX_VERSIONS);
    // Ids are unique even when saved within the same millisecond.
    let ids: std::collections::BTreeSet<&str> = l.iter().filter_map(|v| v["id"].as_str()).collect();
    assert_eq!(ids.len(), l.len());
}

#[test]
fn versions_persist_in_the_file() {
    let dir = std::env::temp_dir().join(format!("figcraft-versions-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("history.figcraft").to_string_lossy().to_string();
    let mut s = Session::new();
    rect(&mut s, 0.0);
    let one = count(&s);
    s.execute("file.saveVersion", &json!({"name": "Start", "description": "a box"})).unwrap();
    rect(&mut s, 20.0);
    s.execute("file.save", &json!({"path": path})).unwrap();
    assert!(!s.ds().unwrap().is_dirty());

    // Reopen in a fresh session: the history is there and restores.
    let mut t = Session::empty();
    t.execute("file.open", &json!({"path": path})).unwrap();
    let l = list(&mut t);
    assert_eq!(l.len(), 1);
    assert_eq!((l[0]["name"].as_str(), l[0]["description"].as_str()), (Some("Start"), Some("a box")));
    assert!(!t.ds().unwrap().is_dirty());
    t.execute("version.restore", &json!({"index": 0})).unwrap();
    assert_eq!(count(&t), one);
    // Deleting a version is an unsaved change; saving drops it from the file.
    t.execute("edit.undo", &json!({})).unwrap();
    t.execute("version.delete", &json!({"index": 0})).unwrap();
    assert!(t.ds().unwrap().is_dirty());
    t.execute("file.save", &json!({})).unwrap();
    t.execute("file.revert", &json!({})).unwrap();
    // Only the checkpoint the restore made of the file as opened is left.
    assert_eq!(list(&mut t).iter().map(|v| v["auto"] == true).collect::<Vec<_>>(), vec![true]);
    let _ = std::fs::remove_dir_all(&dir);
}
