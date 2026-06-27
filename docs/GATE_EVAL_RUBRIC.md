# Gate Evaluation Rubric

_How we decide whether each harness gate earns its place — "right gate, right job, no abuse."_
Methodology template: `docs/stories/US-022-context-rule-measurement.md` (measure before enforce).

## Scoring axes

Each gate is scored on **value** vs **cost**, then assigned a verdict.

- **Value signal** — an *observable* that proves the gate caught a real problem or improved an outcome
  (not "we ran it"). Measured via: `harness-cli audit`, `score-trace`/`score-context`, `query gcr`,
  `recall_eval.py`, or benchmark ablation (when available).
- **Cost signal** — tokens added to agent context, prompts/interrupts, latency, and authoring friction.
- **Verdict** — one of:
  - **KEEP** — value clearly > cost, signal is trustworthy.
  - **FIX** — valuable but the measurement or mechanism is broken/untrustworthy.
  - **DOWNGRADE** — keep as *advisory/opt-in*, not mandatory (cost > value when forced on every lane).
  - **CUT** — cost ≥ value with no path to fix.

## Per-gate rubric + first-pass verdict

| Gate | Value signal (how to prove it helps) | Cost signal | Verdict (evidence) |
|---|---|---|---|
| **T0** Pre-flight | blocks a bad start (dirty tree / wrong base) before work | 1 composite check/session | KEEP — cheap, prevents wasted runs |
| **T1** AC extraction | ACs verbatim → reconciled at T4 (no invented scope) | medium (read ticket) | KEEP — core correctness anchor |
| **M1** Recall | surfaces a *relevant* prior fix that changes the approach | recall query + reading hits | **FIX** — `recall_eval`: 66% stale, precision@3 missed the real query. Untrustworthy until freshness+retire land |
| **T2** Matrix | every AC mapped to a test tier | medium | KEEP for normal/high-risk; **DOWNGRADE** for tiny |
| **T3** Evidence | per-AC proof on disk; catches "claimed pass" | high (run tests, capture) | KEEP for bug-fix/feature; smoke-only for infra/refactor |
| **Review** | reviewer ≠ implementer catches a real defect | 1 agent pass | KEEP for user-facing/high-risk; **DOWNGRADE** to self-verify for docs/refactor |
| **P1** Push permission | stops an un-wanted push/history-rewrite | 1 prompt | KEEP — irreversibility justifies the prompt |
| **P2** Jira tier | no surprise teammate-visible writes | batched prompts | KEEP (batched) — abuse risk if un-batched |
| **T4** Reconciliation | delivered == requested per AC; flags drift | low | KEEP — closes the loop on T1 |
| **M2** Persist | a future session recalls this work usefully | medium (archive+atom) | **conditional** — only worth it if M1 recall is trustworthy (see M1). Today M2 feeds a corpus that's 66% stale → fix M1 first |
| **GCR** metric | measures gate adoption per story | ~0 (one query) | **FIXED** — was miscounting (1/8 → 3/8 for WIN-8254); now counts gate_log T0/M1 + P1=yes |
| **Recall corpus** | reusable knowledge, retrievable | storage | **FIX** — add `verified_at` decay (not mtime), retire/flag stale, code-drift check (tooling: `scripts/recall_eval.py`) |

## Decision rules (anti-abuse)

1. **Lane gates the gate.** tiny lane runs T0+T1+M2 only; normal adds M1/T2/T3/Review/T4; high-risk adds the full set. Never run high-cost gates (T3 evidence, 5-agent review) on tiny/doc/refactor work.
2. **A gate with an untrustworthy signal is FIX, not KEEP** — an unmeasured gate is theater (e.g. M1 today).
3. **Mandatory only if value proven; else DOWNGRADE to advisory** until ablation (G004) shows outcome lift.
4. **Measure cost too** — if a gate adds tokens/prompts without a value signal moving, it's a CUT candidate.

## Open (needs benchmark — deferred, G004)
Empirical per-gate outcome attribution (run agent with/without each gate) requires a `harness-benchmark`
repo that does not yet exist. Until then, verdicts above are evidence-informed (audit/GCR/recall_eval)
but not ablation-proven. Build the benchmark before any CUT of a currently-mandatory gate.
