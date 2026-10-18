//! Dev Mode "Compare changes": snapshots of ready-for-dev frames and the layer-by-layer diff.

use crate::components::tests::{card, copy_of, ex, idv, node};
use crate::devcompare::{ChangeKind, Comparison, compare};
use crate::{EngineError, Session};
use figcraft_doc::*;
use serde_json::json;

/// A ready-for-dev "Screen" frame holding a "Title" text, a "Button" rectangle and a "Badge" group.
struct Screen {
    frame: NodeId,
    title: NodeId,
    button: NodeId,
    badge: NodeId,
    dot: NodeId,
}

fn screen(s: &mut Session) -> Screen {
    let frame = idv(&ex(s, "frame.create", json!({"x": 0, "y": 0, "width": 400, "height": 300, "name": "Screen"})));
    let title = idv(&ex(s, "text.create", json!({"text": "Welcome", "x": 20, "y": 20, "parent": frame, "props": {"name": "Title"}})));
    let button = idv(&ex(
        s,
        "shape.rectangle",
        json!({"x": 20, "y": 200, "width": 120, "height": 40, "parent": frame, "props": {"name": "Button", "fill": "#FF0000"}}),
    ));
    let dot = idv(&ex(s, "shape.ellipse", json!({"x": 300, "y": 20, "width": 20, "height": 20, "parent": frame, "props": {"name": "Dot"}})));
    let badge = idv(&ex(s, "object.group", json!({"ids": [dot], "name": "Badge"})));
    ex(s, "node.set", json!({"id": frame, "props": {"devStatus": "readyForDev"}}));
    Screen { frame, title, button, badge, dot }
}

fn texts(c: &Comparison, id: NodeId) -> Vec<String> {
    c.changes.iter().filter(|l| l.id == id).flat_map(|l| l.properties.iter().map(|p| p.text.clone())).collect()
}

fn kind(c: &Comparison, id: NodeId) -> Option<ChangeKind> {
    c.changes.iter().find(|l| l.id == id).map(|l| l.change)
}

#[test]
fn the_first_change_keeps_a_snapshot_until_resolved() {
    let mut s = Session::new();
    let sc = screen(&mut s);
    assert!(s.doc().unwrap().dev_snapshots.is_empty(), "marking takes no snapshot");
    let c = compare(s.doc().unwrap(), sc.frame);
    assert!(!c.changed && c.changes.is_empty());
    // The first edit keeps what the frame was; later edits keep that snapshot.
    ex(&mut s, "node.set", json!({"id": sc.button, "props": {"width": 136}}));
    let first = s.doc().unwrap().dev_snapshots.get(&sc.frame).cloned().expect("snapshot");
    assert_eq!(first.root().map(|n| n.id), Some(sc.frame));
    assert_eq!(first.node(sc.button).map(|n| n.width), Some(120.0));
    ex(&mut s, "node.set", json!({"id": sc.button, "props": {"width": 150}}));
    assert!(std::sync::Arc::ptr_eq(&first, &s.doc().unwrap().dev_snapshots[&sc.frame]), "later edits keep the first snapshot");
    assert_eq!(texts(&compare(s.doc().unwrap(), sc.frame), sc.button), vec!["width 120 → 150"]);
    // Undoing back to before the first change drops it with the change.
    s.undo().unwrap();
    s.undo().unwrap();
    assert!(s.doc().unwrap().dev_snapshots.is_empty());
    s.redo().unwrap();
    assert!(s.doc().unwrap().dev_snapshots.contains_key(&sc.frame));
    // Resolving, marking again or another status drop it; the next change starts from there.
    ex(&mut s, "dev.resolveChange", json!({"ids": [sc.frame]}));
    assert!(s.doc().unwrap().dev_snapshots.is_empty());
    ex(&mut s, "node.set", json!({"id": sc.button, "props": {"height": 48}}));
    assert_eq!(texts(&compare(s.doc().unwrap(), sc.frame), sc.button), vec!["height 40 → 48"], "compared with the resolved state");
    ex(&mut s, "node.set", json!({"id": sc.frame, "props": {"devStatus": "readyForDev"}}));
    assert!(s.doc().unwrap().dev_snapshots.is_empty(), "marked again");
    ex(&mut s, "node.set", json!({"id": sc.button, "props": {"height": 50}}));
    ex(&mut s, "node.set", json!({"id": sc.frame, "props": {"devStatus": "completed"}}));
    assert!(s.doc().unwrap().dev_snapshots.is_empty(), "completed");
    // Deleting the frame drops its snapshot too.
    ex(&mut s, "node.set", json!({"id": sc.frame, "props": {"devStatus": "readyForDev"}}));
    ex(&mut s, "node.set", json!({"id": sc.button, "props": {"height": 52}}));
    assert_eq!(s.doc().unwrap().dev_snapshots.len(), 1);
    ex(&mut s, "edit.delete", json!({"ids": [sc.frame]}));
    assert!(s.doc().unwrap().dev_snapshots.is_empty());
}

