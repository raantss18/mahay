//! LaTeX `animate` players (#169), played without JavaScript.
//!
//! The `animate` package draws each frame of an animation as a push button that is hidden
//! (`/F 2`) and named `<n>.<frame>`, all on the same rectangle. The player is the visible push
//! button `anm<n>`, which shows frame 0; a document-level script swaps its icon for the next
//! frame's on a timer (`app.setInterval`), and the optional control bar is more push buttons
//! named `<n>.PlayRight`, `<n>.PauseRight`, `<n>.StepLeft` and so on. Taratasy's script sandbox
//! has no timers, so the viewer plays these itself: it hides `anm<n>` and shows one frame
//! widget at a time ([`Player::visibility`]), at the rate the script gives (`a<n>_fps=…`).
//!
//! Everything here is pure: the inventory comes from [`crate::inspect`], and the viewer owns
//! the clock.

/// One `animate` player on a page.
#[derive(Clone, Debug, PartialEq)]
pub struct Animation {
    pub page: usize,
    /// The package's own number for it (`anm<n>`).
    pub number: u32,
    /// Where it is drawn: the display widget's `/Rect`, in user space.
    pub rect: [f32; 4],
    /// The display widget (`anm<n>`): object number and generation.
    pub display: (u32, u16),
    /// The frame widgets in order (`<n>.0`, `<n>.1`, …).
    pub frames: Vec<(u32, u16)>,
    /// Frames per second at normal speed.
    pub fps: f32,
    /// The control bar's buttons, if the document has one.
    pub controls: Vec<Control>,
}

/// A button of an `animate` control bar.
#[derive(Clone, Debug, PartialEq)]
pub struct Control {
    pub action: ControlAction,
    pub rect: [f32; 4],
    pub id: (u32, u16),
}

/// What a control-bar button does; `forward` is the package's "Right".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlAction {
    /// Shown while stopped or paused: play in that direction.
    Play {
        forward: bool,
    },
    /// Shown while playing: pause.
    Pause {
        forward: bool,
    },
    /// One button for both.
    PlayPause {
        forward: bool,
    },
    /// One frame (only while not playing).
    Step {
        forward: bool,
    },
    /// Stop at the last frame (`forward`) or the first.
    End {
        forward: bool,
    },
    Faster,
    Slower,
    NormalSpeed,
}

impl ControlAction {
    /// The action of the button named `<n>.<suffix>`.
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        let dir = |s: &str| match s {
            "Right" => Some(true),
            "Left" => Some(false),
            _ => None,
        };
        Some(match suffix {
            "Plus" => Self::Faster,
            "Minus" => Self::Slower,
            "Reset" => Self::NormalSpeed,
            s if s.starts_with("PlayPause") => Self::PlayPause { forward: dir(&s[9..])? },
            s if s.starts_with("Play") => Self::Play { forward: dir(&s[4..])? },
            s if s.starts_with("Pause") => Self::Pause { forward: dir(&s[5..])? },
            s if s.starts_with("Step") => Self::Step { forward: dir(&s[4..])? },
            s if s.starts_with("End") => Self::End { forward: dir(&s[3..])? },
            _ => return None,
        })
    }
}

/// Fewest and most frames per second a player runs at (a script can ask for any number).
const MIN_FPS: f32 = 0.1;
const MAX_FPS: f32 = 60.0;
/// The package's default rate, when the script can't be read.
pub const DEFAULT_FPS: f32 = 18.0;

/// The rate `a<number>_fps=<value>;` sets in an `animate` script, within [`MIN_FPS`] and
/// [`MAX_FPS`]; `None` when the script doesn't say.
pub fn script_fps(script: &str, number: u32) -> Option<f32> {
    let key = format!("a{number}_fps=");
    let at = script.find(&key)?.checked_add(key.len())?;
    let rest = script.get(at..)?;
    let end = rest.find(|c: char| !(c.is_ascii_digit() || c == '.')).unwrap_or(rest.len());
    let fps: f32 = rest.get(..end)?.parse().ok()?;
    fps.is_finite().then(|| fps.clamp(MIN_FPS, MAX_FPS))
}

