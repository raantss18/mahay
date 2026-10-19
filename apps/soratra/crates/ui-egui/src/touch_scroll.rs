//! A finger drags the page along; a long press selects (D23, touch).
//!
//! The canvas used to take every press for a mouse press, so on a phone a finger drag selected text
//! and could not scroll. Now a gesture that starts on the page with a finger is decided by what the
//! finger does first: it moves past a small slop within [`LONG_PRESS`] seconds, and the page follows
//! the finger (with the coast a touchpad flick gets); it rests that long, and what follows selects,
//! as with a mouse. A stylus (a touch with a pressure under 1) and the mouse keep selecting.

use egui::{Event, Pos2, Rect, Vec2};

/// How far (points) a finger may move before it counts as a drag.
pub const SLOP: f32 = 10.0;
/// Seconds a finger must rest before a drag selects instead of scrolling.
pub const LONG_PRESS: f64 = 0.5;

/// What the finger of a gesture is doing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Not moved past the slop yet, not rested long enough.
    Undecided,
    /// The page follows the finger.
    Scrolling,
    /// A long press: the drag selects.
    Selecting,
    /// Not a finger gesture on the page (the ribbon, a stylus…): not ours.
    Ignored,
}

/// One finger gesture on the canvas, from touch down to lift.
#[derive(Clone, Copy, Debug)]
pub struct Gesture {
    pub start: Pos2,
    pub started_at: f64,
    pub mode: Mode,
}

/// The mode after a frame in which the finger is at `now`, `time` seconds into the clock.
pub fn decide(g: &Gesture, now: Pos2, time: f64) -> Mode {
    if g.mode != Mode::Undecided {
        return g.mode;
    }
    let held = time - g.started_at;
    if (now - g.start).length() > SLOP && held < LONG_PRESS {
        Mode::Scrolling
    } else if held >= LONG_PRESS {
        Mode::Selecting
    } else {
        Mode::Undecided
    }
}

/// Whether the touches in progress are a finger's (a stylus reports a pressure under 1, a finger
/// the full pressure or none).
pub fn finger_down(input: &egui::InputState) -> bool {
    input.any_touches() && !input.events.iter().any(|e| matches!(e, Event::Touch { force: Some(f), .. } if *f > 0.0 && *f < 1.0))
}

/// Follows a finger gesture over the canvas. Returns how far the page moves this frame (points,
/// positive = content moves down/right, like a touchpad) and cancels any selection the press had
/// started once the finger turns out to be dragging.
pub fn frame(app: &mut crate::WordApp, ui: &egui::Ui, area: Rect) -> Vec2 {
    let (down, finger, pos, delta, time, dt) =
        ui.input(|i| (i.pointer.primary_down(), finger_down(i), i.pointer.latest_pos(), i.pointer.delta(), i.time, i.unstable_dt));
    if !down || !finger || app.session.view.read_mode || app.dialog.is_some() {
        if let Some(g) = app.canvas.touch.take()
            && g.mode == Mode::Scrolling
        {
            app.canvas.wheel.finger_lift();
        }
        return Vec2::ZERO;
    }
    let Some(p) = pos else { return Vec2::ZERO };
    let mut g =
        *app.canvas.touch.get_or_insert(Gesture { start: p, started_at: time, mode: if area.contains(p) { Mode::Undecided } else { Mode::Ignored } });
    g.mode = decide(&g, p, time);
    if g.mode == Mode::Scrolling && app.canvas.touch.is_some_and(|old| old.mode != Mode::Scrolling) {
        // The press placed the caret and may have begun a drag: it is a scroll, not a selection.
        app.canvas.cancel_drag_select();
    }
    app.canvas.touch = Some(g);
    if g.mode == Mode::Scrolling { app.canvas.wheel.finger(delta, dt) } else { Vec2::ZERO }
}

#[cfg(test)]
mod tests {
    use egui::pos2;

    use super::*;

    fn g(mode: Mode) -> Gesture {
        Gesture { start: pos2(100.0, 100.0), started_at: 10.0, mode }
    }

    #[test]
    fn a_quick_move_scrolls_and_a_long_rest_selects() {
        assert_eq!(decide(&g(Mode::Undecided), pos2(103.0, 104.0), 10.1), Mode::Undecided, "inside the slop");
        assert_eq!(decide(&g(Mode::Undecided), pos2(100.0, 130.0), 10.1), Mode::Scrolling, "moved at once");
        assert_eq!(decide(&g(Mode::Undecided), pos2(100.0, 130.0), 10.6), Mode::Selecting, "moved after a long rest");
        assert_eq!(decide(&g(Mode::Undecided), pos2(101.0, 100.0), 10.6), Mode::Selecting, "rested, not moved yet");
    }

    #[test]
    fn a_decided_gesture_keeps_its_mode() {
        assert_eq!(decide(&g(Mode::Scrolling), pos2(100.0, 100.0), 12.0), Mode::Scrolling);
        assert_eq!(decide(&g(Mode::Selecting), pos2(300.0, 100.0), 10.1), Mode::Selecting);
        assert_eq!(decide(&g(Mode::Ignored), pos2(300.0, 100.0), 10.1), Mode::Ignored);
    }
}
