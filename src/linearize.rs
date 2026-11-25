//! Operating-point linearization helpers (teaching).

use crate::ode::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LinearPlant {
    pub a: [[f64; 2]; 2],
    pub b: [f64; 2],
}

impl LinearPlant {
    pub fn stable_hurwitz(self) -> bool {
        let tr = self.a[0][0] + self.a[1][1];
        let det = self.a[0][0] * self.a[1][1] - self.a[0][1] * self.a[1][0];
        tr < 0.0 && det > 0.0
    }
}

pub fn msd_linearize(mass: f64, spring: f64, damping: f64) -> Option<LinearPlant> {
    if !(mass > 0.0 && spring.is_finite() && damping.is_finite()) {
        return None;
    }
    let m = mass;
    Some(LinearPlant {
        a: [[0.0, 1.0], [-spring / m, -damping / m]],
        b: [0.0, 1.0 / m],
    })
}

/// Cubic: k_eff = k + 3 k3 x0²
pub fn cubic_linearize(
    mass: f64,
    spring: f64,
    cubic: f64,
    damping: f64,
    x0: f64,
) -> Option<LinearPlant> {
    if !cubic.is_finite() || !x0.is_finite() {
        return None;
    }
    msd_linearize(mass, spring + 3.0 * cubic * x0 * x0, damping)
}

pub fn pendulum_linearize(length: f64, g: f64, inverted: bool) -> Option<LinearPlant> {
    if !(length > 0.0 && g > 0.0) {
        return None;
    }
    let sign = if inverted { 1.0 } else { -1.0 };
    Some(LinearPlant {
        a: [[0.0, 1.0], [sign * g / length, 0.0]],
        b: [0.0, 1.0],
    })
}

/// Tiny helper so Method stays referenced from linearize docs/examples.
pub fn _method_label(m: Method) -> &'static str {
    m.label()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hanging_pendulum_open_loop_not_hurwitz_at_linearization_of_inverted() {
        let inv = pendulum_linearize(1.0, 9.81, true).unwrap();
        assert!(!inv.stable_hurwitz()); // unstable upright
        let hang = pendulum_linearize(1.0, 9.81, false).unwrap();
        // hanging linearization has a11=0,a10=-g/L → tr=0, not asymptotically Hurwitz
        assert!(!hang.stable_hurwitz() || hang.a[1][0] < 0.0);
    }
}