/// Where an `animate` player is in its animation. Starts stopped on the first frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub frame: usize,
    /// `Some(forward)` while playing.
    pub playing: Option<bool>,
    /// Paused part-way (as opposed to stopped): playing again resumes from here.
    pub paused: bool,
    /// Speed factor (the control bar's Plus and Minus change it by 10 %).
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { frame: 0, playing: None, paused: false, speed: 1.0 }
    }
}

impl Player {
    /// Seconds between frames for `anim`.
    pub fn interval(&self, anim: &Animation) -> f64 {
        1.0 / f64::from((anim.fps * self.speed).clamp(MIN_FPS, MAX_FPS * 4.0))
    }

    fn last(anim: &Animation) -> usize {
        anim.frames.len().saturating_sub(1)
    }

    fn play(&mut self, anim: &Animation, forward: bool) {
        // From a stop, start over (at the far end when playing backwards), as the package does.
        if !self.paused || (forward && self.frame >= Self::last(anim)) || (!forward && self.frame == 0) {
            self.frame = if forward { 0 } else { Self::last(anim) };
        }
        self.playing = Some(forward);
        self.paused = false;
    }

    fn pause(&mut self) {
        if self.playing.is_some() {
            self.playing = None;
            self.paused = true;
        }
    }

    /// A click on the animation itself: play forward, or pause while playing.
    pub fn click(&mut self, anim: &Animation) {
        if self.playing.is_some() { self.pause() } else { self.play(anim, true) }
    }

    /// A control-bar button.
    pub fn control(&mut self, anim: &Animation, action: ControlAction) {
        match action {
            ControlAction::Play { forward } => self.play(anim, forward),
            ControlAction::Pause { .. } => self.pause(),
            ControlAction::PlayPause { forward } => {
                if self.playing.is_some() {
                    self.pause()
                } else {
                    self.play(anim, forward)
                }
            }
            ControlAction::Step { forward } => {
                if self.playing.is_none() {
                    self.frame = if forward { (self.frame + 1).min(Self::last(anim)) } else { self.frame.saturating_sub(1) };
                    self.paused = true;
                }
            }
            ControlAction::End { forward } => {
                self.playing = None;
                self.paused = false;
                self.frame = if forward { Self::last(anim) } else { 0 };
            }
            ControlAction::Faster => self.speed = (self.speed * 1.1).min(4.0),
            ControlAction::Slower => self.speed = (self.speed / 1.1).max(0.1),
            ControlAction::NormalSpeed => self.speed = 1.0,
        }
    }

    /// The frame playing shows next, or `None` when stopped. Playing stops at the last frame
    /// (the first, backwards), as the package does without its `loop` option.
    pub fn next_frame(&self, anim: &Animation) -> Option<usize> {
        match self.playing? {
            true if self.frame < Self::last(anim) => Some(self.frame + 1),
            false if self.frame > 0 => Some(self.frame - 1),
            _ => None,
        }
    }

    /// Move on to the next frame; stop when there is none.
    pub fn advance(&mut self, anim: &Animation) {
        match self.next_frame(anim) {
            Some(f) => self.frame = f,
            None => {
                self.playing = None;
                self.paused = false;
            }
        }
    }

    /// The annotation visibility that draws this state (for
    /// [`crate::PageRenderer::render_annotation_variant`]): the display widget hidden, the
    /// current frame shown, and the control bar's Play buttons swapped for Pause buttons while
    /// playing.
    pub fn visibility(&self, anim: &Animation) -> Vec<(i32, i32, bool)> {
        let id = |(n, g): (u32, u16)| (i32::try_from(n).unwrap_or(i32::MAX), i32::from(g));
        let mut out = Vec::with_capacity(2 + anim.controls.len());
        let (n, g) = id(anim.display);
        out.push((n, g, false));
        if let Some(&frame) = anim.frames.get(self.frame) {
            let (n, g) = id(frame);
            out.push((n, g, true));
        }
        let playing = self.playing.is_some();
        for c in &anim.controls {
            let shown = match c.action {
                ControlAction::Play { .. } => !playing,
                ControlAction::Pause { .. } => playing,
                _ => continue,
            };
            let (n, g) = id(c.id);
            out.push((n, g, shown));
        }
        out
    }

