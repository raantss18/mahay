//! Per-user undo and redo in a live session: an undo reverts only the user's own writes, creates
//! and deletes invert, rejected work leaves the history, and random sessions with undo converge
//! without ever reverting someone else's change.

use figcraft_doc::{Document, NodeId, NodeKind};
use figcraft_geom::Affine;

use crate::sim::{Rng, Sim};
use crate::sim_tests::{Width, base, random_edit};
use crate::{BatchKind, NoRecompute, Notice, Recompute};

/// Client `i` edits its document with `f`.
fn edit(sim: &mut Sim, i: usize, f: impl FnOnce(&mut Document)) {
    let mut d = sim.document(i).unwrap().clone();
    f(&mut d);
    sim.edit(i, &d).unwrap();
}

fn settle(sim: &mut Sim) {
    sim.settle(&NoRecompute, 10_000).unwrap();
    sim.converged().unwrap();
}

/// The first frame of the shared file and its first child.
fn frame(sim: &Sim) -> (NodeId, NodeId) {
    let doc = sim.server.document();
    let page = doc.first_page().unwrap();
    let f = doc.node(page).unwrap().children[0];
    (f, doc.node(f).unwrap().children[0])
}

#[test]
fn undo_reverts_only_your_own_writes() {
    let mut sim = Sim::new(base(), 2, 1, 5).unwrap();
    let (f, _) = frame(&sim);
    let old = sim.server.document().node(f).unwrap().clone();
    edit(&mut sim, 0, |d| {
        let n = d.node_mut(f).unwrap();
        n.name = "Mine".into();
        n.transform = Affine::translate((10.0, 10.0));
    });
    settle(&mut sim);
    // Someone else moves the frame after that.
    edit(&mut sim, 1, |d| d.node_mut(f).unwrap().transform = Affine::translate((99.0, 0.0)));
    settle(&mut sim);
    assert!(sim.undo(0, &NoRecompute).unwrap());
    settle(&mut sim);
    let n = sim.server.document().node(f).unwrap();
    assert_eq!(n.name, old.name, "my rename is undone");
    assert_eq!(n.transform, Affine::translate((99.0, 0.0)), "their later move stays");
    // Redo puts the rename back, still without touching their move.
    assert!(sim.redo(0, &NoRecompute).unwrap());
    settle(&mut sim);
    let n = sim.server.document().node(f).unwrap();
    assert_eq!((n.name.as_str(), n.transform), ("Mine", Affine::translate((99.0, 0.0))));
    // Remote batches never enter the history: b has its own step, a has nothing left to redo.
    assert!(!sim.clients()[0].can_redo());
    assert!(sim.clients()[1].can_undo());
    assert!(sim.violations.is_empty(), "{:?}", sim.violations);
}

#[test]
fn creates_and_deletes_invert() {
    let mut sim = Sim::new(base(), 2, 2, 5).unwrap();
    let (f, _) = frame(&sim);
    let start = sim.server.replica().clone();
    edit(&mut sim, 0, |d| {
        d.create(f, NodeKind::Rectangle, "New", 1.0, 1.0, 5.0, 5.0).unwrap();
    });
    settle(&mut sim);
    let created = sim.server.replica().clone();
    sim.undo(0, &NoRecompute).unwrap();
    settle(&mut sim);
    assert_eq!(sim.server.document(), start.document(), "the creation is undone");
    sim.redo(0, &NoRecompute).unwrap();
    settle(&mut sim);
    assert_eq!(sim.server.replica(), &created, "redo brings it back with its id and position");
    // Deleting a frame with layers: undo brings the subtree back whole, in place.
    edit(&mut sim, 1, |d| d.remove(f).unwrap());
    settle(&mut sim);
    assert!(!sim.server.document().contains(f));
    sim.undo(1, &NoRecompute).unwrap();
    settle(&mut sim);
    assert_eq!(sim.server.replica(), &created);
    sim.redo(1, &NoRecompute).unwrap();
    settle(&mut sim);
    assert!(!sim.server.document().contains(f));
    assert!(sim.violations.is_empty(), "{:?}", sim.violations);
}

#[test]
fn a_created_frame_someone_built_in_stays() {
    let mut sim = Sim::new(base(), 2, 3, 5).unwrap();
    let page = sim.server.document().first_page().unwrap();
    edit(&mut sim, 0, |d| {
        d.create(page, NodeKind::Frame, "Mine", 500.0, 0.0, 100.0, 100.0).unwrap();
    });
    settle(&mut sim);
    let mine = *sim.server.document().node(page).unwrap().children.last().unwrap();
    edit(&mut sim, 1, |d| {
        d.create(mine, NodeKind::Rectangle, "Theirs", 0.0, 0.0, 5.0, 5.0).unwrap();
    });
    settle(&mut sim);
    // Nothing of a's step can be undone without deleting b's rectangle: the step is skipped.
    assert!(!sim.undo(0, &NoRecompute).unwrap());
    settle(&mut sim);
    assert_eq!(sim.server.document().node(mine).unwrap().children.len(), 1);
}

