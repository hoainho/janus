# Agent Instructions

## CRITICAL: EVAL-HARNESS Pre-Check

**BEFORE ANY WORK**, run EVAL-HARNESS checklist from `docs/EVAL_HARNESS.md`:

1. **Check readiness** — Are all prerequisites met?
2. **Ask questions** — If < 90% ready, ask Product (user) to fill gaps
3. **Wait for answers** — Do NOT proceed without clarity
4. **Re-eval** — Check again after answers
5. **Only then** — Proceed to Feature Intake

**Better to ask 3 questions upfront than build wrong thing.**

---

<!-- HARNESS:BEGIN -->
## Harness

This repo uses Harness. Before work, read:

- `README.md`
- `docs/EVAL_HARNESS.md` — **FIRST: Pre-check gate**
- `docs/HARNESS.md` — Operating model
- `docs/FEATURE_INTAKE.md` — Risk classification
- `docs/GATE_EVAL_RUBRIC.md` — Gate evaluation criteria
- `docs/TEST_MATRIX.md` — Validation matrix

Use the Rust Harness CLI at `scripts/bin/harness-cli` on macOS/Linux or
`scripts/bin/harness-cli.exe` on Windows as the main operational tool.

### Quick Commands

```bash
# Initialize database
scripts/bin/harness-cli init

# Check harness state
scripts/bin/harness-cli query matrix
scripts/bin/harness-cli query stories
scripts/bin/harness-cli query decisions

# Record intake
scripts/bin/harness-cli intake --type <type> --summary "<text>" --lane <lane>
```

### Workflow

```text
1. EVAL-HARNESS (pre-check) ← START HERE
2. Feature Intake (classify risk)
3. Gate Evaluation (per rubric)
4. Implementation
5. Validation
6. Archive
```

<!-- HARNESS:END -->
