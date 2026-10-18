//! Several people creating styles, variables, collections, modes and comments at once with real
//! commands: each gets ids from its own client space, so nothing created is overwritten.

use figcraft_collab::sim::Sim;
use figcraft_collab::{Change, Entry};
use serde_json::{Value, json};

use crate::Session;
use crate::collab::EngineRecompute;
use crate::collab_tests::base;

/// The commands one person runs in a round, before seeing anyone else's edits.
fn creations(who: usize, round: usize) -> Vec<(&'static str, Value)> {
    let name = format!("p{who} r{round}");
    vec![
        ("style.create", json!({"type": "paint", "name": name, "value": "#3366FF"})),
        ("variable.create", json!({"collection": "Tokens", "name": name, "type": "number", "value": who})),
        ("variable.duplicate", json!({"variable": "v0"})),
        ("variable.createCollection", json!({"name": name})),
        ("variable.addMode", json!({"collection": name, "name": "Dark"})),
        ("comment.add", json!({"x": 10 * who, "y": 10 * round, "text": name})),
    ]
}

#[test]
fn concurrent_creations_with_real_commands_all_survive() {
    let start = base();
    let (styles, vars, colls, comments) =
        (start.styles.len(), start.variables.variables.len(), start.variables.collections.len(), start.comments.len());
    for seed in 0..8 {
        let n = 2 + (seed as usize % 3);
        let rounds = 2;
        let mut sim = Sim::new(start.clone(), n, seed, 10).unwrap();
        for round in 0..rounds {
            for i in 0..n {
                let mut s = Session::with_document(sim.document(i).unwrap().clone());
                for (cmd, params) in creations(i, round) {
                    s.execute(cmd, &params).unwrap_or_else(|e| panic!("seed {seed}: {cmd} {params}: {e}"));
                }
                sim.edit(i, s.doc().unwrap()).unwrap();
            }
            for _ in 0..sim.rng().below(5) {
                sim.step(&EngineRecompute).unwrap();
            }
        }
        sim.settle(&EngineRecompute, 1_000_000).unwrap();
        sim.converged().unwrap_or_else(|e| panic!("seed {seed}: {e}"));
        assert_eq!(sim.rejected, 0, "seed {seed}");
        let d = sim.server.document();
        let made = n * rounds;
        assert_eq!(d.styles.len(), styles + made, "seed {seed}: every style survives");
        // Each round: one new variable and one copy.
        assert_eq!(d.variables.variables.len(), vars + 2 * made, "seed {seed}: every variable survives");
        assert_eq!(d.variables.collections.len(), colls + made, "seed {seed}");
        assert_eq!(d.comments.len(), comments + made, "seed {seed}: every comment survives");
        for who in 0..n {
            for round in 0..rounds {
                let name = format!("p{who} r{round}");
                let coll = d.variables.collections.iter().find(|c| c.name == name).unwrap_or_else(|| panic!("seed {seed}: {name}'s collection"));
                assert_eq!(coll.modes.len(), 2, "seed {seed}: {name}'s modes");
                assert!(d.styles.iter().any(|s| s.name == name) && d.comments.iter().any(|c| c.first.text == name), "seed {seed}: {name}");
            }
        }
    }
}

#[test]
fn outside_a_session_the_ids_are_unchanged() {
    let mut s = Session::with_document(base());
    let id = |v: Value| v["id"].clone();
    assert_eq!(id(s.execute("style.create", &json!({"type": "paint", "name": "A", "value": "#000000"})).unwrap()), "S:1");
    assert_eq!(id(s.execute("variable.createCollection", &json!({"name": "More"})).unwrap()), "VariableCollectionId:2");
    assert_eq!(id(s.execute("variable.addMode", &json!({"collection": "More", "name": "Dark"})).unwrap()), "VariableCollectionId:2/mode2");
    assert_eq!(id(s.execute("variable.create", &json!({"collection": "More", "name": "x", "type": "number", "value": 1})).unwrap()), "VariableID:5");
    assert_eq!(id(s.execute("comment.add", &json!({"x": 0, "y": 0, "text": "hi"})).unwrap()), 1);
}

#[test]
fn a_style_made_in_a_session_syncs_as_an_entry_of_the_creators_space() {
    let mut sim = Sim::new(base(), 2, 7, 3).unwrap();
    let mut s = Session::with_document(sim.document(1).unwrap().clone());
    let me = sim.clients()[1].id();
    let r = s.execute("style.create", &json!({"type": "paint", "name": "Mine", "value": "#FF0000"})).unwrap();
    let sid = r["id"].as_str().unwrap().to_string();
    let doc = s.doc().unwrap().clone();
    let batch = figcraft_collab::diff(sim.clients()[1].view(), &doc).unwrap();
    let entry =
        batch.changes.iter().find_map(|c| if let Change::Put { entry: e @ Entry::Style(_), .. } = c { Some(e.clone()) } else { None }).unwrap();
    assert_eq!(entry, Entry::Style(sid));
    assert_eq!(entry.id_space(), Some(me));
    sim.edit(1, &doc).unwrap();
    sim.settle(&EngineRecompute, 10_000).unwrap();
    assert!(sim.server.document().styles.iter().any(|st| st.name == "Mine"));
}
