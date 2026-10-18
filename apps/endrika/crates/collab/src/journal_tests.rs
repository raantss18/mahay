//! View-only participants and the journal a store keeps between snapshots: a viewer's batches are
//! refused with the reason and rolled back, everything else reaches it; replaying the journal over
//! the last saved state gives the same server (clients resume into it), entries already in the
//! state change nothing, and a log with a gap is refused.

use figcraft_doc::{Document, NodeKind};

use crate::message::{self, Access, ClientMsg, Hello, ServerMsg};
use crate::sim_tests::base;
use crate::{Client, Journal, Limits, NoRecompute, Outgoing, Server, VIEW_ONLY};

fn hello() -> Hello {
    let ClientMsg::Hello(h) = Client::hello("f", "", "Ada", "#FF8800") else { panic!("not a hello") };
    h
}

fn join_as(s: &mut Server, access: Access) -> Client {
    let (_, out) = s.join_as(hello(), access).unwrap();
    let Some(Outgoing::To(_, welcome)) = out.into_iter().next() else { panic!("no welcome") };
    let c = Client::join(welcome).unwrap();
    assert_eq!(c.access(), access, "the welcome says what the client may do");
    c
}

/// Deliver everything in `out` meant for `c`; returns what it sends back.
fn deliver(c: &mut Client, out: &[Outgoing]) -> Vec<ClientMsg> {
    let id = c.id();
    let mut replies = Vec::new();
    for o in out {
        let m = match o {
            Outgoing::To(t, m) if *t == id => m,
            Outgoing::All(m) => m,
            Outgoing::Others(not, m) if *not != id => m,
            _ => continue,
        };
        replies.extend(c.receive(m.clone(), &NoRecompute).unwrap());
    }
    replies
}

fn edit(c: &mut Client, f: impl FnOnce(&mut Document)) -> ClientMsg {
    let mut d = c.document().clone();
    f(&mut d);
    c.edit(&d).unwrap().expect("a change")
}

/// `c` sends `m`; everyone in `all` gets what the server says.
fn send(s: &mut Server, from: u32, m: ClientMsg, all: &mut [&mut Client]) -> Vec<Outgoing> {
    let out = s.receive(from, m);
    for c in all.iter_mut() {
        deliver(c, &out);
    }
    out
}

#[test]
fn a_viewer_sees_everything_and_changes_nothing() {
    let mut s = Server::new(base(), "", Limits::default());
    let mut editor = join_as(&mut s, Access::Edit);
    let mut viewer = join_as(&mut s, Access::View);
    let page = s.document().first_page().unwrap();
    // The editor's edit reaches the viewer.
    let m = edit(&mut editor, |d| {
        d.create(page, NodeKind::Rectangle, "Card", 0.0, 0.0, 10.0, 10.0).unwrap();
    });
    send(&mut s, editor.id(), m, &mut [&mut editor, &mut viewer]);
    assert_eq!(viewer.view(), s.replica());
    // The viewer's edit is refused with the reason, and rolled back.
    let seq = s.seq();
    let m = edit(&mut viewer, |d| d.node_mut(page).unwrap().name = "Mine".into());
    let out = send(&mut s, viewer.id(), m, &mut [&mut editor, &mut viewer]);
    assert!(matches!(out.as_slice(), [Outgoing::To(_, ServerMsg::Reject { reason, .. })] if reason == VIEW_ONLY));
    assert_eq!(s.seq(), seq, "nothing accepted");
    assert_eq!(viewer.view(), s.replica(), "the viewer's copy is the server's again");
    assert!(viewer.take_notices().iter().any(|n| n.message().contains("view")), "the viewer is told why");
    // Presence still goes both ways.
    let out = s.receive(viewer.id(), ClientMsg::Presence(Default::default()));
    assert!(matches!(out.as_slice(), [Outgoing::Others(_, ServerMsg::Presence { .. })]));
    // Joining again with an edit link lifts it.
    let id = viewer.id();
    s.leave(id);
    let mut again = join_as(&mut s, Access::Edit);
    let m = edit(&mut again, |d| d.node_mut(page).unwrap().name = "Now mine".into());
    let out = s.receive(again.id(), m);
    assert!(matches!(out.as_slice(), [Outgoing::All(ServerMsg::Changes(_))]));
}

