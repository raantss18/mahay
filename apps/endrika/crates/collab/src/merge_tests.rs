//! Three-way merges of documents edited directly (the engine's branch tests run real commands).

use figcraft_color::{Color, Paint};
use figcraft_doc::{Document, NodeId, NodeKind, Style, StyleKind};

use crate::merge::{Key, Plan, Side, plan};

struct Files {
    base: Document,
    branch: Document,
    main: Document,
    frame: NodeId,
    kids: Vec<NodeId>,
}

/// A frame with five rectangles; the branch allocates ids in its own space.
fn files() -> Files {
    let mut base = Document::new("File");
    let page = base.first_page().unwrap();
    let frame = base.create(page, NodeKind::Frame, "Frame", 0.0, 0.0, 200.0, 200.0).unwrap();
    let kids = (0..5).map(|i| base.create(frame, NodeKind::Rectangle, &format!("R{i}"), 0.0, 0.0, 10.0, 10.0).unwrap()).collect();
    base.styles.push(style("S:1", Color::BLACK));
    let mut branch = base.clone();
    branch.set_client_id(base.highest_id_space() + 1);
    let main = base.clone();
    Files { base, branch, main, frame, kids }
}

fn style(id: &str, c: Color) -> Style {
    Style {
        id: id.into(),
        name: format!("Style {id}"),
        description: String::new(),
        kind: StyleKind::Paint { paints: vec![Paint::solid(c)] },
        bound: Default::default(),
    }
}

fn plan_of(f: &Files) -> Plan {
    plan(&f.base, &f.branch, &f.main).unwrap()
}

fn all(side: Side) -> impl Fn(&Key) -> Option<Side> {
    move |_| Some(side)
}

fn merged(f: &Files, p: &Plan, pick: Side) -> Document {
    let (doc, _) = p.merge(&f.main, Side::Branch, &all(pick)).unwrap();
    assert!(doc.validate().is_empty(), "{:?}", doc.validate());
    doc
}

#[test]
fn disjoint_edits_merge_both_ways() {
    let mut f = files();
    let (frame, k) = (f.frame, f.kids.clone());
    f.branch.get_mut(k[0]).unwrap().opacity = 0.5;
    let added = f.branch.create(frame, NodeKind::Rectangle, "New", 5.0, 5.0, 20.0, 20.0).unwrap();
    f.main.get_mut(k[1]).unwrap().fills = vec![Paint::solid(Color::WHITE)];
    f.main.remove(k[4]).unwrap();
    let main_new = f.main.create(frame, NodeKind::Rectangle, "Main new", 0.0, 0.0, 5.0, 5.0).unwrap();
    assert_ne!(added, main_new, "the branch makes ids in its own space");
    let p = plan_of(&f);
    assert!(p.conflicts.is_empty(), "{:?}", p.conflicts);
    let none = |_: &Key| None;
    for (into, from) in [(&f.main, Side::Branch), (&f.branch, Side::Main)] {
        let (doc, _) = p.merge(into, from, &none).unwrap();
        assert_eq!(doc.node(k[0]).unwrap().opacity, 0.5);
        assert_eq!(doc.node(k[1]).unwrap().fills, vec![Paint::solid(Color::WHITE)]);
        assert!(doc.node(k[4]).is_none() && doc.node(added).is_some() && doc.node(main_new).is_some());
        assert!(doc.validate().is_empty());
    }
}

#[test]
fn the_same_property_changed_on_both_sides_is_a_conflict() {
    let mut f = files();
    let r = f.kids[0];
    f.branch.get_mut(r).unwrap().opacity = 0.25;
    f.main.get_mut(r).unwrap().opacity = 0.75;
    // The same change on both sides is no conflict.
    f.branch.get_mut(f.kids[1]).unwrap().visible = false;
    f.main.get_mut(f.kids[1]).unwrap().visible = false;
    let p = plan_of(&f);
    assert_eq!(p.conflicts.len(), 1);
    assert_eq!(p.conflicts[0].key.to_string(), format!("{r}/opacity"));
    let err = p.merge(&f.main, Side::Branch, &|_| None).unwrap_err();
    assert!(err.to_string().contains("unresolved"), "{err}");
    assert_eq!(merged(&f, &p, Side::Branch).node(r).unwrap().opacity, 0.25);
    assert_eq!(merged(&f, &p, Side::Main).node(r).unwrap().opacity, 0.75);
    assert!(!merged(&f, &p, Side::Main).node(f.kids[1]).unwrap().visible);
}

