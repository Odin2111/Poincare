# AGENTS.md — Poincare

Pure **Rust** nonlinear-dynamics **library**. No egui. No Python.

**Orchestrate does not implement Poincare.** Orchestrate may courier `poincare.case/v1` JSON and launch sibling apps that link this crate.

## Layout

```text
src/
  lib.rs        re-exports
  ode.rs        Euler / RK4
  plant.rs      MSD, cubic, pendulum, soft-landing
  linearize.rs  operating-point A,B helpers
  spectrum.rs   teaching DFT
  case.rs       poincare.case/v1 serde
```

## Commands

```sh
cargo fmt
cargo test
cargo run --bin poincare_demo
```
