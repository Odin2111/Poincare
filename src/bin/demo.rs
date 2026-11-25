//! Tiny CLI demo: integrate cubic spring and print a few samples.

use poincare::ode::{step2, Method};
use poincare::plant::cubic_accel;
use poincare::{CaseDoc, CASE_SCHEMA};

fn main() {
    let case = CaseDoc::default();
    println!("schema={CASE_SCHEMA} plant={:?} method={}", case.plant, case.method);
    let mut s = [case.x0, case.v0];
    let dt = case.dt_s;
    let method = case.method_enum();
    for i in 0..=20 {
        let t = i as f64 * dt;
        if i % 5 == 0 {
            println!("t={t:.2} x={:.4} v={:.4}", s[0], s[1]);
        }
        s = step2(method, t, s, dt, |_t, x, v| {
            cubic_accel(case.mass, case.damping, case.spring, case.cubic, x, v, case.u)
        });
    }
    let _ = Method::Euler; // keep demo honest about available methods
}
