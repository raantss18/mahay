//! Plugin panels and settings (ABI v2): a panel round trip with state kept between callbacks and
//! one undo step per callback that edits, settings that persist and are capped, junk panels and
//! values refused, a callback that runs out of fuel rolled back, closing ends the session. Test
//! plugins are WAT that do step `k` on their `k`-th call and output their input.

use serde_json::{Value, json};

use crate::components::tests::ex;
use crate::plugins_tests::{install, layer_count, lit, manifest, rect, undo_len};
use crate::{EngineError, Session};

/// Lays out a test plugin's strings and builds its module.
#[derive(Default)]
struct Wat {
    data: String,
    at: usize,
}

/// Where a refusal is copied to, and where the input is kept.
const REFUSAL: usize = 180_000;
const INPUT: usize = 200_000;

impl Wat {
    /// `text` in the module's memory: (address, length).
    fn s(&mut self, text: &str) -> (usize, usize) {
        let a = 1000 + self.at;
        self.data += &format!("(data (i32.const {a}) \"{}\")\n", lit(text));
        self.at += text.len();
        (a, text.len())
    }

    /// Output the reply (a refusal) and stop.
    fn refused() -> String {
        format!("(call $set_output (i32.const {REFUSAL}) (call $reply_read (i32.const {REFUSAL}) (i32.const 20000))) (return (i32.const 0))")
    }

    fn show(&mut self, spec: &Value) -> String {
        let (a, l) = self.s(&spec.to_string());
        format!("(if (call $ui_show (i32.const {a}) (i32.const {l})) (then {}))", Self::refused())
    }

    fn call(&mut self, cmd: &str, params: &Value) -> String {
        let ((a, l), (b, m)) = (self.s(cmd), self.s(&params.to_string()));
        format!(
            "(if (call $call (i32.const {a}) (i32.const {l}) (i32.const {b}) (i32.const {m})) (then {} ))",
            Self::refused().replace("(i32.const 0))", "(i32.const 1))")
        )
    }

    /// Set `key` to `json`; a refusal is the output.
    fn set(&mut self, key: &str, json: &str) -> String {
        let ((a, l), (b, m)) = (self.s(key), self.s(json));
        format!("(if (call $setting_set (i32.const {a}) (i32.const {l}) (i32.const {b}) (i32.const {m})) (then {}))", Self::refused())
    }

    /// When `key` is set, output its value and stop.
    fn get(&mut self, key: &str) -> String {
        let (a, l) = self.s(key);
        format!("(if (i32.eqz (call $setting_get (i32.const {a}) (i32.const {l}))) (then {}))", Self::refused())
    }

    fn build(self, steps: &[String]) -> Vec<u8> {
        let body: String =
            steps.iter().enumerate().map(|(i, s)| format!("(if (i32.eq (local.get $k) (i32.const {})) (then {s}))\n", i + 1)).collect();
        let wat = format!(
            r#"(module
              (import "figcraft" "call" (func $call (param i32 i32 i32 i32) (result i32)))
              (import "figcraft" "reply_read" (func $reply_read (param i32 i32) (result i32)))
              (import "figcraft" "set_output" (func $set_output (param i32 i32)))
              (import "figcraft" "ui_show" (func $ui_show (param i32 i32) (result i32)))
              (import "figcraft" "ui_close" (func $ui_close))
              (import "figcraft" "setting_get" (func $setting_get (param i32 i32) (result i32)))
              (import "figcraft" "setting_set" (func $setting_set (param i32 i32 i32 i32) (result i32)))
              (memory (export "memory") 4)
              {}
              (func (export "figcraft_run") (param $n i32) (result i32) (local $k i32)
                (drop (call $reply_read (i32.const {INPUT}) (local.get $n)))
                (i32.store (i32.const 0) (i32.add (i32.load (i32.const 0)) (i32.const 1)))
                (local.set $k (i32.load (i32.const 0)))
                {body}
                (call $set_output (i32.const {INPUT}) (local.get $n))
                (i32.const 0)))"#,
            self.data
        );
        wat::parse_str(&wat).unwrap_or_else(|e| panic!("{e}\n{wat}"))
    }
}

fn v2(id: &str) -> Value {
    let mut m = manifest(id, "write", &[]);
    m["api"] = json!(2);
    m
}

