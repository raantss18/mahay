//! The prototyping tail, written from the public documentation: controller buttons on key/gamepad
//! triggers, widgets reacting to clicks in a presented prototype, and variable mode lookups in
//! expressions. Each case states the rule and a one-line derivation; where the documentation is
//! silent the case says so and pins what FigCraft does. The player's side (injected controller
//! presses, clicks on a widget in the presenter) is `ui-egui`'s `present_tail_tests`.

use figcraft_doc::proto::{Action, GAMEPAD_BUTTONS, KeyMods, Trigger, gamepad_button, uses_gamepad};
use figcraft_doc::{NodeId, VariableValue};
use serde_json::{Value, json};

use crate::Session;
use crate::params::id_of;
use crate::proto_run::{branch, set_mode, set_variable};
use crate::widgets::click_on;

fn ex(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd} {p}: {e}"))
}

fn idv(v: &Value) -> NodeId {
    id_of(&v["id"]).unwrap()
}

#[test]
fn controller_buttons_fire_key_gamepad_triggers() {
    let none = KeyMods::default();
    let shift = KeyMods { shift: true, ..none };
    // (the trigger's key, key or button pressed, modifiers held, fires, derivation)
    let cases = [
        ("Gamepad A", "Gamepad A", none, true, "the trigger takes a controller button as its key"),
        ("gamepad a", "Gamepad A", none, true, "names in any case, like keys"),
        ("Gamepad A", "Gamepad B", none, false, "another button"),
        ("Gamepad A", "A", none, false, "the keyboard's A is not the controller's A"),
        ("A", "Gamepad A", none, false, "nor the other way round"),
        ("Gamepad Up", "ArrowUp", none, false, "the d-pad is not the arrow keys (docs silent; pinned)"),
        (
            "Gamepad A",
            "Gamepad A",
            shift,
            false,
            "a controller button is pressed alone: a keyboard modifier held makes it another input (docs silent; pinned)",
        ),
    ];
    for (want, key, mods, fires, why) in cases {
        let t = Trigger::KeyPress { key: want.into() };
        assert_eq!(t.fires_on_key(key, mods), fires, "{want:?} vs {mods:?}+{key}: {why}");
    }
    // The standard layout's buttons, each with its own name.
    assert_eq!(gamepad_button(0), Some("Gamepad A"));
    assert_eq!(gamepad_button(16), Some("Gamepad Home"));
    assert_eq!(gamepad_button(17), None);
    let mut names: Vec<&str> = GAMEPAD_BUTTONS.to_vec();
    names.dedup();
    assert_eq!(names.len(), GAMEPAD_BUTTONS.len());
    // Authored through the interaction commands (and so the MCP server), stored as the key.
    let mut s = Session::new();
    let a = idv(&ex(&mut s, "frame.create", json!({"x": 0, "y": 0, "width": 100, "height": 100})));
    assert!(!uses_gamepad(s.doc().unwrap()));
    ex(&mut s, "proto.interaction.add", json!({"id": a, "trigger": {"type": "keyPress", "key": "Gamepad Menu"}, "action": {"type": "back"}}));
    let doc = s.doc().unwrap();
    assert_eq!(doc.node(a).unwrap().interactions[0].trigger, Trigger::KeyPress { key: "Gamepad Menu".into() });
    assert!(uses_gamepad(doc), "the player polls controllers only for prototypes that use them");
}

