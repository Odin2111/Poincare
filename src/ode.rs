//! Fixed-step explicit ODE steppers (teaching — not adaptive DAE).

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Method {
    #[default]
    Euler,
    Rk4,
}

impl Method {
    pub fn label(self) -> &'static str {
        match self {
            Self::Euler => "Euler",
            Self::Rk4 => "RK4",
        }
    }
}

/// Advance scalar state `x` with `dx/dt = f(t, x)`.
pub fn step(method: Method, t: f64, x: f64, dt: f64, f: impl Fn(f64, f64) -> f64) -> f64 {
    match method {
        Method::Euler => x + dt * f(t, x),
        Method::Rk4 => {
            let k1 = f(t, x);
            let k2 = f(t + 0.5 * dt, x + 0.5 * dt * k1);
            let k3 = f(t + 0.5 * dt, x + 0.5 * dt * k2);
            let k4 = f(t + dt, x + dt * k3);
            x + (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
        }
    }
}

/// Advance 2-state `[x, v]` with `a = accel(t, x, v)`.
pub fn step2(
    method: Method,
    t: f64,
    mut s: [f64; 2],
    dt: f64,
    accel: impl Fn(f64, f64, f64) -> f64,
) -> [f64; 2] {
    match method {
        Method::Euler => {
            let a = accel(t, s[0], s[1]);
            s[0] += dt * s[1];
            s[1] += dt * a;
            s
        }
        Method::Rk4 => {
            // Treat as ẏ = [v, a(t,x,v)]
            let f = |t: f64, y: [f64; 2]| -> [f64; 2] { [y[1], accel(t, y[0], y[1])] };
            let k1 = f(t, s);
            let y2 = [s[0] + 0.5 * dt * k1[0], s[1] + 0.5 * dt * k1[1]];
            let k2 = f(t + 0.5 * dt, y2);
            let y3 = [s[0] + 0.5 * dt * k2[0], s[1] + 0.5 * dt * k2[1]];
            let k3 = f(t + 0.5 * dt, y3);
            let y4 = [s[0] + dt * k3[0], s[1] + dt * k3[1]];
            let k4 = f(t + dt, y4);
            [
                s[0] + (dt / 6.0) * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]),
                s[1] + (dt / 6.0) * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rk4_exp_decay() {
        // ẋ = −x, x(0)=1 → e^{−t}
        let mut x = 1.0;
        let dt = 0.01;
        for i in 0..100 {
            let t = i as f64 * dt;
            x = step(Method::Rk4, t, x, dt, |_t, x| -x);
        }
        let exact = (-1.0_f64).exp();
        assert!((x - exact).abs() < 1e-6, "x={x} exact={exact}");
    }
}