#[test]
fn property_changes_read_like_inspect() {
    let mut s = Session::new();
    let sc = screen(&mut s);
    ex(&mut s, "node.set", json!({"id": sc.button, "props": {"width": 136, "fill": "#E03030", "name": "Primary", "cornerRadius": 8}}));
    ex(&mut s, "node.set", json!({"id": sc.button, "props": {"opacity": 0.5, "x": 30}}));
    ex(&mut s, "text.set", json!({"id": sc.title, "characters": "Hello there"}));
    ex(&mut s, "node.set", json!({"id": sc.title, "props": {"fontSize": 24}}));
    ex(&mut s, "node.set", json!({"id": sc.frame, "props": {"fills": ["#F0F0F0"]}}));
    let c = compare(s.doc().unwrap(), sc.frame);
    assert!(c.changed && c.snapshot && !c.truncated);
    let button = texts(&c, sc.button);
    for want in ["name Button → Primary", "width 120 → 136", "left 20 → 30", "opacity 100% → 50%", "corner radius 0 → 8", "fill #FF0000 → #E03030"]
    {
        assert!(button.contains(&want.to_string()), "{want} in {button:?}");
    }
    let title = texts(&c, sc.title);
    assert!(title.contains(&"text “Welcome” → “Hello there”".to_string()), "{title:?}");
    assert!(title.iter().any(|t| t.starts_with("font size") && t.ends_with("→ 24")), "{title:?}");
    assert!(texts(&c, sc.frame).contains(&"fill #FFFFFF → #F0F0F0".to_string()));
    assert_eq!(kind(&c, sc.badge), None, "untouched layers aren't listed");
    assert_eq!((c.added, c.removed, c.modified), (0, 0, 3));
    // Sections and levels are reported with each change.
    let fill = c.changes.iter().find(|l| l.id == sc.button).and_then(|l| l.properties.iter().find(|p| p.property == "fill")).unwrap();
    assert_eq!((fill.section, fill.before.as_str(), fill.after.as_str()), ("Fills", "#FF0000", "#E03030"));
    assert_eq!(c.changes.iter().find(|l| l.id == sc.button).map(|l| l.depth), Some(1));
    // Auto layout and effects.
    ex(&mut s, "layout.add", json!({"ids": [sc.frame]}));
    ex(&mut s, "node.set", json!({"id": sc.dot, "props": {"effects": [{"type": "dropShadow"}]}}));
    let c = compare(s.doc().unwrap(), sc.frame);
    assert!(texts(&c, sc.frame).iter().any(|t| t.starts_with("auto layout none → ")), "{:?}", texts(&c, sc.frame));
    assert!(texts(&c, sc.dot).iter().any(|t| t.starts_with("drop shadow none → ")), "{:?}", texts(&c, sc.dot));
}

#[test]
fn added_removed_moved_and_reordered_layers() {
    let mut s = Session::new();
    let sc = screen(&mut s);
    let extra = idv(&ex(&mut s, "shape.rectangle", json!({"x": 200, "y": 100, "width": 10, "height": 10, "parent": sc.frame})));
    let wrap = idv(&ex(&mut s, "object.group", json!({"ids": [extra], "name": "New group"})));
    ex(&mut s, "edit.delete", json!({"ids": [sc.title]}));
    // The dot leaves its group (the group, emptied, disappears) for the frame.
    ex(&mut s, "node.reparent", json!({"ids": [sc.dot], "parent": sc.frame}));
    let c = compare(s.doc().unwrap(), sc.frame);
    assert_eq!(kind(&c, wrap), Some(ChangeKind::Added));
    assert_eq!(c.changes.iter().find(|l| l.id == wrap).map(|l| l.layers), Some(2), "the group and its rectangle");
    assert_eq!(kind(&c, extra), None, "listed with its group");
    assert_eq!(kind(&c, sc.title), Some(ChangeKind::Removed));
    assert_eq!(kind(&c, sc.badge), Some(ChangeKind::Removed));
    assert!(texts(&c, sc.dot).contains(&"parent Badge → Screen".to_string()), "{:?}", texts(&c, sc.dot));
    // Removed layers come from the snapshot, with their names.
    let removed = c.changes.iter().find(|l| l.id == sc.title).unwrap();
    assert_eq!((removed.name.as_str(), removed.type_name), ("Title", "TEXT"));
    // Reordering among the layers both versions have.
    let mut s = Session::new();
    let sc = screen(&mut s);
    ex(&mut s, "arrange.front", json!({"ids": [sc.title]}));
    let c = compare(s.doc().unwrap(), sc.frame);
    assert!(texts(&c, sc.title).iter().any(|t| t.starts_with("order ") && t.ends_with("→ 1")), "{:?}", texts(&c, sc.title));
}

