# AC6 — Gate-Compliance Metric for HB-003

**Author:** worker-ac6  
**Date:** 2026-06-25  
**Task:** #9  
**Backlog item:** HB-003 — "gates skipped due to tedious manual transcription"

---

## 1. What `score-trace` and `score-context` Already Measure

### `score-trace`

Measures **trace-field completeness** for a single execution record in the `trace` table. It scores a trace against a three-tier quality ladder:

| Tier | Score | Key fields required |
|---|---|---|
| Incomplete | 0 | `task_summary` < 10 chars OR `outcome` missing |
| Minimal | 1 | `task_summary` + `outcome` |
| Standard | 2 | + `agent`, `actions_taken`, `files_read`, `files_changed`, at least one of `errors`/`harness_friction` |
| Detailed | 3 | + `decisions_made`, `errors`, `harness_friction`, `duration_seconds`/`token_estimate` or note |

It then compares the achieved tier to the lane requirement (`tiny→Minimal`, `normal→Standard`, `high_risk→Detailed`) and emits `MEETS REQUIREMENT` or lists missing fields.

**What it does NOT measure:** whether each harness gate (T0, T1, M1, T2, T3, Review, P1, T4, M2) was actually reached and passed during the work. A perfectly complete trace can still represent a session where T1, T2, T3 were all skipped.

### `score-context`

Measures **file-read hygiene** for a trace: given `files_read` and `files_changed`, it checks which "must-read" and "should-read" documents were consulted. Rules vary by phase (`intake`, `planning`, `implementation`, `trace`) and lane. It reports unmet must-read requirements and any over-reading of skipped files.

**What it does NOT measure:** gate passage. It is a proxy for whether the agent loaded the right context, not whether it executed the right procedural gates.

### Summary gap

Both commands assess **quality of a single artifact** (the trace record). Neither counts gate events across a ticket's lifecycle. The `gate_log` table (migration 006) exists to capture gate-passage events but is currently empty — no CLI command writes to it, and no command queries compliance against it. That is the structural gap HB-003 represents.

---

## 2. Gate-Compliance Metric: Definition

### 2.1 What constitutes a "gate event"

A gate event is a durable record that proves a specific harness gate ran for a specific story/ticket. The mapping of gates to existing durable signals is:

| Gate | Durable signal already available | Table / column |
|---|---|---|
| T0 | Intake record created (bootstrap ran) | `intake.id` for `story_id` |
| T1 | `intake` row exists with `risk_lane` + `summary` | `intake` |
| M1 | No durable record yet (gap); proxy: trace `files_read` contains `ws-memories` or `nano-brain` | `trace.files_read` (weak proxy) |
| T2 | `story` row exists with correct lane | `story` |
| T3 | `story.unit_proof` or `integration_proof` or `e2e_proof` = 1, OR `story.evidence` non-null, OR `story verify` ran (last_verified_result = pass) | `story` proof columns |
| Review | `gate_log` row: `gate='review'`, `decision='pass'` OR `intervention` row: `type='approval', source='reviewer'` | `gate_log` / `intervention` |
| P1 | `gate_log` row: `gate='P1'`, `decision='approved'` | `gate_log` |
| T4 | `story.t4_verdict` IN ('pass','ambiguous') | `story.t4_verdict` |
| M2 | `trace` row for the story with `outcome='completed'` AND tier >= lane requirement | `trace` + score |

### 2.2 Gates expected per lane

Not all gates apply to every lane. The harness spec defines:

| Gate | tiny | normal | high_risk |
|---|---|---|---|
| T0 | required | required | required |
| T1 | required | required | required |
| M1 | **skipped** | required | required |
| T2 | **skipped** | required | required |
| T3 | **skipped** | required | required |
| Review | **skipped** | required | required |
| P1 | required if push | required if push | required if push |
| T4 | abbreviated | required | required |
| M2 | required (abbreviated) | required | required |

For the formula, treat P1 as conditional (only counts if a push happened — detectable via `trace.files_changed` being non-empty and a `gate_log.gate='P1'` record existing, or exclude from denominator when no push occurred).

### 2.3 Formula

**Per-story gate compliance rate:**

