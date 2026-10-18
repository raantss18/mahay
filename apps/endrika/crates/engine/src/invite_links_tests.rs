//! `collab.registerLinks` and the startup registration, against a fake registrar (tests never
//! touch the real system).

use std::sync::{Arc, Mutex};

use serde_json::json;

use crate::invite_links::LinkRegistrar;
use crate::{EngineError, Session};

#[derive(Default)]
struct State {
    registered: bool,
    calls: Vec<bool>,
    fail: bool,
}

struct Fake {
    installed: bool,
    state: Arc<Mutex<State>>,
}

impl LinkRegistrar for Fake {
    fn registered(&self) -> bool {
        self.state.lock().unwrap().registered
    }
    fn installed(&self) -> bool {
        self.installed
    }
    fn set(&mut self, on: bool) -> Result<(), String> {
        let mut s = self.state.lock().unwrap();
        if s.fail {
            return Err("access denied".into());
        }
        s.calls.push(on);
        s.registered = on;
        Ok(())
    }
}

fn session(installed: bool, registered: bool) -> (Session, Arc<Mutex<State>>) {
    let state = Arc::new(Mutex::new(State { registered, ..State::default() }));
    let mut s = Session::new();
    s.live.links = Some(Box::new(Fake { installed, state: state.clone() }));
    (s, state)
}

#[test]
fn without_a_registrar_the_command_is_disabled() {
    let mut s = Session::new();
    assert_eq!(s.invite_links_on(), None);
    s.invite_links_startup();
    let e = s.execute("collab.registerLinks", &json!({"on": true})).unwrap_err();
    assert!(matches!(e, EngineError::Disabled(..)), "{e}");
    assert_eq!(s.prefs.invite_links, None, "the preference is untouched");
}

#[test]
fn installed_builds_register_on_first_run() {
    let (mut s, state) = session(true, false);
    assert_eq!(s.invite_links_on(), Some(true));
    s.invite_links_startup();
    assert_eq!(state.lock().unwrap().calls, vec![true]);
    // Already this copy: nothing written again.
    s.invite_links_startup();
    assert_eq!(state.lock().unwrap().calls, vec![true]);
}

#[test]
fn development_builds_leave_the_registration_alone() {
    // Not registered, or another copy's registration: a build from source changes neither.
    for registered in [false, true] {
        let (mut s, state) = session(false, registered);
        assert_eq!(s.invite_links_on(), Some(false));
        s.invite_links_startup();
        assert!(state.lock().unwrap().calls.is_empty());
    }
}

#[test]
fn the_preference_wins_at_startup() {
    let (mut s, state) = session(false, false);
    s.prefs.invite_links = Some(true);
    s.invite_links_startup();
    assert_eq!(state.lock().unwrap().calls, vec![true]);
    let (mut s, state) = session(true, true);
    s.prefs.invite_links = Some(false);
    s.invite_links_startup();
    assert_eq!(state.lock().unwrap().calls, vec![false], "turned off: this copy's registration is removed");
}

#[test]
fn the_command_switches_and_remembers() {
    let (mut s, state) = session(true, false);
    // Without `on` it switches the effective setting (on for an installed build) off.
    let r = s.execute("collab.registerLinks", &json!({})).unwrap();
    assert_eq!(r, json!({"on": false, "registered": false}));
    assert_eq!(s.prefs.invite_links, Some(false));
    let r = s.execute("collab.registerLinks", &json!({"on": true})).unwrap();
    assert_eq!(r, json!({"on": true, "registered": true}));
    assert_eq!(s.prefs.invite_links, Some(true));
    assert_eq!(s.invite_links_on(), Some(true));
    assert_eq!(state.lock().unwrap().calls, vec![false, true]);
}

#[test]
fn a_failed_registration_keeps_the_preference() {
    let (mut s, state) = session(true, false);
    state.lock().unwrap().fail = true;
    let e = s.execute("collab.registerLinks", &json!({"on": true})).unwrap_err();
    assert!(e.to_string().contains("access denied"), "{e}");
    assert_eq!(s.prefs.invite_links, None);
    // Startup logs the failure and carries on.
    s.invite_links_startup();
}

#[test]
fn junk_parameters_never_panic() {
    let (mut s, _) = session(true, false);
    for p in [json!(null), json!([]), json!({"on": "yes"}), json!({"on": 3}), json!({"on": null})] {
        let _ = s.execute("collab.registerLinks", &p);
    }
}
