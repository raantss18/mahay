//! `Document::changed_ids`: what differs between two versions of a document.

use crate::{Document, NodeKind};

#[test]
fn changed_ids_lists_added_removed_and_edited_nodes() {
    let mut a = Document::new("a");
    let page = a.first_page().unwrap();
    let keep = a.create(page, NodeKind::Rectangle, "Keep", 0.0, 0.0, 10.0, 10.0).unwrap();
    let edit = a.create(page, NodeKind::Rectangle, "Edit", 0.0, 0.0, 10.0, 10.0).unwrap();
    let gone = a.create(page, NodeKind::Rectangle, "Gone", 0.0, 0.0, 10.0, 10.0).unwrap();
    assert_eq!(a.changed_ids(&a, 10), Some(vec![]), "a version against itself");
    let mut b = a.clone();
    assert_eq!(b.changed_ids(&a, 10), Some(vec![]), "a copy shares everything");
    b.get_mut(edit).unwrap().opacity = 0.5;
    b.remove(gone).unwrap();
    let new = b.create(page, NodeKind::Ellipse { start: 0.0, sweep: 1.0, inner: 0.0 }, "New", 0.0, 0.0, 5.0, 5.0).unwrap();
    // The page changed too (its children).
    assert_eq!(b.changed_ids(&a, 10), Some(vec![page, edit, gone, new]));
    assert_eq!(a.changed_ids(&b, 10), Some(vec![page, edit, gone, new]), "symmetric");
    assert!(!b.changed_ids(&a, 10).unwrap().contains(&keep));
    // A node copied on write but equal in value isn't a change.
    let mut c = a.clone();
    let _ = c.get_mut(keep).unwrap();
    assert_eq!(c.changed_ids(&a, 10), Some(vec![]));
    assert_eq!(b.changed_ids(&a, 3), None, "more than the limit");
}