#[test]
fn widgets_react_to_clicks_on_the_prototypes_own_copy() {
    let mut s = crate::widgets_tests::session();
    let (w, _) = crate::widgets_tests::insert(&mut s);
    let editor = s.doc().unwrap().clone();
    let undo = crate::plugins_tests::undo_len(&s);
    let state = |d: &figcraft_doc::Document| d.node(w).and_then(|n| n.plugin_data.get("figcraft.widget.state").cloned()).unwrap_or_default();
    // Clicking calls the plugin back with the prototype's state of the widget, which it redraws.
    let (copy, out) = click_on(&s, editor.clone(), w, "inc").unwrap();
    assert_eq!(out["state"], json!({"n": 2}));
    assert_eq!(out["targets"], json!(["inc", "dec"]));
    assert_eq!(out["output"]["widget"]["event"], json!({"target": "inc"}), "the same callback as on the canvas");
    assert_eq!(state(&copy), r#"{"n":2}"#);
    // The editor's document and its history don't change: the state is the prototype's.
    assert_eq!(state(s.doc().unwrap()), r#"{"n":1}"#);
    assert_eq!(crate::plugins_tests::undo_len(&s), undo);
    // Later clicks start from the copy's state (a target only the redrawn widget has).
    let (copy, out) = click_on(&s, copy, w, "dec").unwrap();
    assert_eq!(out["state"], json!({"n": 0}));
    // Refusals leave the copy with the caller.
    let e = click_on(&s, copy.clone(), w, "nope").unwrap_err().to_string();
    assert!(e.contains("it has: inc"), "{e}");
    let rect = idv(&ex(&mut s, "shape.rectangle", json!({"x": 400, "y": 0, "width": 10, "height": 10})));
    assert!(click_on(&s, s.doc().unwrap().clone(), rect, "inc").is_err(), "not a widget");
    ex(&mut s, "plugin.remove", json!({"id": "org.test.tally"}));
    assert!(click_on(&s, copy, w, "inc").unwrap_err().to_string().contains("no plugin"), "without its plugin the widget stays as it is");
}

/// Screen A (a number `size`: 10 in Light, 20 in Dark; `label`: "light" / "dark"), its button, and a
/// card inside A that chose Dark.
fn modes_session() -> (Session, NodeId, NodeId, NodeId) {
    let mut s = Session::new();
    let a = idv(&ex(&mut s, "frame.create", json!({"x": 0, "y": 0, "width": 200, "height": 200, "name": "A"})));
    let button = idv(&ex(&mut s, "shape.rectangle", json!({"x": 10, "y": 10, "width": 30, "height": 20, "parent": a})));
    let card = idv(&ex(&mut s, "frame.create", json!({"x": 50, "y": 50, "width": 100, "height": 100, "parent": a, "name": "Card"})));
    let inner = idv(&ex(&mut s, "shape.rectangle", json!({"x": 60, "y": 60, "width": 10, "height": 10, "parent": card})));
    ex(&mut s, "variable.createCollection", json!({"name": "Theme", "modes": ["Light", "Dark"]}));
    ex(&mut s, "variable.create", json!({"collection": "Theme", "name": "size", "type": "number", "value": 10}));
    ex(&mut s, "variable.set", json!({"variable": "size", "mode": "Dark", "value": 20}));
    ex(&mut s, "variable.create", json!({"collection": "Theme", "name": "label", "type": "string", "value": "light"}));
    ex(&mut s, "variable.set", json!({"variable": "label", "mode": "Dark", "value": "dark"}));
    ex(&mut s, "variable.setMode", json!({"ids": [card], "collection": "Theme", "mode": "Dark"}));
    (s, button, inner, a)
}

fn eval(s: &mut Session, text: &str, at: NodeId) -> Value {
    ex(s, "proto.evaluate", json!({"expression": text, "id": at}))["value"].clone()
}

#[test]
fn mode_lookups_read_a_given_mode_or_the_layers_current_one() {
    let (mut s, button, inner, _) = modes_session();
    let num = |n: f64| json!({"type": "number", "value": n});
    let text = |t: &str| json!({"type": "string", "value": t});
    // (expression, layer, value, derivation)
    let cases = [
        ("size", button, num(10.0), "a plain reference reads the mode where the interaction runs: the screen is on the default"),
        ("size", inner, num(20.0), "inside a frame that chose Dark it reads Dark"),
        ("size[Dark]", button, num(20.0), "a lookup reads the mode it names, whatever mode is chosen"),
        ("size[Light]", inner, num(10.0), "also where another mode is chosen"),
        ("mode(Theme)", inner, text("Dark"), "the current mode by name"),
        ("size[Dark] - size", button, num(10.0), "lookups mix with operators"),
        ("label + \" mode\"", inner, text("dark mode"), "text joins with +"),
        ("size * 2 >= 40 and not (mode(Theme) == \"Light\")", inner, json!({"type": "boolean", "value": true}), "comparisons and boolean operators"),
        ("size / 4", button, num(2.5), "division gives fractions"),
        ("-size[Dark]", button, num(-20.0), "negation"),
    ];
    for (expr, at, want, why) in cases {
        assert_eq!(eval(&mut s, expr, at), want, "{expr} on {at}: {why}");
    }
    // Without a layer: the default modes.
    assert_eq!(ex(&mut s, "proto.evaluate", json!({"expression": "size"}))["value"], num(10.0));
    let r = ex(&mut s, "proto.evaluate", json!({"expression": "size [ Dark ]"}));
    assert_eq!((r["type"].clone(), r["text"].clone()), (json!("number"), json!("size[Dark]")));
    // Mistakes are errors, not values; nothing is changed.
    let rev = s.ds().unwrap().revision;
    for (bad, why) in [
        ("size[Sepia]", "Theme has no mode `Sepia`"),
        ("size / 0", "gives no value"),
        ("size +", "missing at the end"),
        ("label - 1", "needs numbers"),
    ] {
        let e = s.execute("proto.evaluate", &json!({"expression": bad, "id": button})).unwrap_err().to_string();
        assert!(e.contains(why), "{bad}: {e}");
    }
    assert!(s.execute("proto.evaluate", &json!({"expression": "size", "id": 99999})).is_err());
    assert!(s.execute("proto.evaluate", &json!({})).is_err());
    assert_eq!(s.ds().unwrap().revision, rev);
}

#[test]
fn set_variable_mode_changes_what_later_expressions_read() {
    let (s, button, inner, a) = modes_session();
    let mut doc = s.doc().unwrap().clone();
    let size = doc.variables.get("size").unwrap().id.clone();
    let cond = |text: &str, d: &figcraft_doc::Document| {
        let e = figcraft_doc::proto::Expr::parse(text, &d.variables).unwrap();
        vec![figcraft_doc::proto::ConditionalBlock { condition: Some(e), actions: vec![Action::Back] }]
    };
    assert!(branch(&doc, button, &cond("mode(Theme) == \"Dark\"", &doc)).is_empty(), "Light before the action");
    // The action switches the screen's mode for the prototype: a conditional after it sees Dark.
    assert!(set_mode(&mut doc, a, "Theme", "Dark"));
    assert_eq!(branch(&doc, button, &cond("mode(Theme) == \"Dark\"", &doc)), vec![Action::Back]);
    assert_eq!(branch(&doc, button, &cond("size == 20", &doc)), vec![Action::Back], "plain references follow the new mode");
    // A set variable action computes its expression in the new mode.
    let e = figcraft_doc::proto::Expr::parse("size[Light] + size", &doc.variables).unwrap();
    assert!(set_variable(&mut doc, button, &size, &Value::Null, Some(&e)));
    let resolve = |d: &figcraft_doc::Document, mode: &str| {
        let m = std::collections::BTreeMap::from([(
            d.variables.collection("Theme").unwrap().id.clone(),
            d.variables.collection("Theme").unwrap().find_mode(mode).unwrap().id.clone(),
        )]);
        d.variables.resolve(&size, &m)
    };
    // Docs silent: a set variable writes every mode (pinned), so lookups of any mode read it.
    assert_eq!(resolve(&doc, "Light"), Some(VariableValue::Number(30.0)));
    assert_eq!(resolve(&doc, "Dark"), Some(VariableValue::Number(30.0)));
    // Back to Light; a frame inside the screen that chose a mode keeps it (docs silent; pinned).
    assert!(set_mode(&mut doc, a, "Theme", "Light"));
    assert_eq!(branch(&doc, inner, &cond("mode(Theme) == \"Dark\"", &doc)), vec![Action::Back]);
    assert!(!set_mode(&mut doc, a, "Theme", "Sepia"), "an unknown mode changes nothing");
}
