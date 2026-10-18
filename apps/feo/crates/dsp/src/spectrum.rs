//! FFT magnitude analysis for analyzers and EQ displays.

use rustfft::{FftPlanner, num_complex::Complex};

/// Largest FFT size accepted (larger requests are clamped).
pub const MAX_FFT: usize = 1 << 18;

/// Hann-windowed magnitude spectrum in dBFS, `fft_size / 2 + 1` bins (bin `k` is at
/// `k · sr / fft_size` Hz). A full-scale sine reads ≈ 0 dB. Input shorter than `fft_size` is
/// zero-padded; longer input uses the first `fft_size` samples. Floored at -144 dB.
pub fn magnitude_db(samples: &[f32], fft_size: usize) -> Vec<f32> {
    let n = fft_size.min(MAX_FFT);
    if n < 2 {
        return Vec::new();
    }
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(n);
    let mut buf: Vec<Complex<f32>> = (0..n)
        .map(|i| {
            let w = 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / n as f32).cos();
            let x = samples.get(i).copied().filter(|v| v.is_finite()).unwrap_or(0.0);
            Complex::new(x * w, 0.0)
        })
        .collect();
    fft.process(&mut buf);
    // Hann coherent gain is 0.5; a sine's energy splits across ±f.
    let scale = 2.0 / (n as f32 * 0.5);
    buf.iter().take(n / 2 + 1).map(|c| crate::gain_to_db(c.norm() * scale)).collect()
}

/// Frequency in Hz of bin `k` for a given FFT size.
pub fn bin_hz(k: usize, fft_size: usize, sample_rate: f32) -> f32 {
    if fft_size == 0 { 0.0 } else { k as f32 * sample_rate / fft_size as f32 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine_peak_near_zero_db() {
        let sr = 48_000.0;
        let n = 4096;
        let k = 100;
        let f = bin_hz(k, n, sr);
        let x: Vec<f32> = (0..n).map(|i| (std::f32::consts::TAU * f * i as f32 / sr).sin()).collect();
        let m = magnitude_db(&x, n);
        assert_eq!(m.len(), n / 2 + 1);
        assert!(m[k].abs() < 0.1, "{}", m[k]);
        assert!(magnitude_db(&x, 0).is_empty());
    }
}
