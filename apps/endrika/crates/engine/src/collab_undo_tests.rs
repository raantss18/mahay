//! Per-user undo with real commands: several people running random commands, undoing and
//! redoing, some dropping their connection now and then, all end up with the server's document,
//! and no undo or redo ever reverts a change by someone else that its author had seen.

use figcraft_collab::sim::{Rng, Sim};
use figcraft_collab::{Limits, Recompute};

use crate::Session;
use crate::collab::EngineRecompute;
use crate::collab_tests::{base, random_command};

fn session(seed: u64, clients: usize, rounds: usize, drops: bool, limits: Limits, recompute: &dyn Recompute) -> Sim {
    let mut sim = Sim::with_limits(base(), clients, seed, 10, limits).unwrap();
    for _ in 0..rounds {
        let i = sim.rng().below(clients);
        match sim.rng().below(12) {
            0 | 1 => {
                sim.undo(i, recompute).unwrap();
            }
            2 => {
                sim.redo(i, recompute).unwrap();
            }
            3 if drops => {
                if sim.is_online(i) {
                    sim.disconnect(i).unwrap();
                } else {
                    sim.reconnect(i).unwrap();
                }
            }
            _ => {
                let mut s = Session::with_document(sim.document(i).unwrap().clone());
                let mut rng = Rng::new(sim.rng().next_u64());
                let (cmd, params) = random_command(s.doc().unwrap(), &mut rng, false);
                if s.execute(cmd, &params).is_ok() {
                    sim.edit(i, s.doc().unwrap()).unwrap_or_else(|e| panic!("seed {seed}: {cmd} {params}: {e}"));
                }
            }
        }
        for _ in 0..sim.rng().below(4) {
            sim.step(recompute).unwrap();
        }
    }
    for i in 0..clients {
        sim.reconnect(i).unwrap();
    }
    sim.settle(recompute, 1_000_000).unwrap();
    if let Err(e) = sim.converged() {
        panic!("seed {seed}: {e}");
    }
    assert!(sim.violations.is_empty(), "seed {seed}: {:?}", sim.violations);
    assert!(sim.server.document().validate().is_empty(), "seed {seed}: {:?}", sim.server.document().validate());
    sim
}

#[test]
fn people_undoing_real_commands_converge_without_reverting_others() {
    let mut undos = 0;
    for seed in 0..4 {
        undos += session(seed, 3, 60, false, Limits::default(), &EngineRecompute).undos;
    }
    // Dropped connections, and a log short enough that long absences catch up with snapshots.
    for seed in 10..13 {
        undos += session(seed, 3, 60, true, Limits { max_log_batches: 4, ..Limits::default() }, &EngineRecompute).undos;
    }
    assert!(undos > 20, "only {undos} undos and redos accepted");
}

#[test]
#[ignore = "longer run for development: cargo test -p figcraft-engine collab -- --ignored"]
fn many_people_undoing_real_commands_converge() {
    for seed in 100..160 {
        let limits = Limits { max_log_batches: if seed % 2 == 0 { 5 } else { 10_000 }, ..Limits::default() };
        session(seed, 4, 150, seed % 3 == 0, limits, &EngineRecompute);
    }
}
