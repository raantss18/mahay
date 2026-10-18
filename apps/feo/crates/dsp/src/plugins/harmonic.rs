//! Harmonic effects: Saturator (tanh), Lo-Fi (bit/sample-rate reduction) and Rectifier.

use crate::params::{Params, choice, lin, log, param_plumbing};
use crate::util::{DcBlocker, Rng, Smoother, flush, frames_in, sane_channels, sane_sr};
use crate::{Category, ParamInfo, Plugin, PluginInfo, Unit, db_to_gain};

static SAT_PARAMS: [ParamInfo; 5] = [
    lin("drive", "Drive", 0.0, 36.0, 6.0, Unit::Db),
    lin("bias", "Asymmetry", 0.0, 100.0, 0.0, Unit::Percent),
    log("tone", "Tone", 1000.0, 20000.0, 20000.0, Unit::Hz),
    lin("mix", "Mix", 0.0, 100.0, 100.0, Unit::Percent),
    lin("output", "Output", -24.0, 12.0, 0.0, Unit::Db),
];

pub static SAT_INFO: PluginInfo = PluginInfo {
    id: "saturator",
    name: "Saturator",
    short_name: "Satur",
    category: Category::Harmonic,
    params: &SAT_PARAMS,
    is_instrument: false,
    audiosuite: true,
};

/// Smooth tanh waveshaper, normalized so a full-scale input still peaks near full scale (more
/// drive = more harmonics and density). Asymmetry adds even harmonics (the DC is removed).
pub struct Saturator {
    p: Params,
    sr: f32,
    dc: Vec<DcBlocker>,
    lp: Vec<f32>,
    drive: Smoother,
    bias: Smoother,
    mix: Smoother,
    out: Smoother,
    tone: f32,
}

impl Default for Saturator {
    fn default() -> Self {
        Self::new()
    }
}

impl Saturator {
    pub fn new() -> Self {
        let mut s = Self {
            p: Params::new(&SAT_PARAMS),
            sr: 48_000.0,
            dc: Vec::new(),
            lp: Vec::new(),
            drive: Smoother::new(1.0),
            bias: Smoother::new(0.0),
            mix: Smoother::new(1.0),
            out: Smoother::new(1.0),
            tone: 0.0,
        };
        s.update();
        s
    }

    fn update(&mut self) {
        self.drive.set(db_to_gain(self.p.v(0)));
        self.bias.set(self.p.v(1) / 100.0 * 0.5);
        self.mix.set(self.p.v(3) / 100.0);
        self.out.set(db_to_gain(self.p.v(4)));
        let f = self.p.v(2);
        self.tone = if f >= 19_999.0 { 0.0 } else { (-std::f32::consts::TAU * f / self.sr).exp() };
    }
}

impl Plugin for Saturator {
    param_plumbing!(SAT_INFO);

    fn prepare(&mut self, sample_rate: f32, _max_block: usize, channels: usize) {
        self.sr = sane_sr(sample_rate);
        let ch = sane_channels(channels);
        self.dc = vec![DcBlocker::new(10.0, self.sr); ch];
        self.lp = vec![0.0; ch];
        for s in [&mut self.drive, &mut self.bias, &mut self.mix, &mut self.out] {
            s.set_time(20.0, self.sr);
        }
        self.update();
        self.reset();
    }

    fn reset(&mut self) {
        self.dc.iter_mut().for_each(DcBlocker::reset);
        self.lp.iter_mut().for_each(|v| *v = 0.0);
        for s in [&mut self.drive, &mut self.bias, &mut self.mix, &mut self.out] {
            s.snap();
        }
    }

