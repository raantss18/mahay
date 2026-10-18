//! Cursor chat and spotlight drawn headless: others' chat bubbles fading, your chat box (`/`,
//! typing, Enter, Escape), the spotlight banner and your avatar's menu.

use egui::{Event, Key, Modifiers, pos2};
use figcraft_engine::live::{Event as LiveEvent, Signal};

use crate::live_tests::{Harness, cursor, has_text, text_center, texts};

fn key(h: &mut Harness, k: Key, text: Option<&str>) {
    let mut down = vec![Event::Key { key: k, physical_key: None, pressed: true, repeat: false, modifiers: Modifiers::NONE }];
    down.extend(text.map(|t| Event::Text(t.into())));
    h.frame(down);
    h.frame(vec![Event::Key { key: k, physical_key: None, pressed: false, repeat: false, modifiers: Modifiers::NONE }]);
}

/// Joined as Ada with Hana here, and the host reads signals.
fn ready() -> Harness {
    let (mut h, _) = Harness::new();
    h.frames(3);
    h.joined();
    h.send(vec![LiveEvent::Signal(2, Signal::ready())]);
    h.frames(2);
    assert!(h.app.session.live().signals);
    h
}

#[test]
fn others_chat_shows_by_their_cursor_then_fades() {
    let mut h = ready();
    let page = h.page();
    h.send(vec![LiveEvent::Presence(1, cursor(page, 400.0, 300.0)), LiveEvent::Signal(1, Signal::chat("Looks good"))]);
    let shapes = h.frames(2);
    assert!(has_text(&shapes, "Looks good") && has_text(&shapes, "Hana"), "the message under her name");
    // Let the canvas render thread settle first: a tile landing asks for a frame of its own.
    let delay = (0..30)
        .find_map(|_| {
            crate::idle_tests::wait_for_render(&h.app);
            Some(h.frame(vec![]).1).filter(|d| !d.is_zero())
        })
        .unwrap_or_default();
    assert!(delay.as_secs_f64() > 4.0, "no frames until it starts fading: {delay:?} {:?}", h.ctx.repaint_causes());
    h.skip(5.5);
    let (shapes, delay) = h.frame(vec![]);
    assert!(has_text(&shapes, "Looks good") && delay.is_zero(), "fading");
    h.skip(1.0);
    let shapes = h.frames(1);
    assert!(!has_text(&shapes, "Looks good") && has_text(&shapes, "Hana"), "gone: the name tag is back");
    assert!(h.goes_quiet());
    // A new message shows again.
    h.send(vec![LiveEvent::Signal(1, Signal::chat("Again"))]);
    assert!(has_text(&h.frames(2), "Again"));
}

#[test]
fn slash_opens_your_chat_and_enter_or_escape_ends_it() {
    let mut h = ready();
    h.frame(vec![Event::PointerMoved(pos2(700.0, 500.0))]);
    key(&mut h, Key::Slash, Some("/"));
    h.frames(1);
    assert!(h.app.ui.live.chat.as_ref().is_some_and(|c| c.editing), "the box opened");
    h.frame(vec![Event::Text("hello".into())]);
    assert_eq!(h.app.session.live().chat, "hello", "sent as you type, without the slash");
    assert!(has_text(&h.frames(1), "hello"));
    key(&mut h, Key::Enter, None);
    assert!(h.app.ui.live.chat.as_ref().is_some_and(|c| !c.editing), "sent: the box closed, the message shows");
    assert_eq!(h.app.session.live().chat, "hello");
    h.skip(7.0);
    h.frames(2);
    assert!(h.app.ui.live.chat.is_none() && h.app.session.live().chat.is_empty(), "faded and taken back");
    // Escape takes it back at once and doesn't deselect.
    let rect = h.app.session.doc().unwrap().pages.first().copied();
    let layer = h.app.session.doc().unwrap().node(rect.unwrap()).unwrap().children[0];
    h.app.session.execute("select.set", &serde_json::json!({"ids": [layer.0]})).unwrap();
    key(&mut h, Key::Slash, Some("/"));
    h.frames(1);
    h.frame(vec![Event::Text("oops".into())]);
    assert_eq!(h.app.session.live().chat, "oops");
    key(&mut h, Key::Escape, None);
    h.frames(1);
    assert!(h.app.ui.live.chat.is_none() && h.app.session.live().chat.is_empty());
    assert_eq!(h.app.session.selection(), vec![layer], "Escape only closed the chat");
    // Left open without typing, the box closes by itself.
    key(&mut h, Key::Slash, Some("/"));
    h.frames(1);
    h.skip(7.0);
    h.frames(2);
    assert!(h.app.ui.live.chat.is_none());
}

#[test]
fn spotlight_banner_and_your_avatar_menu() {
    let mut h = ready();
    let page = h.page();
    h.send(vec![
        LiveEvent::Signal(1, Signal::spotlight(true)),
        LiveEvent::Presence(1, figcraft_engine::live::Presence { viewport: Some((page, 0.0, 0.0, 600.0, 400.0)), ..Default::default() }),
    ]);
    let shapes = h.frames(2);
    assert_eq!(h.app.session.live().follow, Some(1));
    assert!(texts(&shapes).iter().any(|(t, _, _)| t.starts_with("Following Hana")));
    // Stop following: the banner offers to follow again.
    h.app.session.escape().unwrap();
    let shapes = h.frames(2);
    assert!(has_text(&shapes, "Hana is spotlighting"));
    h.click(text_center(&shapes, "Follow"));
    assert_eq!(h.app.session.live().follow, Some(1));
    // Your avatar's menu: spotlight yourself, which takes over.
    let shapes = h.frames(2);
    h.click(text_center(&shapes, "AL"));
    let shapes = h.frames(2);
    h.click(text_center(&shapes, "Spotlight me"));
    assert!(h.app.session.live().spotlighting);
    assert_eq!(h.app.session.live().follow, None);
    let shapes = h.frames(2);
    assert!(has_text(&shapes, "You're spotlighting: everyone follows your view"));
    h.click(text_center(&shapes, "Stop"));
    assert!(!h.app.session.live().spotlighting);
    assert!(!has_text(&h.frames(2), "You're spotlighting: everyone follows your view"));
}
