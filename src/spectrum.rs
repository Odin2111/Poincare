//! Teaching DFT / PSD peak detect.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpectrumReport {
    pub peak_hz: f64,
    pub peak_power: f64,
    pub median_power: f64,
    pub prominence: f64,
    pub oscillating: bool,
    pub label: String,
}

pub fn periodogram(x: &[f64], dt: f64) -> Vec<(f64, f64)> {
    let n = x.len();
    if n < 8 || dt <= 0.0 {
        return Vec::new();
    }
    let n_use = (n.next_power_of_two() / 2).clamp(8, n);
    let slice = &x[n - n_use..];
    let mut out = Vec::with_capacity(n_use / 2);
    for k in 1..n_use / 2 {
        let mut re = 0.0;
        let mut im = 0.0;
        let w = 2.0 * std::f64::consts::PI * k as f64 / n_use as f64;
        for (i, &xi) in slice.iter().enumerate() {
            let a = w * i as f64;
            re += xi * a.cos();
            im -= xi * a.sin();
        }
        let p = (re * re + im * im) / (n_use * n_use) as f64;
        let hz = k as f64 / (n_use as f64 * dt);
        out.push((hz, p));
    }
    out
}

pub fn peak_report(
    x: &[f64],
    dt: f64,
    prominence_min: f64,
    f_max_hz: f64,
    label: &str,
) -> SpectrumReport {
    let bins = periodogram(x, dt);
    if bins.is_empty() {
        return SpectrumReport {
            label: label.into(),
            ..Default::default()
        };
    }
    let mut powers: Vec<f64> = bins
        .iter()
        .filter(|(f, _)| f_max_hz <= 0.0 || *f <= f_max_hz)
        .map(|(_, p)| *p)
        .collect();
    if powers.is_empty() {
        powers = bins.iter().map(|(_, p)| *p).collect();
    }
    powers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = powers[powers.len() / 2].max(1e-30);
    let mut best = (0.0_f64, 0.0_f64);
    for &(f, p) in &bins {
        if f_max_hz > 0.0 && f > f_max_hz {
            continue;
        }
        if p > best.1 {
            best = (f, p);
        }
    }
    let prom = best.1 / median;
    SpectrumReport {
        peak_hz: best.0,
        peak_power: best.1,
        median_power: median,
        prominence: prom,
        oscillating: prom >= prominence_min,
        label: label.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_tone() {
        let dt = 0.01;
        let f0 = 4.0;
        let x: Vec<_> = (0..512)
            .map(|i| (2.0 * std::f64::consts::PI * f0 * i as f64 * dt).sin())
            .collect();
        let r = peak_report(&x, dt, 3.0, 20.0, "x");
        assert!(r.oscillating);
        assert!((r.peak_hz - f0).abs() < 1.0);
    }
}
