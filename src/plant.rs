//! Nonlinear / linear teaching plants (ODEs) — not Madar FE.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlantKind {
    Msd,
    Cubic,
    Pendulum,
    SoftLanding,
}

impl PlantKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Msd => "mass–spring–damper",
            Self::Cubic => "cubic spring",
            Self::Pendulum => "pendulum",
            Self::SoftLanding => "soft-landing hopper",
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct State2 {
    pub x: f64,
    pub v: f64,
}

impl State2 {
    pub fn as_array(self) -> [f64; 2] {
        [self.x, self.v]
    }

    pub fn from_array(a: [f64; 2]) -> Self {
        Self { x: a[0], v: a[1] }
    }
}

/// mẍ + cẋ + k x = u
pub fn msd_accel(m: f64, c: f64, k: f64, x: f64, v: f64, u: f64) -> f64 {
    (u - c * v - k * x) / m.max(1e-12)
}

/// mẍ + cẋ + k x + k3 x³ = u
pub fn cubic_accel(m: f64, c: f64, k: f64, k3: f64, x: f64, v: f64, u: f64) -> f64 {
    (u - c * v - k * x - k3 * x * x * x) / m.max(1e-12)
}

/// θ̈ = (g/L) sin(θ) + u   (inverted if `inverted`, else hanging −sin)
pub fn pendulum_accel(length: f64, g: f64, inverted: bool, theta: f64, _omega: f64, u: f64) -> f64 {
    let sign = if inverted { 1.0 } else { -1.0 };
    sign * g / length.max(1e-12) * theta.sin() + u
}

/// Soft-landing hopper: ḧ = throttle·T/m − g, throttle in [0,1].
pub fn soft_landing_accel(m: f64, thrust_n: f64, throttle: f64) -> f64 {
    let thr = throttle.clamp(0.0, 1.0);
    thr * thrust_n.max(0.0) / m.max(1e-12) - 9.81
}

/// v_des = −h/τ (positive up).
pub fn soft_landing_v_des(h_m: f64, ttg_s: f64) -> f64 {
    -(h_m.max(0.0) / ttg_s.max(0.2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ode::{step2, Method};

    #[test]
    fn cubic_hardens_vs_linear() {
        let a_lin = msd_accel(1.0, 0.0, 1.0, 2.0, 0.0, 0.0);
        let a_cub = cubic_accel(1.0, 0.0, 1.0, 1.0, 2.0, 0.0, 0.0);
        assert!(a_cub < a_lin); // stronger restoring
    }

    #[test]
    fn pendulum_integrates() {
        let mut s = [0.3, 0.0]; // hanging
        let dt = 0.01;
        for i in 0..200 {
            let t = i as f64 * dt;
            s = step2(Method::Rk4, t, s, dt, |_t, th, w| {
                pendulum_accel(1.0, 9.81, false, th, w, 0.0)
            });
        }
        assert!(s[0].is_finite() && s[1].is_finite());
    }
}