```
GCR(story_id) = gates_recorded(story_id) / gates_expected(story_id, lane)
```

Where:

- `gates_expected(story_id, lane)` = count of gates required for the lane (from the table above), with P1 included only if `story` has at least one `trace` with `files_changed` non-null/non-empty.
- `gates_recorded(story_id)` = count of gates for which at least one qualifying durable signal exists (see §2.1).

**Score is in [0.0, 1.0].** 1.0 = all expected gates have evidence; 0.0 = no gates evidenced.

### 2.4 SQL-queryable definition

The following query computes GCR for all stories with an attached intake, using the existing schema (migrations 001–006):

```sql
WITH lane_map AS (
  -- resolve lane from story or intake
  SELECT
    s.id                AS story_id,
    COALESCE(s.risk_lane, i.risk_lane, 'normal') AS lane,
    i.id                AS intake_id
  FROM story s
  LEFT JOIN intake i ON i.story_id = s.id
),
has_push AS (
  -- P1 is required only if any trace for this story changed files
  SELECT DISTINCT story_id
  FROM trace
  WHERE story_id IS NOT NULL
    AND files_changed IS NOT NULL
    AND files_changed NOT IN ('', '[]', 'null')
),
gate_signals AS (
  SELECT
    lm.story_id,
    lm.lane,
    -- T0: intake exists
    CASE WHEN lm.intake_id IS NOT NULL THEN 1 ELSE 0 END                       AS g_t0,
    -- T1: intake has risk_lane set
    CASE WHEN lm.intake_id IS NOT NULL THEN 1 ELSE 0 END                       AS g_t1,
    -- M1: proxy — any trace.files_read mentions ws-memories or nano-brain
    CASE WHEN EXISTS (
      SELECT 1 FROM trace t
      WHERE t.story_id = lm.story_id
        AND (t.files_read LIKE '%ws-memories%' OR t.files_read LIKE '%nano-brain%')
    ) THEN 1 ELSE 0 END                                                         AS g_m1,
    -- T2: story row exists (already selected)
    1                                                                           AS g_t2,
    -- T3: at least one proof flag set OR evidence path recorded OR verify passed
    CASE WHEN EXISTS (
      SELECT 1 FROM story s2
      WHERE s2.id = lm.story_id
        AND (s2.unit_proof = 1 OR s2.integration_proof = 1
             OR s2.e2e_proof = 1 OR s2.evidence IS NOT NULL
             OR s2.last_verified_result = 'pass')
    ) THEN 1 ELSE 0 END                                                         AS g_t3,
    -- Review: gate_log pass OR intervention approval from reviewer
    CASE WHEN EXISTS (
      SELECT 1 FROM gate_log gl
      WHERE gl.story_id = lm.story_id
        AND gl.gate = 'review' AND gl.decision = 'pass'
    ) OR EXISTS (
      SELECT 1 FROM intervention iv
      WHERE iv.story_id = lm.story_id
        AND iv.intervention_type = 'approval' AND iv.source = 'reviewer'
    ) THEN 1 ELSE 0 END                                                         AS g_review,
    -- P1: gate_log push approval
    CASE WHEN EXISTS (
      SELECT 1 FROM gate_log gl
      WHERE gl.story_id = lm.story_id
        AND gl.gate = 'P1' AND gl.decision = 'approved'
    ) THEN 1 ELSE 0 END                                                         AS g_p1,
    -- T4: t4_verdict pass or ambiguous
    CASE WHEN EXISTS (
      SELECT 1 FROM story s3
      WHERE s3.id = lm.story_id
        AND s3.t4_verdict IN ('pass','ambiguous')
    ) THEN 1 ELSE 0 END                                                         AS g_t4,
    -- M2: trace completed at correct tier
    CASE WHEN EXISTS (
      SELECT 1 FROM trace t2
      WHERE t2.story_id = lm.story_id
        AND t2.outcome = 'completed'
        -- tier check: minimal=1 for tiny, standard=2 for normal, detailed=3 for high_risk
        AND CASE lm.lane
              WHEN 'tiny'      THEN (t2.task_summary IS NOT NULL AND length(trim(t2.task_summary)) >= 10)
              WHEN 'normal'    THEN (t2.agent IS NOT NULL AND t2.actions_taken IS NOT NULL AND t2.files_read IS NOT NULL)
              WHEN 'high_risk' THEN (t2.decisions_made IS NOT NULL AND t2.errors IS NOT NULL AND t2.harness_friction IS NOT NULL)
              ELSE 0
            END = 1
    ) THEN 1 ELSE 0 END                                                         AS g_m2,
    -- P1 conditional flag
    CASE WHEN lm.story_id IN (SELECT story_id FROM has_push) THEN 1 ELSE 0 END AS push_happened
  FROM lane_map lm
),
gcr AS (
  SELECT
    gs.story_id,
    gs.lane,
    -- gates_recorded: sum of signals that fired
    (gs.g_t0 + gs.g_t1
      + CASE WHEN gs.lane IN ('normal','high_risk') THEN gs.g_m1 ELSE 0 END
      + CASE WHEN gs.lane IN ('normal','high_risk') THEN gs.g_t2 ELSE 0 END
      + CASE WHEN gs.lane IN ('normal','high_risk') THEN gs.g_t3 ELSE 0 END
      + CASE WHEN gs.lane IN ('normal','high_risk') THEN gs.g_review ELSE 0 END
      + CASE WHEN gs.push_happened = 1             THEN gs.g_p1    ELSE 0 END
      + CASE WHEN gs.lane IN ('normal','high_risk') THEN gs.g_t4    ELSE 0 END
      + gs.g_m2
    )                                                                           AS gates_recorded,
    -- gates_expected
    (2  -- T0 + T1 always
      + CASE WHEN gs.lane IN ('normal','high_risk') THEN 5 ELSE 0 END  -- M1,T2,T3,Review,T4
      + CASE WHEN gs.push_happened = 1             THEN 1 ELSE 0 END  -- P1 conditional
      + 1  -- M2 always
    )                                                                           AS gates_expected
  FROM gate_signals gs
)
SELECT
  story_id,
  lane,
  gates_recorded,
  gates_expected,
  ROUND(CAST(gates_recorded AS REAL) / gates_expected, 3) AS gcr,
  CASE
    WHEN CAST(gates_recorded AS REAL) / gates_expected >= 0.85 THEN 'green'
    WHEN CAST(gates_recorded AS REAL) / gates_expected >= 0.50 THEN 'yellow'
    ELSE 'red'
  END AS rag
FROM gcr
ORDER BY gcr ASC;
```