    fn process(&mut self, io: &mut [Vec<f32>], frames: usize) {
        let ch = self.dc.len().min(io.len());
        let frames = frames_in(io, frames, ch);
        let a = self.tone;
        for n in 0..frames {
            let g = self.drive.next();
            let b = self.bias.next();
            let m = self.mix.next();
            let o = self.out.next();
            let tb = b.tanh();
            let norm = 1.0 / ((g + b).tanh() - tb).max(1e-3);
            for c in 0..ch {
                let (Some(buf), Some(dc), Some(lp)) = (io.get_mut(c), self.dc.get_mut(c), self.lp.get_mut(c)) else { continue };
                let Some(x) = buf.get_mut(n) else { continue };
                let dry = if x.is_finite() { *x } else { 0.0 };
                let mut y = ((g * dry + b).tanh() - tb) * norm;
                if b > 0.0 {
                    y = dc.process(y);
                }
                if a > 0.0 {
                    *lp = flush(y + (*lp - y) * a);
                    y = *lp;
                }
                *x = (dry * (1.0 - m) + y * m) * o;
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------

static LOFI_PARAMS: [ParamInfo; 4] = [
    lin("bits", "Bit Depth", 1.0, 24.0, 8.0, Unit::None),
    log("sample_rate", "Sample Rate", 500.0, 48000.0, 11025.0, Unit::Hz),
    lin("noise", "Noise", 0.0, 100.0, 0.0, Unit::Percent),
    lin("mix", "Mix", 0.0, 100.0, 100.0, Unit::Percent),
];

pub static LOFI_INFO: PluginInfo = PluginInfo {
    id: "lofi",
    name: "Lo-Fi",
    short_name: "LoFi",
    category: Category::Harmonic,
    params: &LOFI_PARAMS,
    is_instrument: false,
    audiosuite: true,
};

/// Bit crusher and sample-and-hold decimator with optional hiss.
pub struct LoFi {
    p: Params,
    sr: f32,
    held: Vec<f32>,
    phase: f32,
    rng: Rng,
    mix: Smoother,
    noise: Smoother,
}

impl Default for LoFi {
    fn default() -> Self {
        Self::new()
    }
}

impl LoFi {
    pub fn new() -> Self {
        let mut l = Self {
            p: Params::new(&LOFI_PARAMS),
            sr: 48_000.0,
            held: Vec::new(),
            phase: 1.0,
            rng: Rng::new(77),
            mix: Smoother::new(1.0),
            noise: Smoother::new(0.0),
        };
        l.update();
        l
    }

    fn update(&mut self) {
        self.mix.set(self.p.v(3) / 100.0);
        let n = self.p.v(2) / 100.0;
        self.noise.set(n * n * 0.1);
    }
}

impl Plugin for LoFi {
    param_plumbing!(LOFI_INFO);

    fn prepare(&mut self, sample_rate: f32, _max_block: usize, channels: usize) {
        self.sr = sane_sr(sample_rate);
        self.held = vec![0.0; sane_channels(channels)];
        self.mix.set_time(20.0, self.sr);
        self.noise.set_time(20.0, self.sr);
        self.update();
        self.reset();
    }

    fn reset(&mut self) {
        self.held.iter_mut().for_each(|v| *v = 0.0);
        self.phase = 1.0;
        self.mix.snap();
        self.noise.snap();
    }

    fn process(&mut self, io: &mut [Vec<f32>], frames: usize) {
        let ch = self.held.len().min(io.len());
        let frames = frames_in(io, frames, ch);
        let step = 2f32.powf(1.0 - self.p.v(0));
        let inc = (self.p.v(1) / self.sr).min(1.0);
        for n in 0..frames {
            let m = self.mix.next();
            let nz = self.noise.next();
            self.phase += inc;
            let grab = self.phase >= 1.0;
            if grab {
                self.phase -= 1.0;
            }
            for c in 0..ch {
                let (Some(buf), Some(h)) = (io.get_mut(c), self.held.get_mut(c)) else { continue };
                let Some(x) = buf.get_mut(n) else { continue };
                let dry = if x.is_finite() { *x } else { 0.0 };
                if grab {
                    let noisy = dry + if nz > 0.0 { self.rng.bipolar() * nz } else { 0.0 };
                    *h = (noisy / step).round() * step;
                }
                *x = dry * (1.0 - m) + *h * m;
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------

static RECT_PARAMS: [ParamInfo; 3] = [
    choice("mode", "Mode", &["Half Wave", "Full Wave"], 1),
    lin("mix", "Mix", 0.0, 100.0, 100.0, Unit::Percent),
    lin("output", "Output", -24.0, 12.0, 0.0, Unit::Db),
];

pub static RECT_INFO: PluginInfo = PluginInfo {
    id: "rectifier",
    name: "Rectifier",
    short_name: "Rectify",
    category: Category::Harmonic,
    params: &RECT_PARAMS,
    is_instrument: false,
    audiosuite: true,
};

/// Half/full-wave rectifier (octave-up harmonics) with DC removal.
pub struct Rectifier {
    p: Params,
    sr: f32,
    dc: Vec<DcBlocker>,
    mix: Smoother,
    out: Smoother,
}

impl Default for Rectifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Rectifier {
    pub fn new() -> Self {
        let mut r = Self { p: Params::new(&RECT_PARAMS), sr: 48_000.0, dc: Vec::new(), mix: Smoother::new(1.0), out: Smoother::new(1.0) };
        r.update();
        r
    }

    fn update(&mut self) {
        self.mix.set(self.p.v(1) / 100.0);
        self.out.set(db_to_gain(self.p.v(2)));
    }
}

impl Plugin for Rectifier {
    param_plumbing!(RECT_INFO);

    fn prepare(&mut self, sample_rate: f32, _max_block: usize, channels: usize) {
        self.sr = sane_sr(sample_rate);
        self.dc = vec![DcBlocker::new(10.0, self.sr); sane_channels(channels)];
        self.mix.set_time(20.0, self.sr);
        self.out.set_time(20.0, self.sr);
        self.update();
        self.reset();
    }

    fn reset(&mut self) {
        self.dc.iter_mut().for_each(DcBlocker::reset);
        self.mix.snap();
        self.out.snap();
    }

    fn process(&mut self, io: &mut [Vec<f32>], frames: usize) {
        let ch = self.dc.len().min(io.len());
        let frames = frames_in(io, frames, ch);
        let full = self.p.choice(0) == 1;
        for n in 0..frames {
            let m = self.mix.next();
            let o = self.out.next();
            for c in 0..ch {
                let (Some(buf), Some(dc)) = (io.get_mut(c), self.dc.get_mut(c)) else { continue };
                let Some(x) = buf.get_mut(n) else { continue };
                let dry = if x.is_finite() { *x } else { 0.0 };
                let r = if full { dry.abs() } else { dry.max(0.0) };
                let wet = dc.process(r);
                *x = (dry * (1.0 - m) + wet * m) * o;
            }
        }
    }
}
