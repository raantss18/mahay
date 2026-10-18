//! Delays on mouse triggers and manual overlay offsets through commands: `delayMs` and the open
//! overlay action's `offset`, what they apply to, undo, the REST and `.fig` mappings, junk values.

use serde_json::{Value, json};

use crate::params::id_of;
use crate::{EngineError, Session};
use figcraft_doc::proto::{Action, Interaction, OverlayPosition, Trigger};
use figcraft_doc::{Document, NodeId};

fn frame(s: &mut Session, x: f64) -> NodeId {
    id_of(&s.execute("frame.create", &json!({"x": x, "y": 0, "width": 100, "height": 200})).unwrap()["id"]).unwrap()
}

fn first(s: &Session, id: NodeId) -> Interaction {
    s.doc().unwrap().node(id).unwrap().interactions[0].clone()
}

#[test]
fn delays_belong_to_mouse_triggers() {
    let mut s = Session::new();
    let (a, b) = (frame(&mut s, 0.0), frame(&mut s, 200.0));
    let add = |s: &mut Session, trigger: &str, delay: Value| {
        s.execute("proto.interaction.clear", &json!({"ids": [a]})).unwrap();
        s.execute(
            "proto.interaction.add",
            &json!({"id": a, "trigger": {"type": trigger}, "action": {"type": "navigate", "dest": b}, "delayMs": delay}),
        )
        .unwrap();
        first(s, a).delay_ms
    };
    // (trigger, delay asked for, delay kept, why)
    for (trigger, asked, kept, why) in [
        ("mouseEnter", json!(300), 300, "mouse enter waits"),
        ("mouseLeave", json!(300), 300, "mouse leave waits"),
        ("mouseDown", json!(1), 1, "mouse down waits"),
        ("mouseUp", json!(250.4), 250, "whole milliseconds"),
        ("mouseUp", json!(9e9), 600_000, "ten minutes at most"),
        ("onClick", json!(300), 0, "a click has no delay"),
        ("onHover", json!(300), 0, "while hovering has none"),
        ("onDrag", json!(300), 0, "a drag has none"),
    ] {
        assert_eq!(add(&mut s, trigger, asked.clone()), kept, "{trigger} {asked}: {why}");
    }
    // Patched, then lost with a trigger it doesn't apply to; each edit is one undo step.
    add(&mut s, "mouseEnter", json!(0));
    s.execute("proto.interaction.set", &json!({"id": a, "index": 0, "delayMs": 450})).unwrap();
    assert_eq!(first(&s, a).delay_ms, 450);
    s.execute("proto.interaction.set", &json!({"id": a, "index": 0, "trigger": {"type": "onClick"}})).unwrap();
    assert_eq!(first(&s, a).delay_ms, 0);
    s.execute("edit.undo", &json!({})).unwrap();
    assert_eq!((first(&s, a).trigger, first(&s, a).delay_ms), (Trigger::MouseEnter, 450));
    // Stored without the field when there is none, so older files read the same.
    let json = serde_json::to_value(first(&s, a)).unwrap();
    assert_eq!(json["delay_ms"], 450);
    s.execute("proto.interaction.set", &json!({"id": a, "index": 0, "delayMs": 0})).unwrap();
    assert!(serde_json::to_value(first(&s, a)).unwrap().get("delay_ms").is_none());
}

#[test]
fn an_offset_belongs_to_a_manual_overlay() {
    let mut s = Session::new();
    let (a, menu) = (frame(&mut s, 0.0), frame(&mut s, 200.0));
    let open = |s: &mut Session, position: &str, offset: Value| {
        s.execute("proto.interaction.clear", &json!({"ids": [a]})).unwrap();
        let action = json!({"type": "openOverlay", "dest": menu, "position": position, "offset": offset});
        s.execute("proto.interaction.add", &json!({"id": a, "action": action})).unwrap();
        match first(s, a).actions.remove(0) {
            Action::OpenOverlay { offset, .. } => offset,
            other => panic!("{other:?}"),
        }
    };
    assert_eq!(open(&mut s, "manual", json!([12, -8])), Some([12.0, -8.0]));
    assert_eq!(open(&mut s, "manual", json!(null)), None, "none: right below the layer that opens it");
    assert_eq!(open(&mut s, "manual", json!([1e12, -1e12])), Some([1e5, -1e5]), "kept within reach");
    assert_eq!(open(&mut s, "center", json!([12, -8])), None, "only a manual overlay has one");
    // A file from before the field reads as no offset.
    let old: Action = serde_json::from_value(json!({"type": "openOverlay", "dest": menu, "position": "manual"})).unwrap();
    assert!(matches!(old, Action::OpenOverlay { offset: None, position: OverlayPosition::Manual, .. }));
}