fn panel(title: &str) -> Value {
    json!({"title": title, "controls": [
        {"type": "note", "label": "Pick a size"},
        {"type": "number", "name": "n", "label": "Size", "value": 10, "min": 1, "max": 100},
        {"type": "list", "name": "palette", "options": ["Warm", "Cool"]},
        {"type": "color", "name": "accent", "value": "#FF5A5F"},
        {"type": "button", "name": "go", "label": "Go"}
    ]})
}

fn ui(s: &mut Session) -> Value {
    ex(s, "plugin.ui", json!({}))
}

fn event_err(s: &mut Session, p: Value) -> String {
    match s.execute("plugin.event", &p) {
        Ok(v) => panic!("accepted {p}: {v}"),
        Err(e) => {
            assert!(!matches!(e, EngineError::Internal { .. }), "{e}");
            e.to_string()
        }
    }
}

#[test]
fn a_panel_round_trip_keeps_state_and_edits_are_one_undo_step_per_callback() {
    let mut s = Session::new();
    let mut w = Wat::default();
    let steps = [
        w.show(&panel("Sizes")),
        String::new(),
        format!(
            "{} {}",
            w.call("shape.rectangle", &json!({"x": 0, "y": 0, "width": 10, "height": 10})),
            w.show(&json!({"title": "Done", "controls": [{"type": "number", "name": "n"}]}))
        ),
        "(call $ui_close)".into(),
    ];
    install(&mut s, v2("org.test.panel"), &w.build(&steps));
    let out = ex(&mut s, "plugin.run", json!({"id": "org.test.panel"}));
    assert_eq!(out["output"]["api"], 2);
    assert_eq!(out["panel"]["title"], "Sizes");
    assert_eq!(out["panel"]["values"], json!({"n": 10.0, "palette": "Warm", "accent": "#FF5A5F"}));
    assert_eq!(ui(&mut s), out["panel"]);
    assert_eq!(undo_len(&s), 0);
    // A changed value calls back with the event and every value; nothing edited, no undo step.
    let out = ex(&mut s, "plugin.event", json!({"id": "org.test.panel", "control": "n", "value": "250"}));
    assert_eq!(out["output"]["event"], json!({"control": "n", "value": 100.0}), "coerced like a parameter");
    assert_eq!(out["output"]["values"]["n"], 100.0);
    assert_eq!(out["output"]["command"], "run");
    assert_eq!(undo_len(&s), 0);
    // The third call (the module remembers it is the third) edits and updates the panel.
    let layers = layer_count(&s);
    let out = ex(&mut s, "plugin.event", json!({"id": "org.test.panel", "control": "go"}));
    assert_eq!(out["output"]["event"], json!({"control": "go", "value": null}));
    assert_eq!(layer_count(&s), layers + 1);
    assert_eq!(undo_len(&s), 1, "one undo step for the callback");
    assert_eq!(s.ds().unwrap().history.undo.last().unwrap().label, "Plugin org.test.panel");
    assert_eq!(out["panel"]["title"], "Done");
    assert_eq!(out["panel"]["values"], json!({"n": 100.0}), "a value entered before the update is kept");
    ex(&mut s, "edit.undo", json!({}));
    assert_eq!(layer_count(&s), layers);
    // The fourth closes it.
    let out = ex(&mut s, "plugin.event", json!({"id": "org.test.panel", "control": "n", "value": 5}));
    assert_eq!(out["panel"], Value::Null);
    assert_eq!(ui(&mut s), Value::Null);
    assert!(event_err(&mut s, json!({"id": "org.test.panel", "control": "go"})).contains("no panel open"));
}

#[test]
fn v1_plugins_get_the_input_they_always_got() {
    let mut s = Session::new();
    let w = Wat::default();
    let wasm = w.build(&[]);
    install(&mut s, manifest("org.test.v1", "read", &[]), &wasm);
    let out = ex(&mut s, "plugin.run", json!({"id": "org.test.v1"}));
    assert_eq!(out["output"], json!({"api": 1, "command": "run", "params": {}, "selection": []}));
    assert_eq!(out["panel"], Value::Null);
}

