//! `poincare.case/v1` — Orchestrate courier payload (validate/copy only).

use crate::ode::Method;
use crate::plant::PlantKind;
use serde::{Deserialize, Serialize};

pub const SCHEMA: &str = "poincare.case/v1";

pub const HONESTY: &str = "Teaching Poincare nonlinear-dynamics case — not Modelica, not certified SIL, not Madar FE Newmark dump. Orchestrate copies this JSON; solvers stay in sibling apps linking the poincare crate.";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseDoc {
    pub schema: String,
    pub honesty: String,
    pub plant: PlantKind,
    pub method: String,
    pub dt_s: f64,
    pub t_end_s: f64,
    pub mass: f64,
    pub spring: f64,
    pub damping: f64,
    pub cubic: f64,
    pub length: f64,
    pub inverted_pendulum: bool,
    pub x0: f64,
    pub v0: f64,
    pub u: f64,
}

impl Default for CaseDoc {
    fn default() -> Self {
        Self {
            schema: SCHEMA.into(),
            honesty: HONESTY.into(),
            plant: PlantKind::Cubic,
            method: Method::Rk4.label().to_ascii_lowercase(),
            dt_s: 0.01,
            t_end_s: 10.0,
            mass: 1.0,
            spring: 1.0,
            damping: 0.1,
            cubic: 0.5,
            length: 1.0,
            inverted_pendulum: false,
            x0: 1.0,
            v0: 0.0,
            u: 0.0,
        }
    }
}

impl CaseDoc {
    pub fn to_pretty_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    pub fn method_enum(&self) -> Method {
        match self.method.to_ascii_lowercase().as_str() {
            "rk4" => Method::Rk4,
            _ => Method::Euler,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let c = CaseDoc::default();
        let j = c.to_pretty_json().unwrap();
        let c2 = CaseDoc::from_json(&j).unwrap();
        assert_eq!(c2.schema, SCHEMA);
        assert_eq!(c2.plant, PlantKind::Cubic);
    }
}
