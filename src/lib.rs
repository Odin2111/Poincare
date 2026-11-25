//! Poincare — teaching nonlinear dynamics kernels.
//!
//! Shared math for Odin apps. Orchestrate is file courier only — not a solver host.
//! Structural FE Newmark / arc-length remain in Madar until extracted.

pub mod case;
pub mod linearize;
pub mod ode;
pub mod plant;
pub mod spectrum;

pub use case::{CaseDoc, SCHEMA as CASE_SCHEMA};
pub use linearize::{cubic_linearize, msd_linearize, pendulum_linearize, LinearPlant};
pub use ode::{step, Method};
pub use plant::{
    cubic_accel, msd_accel, pendulum_accel, soft_landing_accel, soft_landing_v_des, PlantKind,
    State2,
};
pub use spectrum::{peak_report, periodogram, SpectrumReport};
