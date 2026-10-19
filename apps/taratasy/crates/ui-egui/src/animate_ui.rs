//! Playing LaTeX `animate` players (#169) on the page, without their scripts.
//!
//! [`taratasy_render::animate`] says which frame widget to show; this module owns the clock,
//! the clicks and the pictures. Each frame is drawn once, on a worker thread with its own
//! renderer, as a small raster of the player's rectangle (and its control bar) at the current
//! zoom, then kept as a texture: the first time through, the animation advances as fast as
//! frames arrive (never skipping one); after that, playing only swaps textures. The page raster
//! underneath shows the document as saved (frame 1, stopped), so nothing is drawn over it until
//! the player leaves that state.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};

use egui::{TextureHandle, TextureOptions};
use taratasy_render::animate::{Animation, ControlAction, Player};
use taratasy_render::{DocInfo, RenderRequest, RequestKind, Tile, device_pixels};

use crate::canvas::PageXform;

/// Frames rendered ahead of the one on screen while playing.
const PREFETCH: usize = 4;
/// Most frame pictures kept for one document (each is a few hundred kilobytes at most).
const MAX_TEXTURES: usize = 240;

/// A frame picture: (page, player number, frame, playing).
type Key = (usize, u32, usize, bool);

struct Job {
    key: Key,
    scale_bits: u32,
    req: RenderRequest,
    visibility: Arc<Vec<(i32, i32, bool)>>,
}

struct Done {
    key: Key,
    scale_bits: u32,
    image: Option<egui::ColorImage>,
}

/// The renderer thread of one document: frames go in as jobs and come back as pictures.
struct Worker {
    display: Arc<Vec<u8>>,
    jobs: Sender<Job>,
    done: Receiver<Done>,
}

impl Worker {
    fn start(doc: &taratasy_engine::Document, ctx: &egui::Context) -> Option<Self> {
        let (jobs, inbox) = channel::<Job>();
        let (outbox, done) = channel::<Done>();
        let mut renderer = doc.animation_renderer();
        let ctx = ctx.clone();
        std::thread::Builder::new()
            .name("taratasy-animate".into())
            .spawn(move || {
                while let Ok(job) = inbox.recv() {
                    let page = renderer.render_annotation_variant(job.req, job.visibility);
                    let image = (page.error.is_none() && page.width > 0 && page.height > 0)
                        .then(|| egui::ColorImage::from_rgba_premultiplied([page.width as usize, page.height as usize], &page.rgba));
                    if outbox.send(Done { key: job.key, scale_bits: job.scale_bits, image }).is_err() {
                        break;
                    }
                    ctx.request_repaint();
                }
            })
            .ok()?;
        Some(Self { display: doc.display.clone(), jobs, done })
    }
}

#[derive(Default)]
struct State {
    player: Player,
    /// When the frame on screen came up (egui time, seconds).
    shown_at: f64,
}

/// The `animate` players of one document view.
#[derive(Default)]
pub struct Players {
    states: HashMap<(usize, u32), State>,
    worker: Option<Worker>,
    /// Pictures at `scale_bits`, the zoom they were drawn at.
    textures: HashMap<Key, TextureHandle>,
    scale_bits: u32,
    pending: HashSet<Key>,
}

impl Players {
    /// Where the player `number` on `page` is: its frame (0-based) and whether it is playing.
    /// `None` until its page has been drawn.
    pub fn state(&self, page: usize, number: u32) -> Option<(usize, bool)> {
        self.states.get(&(page, number)).map(|s| (s.player.frame, s.player.playing.is_some()))
    }

    /// Take in finished frames; drop everything when the document or the zoom changed.
    fn sync(&mut self, ctx: &egui::Context, doc: &taratasy_engine::Document, scale: f32) {
        if self.worker.as_ref().is_some_and(|w| !Arc::ptr_eq(&w.display, &doc.display)) {
            *self = Self::default();
        }
        if scale.to_bits() != self.scale_bits {
            self.scale_bits = scale.to_bits();
            self.textures.clear();
            self.pending.clear();
        }
        let Some(worker) = &self.worker else { return };
        while let Ok(done) = worker.done.try_recv() {
            self.pending.remove(&done.key);
            if done.scale_bits != self.scale_bits {
                continue;
            }
            if let Some(image) = done.image {
                if self.textures.len() >= MAX_TEXTURES {
                    self.textures.clear();
                }
                let name = format!("animate-{}-{}-{}", done.key.0, done.key.1, done.key.2);
                self.textures.insert(done.key, ctx.load_texture(name, image, TextureOptions::LINEAR));
            }
        }
    }

    /// Ask for the picture of `anim` in `player`'s state, unless it is there or on its way.
    fn request(&mut self, ctx: &egui::Context, doc: &taratasy_engine::Document, info: &DocInfo, anim: &Animation, player: Player, scale: f32) {
        let key = (anim.page, anim.number, player.frame, player.playing.is_some());
        if self.textures.contains_key(&key) || self.pending.contains(&key) {
            return;
        }
        let Some(tile) = tile(info, anim, scale) else { return };
        if self.worker.is_none() {
            self.worker = Worker::start(doc, ctx);
        }
        let Some(worker) = &self.worker else { return };
        let req = RenderRequest { page: anim.page, kind: RequestKind::Pixels, tile: Some(tile.0), scale, tag: 0, preview: false };
        let job = Job { key, scale_bits: scale.to_bits(), req, visibility: Arc::new(player.visibility(anim)) };
        if worker.jobs.send(job).is_ok() {
            self.pending.insert(key);
        }
    }
}

