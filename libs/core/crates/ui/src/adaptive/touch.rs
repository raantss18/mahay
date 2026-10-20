//! Touch: a finger drag pans a view, a long press selects, a stylus and a mouse select at once.
//!
//! egui turns a finger into a pointer, so an app that selects on a pointer drag selects on a finger
//! drag and cannot scroll. [`FingerPan`] tells the two apart: it follows one finger gesture that
//! starts in an area, decides by what the finger does first (moves past [`SLOP`] within
//! [`LONG_PRESS`] seconds: a pan; rests that long: a selection), and after a fast pan carries the
//! view on with a decaying coast. The app adds [`PanStep::delta`] to its scroll and ignores the drag
//! while [`PanStep::panning`] is set.

use egui::{Event, InputState, Pos2, Rect, Vec2};

/// How far (points) a finger may move before it counts as a drag.
pub const SLOP: f32 = 10.0;
/// Seconds a finger must rest before a drag selects instead of panning.
pub const LONG_PRESS: f64 = 0.5;
/// Slower flicks than this (points/s) do not coast.
const MIN_FLING: f32 = 150.0;
/// Fastest coast (points/s).
const MAX_SPEED: f32 = 8000.0;
/// Exponential decay rate of the coast (1/s).
const DECAY: f32 = 3.0;
/// The coast stops below this speed (points/s).
const STOP_SPEED: f32 = 20.0;
/// Longest frame time trusted (a stall must not fling the view).
const MAX_DT: f32 = 0.1;

/// What a finger gesture is doing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Not moved past the slop, not rested long enough.
    Undecided,
    /// The view follows the finger.
    Panning,
    /// A long press: the drag selects, as with a mouse.
    Selecting,
    /// Not a finger gesture on the area (a stylus, the mouse, a press outside): not ours.
    Ignored,
}

/// One gesture, from touch down to lift.
#[derive(Clone, Copy, Debug)]
pub struct Gesture {
    pub start: Pos2,
    pub started_at: f64,
    pub mode: Mode,
}

/// The mode of `g` after a frame in which the finger is at `now`, `time` seconds into the clock.
pub fn decide(g: &Gesture, now: Pos2, time: f64) -> Mode {
    if g.mode != Mode::Undecided {
        return g.mode;
    }
    let held = time - g.started_at;
    if (now - g.start).length() > SLOP && held < LONG_PRESS {
        Mode::Panning
    } else if held >= LONG_PRESS {
        Mode::Selecting
    } else {
        Mode::Undecided
    }
}

/// Whether the touches in progress are a finger's: a stylus reports a pressure under 1, a finger the
/// full pressure or none.
pub fn finger_down(input: &InputState) -> bool {
    input.any_touches() && !input.events.iter().any(|e| matches!(e, Event::Touch { force: Some(f), .. } if *f > 0.0 && *f < 1.0))
}

/// What a frame of a finger gesture asks of the app.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PanStep {
    /// How far the content moves this frame, in points, in the direction of the finger (a finger
    /// moving up moves it up). Includes the coast after a lift.
    pub delta: Vec2,
    /// A finger is panning: the app must not start or extend a selection from this drag.
    pub panning: bool,
    /// The pan began this frame: cancel a selection the press started.
    pub just_started: bool,
    /// The view is still coasting: keep repainting.
    pub coasting: bool,
}

/// Follows finger gestures over an area. Keep one per scrolling view.
#[derive(Clone, Debug, Default)]
pub struct FingerPan {
    gesture: Option<Gesture>,
    velocity: Vec2,
    coasting: bool,
}

impl FingerPan {
    /// The mode of the gesture in progress, if any.
    pub fn mode(&self) -> Option<Mode> {
        self.gesture.map(|g| g.mode)
    }

    /// One frame. `area`: where a gesture must start to be a pan. `enabled`: false while something
    /// else owns the pointer (a dialog, a text editor).
    pub fn update(&mut self, input: &InputState, area: Rect, enabled: bool) -> PanStep {
        let dt = if input.unstable_dt.is_finite() { input.unstable_dt.clamp(0.0, MAX_DT) } else { 0.0 };
        let down = input.pointer.primary_down() && finger_down(input) && enabled;
        let (Some(p), true) = (input.pointer.latest_pos(), down) else {
            return self.lift(dt);
        };
        let first = self.gesture.is_none();
        let mut g = self.gesture.unwrap_or(Gesture {
            start: p,
            started_at: input.time,
            mode: if area.contains(p) { Mode::Undecided } else { Mode::Ignored },
        });
        let before = g.mode;
        g.mode = decide(&g, p, input.time);
        let just_started = g.mode == Mode::Panning && before != Mode::Panning;
        self.gesture = Some(g);
        self.coasting = false;
        if g.mode != Mode::Panning {
            return PanStep { just_started: false, ..PanStep::default() };
        }
        let delta = input.pointer.delta();
        if delta.is_finite() && !first && dt > 0.0 {
            let sample = delta / dt.max(1.0 / 500.0);
            let a = 1.0 - (-dt / 0.04).exp();
            self.velocity = if just_started { sample } else { self.velocity + (sample - self.velocity) * a };
            let speed = self.velocity.length();
            if speed > MAX_SPEED {
                self.velocity *= MAX_SPEED / speed;
            }
        }
        PanStep { delta: if delta.is_finite() { delta } else { Vec2::ZERO }, panning: true, just_started, coasting: false }
    }

    /// The finger is up (or never was down): coast if the last pan was a fast flick.
    fn lift(&mut self, dt: f32) -> PanStep {
        if let Some(g) = self.gesture.take()
            && g.mode == Mode::Panning
            && self.velocity.length() >= MIN_FLING
        {
            self.coasting = true;
        }
        if !self.coasting {
            self.velocity = Vec2::ZERO;
            return PanStep::default();
        }
        let keep = (-DECAY * dt).exp();
        let step = self.velocity * ((1.0 - keep) / DECAY);
        self.velocity *= keep;
        if self.velocity.length() < STOP_SPEED {
            self.velocity = Vec2::ZERO;
            self.coasting = false;
        }
        PanStep { delta: step, panning: false, just_started: false, coasting: self.coasting }
    }

    /// Stop dead (the view hit its end, a key, a click).
    pub fn cancel(&mut self) {
        self.velocity = Vec2::ZERO;
        self.coasting = false;
    }
}