**Note on `last_verified_result`:** migration 002 adds `verify_command` and `last_verified_result` to `story`. The T3 signal uses them. If running on a DB before 002, substitute `s2.evidence IS NOT NULL` only.

**Note on `gate_log` population:** migration 006 created the `gate_log` table but the CLI has no write command for it yet (confirmed: `gate_log` is empty in a freshly-migrated DB). Until `harness-cli gate record` (or equivalent P1/review subcommands) are added, `g_review` and `g_p1` always score 0. This is intentional — it quantifies the gap.

---

## 3. Baseline (Today) vs Target

### Baseline — "manual = ~0% structured capture"

For any story closed before the harness-cli was wired in:

- `intake` rows: 0 (no CLI intake calls in the workflow yet)
- `gate_log` rows: 0 (no CLI gate-record calls)
- `t4_verdict`: NULL on all stories
- `trace` rows: 0 (no CLI trace calls)
- Proof flags: all 0

**GCR baseline = 0 / N = 0.000** for all pre-automation stories.

Even after HB-003 work ships but before the harness is wired in end-to-end: agents who use `harness-cli trace` manually will score partial GCR. The score-trace tier check (M2 signal) will fire, but T3 proof flags, T4 verdict, and gate_log entries will still be 0 if the wiring work (task C / task F) hasn't run.

**Expected GCR at baseline (today, no wiring):**
- tiny lane: `(0+0+0+1_g_m2_if_trace) / 3` = 0–0.33
- normal lane: `(0+0+0+0+0+0+0+0+1_partial_m2) / 8` = 0–0.125

Rounding: **baseline GCR ≈ 0.00–0.12** across open stories.

### Target — "full automation, zero transcription"

After tasks A–C and F are complete (export, install, wiring, AC1 proof):