#[test]
fn deleting_what_the_other_side_edited_is_a_conflict() {
    // The branch deletes the frame; main edits a layer in it.
    let mut f = files();
    let (frame, r) = (f.frame, f.kids[0]);
    f.branch.remove(frame).unwrap();
    f.main.get_mut(r).unwrap().name = "Edited".into();
    let p = plan_of(&f);
    assert_eq!(p.conflicts.len(), 1);
    assert_eq!((p.conflicts[0].key.clone(), p.conflicts[0].deleted_by), (Key::Layer(frame), Some(Side::Branch)));
    assert_eq!(merged(&f, &p, Side::Main).node(r).unwrap().name, "Edited", "main's edit is kept");
    assert!(merged(&f, &p, Side::Branch).node(frame).is_none(), "the deletion wins");

    // Main deletes the frame; the branch adds a layer to it and edits another.
    let mut f = files();
    let (frame, r) = (f.frame, f.kids[1]);
    f.main.remove(frame).unwrap();
    let added = f.branch.create(frame, NodeKind::Rectangle, "New", 0.0, 0.0, 5.0, 5.0).unwrap();
    f.branch.get_mut(r).unwrap().opacity = 0.5;
    let p = plan_of(&f);
    assert_eq!(p.conflicts.len(), 1);
    assert_eq!(p.conflicts[0].deleted_by, Some(Side::Main));
    let kept = merged(&f, &p, Side::Branch);
    assert_eq!(kept.node(frame).unwrap().children.len(), 6, "the frame comes back with the branch's layer");
    assert!(kept.node(added).is_some());
    assert_eq!(kept.node(r).unwrap().opacity, 0.5);
    assert!(merged(&f, &p, Side::Main).node(frame).is_none());
    // Updating the branch from main the same way: main's deletion wins or the branch keeps it.
    let (doc, _) = p.merge(&f.branch, Side::Main, &all(Side::Main)).unwrap();
    assert!(doc.node(frame).is_none() && doc.node(added).is_none());
    let (doc, _) = p.merge(&f.branch, Side::Main, &all(Side::Branch)).unwrap();
    assert_eq!(doc, f.branch);

    // Deleting a layer nobody else touched is no conflict.
    let mut f = files();
    f.branch.remove(f.kids[2]).unwrap();
    f.main.get_mut(f.kids[3]).unwrap().opacity = 0.1;
    let p = plan_of(&f);
    assert!(p.conflicts.is_empty());
    assert!(merged(&f, &p, Side::Main).node(f.kids[2]).is_none());
}

#[test]
fn tree_moves_merge_and_conflict_per_layer() {
    let mut f = files();
    let (frame, k) = (f.frame, f.kids.clone());
    let page = f.base.first_page().unwrap();
    // The branch moves R0 into a new group; main moves R2 to the page.
    let g = f.branch.create(frame, NodeKind::Group, "G", 0.0, 0.0, 1.0, 1.0).unwrap();
    f.branch.place(k[0], g, 0).unwrap();
    let at = f.main.node(page).unwrap().children.len();
    f.main.place(k[2], page, at).unwrap();
    let p = plan_of(&f);
    assert!(p.conflicts.is_empty(), "{:?}", p.conflicts);
    let doc = merged(&f, &p, Side::Branch);
    assert_eq!(doc.node(k[0]).unwrap().parent, Some(g));
    assert_eq!(doc.node(k[2]).unwrap().parent, Some(page));

    // Both move R3, to different places: one conflict about where it is.
    let mut f = files();
    let k = f.kids.clone();
    let at = f.branch.node(page).unwrap().children.len();
    f.branch.place(k[3], page, at).unwrap();
    let g = f.main.create(f.frame, NodeKind::Group, "G", 0.0, 0.0, 1.0, 1.0).unwrap();
    f.main.place(k[3], g, 0).unwrap();
    let p = plan_of(&f);
    let keys: Vec<String> = p.conflicts.iter().map(|c| c.key.to_string()).collect();
    assert_eq!(keys, [format!("{}/parent", k[3])]);
    assert_eq!(merged(&f, &p, Side::Branch).node(k[3]).unwrap().parent, Some(page));
    assert_eq!(merged(&f, &p, Side::Main).node(k[3]).unwrap().parent, Some(g));
}

#[test]
fn styles_merge_per_entry() {
    let mut f = files();
    f.branch.styles.push(style("S:b", Color::WHITE));
    f.main.styles.push(style("S:m", Color::WHITE));
    f.branch.styles[0] = style("S:1", Color::from_rgb8_const(255, 0, 0));
    f.main.styles[0] = style("S:1", Color::from_rgb8_const(0, 0, 255));
    let p = plan_of(&f);
    let keys: Vec<String> = p.conflicts.iter().map(|c| c.key.to_string()).collect();
    assert_eq!(keys, ["style/S:1"]);
    let doc = merged(&f, &p, Side::Branch);
    let ids: Vec<&str> = doc.styles.iter().map(|s| s.id.as_str()).collect();
    assert_eq!(ids, ["S:1", "S:m", "S:b"], "both new styles are kept");
    assert_eq!(doc.styles[0], f.branch.styles[0]);
    assert_eq!(merged(&f, &p, Side::Main).styles[0], f.main.styles[0]);
    // A style removed on one side and edited on the other conflicts too.
    let mut f = files();
    f.branch.styles.clear();
    f.main.styles[0].name = "Renamed".into();
    assert_eq!(plan_of(&f).conflicts.len(), 1);
}
