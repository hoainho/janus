# Harness Benchmark Report

_Generated: 2026-06-27. Measured-state scorecard — not an ablation benchmark._

---

## What this is / isn't

This is a **measured-state scorecard** synthesized from existing eval assets:
`harness-cli audit`, `harness-cli query gcr`, `scripts/recall_eval.py`, and
`docs/HARNESS_MATURITY.md`. It reports observable numbers from the current harness
corpus and tooling.

This is **NOT** an empirical ablation benchmark. Per-gate outcome attribution
(run agent with gate vs. without gate, measure correctness delta) requires a
dedicated `harness-benchmark` repo that does not yet exist. That work is
deferred as **G004**. Until G004 runs, verdicts in `docs/GATE_EVAL_RUBRIC.md`
are evidence-informed but not ablation-proven. No gate should be CUT on the
basis of this report alone.

---

## Scorecard

| Dimension | Metric | Value | Status |
|---|---|---|---|
| **Maturity level** | Highest fully-achieved H-level | **H4** (H1, H2, H4 achieved; H3, H5 partial) | green |
| **Audit drift** | Entropy score (lower = better) | **10 / 100** — 1 orphaned story (WIN-8254, no traces) | green |
| **GCR — overall** | Gates recorded / gates expected | **3 / 8 = 37.5%** (single story, normal lane) | red |
| **GCR — WIN-8254** | Per-story gate compliance | **37.5%** (red RAG) | red |
| **Recall freshness** | Atoms ≤ 90 days old | **34 % fresh** (57/168); **66.1 % stale** (> 90d) | red |
| **Recall code drift** | Cited path:line still valid | **96.7 %** (2/61 broken) | green |
| **Recall precision@3** | Correct top-1 on golden seed set | **67 %** (2/3 hit; 1 MISS on kinoa query) | yellow |
| **Gate verdicts** | M1 + recall corpus | **FIX** | red |
| **Gate verdicts** | GCR metric | **FIXED** (was 1/8 → now 3/8, correct count) | green |
| **Gate verdicts** | T0, T1, T2, T3, P1, P2, T4 | **KEEP** (7 gates) | green |
| **Gate verdicts** | T2 tiny lane, Review docs/refactor | **DOWNGRADE** (advisory) | yellow |
| **Gate verdicts** | M2 utility | **Conditional** — depends on M1 trust | yellow |

---

## Top-3 Findings + Recommended Next Actions

### 1. Recall corpus is the weakest dimension (red)

66.1% of the 168 atoms in `ws-memories/geargames/` are stale (> 90 days since
`verified_at`). The keyword fallback retriever produced one MISS on a direct
query for the active story (`kinoa popup OnPopupCloseClick`). M1 is rated **FIX**
in the gate rubric for exactly this reason — an untrustworthy recall gate is
theater.

**Recommended actions (in order):**

1. Add a `--retire` flag to `recall_eval.py` that marks atoms with `verified_at`
   > 180d as `status: retired` in their frontmatter, removing them from active
   search.
2. Wire a lightweight decay pass into the M2 write path: after archiving a new
   atom, flag any superseded atom (same `applies_to` + overlapping file refs) as
   `status: superseded`.
3. Expand the `GOLDEN` seed set in `recall_eval.py` from 3 to ≥ 10 verified
   pairs before trusting precision@3 as a stable signal.

### 2. GCR at 37.5% on the only measured story (red)

Only 3 of 8 expected gates were recorded for WIN-8254. The missing gates are
likely not skipped intentionally — they were skipped because the `harness-cli
gate-log record` step was not triggered mid-session. GCR cannot improve until
gate-log calls are embedded in the agent workflow for each gate checkpoint.

**Recommended actions:**

1. Audit the 5 missing gate-log entries for WIN-8254 (run
   `harness-cli query gcr --verbose` once that flag exists) to determine whether
   gates genuinely fired but were not logged, or were skipped entirely.
2. Add a T0 pre-flight check: if any prior story is `in_progress` with GCR < 50%,
   surface a one-line warning before starting new work.

### 3. H3 and H5 are partial — component-level attribution and self-improvement loop are open

H3 requires component-level benchmark attribution (which gate contributed to
which outcome improvement). H5 requires repeated benchmark runs proving the
`propose` → `intervene` → `measure` loop improves the harness over time. Both
block on G004 (harness-benchmark repo).

**Recommended actions:**

1. Start G004: create a minimal `harness-benchmark` repo with 5–10 synthetic
   agent runs (ticket → implement → review → close), half with each gate enabled,
   half without. Measure functional score + lane accuracy across runs.
2. Until G004 completes, do not promote H3 or H5 to "Achieved" and do not CUT
   any currently-mandatory gate based on this scorecard alone.

---

## Appendix — Raw Command Output

### `harness-cli audit`

```
=== Harness Drift Audit ===

Orphaned stories (planned/in-progress, no traces): 1
  - WIN-8254: Kinoa OnPopupCloseClick should only fire on X (close) button

Unverified stories: 0
Unverified decisions: 0
Open backlog without outcomes: 0
Stale stories: 0
Broken tools: 0
Entropy score: 10/100 (lower is better)
```

### `harness-cli query gcr`

```
story_id  lane    recorded  expected  GCR%   rag
--------  ------  --------  --------  -----  ---
WIN-8254  normal  3         8         37.5%  red

Overall: 3/8 gates recorded — avg GCR 37.5% (red)
```

### `python3 scripts/recall_eval.py --today 2026-06-27`

```
# Recall Eval Report (2026-06-27)

Atoms scanned: 168  (dir: /Users/nhonh/Documents/ws-memories/geargames)

## 1. Freshness (by verified_at, not mtime)
- <=30d: 25
- 31-90d: 32
- 91-180d: 111
- >180d: 0
- no-date: 0
- Stale (> 90d): 111/168 = 66.1%

## 2. Code-drift (cited path:line still on disk?)
- refs checked: 61 | valid: 59 | broken: 2 (3.3%)
  - BROKEN: 2026-06-06T05-10-21_geargames_podium-mcp -> lib/exec.test.ts:12
  - BROKEN: 2026-06-06T05-10-21_geargames_podium-mcp -> lib/maestro.ts:48

## 3. precision@3 (seed golden set — degraded fs-grep retriever)
- 2/3 queries hit
  - [HIT] 'paysafe useEffect config id null' -> expect 'paysafe-debug-useeffect'
  - [HIT] 'react debugger tracking side effects' -> expect 'tracking-info-tu-react-debugger'
  - [MISS] 'kinoa popup OnPopupCloseClick close button' -> expect 'kinoa' | top1: 2026-06-25T00-00-00_geargames_new-session-...
```

### Maturity summary (from `docs/HARNESS_MATURITY.md`)

| Level | Status | Evidence |
|---|---|---|
| H0 | Passed (beyond) | No bare env — AGENTS.md + harness docs present |
| H1 | Achieved | AGENTS.md, docs/HARNESS.md, docs/FEATURE_INTAKE.md, templates |
| H2 | Achieved | harness-cli, harness.db, TRACE_SPEC.md, CONTEXT_RULES.md |
| H3 | Partial | score-trace and friction loop present; component benchmark attribution open |
| H4 | Achieved | story verify, story verify-all, trace-time verification warnings |
| H5 | Partial | audit, propose, IMPROVEMENT_PROTOCOL.md; repeated benchmark loop open |