#[test]
fn nested_and_instance_changes_are_found() {
    let mut s = Session::new();
    let (c, bg, title, _dot) = card(&mut s);
    let frame = idv(&ex(&mut s, "frame.create", json!({"x": 500, "y": 0, "width": 400, "height": 300, "name": "Screen"})));
    let inst = idv(&ex(&mut s, "instance.create", json!({"component": c, "x": 10, "y": 10, "parent": frame})));
    let group_child = idv(&ex(&mut s, "shape.rectangle", json!({"x": 10, "y": 200, "width": 10, "height": 10, "parent": frame})));
    let outer = idv(&ex(&mut s, "object.group", json!({"ids": [group_child], "name": "Outer"})));
    let _inner = idv(&ex(&mut s, "object.group", json!({"ids": [group_child], "name": "Inner"})));
    ex(&mut s, "node.set", json!({"id": frame, "props": {"devStatus": "readyForDev"}}));
    // A main component edit reaches the instance in the ready frame.
    ex(&mut s, "node.set", json!({"id": bg, "props": {"fill": "#0000FF"}}));
    assert!(node(&s, frame).dev_changed, "the instance changed");
    let ibg = copy_of(&s, inst, bg);
    let cmp = compare(s.doc().unwrap(), frame);
    assert!(texts(&cmp, ibg).contains(&"fill #EEEEEE → #0000FF".to_string()), "{:?}", cmp.changes);
    // An override inside the instance, and a change deep in nested groups.
    let ititle = copy_of(&s, inst, title);
    ex(&mut s, "text.set", json!({"id": ititle, "characters": "Inbox"}));
    ex(&mut s, "node.set", json!({"id": group_child, "props": {"fill": "#00FF00"}}));
    let cmp = compare(s.doc().unwrap(), frame);
    assert!(texts(&cmp, ititle).contains(&"text “Hello” → “Inbox”".to_string()), "{:?}", cmp.changes);
    let deep = cmp.changes.iter().find(|l| l.id == group_child).expect("nested change");
    assert_eq!(deep.depth, 3, "frame › Outer › Inner › layer");
    assert!(cmp.changes.iter().all(|l| l.id != outer), "the groups themselves didn't change");
    // Swapping the instance's component is reported by name.
    let (c2, ..) = card(&mut s);
    ex(&mut s, "node.set", json!({"id": c2, "props": {"name": "Card 2"}}));
    ex(&mut s, "instance.swap", json!({"ids": [inst], "component": c2}));
    let cmp = compare(s.doc().unwrap(), frame);
    assert!(texts(&cmp, inst).contains(&"component Card → Card 2".to_string()), "{:?}", texts(&cmp, inst));
}

#[test]
fn the_compare_command_and_its_errors() {
    let mut s = Session::new();
    let sc = screen(&mut s);
    let v = ex(&mut s, "dev.compare", json!({"id": sc.frame}));
    assert_eq!((v["changed"].clone(), v["changes"].clone()), (json!(false), json!([])));
    ex(&mut s, "node.set", json!({"id": sc.button, "props": {"fill": "#E03030"}}));
    ex(&mut s, "select.set", json!({"ids": [sc.frame]}));
    s.ui_requests.clear();
    let v = ex(&mut s, "dev.compare", json!({"show": true}));
    assert_eq!(v["snapshot"], json!(true));
    assert_eq!(v["changes"][0]["id"], json!(sc.button));
    assert_eq!(v["changes"][0]["change"], json!("changed"));
    assert_eq!(v["changes"][0]["type"], json!("RECTANGLE"));
    assert_eq!(v["changes"][0]["properties"][0]["text"], json!("fill #FF0000 → #E03030"));
    assert!(s.ui_requests.iter().any(|u| u.kind == "view.devCompare" && u.data["id"] == json!(sc.frame)));
    // Only frames and sections marked ready for dev.
    for p in [json!({"id": sc.button}), json!({"id": 987_654}), json!({"ids": []})] {
        assert!(s.execute("dev.compare", &p).is_err(), "{p}");
    }
    let other = idv(&ex(&mut s, "frame.create", json!({"x": 900, "y": 0, "width": 10, "height": 10})));
    assert!(s.execute("dev.compare", &json!({"id": other})).is_err(), "no status");
    // Changed before snapshots were kept: compared as unknown, not as unchanged.
    let doc = s.doc_mut().unwrap();
    doc.dev_snapshots.clear();
    let v = ex(&mut s, "dev.compare", json!({"id": sc.frame}));
    assert_eq!((v["changed"].clone(), v["snapshot"].clone()), (json!(true), json!(false)));
}