#[test]
fn settings_persist_across_runs_per_plugin_and_are_capped() {
    let mut s = Session::new();
    let mut w = Wat::default();
    let steps = [format!("{} {}", w.get("palette"), w.set("palette", r#"{"name": "Ocean"}"#))];
    let wasm = w.build(&steps);
    install(&mut s, v2("org.test.a"), &wasm);
    install(&mut s, v2("org.test.b"), &wasm);
    assert!(std::mem::take(&mut s.plugins.unsaved), "installing asks the app to save the preferences at once");
    assert_eq!(ex(&mut s, "plugin.run", json!({"id": "org.test.a"}))["output"]["command"], "run", "nothing stored yet: it stores");
    assert!(std::mem::take(&mut s.plugins.unsaved), "a changed setting is saved at once too");
    assert_eq!(ex(&mut s, "plugin.run", json!({"id": "org.test.a"}))["output"], json!({"name": "Ocean"}), "the next run reads it back");
    assert!(!s.plugins.unsaved, "storing the same value again changes nothing");
    assert_eq!(ex(&mut s, "plugin.run", json!({"id": "org.test.b"}))["output"]["command"], "run", "plugins don't see each other's settings");
    // Saved with the preferences and read back.
    let saved = serde_json::to_value(&s.prefs).unwrap();
    assert_eq!(saved["plugin_settings"]["org.test.a"]["palette"], json!({"name": "Ocean"}));
    let back: crate::Prefs = serde_json::from_value(saved).unwrap();
    assert_eq!(back.plugin_settings, s.prefs.plugin_settings);
    // Removing a plugin forgets its settings.
    ex(&mut s, "plugin.remove", json!({"id": "org.test.b"}));
    assert!(!s.prefs.plugin_settings.contains_key("org.test.b"));
    assert!(s.plugins.unsaved);

    // 64 KB per plugin; keys and values are checked.
    let big = format!("\"{}\"", "x".repeat(30_000));
    let mut w = Wat::default();
    let steps = [format!("{} {} {}", w.set("one", &big), w.set("two", &big), w.set("three", &big))];
    install(&mut s, v2("org.test.hoard"), &w.build(&steps));
    let out = ex(&mut s, "plugin.run", json!({"id": "org.test.hoard"}));
    assert!(out["output"].as_str().unwrap_or_default().contains("limited to 64 KB"), "{out}");
    assert_eq!(s.prefs.plugin_settings["org.test.hoard"].len(), 2);
    for (key, value) in [("bad\nkey", "1"), ("", "1"), ("k", "not json"), ("k", &"[".repeat(1000))] {
        let mut w = Wat::default();
        let steps = [w.set(key, value)];
        install(&mut s, v2("org.test.junk"), &w.build(&steps));
        let out = ex(&mut s, "plugin.run", json!({"id": "org.test.junk"}));
        assert!(out["output"].is_string(), "{key:?} = {value}: {out}");
        assert!(!s.prefs.plugin_settings.contains_key("org.test.junk"), "{key:?}");
    }
}

#[test]
fn junk_panels_and_values_are_refused() {
    let mut s = Session::new();
    // A spec over the size cap, deep junk and unknown kinds are refused to the plugin; no panel.
    let huge = json!({"controls": [{"type": "note", "label": "x".repeat(70_000)}]});
    let deep: Value = serde_json::from_str(&format!("{}1{}", "[".repeat(100), "]".repeat(100))).unwrap();
    for spec in [
        huge,
        json!({"controls": [{"type": "number", "name": "n", "value": deep}], "title": 5}),
        json!({"controls": [{"type": "iframe", "name": "x"}]}),
    ] {
        let mut w = Wat::default();
        let steps = [w.show(&spec)];
        install(&mut s, v2("org.test.junk"), &w.build(&steps));
        let out = ex(&mut s, "plugin.run", json!({"id": "org.test.junk"}));
        assert!(out["output"].as_str().unwrap_or_default().contains("panel"), "{out}");
        assert_eq!(ui(&mut s), Value::Null);
    }
    // Junk events never reach the plugin.
    let mut w = Wat::default();
    let steps = [w.show(&panel("P"))];
    install(&mut s, v2("org.test.p"), &w.build(&steps));
    ex(&mut s, "plugin.run", json!({"id": "org.test.p"}));
    let before = ui(&mut s);
    for bad in [
        json!({"id": "org.test.p", "control": "n", "value": "many"}),
        json!({"id": "org.test.p", "control": "n"}),
        json!({"id": "org.test.p", "control": "palette", "value": "Neon"}),
        json!({"id": "org.test.p", "control": "accent", "value": "#12345"}),
        json!({"id": "org.test.p", "control": "accent", "value": {"r": 1}}),
        json!({"id": "org.test.p", "control": "", "value": 1}),
        json!({"id": "org.test.p", "control": "nope"}),
        json!({"id": "org.test.other", "control": "go"}),
        json!({"id": "org.test.p"}),
        json!({"control": "go"}),
        json!(null),
        json!([1]),
    ] {
        event_err(&mut s, bad);
    }
    assert_eq!(ui(&mut s), before, "the panel is as it was");
    for cmd in ["plugin.ui", "plugin.close"] {
        for j in [json!(null), json!([1]), json!({"id": 5})] {
            if let Err(EngineError::Internal { msg, .. }) = s.execute(cmd, &j) {
                panic!("{cmd} panicked on {j}: {msg}");
            }
        }
    }
}

#[test]
fn a_callback_out_of_fuel_rolls_back_and_closes_the_panel() {
    let mut s = Session::new();
    let mut w = Wat::default();
    let edit = w.call("shape.rectangle", &json!({"x": 0, "y": 0, "width": 10, "height": 10}));
    let steps = [w.show(&panel("P")), format!("{edit} (return (i32.const 1))"), format!("{edit} (loop $l (br $l))")];
    install(&mut s, v2("org.test.loop"), &w.build(&steps));
    // The run's lower budget holds for its callbacks too.
    ex(&mut s, "plugin.run", json!({"id": "org.test.loop", "fuel": 2_000_000}));
    let (layers, undo) = (layer_count(&s), undo_len(&s));
    // A plugin that returns an error is rolled back and keeps its panel.
    event_err(&mut s, json!({"id": "org.test.loop", "control": "go"}));
    assert_eq!((layer_count(&s), undo_len(&s)), (layers, undo));
    assert_ne!(ui(&mut s), Value::Null);
    // One that runs out of fuel is rolled back and loses it.
    let e = event_err(&mut s, json!({"id": "org.test.loop", "control": "go"}));
    assert!(e.contains("instruction budget"), "{e}");
    assert_eq!((layer_count(&s), undo_len(&s)), (layers, undo));
    assert_eq!(ui(&mut s), Value::Null);
}

#[test]
fn closing_or_removing_ends_the_session_and_panels_open_side_by_side() {
    let mut s = Session::new();
    rect(&mut s, 0.0);
    let mut w = Wat::default();
    let steps = [w.show(&panel("P"))];
    let wasm = w.build(&steps);
    install(&mut s, v2("org.test.a"), &wasm);
    install(&mut s, v2("org.test.b"), &wasm);
    ex(&mut s, "plugin.run", json!({"id": "org.test.a"}));
    assert_eq!(ex(&mut s, "plugin.close", json!({"id": "org.test.b"}))["closed"], false, "only its own");
    assert_eq!(ex(&mut s, "plugin.close", json!({"id": "org.test.a"}))["closed"], true);
    assert_eq!(ui(&mut s), Value::Null);
    assert!(event_err(&mut s, json!({"id": "org.test.a", "control": "go"})).contains("no panel open"));
    // Two plugins' panels are open side by side (a plugin's own run replaces its panel);
    // removing a plugin closes its panel only.
    ex(&mut s, "plugin.run", json!({"id": "org.test.a"}));
    ex(&mut s, "plugin.run", json!({"id": "org.test.b"}));
    ex(&mut s, "plugin.run", json!({"id": "org.test.a"}));
    assert_eq!(s.plugins.panels.len(), 2);
    assert_eq!(ui(&mut s)["plugin"], "org.test.a", "without an id: the one opened last");
    assert_eq!(ex(&mut s, "plugin.ui", json!({"id": "org.test.b"}))["plugin"], "org.test.b");
    ex(&mut s, "plugin.event", json!({"id": "org.test.b", "control": "n", "value": 7}));
    assert_eq!(ex(&mut s, "plugin.ui", json!({"id": "org.test.b"}))["values"]["n"], 7.0, "the event reached b's panel");
    assert_eq!(ui(&mut s)["plugin"], "org.test.a", "and b's panel kept its place");
    ex(&mut s, "plugin.remove", json!({"id": "org.test.b"}));
    assert_eq!(ex(&mut s, "plugin.ui", json!({"id": "org.test.b"})), Value::Null);
    assert_eq!(ui(&mut s)["plugin"], "org.test.a");
    ex(&mut s, "plugin.close", json!({"id": "org.test.a"}));
    assert_eq!(ui(&mut s), Value::Null);
}
