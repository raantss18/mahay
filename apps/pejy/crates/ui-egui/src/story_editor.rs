//! Story Editor (Edit → Edit in Story Editor, ⌘Y): the story as plain text with each paragraph's
//! style in the left column and the overset point marked. Edits are applied as minimal range
//! replacements, so formatting around them is kept.

use egui::vec2;
use serde_json::json;

use designcraft_doc::StoryId;

use crate::DesignApp;
use crate::theme::Tokens;

/// The story open in the Story Editor. Story ids are counted per document, so the editor is bound
/// to the document the story belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target {
    /// [`designcraft_engine::DocState::uid`] of the story's document.
    pub doc: u64,
    pub story: StoryId,
}

/// The editor is open and its document is the active one.
pub fn visible(app: &DesignApp) -> bool {
    app.story_editor.is_some_and(|t| app.session.active().is_some_and(|st| st.uid == t.doc))
}

/// Draws the editor while its document is active; it hides while another document is active and
/// closes with its document.
pub fn show(app: &mut DesignApp, ctx: &egui::Context) {
    let Some(target) = app.story_editor else { return };
    if !app.session.documents().iter().any(|d| d.uid == target.doc) {
        app.story_editor = None;
        return;
    }
    let Some(st) = app.session.active().filter(|st| st.uid == target.doc) else { return };
    let sid = target.story;
    let Some(story) = st.doc.story(sid).cloned() else {
        app.story_editor = None;
        return;
    };
    let t = Tokens::get(ctx);
    let cs = app.session.cache.get(&st.doc, sid, None);
    let overset = cs.overset_at;
    let mut open = true;
    let mut edit: Option<(usize, usize, String)> = None;
    let words = story.text.split_whitespace().count().to_string();
    egui::Window::new(crate::i18n::tr(&app.ui.language, "Story Editor — {count} words").replace("{count}", &words))
        .id(egui::Id::new("story_editor"))
        .open(&mut open)
        .default_size(vec2(560.0, 520.0))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    // Style column.
                    ui.vertical(|ui| {
                        ui.set_width(130.0);
                        for (i, r) in story.para_ranges().iter().enumerate() {
                            let lines = story.text[r.clone()].len() / 60 + 1;
                            let style = &story.paras[i].style;
                            crate::rtl::label(ui, egui::RichText::new(crate::i18n::style_name(&app.ui.language, style)).size(11.0).color(t.text_dim));
                            for _ in 1..lines {
                                ui.label(egui::RichText::new(" ").size(11.0));
                            }
                        }
                    });
                    ui.separator();
                    let mut text = story.text.clone();
                    let r = ui.add(
                        egui::TextEdit::multiline(&mut text)
                            .font(egui::FontId::proportional(app.ui.story_editor_size.clamp(8.0, 36.0)))
                            .desired_width(f32::INFINITY)
                            .desired_rows(20)
                            .frame(egui::Frame::NONE),
                    );
                    if r.changed() {
                        // Minimal diff: common prefix and suffix.
                        let old = &story.text;
                        let pre = old.bytes().zip(text.bytes()).take_while(|(a, b)| a == b).count();
                        let pre = (0..=pre).rev().find(|&i| old.is_char_boundary(i) && text.is_char_boundary(i)).unwrap_or(0);
                        let max_suf = old.len().min(text.len()) - pre;
                        let suf = old.bytes().rev().zip(text.bytes().rev()).take(max_suf).take_while(|(a, b)| a == b).count();
                        let suf =
                            (0..=suf).rev().find(|&k| old.is_char_boundary(old.len() - k) && text.is_char_boundary(text.len() - k)).unwrap_or(0);
                        edit = Some((pre, old.len() - suf, text[pre..text.len() - suf].to_string()));
                    }
                });
                if let Some(o) = overset {
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label(crate::rtl::widget(
                            ui,
                            egui::RichText::new(crate::i18n::tr(&app.ui.language, "OVERSET")).color(egui::Color32::from_rgb(230, 40, 40)).strong(),
                        ));
                        crate::rtl::label(
                            ui,
                            egui::RichText::new(
                                crate::i18n::tr(&app.ui.language, "{count} characters don't fit")
                                    .replace("{count}", &story.text[o.min(story.text.len())..].chars().count().to_string()),
                            )
                            .color(t.text_dim),
                        );
                    });
                }
            });
        });
    if let Some((a, b, text)) = edit {
        let _ = app.run("story.replaceRange", json!({"story": sid.0, "start": a, "end": b, "text": text}));
    }
    if !open {
        app.story_editor = None;
    }
}

#[cfg(test)]
mod tests {
    use egui_kittest::kittest::Queryable;
    use serde_json::json;

    use crate::DesignApp;

    fn story_text(app: &DesignApp, doc: usize) -> String {
        let st = &app.session.documents()[doc];
        st.doc.stories.values().next().unwrap().text.clone()
    }

    /// Story ids are counted per document: typing in the editor after switching documents edited
    /// the other document's story with the same id.
    #[test]
    fn editor_stays_with_its_document() {
        let mut app = DesignApp::new(designcraft_engine::Session::new(), Default::default());
        let mut stories = Vec::new();
        for text in ["Alpha story", "Bravo story"] {
            app.run("file.new", json!({})).unwrap();
            let r = app.run("frame.create", json!({"rect": [72, 72, 300, 200], "content": "text", "text": text, "caret": false})).unwrap();
            stories.push(r["story"].as_u64().unwrap());
        }
        assert_eq!(stories[0], stories[1], "the fixture needs the same story id in both documents");
        app.run("file.activate", json!({"index": 0})).unwrap();
        app.run("app.storyEditor", json!({"story": stories[0]})).unwrap();
        let mut h = crate::test_window::open(app, egui::vec2(1440.0, 900.0));
        h.get_by(|n| n.role() == egui::accesskit::Role::MultilineTextInput && n.value().as_deref() == Some("Alpha story")).focus();
        h.run_steps(2);
        h.event(egui::Event::Text("!".into()));
        h.run_steps(2);
        let a = story_text(&h.state().app, 0);
        assert!(a.contains('!'), "typing reaches the open story: {a:?}");

        h.state_mut().app.run("file.activate", json!({"index": 1})).unwrap();
        h.run_steps(2);
        h.event(egui::Event::Text("?".into()));
        h.run_steps(2);
        assert_eq!(story_text(&h.state().app, 1), "Bravo story");
        assert_eq!(story_text(&h.state().app, 0), a);
        assert!(h.state().app.story_editor.is_some(), "the editor waits for its document");

        h.state_mut().app.run("file.close", json!({"index": 0})).unwrap();
        h.run_steps(2);
        assert_eq!(h.state().app.story_editor, None);
    }
}
