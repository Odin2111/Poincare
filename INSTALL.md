# Install Poincare on your machine

The cloud agent cannot create `Odin2111/Poincare` on GitHub from this environment
(`Resource not accessible by integration`). Create the empty repo once, then push.

## One-shot (recommended)

```bash
# from a folder that already has Orchestrate checked out on
# cursor/tooling-validation-mission-31aa (or pull that branch):
cd ~/software
git clone https://github.com/Odin2111/Orchestrate.git
cd Orchestrate
git fetch origin cursor/tooling-validation-mission-31aa
git checkout cursor/tooling-validation-mission-31aa
cp -a deliver/poincare ~/software/Poincare
cd ~/software/Poincare

gh repo create Odin2111/Poincare --public --source=. --remote=origin --push \
  --description "Teaching nonlinear dynamics kernels (Odin stack)"

# or without gh:
# git remote add origin https://github.com/Odin2111/Poincare.git
# # create empty repo on GitHub UI first, then:
# git push -u origin main
```

## Layout next to siblings

```text
~/software/
  thermoflow/
  Madar/
  Controller/
  Orchestrate/
  Poincare/     ← this crate
```

## Check

```bash
cd ~/software/Poincare
cargo test
cargo run --bin poincare_demo
```
