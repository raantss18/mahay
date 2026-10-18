//! Accessibility keys and preferences: F6 / Shift+F6 move the keyboard between the window's regions
//! (the UI does the moving; the commands ask it to), live while editing text too, listed in the
//! shortcuts panel; Reduce motion is a preference.

use serde_json::json;

use crate::Session;
use crate::keymap::{Context, GESTURES, Group, KEYS, Mods, Platform, display, lookup};

#[test]
fn f6_and_shift_f6_cycle_the_regions_on_every_platform_and_while_editing_text() {
    let shift = Mods { shift: true, ..Mods::default() };
    for p in [Platform::Mac, Platform::Other] {
        for c in [Context::Canvas, Context::Text] {
            assert_eq!(lookup(p, c, "F6", Mods::default()).map(|b| b.id), Some("view.nextRegion"), "{p:?} {c:?}");
            assert_eq!(lookup(p, c, "F6", shift).map(|b| b.id), Some("view.previousRegion"), "{p:?} {c:?}");
        }
    }
    assert_eq!(display("F6", Platform::Other), "F6");
    assert_eq!(display("Shift+F6", Platform::Mac), "⇧F6");
    assert_eq!(display("Shift+F6", Platform::Other), "Shift+F6");
    // Listed with the view keys, next to the keyboard-mode keys they lead to.
    for id in ["view.nextRegion", "view.previousRegion"] {
        let b = KEYS.iter().find(|b| b.id == id).unwrap();
        assert_eq!(b.group, Group::View);
        assert!(b.title().starts_with("Focus the"), "{}", b.title());
    }
    assert!(GESTURES.iter().any(|g| g.0 == Group::View && g.3.contains("Tab")));
}

#[test]
fn region_commands_ask_the_ui_to_move_the_keyboard() {
    let mut s = Session::new();
    for (cmd, step) in [("view.nextRegion", 1), ("view.previousRegion", -1)] {
        s.ui_requests.clear();
        s.execute(cmd, &json!({})).unwrap();
        let r = s.ui_requests.iter().find(|r| r.kind == "focus.region").unwrap_or_else(|| panic!("{cmd}"));
        assert_eq!(r.data["step"], step, "{cmd}");
        // Nothing to undo: the document didn't change.
        assert!(!s.ds().unwrap().is_dirty());
    }
}

#[test]
fn reduce_motion_is_a_preference_that_persists() {
    let mut s = Session::new();
    assert!(!s.prefs.reduce_motion);
    assert_eq!(s.execute("pref.reduceMotion", &json!({})).unwrap()["on"], true);
    assert!(s.prefs.reduce_motion);
    assert_eq!(s.execute("pref.reduceMotion", &json!({"on": false})).unwrap()["on"], false);
    s.prefs.reduce_motion = true;
    // Saved with the other preferences; older settings files without it read as off.
    let v = serde_json::to_value(&s.prefs).unwrap();
    assert_eq!(v["reduce_motion"], true);
    let old: crate::Prefs = serde_json::from_value(json!({"theme": "dark"})).unwrap();
    assert!(!old.reduce_motion);
}