#[test]
fn rejected_edits_leave_the_history_with_a_reason() {
    let mut sim = Sim::new(base(), 2, 4, 5).unwrap();
    let (f, r) = frame(&sim);
    // At the same time: b deletes the frame, a renames a layer inside it (b's lands first).
    edit(&mut sim, 1, |d| d.remove(f).unwrap());
    while sim.server.seq() == 0 {
        sim.step(&NoRecompute).unwrap();
    }
    edit(&mut sim, 0, |d| d.node_mut(r).unwrap().name = "Too late".into());
    assert!(sim.clients()[0].can_undo());
    settle(&mut sim);
    assert_eq!(sim.rejected, 1);
    let c = &mut sim.clients_mut()[0];
    assert!(!c.can_undo(), "no undo step for rejected work");
    let notices = c.take_notices();
    assert!(matches!(notices.as_slice(), [Notice::Rejected { kind: BatchKind::Edit, .. }]), "{notices:?}");
    assert!(notices[0].message().contains("deleted by someone else"), "{}", notices[0].message());
    assert!(c.take_notices().is_empty());
}

#[test]
fn undo_right_after_an_edit_that_gets_rejected_is_reverted() {
    for seed in 0..20 {
        let mut sim = Sim::new(base(), 2, seed, 8).unwrap();
        let (f, r) = frame(&sim);
        let other = sim.server.document().node(f).unwrap().children[1];
        edit(&mut sim, 1, |d| d.remove(r).unwrap());
        while sim.server.seq() == 0 {
            sim.step(&NoRecompute).unwrap();
        }
        // a renames r (refused: b deleted it) and another layer, and undoes before hearing back.
        edit(&mut sim, 0, |d| {
            d.node_mut(r).unwrap().name = "Gone".into();
            d.node_mut(other).unwrap().name = "Kept?".into();
        });
        sim.undo(0, &NoRecompute).unwrap();
        settle(&mut sim);
        let name = &sim.server.document().node(other).unwrap().name;
        assert_ne!(name, "Kept?", "seed {seed}");
        assert!(!sim.clients()[0].can_undo() && !sim.clients()[0].can_redo(), "seed {seed}");
    }
}

/// Random edits, undos and redos by several clients, with or without dropped connections.
pub(crate) fn run_with_undo(seed: u64, clients: usize, rounds: usize, derive: &dyn Recompute, drops: bool, sim: Option<Sim>) -> Sim {
    let mut sim = sim.unwrap_or_else(|| Sim::new(base(), clients, seed, 12).unwrap());
    for _ in 0..rounds {
        let i = sim.rng().below(clients);
        match sim.rng().below(10) {
            0 | 1 => {
                sim.undo(i, derive).unwrap();
            }
            2 => {
                sim.redo(i, derive).unwrap();
            }
            3 if drops => {
                if sim.is_online(i) {
                    sim.disconnect(i).unwrap();
                } else {
                    sim.reconnect(i).unwrap();
                }
            }
            _ => {
                let mut doc = sim.document(i).unwrap().clone();
                let mut rng = Rng::new(sim.rng().next_u64());
                random_edit(&mut doc, &mut rng, derive);
                sim.edit(i, &doc).unwrap();
            }
        }
        for _ in 0..sim.rng().below(4) {
            sim.step(derive).unwrap();
        }
    }
    for i in 0..clients {
        sim.reconnect(i).unwrap();
    }
    sim.settle(derive, 1_000_000).unwrap();
    if let Err(e) = sim.converged() {
        panic!("seed {seed}: {e}");
    }
    assert!(sim.violations.is_empty(), "seed {seed}: {:?}", sim.violations);
    sim
}

#[test]
fn random_sessions_with_undo_converge_and_never_revert_others() {
    let mut undos = 0;
    for seed in 0..12 {
        let sim = run_with_undo(seed, 3, 80, &NoRecompute, false, None);
        undos += sim.undos;
        assert!(sim.server.document().validate().is_empty());
    }
    for seed in 50..56 {
        let sim = run_with_undo(seed, 3, 60, &Width, false, None);
        undos += sim.undos;
    }
    assert!(undos > 100, "only {undos} undos and redos accepted");
}

#[test]
#[ignore = "longer run for development: cargo test -p figcraft-collab -- --ignored"]
fn many_seeds_with_undo_converge() {
    for seed in 2000..2600 {
        // 2 to 5 people, short and long delays, drops, and logs short enough to need snapshots.
        let n = 2 + (seed % 4) as usize;
        let limits = crate::Limits { max_log_batches: if seed % 3 == 0 { 1 + (seed % 7) as usize } else { 10_000 }, ..crate::Limits::default() };
        let sim = Sim::with_limits(base(), n, seed, 1 + seed % 20, limits).unwrap();
        run_with_undo(seed, n, 200, &Width, seed % 2 == 0, Some(sim));
    }
}