#[test]
fn snapshots_are_saved_and_old_files_open() {
    let mut s = Session::new();
    let sc = screen(&mut s);
    let plain = figcraft_format::to_json(s.doc().unwrap()).unwrap();
    assert!(plain.get("devSnapshots").is_none() && plain.get("dev_snapshots").is_none(), "nothing written without a change");
    ex(&mut s, "node.set", json!({"id": sc.button, "props": {"width": 136}}));
    let bytes = figcraft_format::save_bytes(s.doc().unwrap(), None).unwrap();
    let (back, issues) = figcraft_format::load_bytes(&bytes).unwrap();
    assert!(issues.is_empty(), "{issues:?}");
    assert_eq!(back.dev_snapshots, s.doc().unwrap().dev_snapshots);
    assert_eq!(texts(&compare(&back, sc.frame), sc.button), vec!["width 120 → 136"]);
    // A file from before snapshots (the field missing) still opens.
    let mut v = figcraft_format::to_json(&back).unwrap();
    if let Some(o) = v.as_object_mut() {
        o.retain(|k, _| !k.to_lowercase().contains("snapshot"));
    }
    let (old, _) = figcraft_format::from_json(&v.to_string()).unwrap();
    assert!(old.dev_snapshots.is_empty());
    assert!(!compare(&old, sc.frame).snapshot);
}

#[test]
fn damaged_snapshots_are_ignored() {
    let mut s = Session::new();
    let sc = screen(&mut s);
    ex(&mut s, "node.set", json!({"id": sc.button, "props": {"width": 136}}));
    let page = s.page().unwrap();
    let doc = s.doc().unwrap().clone();
    let good = doc.dev_snapshots[&sc.frame].clone();
    let damaged = [
        DevSnapshot { nodes: vec![] },
        DevSnapshot { nodes: good.nodes.iter().skip(1).cloned().collect() },
        DevSnapshot { nodes: good.nodes.iter().cloned().chain(doc.node_arc(page).cloned()).collect() },
    ];
    for (k, snap) in damaged.into_iter().enumerate() {
        let mut d = doc.clone();
        d.dev_snapshots.insert(sc.frame, std::sync::Arc::new(snap));
        let c = compare(&d, sc.frame);
        if k < 2 {
            assert!(!c.snapshot, "{k}");
        }
        // A page claimed by the snapshot isn't put in the copy.
        if let Some(copy) = d.with_dev_snapshot(sc.frame) {
            assert!(copy.node(page).is_some_and(|p| p.is_page()));
        }
        // Repairing (as after loading) drops snapshots that don't start with their layer.
        d.repair();
        assert_eq!(d.dev_snapshots.contains_key(&sc.frame), k == 2, "{k}");
    }
}

#[test]
fn compare_commands_survive_junk() {
    let junk = [
        json!({}),
        json!(null),
        json!({"id": "x", "show": [1], "direction": 5}),
        json!({"ids": [-1, "0:0"], "id": 1e300, "show": "yes"}),
        json!({"id": {"a": 1}, "direction": "sideways"}),
    ];
    for cmd in ["dev.compare", "dev.focus", "dev.unfocus", "dev.resolveChange"] {
        for j in &junk {
            let mut s = Session::new();
            let sc = screen(&mut s);
            ex(&mut s, "node.set", json!({"id": sc.button, "props": {"width": 136}}));
            ex(&mut s, "select.set", json!({"ids": [sc.frame]}));
            if let Err(EngineError::Internal { cmd, msg }) = s.execute(cmd, j) {
                panic!("{cmd} panicked on {j}: {msg}");
            }
        }
    }
}
