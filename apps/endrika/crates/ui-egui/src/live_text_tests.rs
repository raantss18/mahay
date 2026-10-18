//! Other people's text carets drawn headless: the caret and selection in their colour at the
//! glyphs of the text, their name above it, the note when you type in the same layer, what
//! `collab.peers` reports, and no frames while nothing moves.

use egui::{Color32, Pos2, Shape};
use figcraft_doc::NodeId;
use figcraft_engine::figcraft_collab::EditingText;
use figcraft_engine::live::{Event as LiveEvent, Presence, editing_hint};
use serde_json::json;

use crate::canvas::TextOnScreen;
use crate::live_tests::{HANA, Harness, filled, has_text};
use crate::present_tests::id;

/// In a session with Hana; a text layer "Hello world" in the shared file.
fn session() -> (Harness, NodeId) {
    let (mut h, _) = Harness::new();
    let text = id(&h.app.session.execute("text.create", &json!({"text": "Hello world", "x": 100, "y": 300})).unwrap());
    h.app.session.execute("select.set", &json!({"ids": []})).unwrap();
    h.frames(3);
    h.joined();
    h.sync();
    (h, text)
}

fn typing(h: &mut Harness, node: NodeId, caret: usize, selection: Option<(usize, usize)>) {
    let page = h.page();
    let presence = Presence {
        viewport: Some((page, 0.0, 0.0, 800.0, 600.0)),
        editing_text: Some(Box::new(EditingText { node, caret, selection })),
        ..Presence::default()
    };
    h.send(vec![LiveEvent::Presence(1, presence)]);
}

/// Line segments drawn in `c`.
fn carets(shapes: &[Shape], c: Color32) -> Vec<[Pos2; 2]> {
    shapes
        .iter()
        .filter_map(|s| match s {
            Shape::LineSegment { points, stroke } if stroke.color == c => Some(*points),
            _ => None,
        })
        .collect()
}

/// Where the caret before char `index` of `node` sits on screen now.
fn caret_at(h: &Harness, node: NodeId, index: usize) -> [Pos2; 2] {
    let ds = h.app.session.ds().unwrap();
    TextOnScreen::of(&ds.doc, node).unwrap().caret(&h.app, index, ds.view.zoom)
}

fn near(a: [Pos2; 2], b: [Pos2; 2]) -> bool {
    a.iter().zip(b.iter()).all(|(p, q)| p.distance(*q) < 0.01)
}

#[test]
fn a_remote_caret_and_selection_show_at_the_glyphs() {
    let (mut h, text) = session();
    typing(&mut h, text, 5, Some((0, 5)));
    let shapes = h.frames(2);
    let drawn = carets(&shapes, HANA);
    assert_eq!(drawn.len(), 1, "one caret in Hana's colour");
    assert!(near(drawn[0], caret_at(&h, text, 5)), "after \"Hello\": {drawn:?}");
    assert!(drawn[0][1].y - drawn[0][0].y > 4.0, "a line's height");
    // The selected chars, tinted in her colour (one line, where fonts lay the text out); her name
    // above the caret.
    let lines = {
        let ds = h.app.session.ds().unwrap();
        TextOnScreen::of(&ds.doc, text).unwrap().selection(&h.app, 0, 5).len()
    };
    assert!(lines <= 1);
    assert_eq!(filled(&shapes, HANA.gamma_multiply(0.25)), lines, "the selected line");
    assert!(has_text(&shapes, "Hana"));
    // A caret past the end of your copy (she typed more, not synced yet) sits at its end.
    typing(&mut h, text, 999, None);
    let shapes = h.frames(2);
    let drawn = carets(&shapes, HANA);
    assert!(near(drawn[0], caret_at(&h, text, 11)), "{drawn:?}");
    assert_eq!(filled(&shapes, HANA.gamma_multiply(0.25)), 0, "no selection");
    // collab.peers says who types where.
    let peers = h.app.session.execute("collab.peers", &json!({})).unwrap();
    assert_eq!(peers["peers"][0]["editing"], json!({"node": text, "layer": "Hello world", "caret": 999, "selection": null}));
    assert_eq!(peers["editing"], json!(null));
    // Done typing: gone.
    h.send(vec![LiveEvent::Presence(1, Presence::default())]);
    let shapes = h.frames(2);
    assert!(carets(&shapes, HANA).is_empty() && !has_text(&shapes, "Hana"));
}

#[test]
fn typing_in_the_same_layer_says_so() {
    let (mut h, text) = session();
    typing(&mut h, text, 2, None);
    h.frames(2);
    assert!(h.app.run("text.edit.begin", json!({"id": text, "caret": 4})).is_some());
    let shapes = h.frames(2);
    assert_eq!(h.app.ui.toast.as_ref().map(|t| t.0.as_str()), Some("Hana is editing this text too: the version finished last is kept"));
    assert!(has_text(&shapes, "Hana is editing this text"), "the tag on her caret");
    assert_eq!(carets(&shapes, HANA).len(), 1);
    let peers = h.app.session.execute("collab.peers", &json!({})).unwrap();
    assert_eq!(peers["editing"], json!({"node": text, "layer": "Hello world", "caret": 4, "selection": null}));
    h.app.session.execute("text.edit.end", &json!({})).unwrap();
    // The names in the note.
    assert_eq!(editing_hint(&[]), None);
    assert_eq!(editing_hint(&["Ada", "Hana"]).as_deref(), Some("Ada and Hana are editing this text"));
    assert_eq!(editing_hint(&["Ada", "Hana", "Lin"]).as_deref(), Some("Ada and 2 others are editing this text"));
}

#[test]
fn a_resting_remote_caret_draws_no_frames() {
    let (mut h, text) = session();
    typing(&mut h, text, 3, Some((1, 3)));
    h.frames(2);
    assert!(h.goes_quiet(), "a remote caret asks for no frames: {:?}", h.ctx.repaint_causes());
    // A layer on another page: nothing drawn here.
    h.app.session.execute("page.add", &json!({"name": "Two"})).unwrap();
    h.app.session.execute("page.select", &json!({"index": 1})).unwrap();
    let elsewhere = id(&h.app.session.execute("text.create", &json!({"text": "Elsewhere", "x": 0, "y": 0})).unwrap());
    h.app.session.execute("page.select", &json!({"index": 0})).unwrap();
    assert_ne!(h.app.session.doc().unwrap().page_of(elsewhere), Some(h.page()));
    typing(&mut h, elsewhere, 1, None);
    let shapes = h.frames(2);
    assert!(carets(&shapes, HANA).is_empty(), "on another page");
}