fn rest(trigger: Value, action: Value) -> Document {
    let at = |x: f64| json!([[1, 0, x], [0, 1, 0]]);
    let file = json!({"name": "File", "document": {"id": "0:0", "type": "DOCUMENT", "children": [{"id": "0:1", "name": "Page", "type": "CANVAS", "children": [
        {"id": "1:1", "name": "A", "type": "FRAME", "relativeTransform": at(0.0), "size": {"x": 100, "y": 100}, "children": [
            {"id": "1:2", "name": "Hot", "type": "RECTANGLE", "relativeTransform": at(10.0), "size": {"x": 20, "y": 20},
             "interactions": [{"trigger": trigger, "actions": [action]}]}]},
        {"id": "1:3", "name": "Menu", "type": "FRAME", "relativeTransform": at(200.0), "size": {"x": 50, "y": 50}, "overlayPositionType": "MANUAL", "children": []}
    ]}]}});
    let (doc, warnings) = figcraft_format::rest::import(&file, &figcraft_format::rest::no_images).unwrap();
    assert!(warnings.iter().all(|w| !w.contains("delay")), "{warnings:?}");
    doc
}

fn hot(doc: &Document) -> Interaction {
    doc.nodes().find(|n| n.name == "Hot").unwrap().interactions[0].clone()
}

#[test]
fn rest_and_fig_carry_delays_and_offsets() {
    let overlay = json!({"type": "NODE", "destinationId": "1:3", "navigation": "OVERLAY", "overlayRelativePosition": {"x": 12, "y": -8}});
    let doc = rest(json!({"type": "MOUSE_ENTER", "delay": 300}), overlay.clone());
    let it = hot(&doc);
    assert_eq!((it.trigger.clone(), it.delay_ms), (Trigger::MouseEnter, 300));
    assert!(matches!(it.actions[0], Action::OpenOverlay { position: OverlayPosition::Manual, offset: Some([12.0, -8.0]), .. }), "{it:?}");
    // A click has no delay to read; a manual overlay without a position sits below its layer.
    let plain = hot(&rest(json!({"type": "ON_CLICK", "delay": 300}), json!({"type": "NODE", "destinationId": "1:3", "navigation": "OVERLAY"})));
    assert_eq!(plain.delay_ms, 0);
    assert!(matches!(plain.actions[0], Action::OpenOverlay { offset: None, .. }));
    // Junk reads as nothing.
    let junk = hot(&rest(
        json!({"type": "MOUSE_UP", "delay": "soon"}),
        json!({"type": "NODE", "destinationId": "1:3", "navigation": "OVERLAY", "overlayRelativePosition": {"x": "a"}}),
    ));
    assert_eq!(junk.delay_ms, 0);
    assert!(matches!(junk.actions[0], Action::OpenOverlay { offset: None, .. }));
    // Written to `.fig` and read back.
    let (bytes, _) = figcraft_format::fig::export(&doc, None).unwrap();
    let (back, _) = figcraft_format::fig::import(&bytes, "x").unwrap();
    let again = hot(&back);
    assert_eq!((again.trigger, again.delay_ms), (Trigger::MouseEnter, 300));
    assert!(
        matches!(again.actions[0], Action::OpenOverlay { position: OverlayPosition::Manual, offset: Some([12.0, -8.0]), .. }),
        "{:?}",
        again.actions
    );
}

#[test]
fn junk_values_are_refused_not_fatal() {
    let junk = [json!(true), json!(-1), json!("soon"), json!([1, 2]), json!({"a": 1}), json!(f64::MAX), json!("x".repeat(10_000))];
    for j in &junk {
        let mut s = Session::new();
        let (a, b) = (frame(&mut s, 0.0), frame(&mut s, 200.0));
        let nav = json!({"type": "navigate", "dest": b});
        let adds = [
            json!({"id": a, "trigger": {"type": "mouseEnter"}, "action": nav, "delayMs": j}),
            json!({"id": a, "action": {"type": "openOverlay", "dest": b, "position": "manual", "offset": j}}),
            json!({"id": a, "action": {"type": "openOverlay", "dest": b, "position": "manual", "offset": [j, j]}}),
        ];
        for p in adds {
            let r = s.execute("proto.interaction.add", &p);
            assert!(!matches!(r, Err(EngineError::Internal { .. })), "{p}: {r:?}");
        }
        s.execute("proto.interaction.add", &json!({"id": b, "trigger": {"type": "mouseUp"}, "action": {"type": "back"}})).unwrap();
        let r = s.execute("proto.interaction.set", &json!({"id": b, "index": 0, "delayMs": j}));
        assert!(!matches!(r, Err(EngineError::Internal { .. })), "set {j}: {r:?}");
        for n in [a, b] {
            for it in &s.doc().unwrap().node(n).unwrap().interactions {
                assert!(it.delay_ms <= 600_000, "{j}: {it:?}");
                if let Action::OpenOverlay { offset: Some(o), .. } = &it.actions[0] {
                    assert!(o.iter().all(|c| c.is_finite() && c.abs() <= 1e5), "{j}: {o:?}");
                }
            }
        }
    }
}
