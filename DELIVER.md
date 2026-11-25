# Delivered from cloud agent

Full Poincare crate snapshot for local GitHub create/push.

See `INSTALL.md` in this folder. Create `Odin2111/Poincare` on GitHub, then:

```bash
cp -a deliver/poincare ~/software/Poincare
cd ~/software/Poincare
gh repo create Odin2111/Poincare --public --source=. --remote=origin --push
```

Orchestrate does **not** own Poincare math — courier only (`poincare.case/v1` later).