#[test]
fn managing_messages_are_refused_on_a_session_connection() {
    let mut s = Server::new(base(), "", Limits::default());
    let c = join_as(&mut s, Access::Edit);
    let out = s.receive(c.id(), ClientMsg::Admin(message::Admin::Files));
    assert!(matches!(out.as_slice(), [Outgoing::To(_, ServerMsg::Refused { .. })]));
}

#[test]
fn the_journal_replayed_over_the_last_state_restores_the_server() {
    let mut s = Server::new(base(), "", Limits::default());
    s.journaling(true);
    let mut a = join_as(&mut s, Access::Edit);
    let mut v = join_as(&mut s, Access::View);
    let page = s.document().first_page().unwrap();
    // Saved here: what comes after is only in the journal.
    let saved = s.state().unwrap();
    s.take_journal();
    let mut b = join_as(&mut s, Access::Edit);
    for i in 0..5 {
        let m = edit(&mut a, |d| {
            d.create(page, NodeKind::Rectangle, &format!("A{i}"), f64::from(i), 0.0, 10.0, 10.0).unwrap();
        });
        send(&mut s, a.id(), m, &mut [&mut a, &mut b, &mut v]);
    }
    let m = edit(&mut b, |d| d.node_mut(page).unwrap().name = "Bo's page".into());
    send(&mut s, b.id(), m, &mut [&mut a, &mut b, &mut v]);
    let m = edit(&mut v, |d| d.node_mut(page).unwrap().name = "Refused".into());
    send(&mut s, v.id(), m, &mut [&mut a, &mut b, &mut v]);
    let journal = s.take_journal();
    assert_eq!(journal.iter().filter(|j| matches!(j, Journal::Issued { .. })).count(), 1, "b's id");
    assert_eq!(journal.iter().filter(|j| matches!(j, Journal::Accepted(_))).count(), 6);
    assert_eq!(journal.iter().filter(|j| matches!(j, Journal::Refused { .. })).count(), 1);
    assert!(s.take_journal().is_empty(), "taken");
    // The entries survive both encodings (the store writes the binary one).
    for j in &journal {
        assert_eq!(&message::decode::<Journal>(&message::encode(j).unwrap(), usize::MAX).unwrap(), j);
    }
    // Restart: the saved state, then the journal.
    let mut r = Server::restore(saved, "", Limits::default()).unwrap();
    for j in journal.iter().cloned() {
        r.replay(j).unwrap();
    }
    assert_eq!(r.replica(), s.replica());
    assert_eq!(r.seq(), s.seq());
    let mut live = s.state().unwrap();
    let mut back = r.state().unwrap();
    // The session id is kept; the rest of the lasting state matches too.
    assert_eq!(live.session, back.session);
    live.snapshot.clear();
    back.snapshot.clear();
    assert_eq!(live, back);
    // Replaying again changes nothing (a log that outlived a compaction).
    for j in journal {
        r.replay(j).unwrap();
    }
    assert_eq!(r.replica(), s.replica());
    assert!(r.take_journal().is_empty(), "replays aren't journaled");
    // Every client resumes into the restarted server with nothing to send again, and a newcomer
    // gets an id nobody had.
    for c in [&mut a, &mut b] {
        c.disconnect();
        let ClientMsg::Hello(h) = c.reconnect(hello()) else { panic!() };
        let (id, out) = r.join(h).unwrap();
        assert_eq!(id, c.id());
        assert!(deliver(c, &out).is_empty());
        assert_eq!(c.view(), r.replica());
    }
    let n = join_as(&mut r, Access::Edit);
    assert!(n.id() > b.id());
}

#[test]
fn a_journal_with_a_gap_is_refused() {
    let mut s = Server::new(base(), "", Limits::default());
    s.journaling(true);
    let saved = s.state().unwrap();
    let mut a = join_as(&mut s, Access::Edit);
    let page = s.document().first_page().unwrap();
    for name in ["One", "Two"] {
        let m = edit(&mut a, |d| d.node_mut(page).unwrap().name = name.into());
        send(&mut s, a.id(), m, &mut [&mut a]);
    }
    let journal: Vec<Journal> = s.take_journal().into_iter().filter(|j| !matches!(j, Journal::Accepted(acc) if acc.seq == 1)).collect();
    let mut r = Server::restore(saved, "", Limits::default()).unwrap();
    let errors: Vec<_> = journal.into_iter().filter_map(|j| r.replay(j).err()).collect();
    assert_eq!(errors.len(), 1, "the batch after the gap");
    assert_eq!(r.seq(), 0);
}