    /// Whether drawing this state needs anything but the page as it is (frame 0, stopped).
    pub fn is_initial(&self) -> bool {
        self.frame == 0 && self.playing.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn anim(frames: usize) -> Animation {
        Animation {
            page: 0,
            number: 3,
            rect: [0.0, 0.0, 100.0, 100.0],
            display: (10, 0),
            frames: (0..frames).map(|i| (20 + i as u32, 0)).collect(),
            fps: 10.0,
            controls: vec![
                Control { action: ControlAction::Play { forward: true }, rect: [0.0; 4], id: (90, 0) },
                Control { action: ControlAction::Pause { forward: true }, rect: [0.0; 4], id: (91, 0) },
            ],
        }
    }

    #[test]
    fn fps_comes_from_the_script() {
        assert_eq!(script_fps("var x;a8_fps=18;a8_spd=1;", 8), Some(18.0));
        assert_eq!(script_fps("a12_fps=2.5;", 12), Some(2.5));
        assert_eq!(script_fps("a1_fps=18;", 8), None);
        assert_eq!(script_fps("a8_fps=99999;", 8), Some(MAX_FPS));
        assert_eq!(script_fps("a8_fps=;", 8), None);
    }

    #[test]
    fn control_names_follow_the_package() {
        assert_eq!(ControlAction::from_suffix("PlayRight"), Some(ControlAction::Play { forward: true }));
        assert_eq!(ControlAction::from_suffix("PlayPauseLeft"), Some(ControlAction::PlayPause { forward: false }));
        assert_eq!(ControlAction::from_suffix("EndLeft"), Some(ControlAction::End { forward: false }));
        assert_eq!(ControlAction::from_suffix("Reset"), Some(ControlAction::NormalSpeed));
        assert_eq!(ControlAction::from_suffix("Play"), None);
        assert_eq!(ControlAction::from_suffix("7"), None);
    }

    #[test]
    fn a_click_plays_to_the_end_and_stops_there_then_starts_over() {
        let a = anim(3);
        let mut p = Player::default();
        p.click(&a);
        assert_eq!(p.playing, Some(true));
        p.advance(&a);
        p.advance(&a);
        assert_eq!((p.frame, p.playing), (2, Some(true)));
        p.advance(&a);
        assert_eq!((p.frame, p.playing), (2, None), "stops on the last frame");
        p.click(&a);
        assert_eq!((p.frame, p.playing), (0, Some(true)), "a click after the end starts over");
    }

    #[test]
    fn pause_resumes_where_it_stopped() {
        let a = anim(5);
        let mut p = Player::default();
        p.click(&a);
        p.advance(&a);
        p.click(&a);
        assert_eq!((p.frame, p.playing, p.paused), (1, None, true));
        p.click(&a);
        assert_eq!((p.frame, p.playing), (1, Some(true)));
    }

    #[test]
    fn steps_and_ends_stay_in_range() {
        let a = anim(2);
        let mut p = Player::default();
        p.control(&a, ControlAction::Step { forward: false });
        assert_eq!(p.frame, 0);
        p.control(&a, ControlAction::Step { forward: true });
        p.control(&a, ControlAction::Step { forward: true });
        assert_eq!(p.frame, 1);
        p.control(&a, ControlAction::End { forward: false });
        assert_eq!(p.frame, 0);
        let empty = anim(0);
        let mut q = Player::default();
        q.click(&empty);
        q.advance(&empty);
        assert_eq!((q.frame, q.playing), (0, None));
    }

    #[test]
    fn visibility_shows_one_frame_and_swaps_play_for_pause() {
        let a = anim(3);
        let mut p = Player::default();
        p.click(&a);
        p.advance(&a);
        let v = p.visibility(&a);
        assert!(v.contains(&(10, 0, false)), "the display widget is hidden");
        assert!(v.contains(&(21, 0, true)), "frame 1 is shown");
        assert!(v.contains(&(90, 0, false)) && v.contains(&(91, 0, true)));
        assert!(!p.is_initial());
    }

    #[test]
    fn speed_is_bounded() {
        let a = anim(2);
        let mut p = Player::default();
        for _ in 0..100 {
            p.control(&a, ControlAction::Faster);
        }
        assert!(p.interval(&a) > 0.0);
        p.control(&a, ControlAction::NormalSpeed);
        assert!((p.interval(&a) - 0.1).abs() < 1e-6);
    }
}