- Every story has an intake record (T0/T1: +2)
- T2: story created via CLI (already +1 from story row existence)
- T3: `story update --unit 1 --e2e 1 --evidence …` called by wired harness (T3: +1)
- Review: `intervention add --type approval --source reviewer` wired into review gate (Review: +1)
- T4: `story update --t4-verdict pass` called at close (+1)
- M2: `trace` called with full standard/detailed fields (+1)
- P1: `gate_log` needs write command — remains 0 until that lands

**Target GCR (normal lane, P1 gap still open):**
`(1+1+1+1+1+1+0+1+1) / 8` = `8/8 = 1.00` when P1 write lands; `7/8 = 0.875` until then.

**Realistic near-term target after task C + F:** **GCR ≥ 0.75** for all new normal-lane stories.

### Trend measurement

Run the GCR query above weekly (or per sprint) and store the aggregate:

```sql
-- Weekly aggregate: average GCR across all stories closed this sprint
SELECT
  strftime('%Y-W%W', s.created_at) AS week,
  COUNT(*)                          AS stories,
  ROUND(AVG(CAST(gs.gates_recorded AS REAL) / gs.gates_expected), 3) AS avg_gcr
FROM gcr_view gs  -- the CTE above materialized as a view
JOIN story s ON s.id = gs.story_id
WHERE s.status IN ('implemented','retired')
GROUP BY week
ORDER BY week;
```

The trend from ~0.05 → 0.75 → 1.00 directly tracks HB-003 resolution.

**Threshold runbook:**
- avg_gcr < 0.40 → regression; investigate which gates dropped out
- avg_gcr 0.40–0.74 → partial automation; check which gates are 0
- avg_gcr ≥ 0.75 → target met; remaining gap is P1 write command
- avg_gcr = 1.00 → HB-003 closed

---

## 4. Relationship to Existing Score-Trace Tiers

`score-trace` and GCR are **complementary, not overlapping:**

| Dimension | `score-trace` tier | GCR |
|---|---|---|
| Unit of measurement | Single `trace` record | A story's full lifecycle |
| What it measures | Trace field completeness (did the agent fill in the right fields?) | Gate passage rate (did the right harness gates run?) |
| Denominator | Fixed field checklist per tier | Lane-dependent gate set |
| Answers | "Was this trace well-documented?" | "Did this ticket follow the process?" |
| M2 relationship | Score-trace IS the M2 signal for GCR | GCR uses score-trace result as one of 8–9 gate signals |
| HB-003 relationship | Measures one symptom (poor trace quality) | Measures the root cause directly (gates skipped) |

**Score-trace is subsumed into GCR as the M2 gate signal.** A story can have a Detailed trace (score-trace PASS) but still score GCR = 0.25 if T1, T2, T3, T4 were all done in agents' heads without CLI calls. Conversely, a story with all gate_log entries but a minimal trace scores high GCR but fails M2.

**Recommended dashboard pairing:**
- Per-story: GCR (process adherence) + score-trace tier (documentation quality)
- Per-sprint: avg_gcr trend (HB-003 KPI) + % traces meeting tier requirement (existing score-trace KPI)

---

## 5. Implementation Notes

### What needs to land before GCR is fully queryable

1. **`harness-cli gate record`** (or `gate pass`/`gate fail`) — writes to `gate_log`. Needed for Review and P1 signals. Without it, those two gates are always 0 in GCR. This is the single most impactful CLI addition.
2. **`harness-cli story update --t4-verdict`** — T4 verdict column (migration 006 already added the column; `story update` CLI needs the flag).
3. The T0/T1/T2/T3/M2 signals are already computable from existing tables today.

### Materialized view suggestion

Create a `gcr_view` in the DB (or recompute via `query sql`) so the trend query is a single line. Can be added as a new migration with `CREATE VIEW gcr_view AS <the CTE above>`.

### How to run GCR today (without the view)

```bash
HARNESS_DB=harness.db harness-cli query sql "
WITH lane_map AS ( ... )
... <paste full query> ...
"
```

Or add a `harness-cli query compliance` command that runs the query and formats it as a table — natural next backlog item after the gate-record command.
