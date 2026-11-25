# Poincare

Standalone **Rust** teaching library for **nonlinear dynamics** kernels shared across the Odin desktop stack.

| Sibling | Role with Poincare |
| --- | --- |
| [Madar](https://github.com/Odin2111/Madar) | Structural NL / flight GNC stay in Madar for now; plant ODEs can migrate here |
| [Controller](https://github.com/Odin2111/Controller) | Teaching PID bench should depend on Poincare plants / integrators |
| [ThermoFlow](https://github.com/Odin2111/thermoflow) | Process ODEs stay in `tf_ode`; optional shared stepper vocabulary later |
| [Orchestrate](https://github.com/Odin2111/Orchestrate) | **Courier only** — validate/copy `poincare.case/v1`, spawn apps; does **not** own the math |

## What is in v0.1

- Fixed-step ODE: Euler, RK4  
- Nonlinear plants: MSD, cubic spring, pendulum (`sin θ`), soft-landing 1-D hopper  
- Operating-point linearization helpers (MSD / cubic / pendulum)  
- Teaching DFT peak detect  
- Schema stub `poincare.case/v1` for Orchestrate file handoff  

## What is not in v0.1

- Madar co-rotational Newmark / arc-length FE (`nl_dynamics` / `geometric`) — still Madar  
- MIMO LQR / flight mixers — still Madar / Controller  
- CoolProp / cycle close — still ThermoFlow  

## Commands

```sh
cd ~/software/Poincare
cargo test
cargo run --bin poincare_demo
```

## Honesty

Teaching numerics — not Modelica, not certified SIL, not a commercial multibody package.