/// What a player covers: the display widget and its control bar, in user space.
fn area(anim: &Animation) -> [f32; 4] {
    anim.controls.iter().fold(anim.rect, |r, c| [r[0].min(c.rect[0]), r[1].min(c.rect[1]), r[2].max(c.rect[2]), r[3].max(c.rect[3])])
}

/// The device-pixel tile of the page raster that covers `anim` at `scale`, and the same region
/// as fractions of the page (u0, v0, u1, v1) for painting it back.
fn tile(info: &DocInfo, anim: &Animation, scale: f32) -> Option<(Tile, [f32; 4])> {
    let p = info.pages.get(anim.page)?;
    let r = area(anim);
    let (pw, ph) = (p.width.max(1.0), p.height.max(1.0));
    let (dw, dh) = (device_pixels(pw, scale), device_pixels(ph, scale));
    let a = p.user_to_view(r[0], r[1]);
    let b = p.user_to_view(r[2], r[3]);
    let (u0, u1) = ((a[0].min(b[0]) / pw).clamp(0.0, 1.0), (a[0].max(b[0]) / pw).clamp(0.0, 1.0));
    let (v0, v1) = ((a[1].min(b[1]) / ph).clamp(0.0, 1.0), (a[1].max(b[1]) / ph).clamp(0.0, 1.0));
    let x = (u0 * dw as f32).floor() as u32;
    let y = (v0 * dh as f32).floor() as u32;
    let x1 = ((u1 * dw as f32).ceil() as u32).min(dw);
    let y1 = ((v1 * dh as f32).ceil() as u32).min(dh);
    let (w, h) = (x1.checked_sub(x)?, y1.checked_sub(y)?);
    if w == 0 || h == 0 || w > 4096 || h > 4096 {
        return None;
    }
    // Paint back exactly the pixels drawn, not the unrounded region.
    let norm = [x as f32 / dw as f32, y as f32 / dh as f32, x1 as f32 / dw as f32, y1 as f32 / dh as f32];
    Some((Tile { x, y, w, h }, norm))
}

/// The control-bar button under `p` that the player shows in its current state.
fn control_at(xf: &PageXform, info: &DocInfo, anim: &Animation, player: &Player, p: egui::Pos2) -> Option<ControlAction> {
    let playing = player.playing.is_some();
    anim.controls
        .iter()
        .filter(|c| match c.action {
            ControlAction::Play { .. } => !playing,
            ControlAction::Pause { .. } => playing,
            _ => true,
        })
        .find(|c| xf.user_rect(info, anim.page, c.rect).contains(p))
        .map(|c| c.action)
}

/// Draw and run the players on page `page`. Returns whether a click went to one of them (the
/// form layer then leaves it alone: their frames are push buttons).
#[allow(clippy::too_many_arguments)]
pub(crate) fn page(
    ui: &egui::Ui,
    painter: &egui::Painter,
    resp: &egui::Response,
    xf: &PageXform,
    page: usize,
    info: &DocInfo,
    doc: &taratasy_engine::Document,
    players: &mut Players,
    scale: f32,
    interactive: bool,
) -> bool {
    if !info.animations.iter().any(|a| a.page == page) {
        return false;
    }
    let ctx = ui.ctx().clone();
    players.sync(&ctx, doc, scale);
    let now = ui.input(|i| i.time);
    let pointer = ui.input(|i| i.pointer.interact_pos());
    let mut consumed = false;
    for anim in info.animations.iter().filter(|a| a.page == page) {
        let state = players.states.entry((page, anim.number)).or_default();
        // Clicks: a control-bar button, or the animation itself.
        if interactive && let Some(p) = pointer {
            let control = control_at(xf, info, anim, &state.player, p);
            let on_player = xf.user_rect(info, page, anim.rect).contains(p);
            if control.is_some() || on_player {
                ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                if resp.clicked() {
                    match control {
                        Some(action) => state.player.control(anim, action),
                        None => state.player.click(anim),
                    }
                    state.shown_at = now;
                    consumed = true;
                }
            }
        }
        // The clock: move on once the frame's time is up and the next picture is ready.
        let mut player = state.player;
        if let Some(next) = player.next_frame(anim) {
            let ready = players.textures.contains_key(&(page, anim.number, next, true));
            let due = now - state.shown_at >= player.interval(anim);
            if due && ready {
                player.advance(anim);
                state.shown_at = now;
            }
            ctx.request_repaint_after(std::time::Duration::from_secs_f64((player.interval(anim) - (now - state.shown_at)).max(0.005)));
        } else if player.playing.is_some() {
            // At the end: stop there.
            player.advance(anim);
        }
        state.player = player;
        if player.is_initial() {
            continue;
        }
        // Pictures: this one, then the next few while playing.
        players.request(&ctx, doc, info, anim, player, scale);
        if player.playing.is_some() {
            let mut ahead = player;
            for _ in 0..PREFETCH {
                match ahead.next_frame(anim) {
                    Some(f) => ahead.frame = f,
                    None => break,
                }
                players.request(&ctx, doc, info, anim, ahead, scale);
            }
        }
        let key = (page, anim.number, player.frame, player.playing.is_some());
        let shown = players.textures.get(&key).or_else(|| {
            // Until it arrives, the nearest earlier frame drawn in either state.
            (0..player.frame)
                .rev()
                .find_map(|f| players.textures.get(&(page, anim.number, f, true)).or_else(|| players.textures.get(&(page, anim.number, f, false))))
        });
        if let (Some(tex), Some((_, [u0, v0, u1, v1]))) = (shown, tile(info, anim, scale)) {
            xf.paint_image(painter, tex.id(), u0, v0, u1, v1);
        }
    }
    consumed
}
