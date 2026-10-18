//! REST import of key/gamepad triggers on controllers. Fixtures are synthetic, written by hand
//! from the published schema (`KeyTrigger`: `device`, `keyCodes`).

use super::proto_variable_tests::at;
use super::*;
use figcraft_doc::proto::{GAMEPAD_BUTTONS, Trigger};
use serde_json::json;

fn import_trigger(trigger: Value) -> (Option<Trigger>, Vec<String>) {
    let file = json!({"name": "Pad", "document": {"id": "0:0", "type": "DOCUMENT", "children": [
        {"id": "0:1", "name": "Page", "type": "CANVAS", "children": [
            {"id": "1:1", "name": "Home", "type": "FRAME", "relativeTransform": at(0.0, 0.0), "size": {"x": 200, "y": 200}, "children": [
                {"id": "1:2", "name": "Button", "type": "RECTANGLE", "relativeTransform": at(10.0, 10.0), "size": {"x": 40, "y": 20},
                 "interactions": [{"trigger": trigger, "actions": [{"type": "BACK"}]}]}
            ]}
        ]}
    ]}});
    let (doc, warnings) = import_with(&file, None, &no_images).unwrap();
    let t = doc.nodes().find(|n| n.name == "Button").and_then(|n| n.interactions.first()).map(|it| it.trigger.clone());
    (t, warnings)
}

#[test]
fn controller_triggers_import_as_their_button() {
    let key = |k: &str| Some(Trigger::KeyPress { key: k.into() });
    // (trigger JSON, imported trigger, derivation)
    let cases = [
        (json!({"type": "ON_KEY_DOWN", "device": "XBOX_ONE", "keyCodes": [0]}), key("Gamepad A"), "button 0 is the bottom face button"),
        (
            json!({"type": "ON_KEY_DOWN", "device": "PS4", "keyCodes": [3]}),
            key("Gamepad Y"),
            "the same layout on any controller: 3 is the top face button",
        ),
        (json!({"type": "ON_KEY_DOWN", "device": "SWITCH_PRO", "keyCodes": [12]}), key("Gamepad Up"), "the d-pad"),
        (json!({"type": "ON_KEY_DOWN", "device": "UNKNOWN_CONTROLLER", "keyCodes": [16]}), key("Gamepad Home"), "the last standard button"),
        (json!({"type": "ON_KEY_UP", "device": "XBOX_ONE", "keyCodes": [9]}), key("Gamepad Menu"), "key up imports as a press, like keyboard keys"),
        (json!({"type": "ON_KEY_DOWN", "device": "XBOX_ONE", "keyCodes": [17]}), None, "beyond the standard layout: skipped"),
        (json!({"type": "ON_KEY_DOWN", "device": "XBOX_ONE", "keyCodes": [0, 1]}), None, "two buttons at once (docs silent): skipped, not guessed"),
        (json!({"type": "ON_KEY_DOWN", "device": "XBOX_ONE", "keyCodes": [1.5]}), None, "not an index"),
        (json!({"type": "ON_KEY_DOWN", "device": "XBOX_ONE", "keyCodes": [-1]}), None, "negative"),
        (json!({"type": "ON_KEY_DOWN", "device": "XBOX_ONE"}), None, "no button"),
        (json!({"type": "ON_KEY_DOWN", "device": "KEYBOARD", "keyCodes": [65]}), key("A"), "keyboards are unchanged"),
    ];
    for (trigger, want, why) in cases {
        let (got, warnings) = import_trigger(trigger.clone());
        assert_eq!(got, want, "{trigger}: {why} ({warnings:?})");
        assert_eq!(got.is_none(), warnings.iter().any(|w| w.contains("gamepad triggers without")), "{trigger}: a skip says why");
        assert!(!warnings.iter().any(|w| w.contains("not supported")), "{trigger}: {warnings:?}");
    }
    assert_eq!(GAMEPAD_BUTTONS.len(), 17, "the standard layout's buttons");
}

#[test]
fn junk_controller_triggers_never_panic() {
    for codes in [json!(null), json!("0"), json!([null]), json!([1e300]), json!([f64::MAX]), json!([i64::MIN]), json!({"0": 1})] {
        for device in [json!("XBOX_ONE"), json!(""), json!(7), json!(null)] {
            let _ = import_trigger(json!({"type": "ON_KEY_DOWN", "device": device, "keyCodes": codes}));
        }
    }
}
