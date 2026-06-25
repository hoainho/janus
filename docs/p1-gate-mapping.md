# Gate→Command Mapping: harness-cli ↔ PS-Sweeps 9 Gates

Generated: 2026-06-25  
Source: `crates/harness-cli/src/interface.rs` + `scripts/schema/001–005-*.sql` + `.claude/rules/harness.md`

---

## 1. Command / Table → Gate Mapping

| CLI Command | Schema Table(s) | Our Gate(s) | Write Command | Query/Export Command | Regenerates Artifact |
|---|---|---|---|---|---|
| `init` | `schema_version` | T0 (bootstrap) | `harness-cli init` | — | — |
| `migrate` | `schema_version` | T0 (bootstrap) | `harness-cli migrate` | — | — |
| `import brownfield` | `story`, `decision`, `backlog` | T0 seed | `harness-cli import brownfield` | — | Bootstraps DB from existing `docs/TEST_MATRIX.md`, `docs/decisions/`, `docs/HARNESS_BACKLOG.md` |
| `intake` | `intake` | T1 (AC extraction / classification) | `harness-cli intake --type … --summary … --lane … --flags …` | `harness-cli query intakes` | Lane classification record; human-readable via `query intakes` table |
| `story add` | `story` | T1 (create story packet), T2 (register story for matrix) | `harness-cli story add --id … --title … --lane … --contract … --verify …` | `harness-cli query matrix` | One row in `docs/TEST_MATRIX.md` equivalent (`query matrix` output) |
| `story update` | `story` | T2 (update proof flags), T3 (record evidence path + proof flags) | `harness-cli story update --id … --status … --evidence … --unit 1 --integration 1 --e2e 1` | `harness-cli query matrix` | Updated row in `docs/TEST_MATRIX.md` equivalent |
| `story verify` | `story` (sets `last_verified_result`) | T3 (per-AC evidence: runs verify_command) | `harness-cli story verify <id>` | `harness-cli query matrix` | Verification result stamped on story; exits 1 on fail |
| `story verify-all` | `story` | T3 (bulk verification sweep) | `harness-cli story verify-all` | — | Bulk pass/fail summary; exits 1 if any fail |
| `decision add` | `decision` | M2 (DECISION atom / ADR record) | `harness-cli decision add --id … --title … --status … --doc … --verify …` | `harness-cli query decisions` | Row in `docs/decisions/` index equivalent |
| `decision verify` | `decision` (sets `last_verified_result`) | M2 / post-merge ADR health check | `harness-cli decision verify <id>` | `harness-cli query decisions` | Verification stamp on decision record |
| `backlog add` | `backlog` | M2 / HARNESS_BACKLOG friction logging | `harness-cli backlog add --title … --while … --pain … --suggestion … --risk …` | `harness-cli query backlog` | Row in `docs/HARNESS_BACKLOG.md` equivalent |
| `backlog close` | `backlog` (sets `status`, `actual_outcome`) | M2 / close friction item with outcome | `harness-cli backlog close --id … --status … --outcome …` | `harness-cli query backlog` | Closed row in `docs/HARNESS_BACKLOG.md` equivalent |
| `tool register` | `tool` | T0 (tool pre-flight check registry) | `harness-cli tool register --name … --command … --responsibility … --kind … --capability … --scan …` | `harness-cli query tools` | Tool manifest (JSON or table) |
| `tool check` | `tool` (updates `status`, `checked_at`) | T0 (verifies registered tools present/missing) | `harness-cli tool check [--name …]` | `harness-cli query tools --status missing` | Broken-tool list surfaced by `audit` |
| `tool remove` | `tool` | T0 (deregister obsolete tool) | `harness-cli tool remove --name …` | — | — |
| `intervention add` | `intervention` | Review gate (record reviewer correction/override/approval) | `harness-cli intervention add --trace … --story … --type correction --source reviewer --description …` | `harness-cli query interventions` | Reviewer trail per story/trace; supports Forbidden #2 audit |
| `trace` | `trace` | M2 (agent execution observability) + auto-scores tier | `harness-cli trace --summary … --intake … --story … --outcome … --friction …` | `harness-cli query traces` / `query friction` | Friction rows in `docs/HARNESS_BACKLOG.md` supplement |
| `score-trace` | `trace` (read-only scoring) | T3 quality check on agent trace | `harness-cli score-trace [--id …]` | — | Prints tier (Minimal/Standard/Detailed) + meets_requirement verdict; exits 1 on below |
| `score-context` | `trace` (read-only) | T0/M1 context compliance | `harness-cli score-context <trace_id>` | — | Must-read / should-read compliance report for a trace |
| `audit` | all tables (read-only) | T0 / T4 health sweep | `harness-cli audit` | — | Orphaned stories, unverified decisions, broken tools, entropy score |
| `propose [--commit]` | `backlog` (writes when --commit) | M2 / continuous harness improvement | `harness-cli propose --commit` | `harness-cli query backlog --open` | Improvement proposals auto-promoted to backlog items |
| `query matrix` | `story` | T2 output / T3 evidence summary | — | `harness-cli query matrix [--numeric]` | Equivalent of `docs/TEST_MATRIX.md` (id, title, status, unit, integ, e2e, plat, evidence) |
| `query backlog` | `backlog` | M2 / HARNESS_BACKLOG view | — | `harness-cli query backlog [--open\|--closed]` | Equivalent of `docs/HARNESS_BACKLOG.md` |
| `query decisions` | `decision` | M2 / ADR index view | — | `harness-cli query decisions` | Equivalent of `docs/decisions/` index |
| `query intakes` | `intake` | T1 history | — | `harness-cli query intakes` | Lane + risk-flag history |
| `query traces` | `trace` | M2 observability | — | `harness-cli query traces` | Execution history |
| `query friction` | `trace` (filtered) | HARNESS_BACKLOG supplement | — | `harness-cli query friction` | Traces with recorded `harness_friction` |
| `query tools` | `tool` | T0 tool manifest | — | `harness-cli query tools [--json\|--capability\|--status]` | JSON tool manifest |
| `query interventions` | `intervention` | Review gate audit trail | — | `harness-cli query interventions [--trace\|--story\|--type]` | Reviewer corrections per story |
| `query stats` | all tables | Summary counts | — | `harness-cli query stats` | Counts: intakes, stories, decisions, backlog, traces |
| `query sql` | any | Ad-hoc / escape hatch | — | `harness-cli query sql "…"` | Raw query output |

---

## 2. Coverage Gaps

The following gates from `.claude/rules/harness.md` have **no backing table, column, or CLI command** in the upstream schema/CLI:

### GAP-1: P1 — Push Permission Log
**Gate behavior:** Before every push/history rewrite, structured preview is shown; user must explicitly confirm (`yes`). Force-push, rebase, cherry-pick, branch-delete need a second confirm.  
**What's missing:** No table or column records: (a) that a P1 prompt was shown, (b) what the user approved, (c) when it was approved, or (d) the SHA/branch targeted.  
**Needed:** New table `push_permission_log` (columns: `id`, `created_at`, `story_id`, `ref_type` [branch/tag], `target_ref`, `operation` [push/force-push/rebase/cherry-pick/branch-delete], `approved_by`, `approved_at`, `sha_before`, `sha_after`). Would require a new migration (006) and a `harness-cli p1 record` write command.  
**Alternatively:** Could be argued as out-of-CLI-scope (a git hook responsibility), but without a record the Forbidden #7 audit check has no evidence.

### GAP-2: P2 — Jira Write Tier + Jira Permission Log
**Gate behavior:** Every Jira write is tiered (A silent / B 2 batched previews / C explicit yes). Tier C requires an explicit confirmation before the Jira MCP call.  
**What's missing:** No table records Jira write events, their tier classification, or approval status. The CLI has no `p2` subcommand.  
**Needed:** New table `jira_write_log` (columns: `id`, `created_at`, `ticket`, `operation` [create/update/comment/transition], `tier` [A/B/C], `payload_summary`, `approved`). New migration (007) + `harness-cli p2 record` command.  
**Alternatively:** Purely agent-runtime behavior — the CLI is not the right enforcer here. But without a log, no audit trail for Forbidden #9 ("GitHub writes without P1 approval / starting work without a Jira issue").

### GAP-3: M1 — Recall (ws-memories / nano-brain)
**Gate behavior:** After T1, for normal + high-risk lanes, agents must query ws-memories + nano-brain + `docs/decisions/`. Results must be surfaced to the user with provenance.  
**What's missing:** No table records that a recall was run, what query was used, how many hits were returned, or which atoms were consulted. The `score-context` command checks file reads against CONTEXT_RULES.md but does not model the recall step specifically.  
**Needed:** A `recall_log` table (columns: `id`, `created_at`, `trace_id`, `story_id`, `query`, `source` [ws-memories/nano-brain/grep], `hits`, `atoms_cited`) + `harness-cli recall record` command. Alternatively, the `trace` command's `--decisions` CSV field is a partial proxy (records decision IDs consulted), but it does not capture the recall query or hit count.  
**Verdict:** This is a tracking gap, not a gate-execution gap — the CLI cannot run the recall itself (ws-memories is an external MCP tool). Recording the outcome is the correct scope. Needs a new table or extension to `trace`.

### GAP-4: T0 — Pre-flight Composite Checks
**Gate behavior:** Composite check: Jira MCP auth · git tree clean · sane base branch · `AGENTS.<repo>.md` present · ws-memories writable.  
**What's missing:** The `tool check` command handles the "registered tools present" subset (Jira MCP auth is a registered tool; ws-memories MCP is a registered tool). However, there is no command that checks and records: (a) git working tree cleanliness, (b) correct base branch (gear_release / gear_develop / etc.), (c) presence of `AGENTS.<repo>.md`.  
**What already covers part of it:** `harness-cli tool check` → `tool` table. `harness-cli audit` surfaces broken tools.  
**Needed:** A `harness-cli preflight` command that writes a `preflight_log` record (columns: `id`, `created_at`, `story_id`, `jira_auth` bool, `git_clean` bool, `base_branch`, `agents_file_present` bool, `ws_memories_writable` bool, `passed` bool). Or these checks can be embedded as fields on the `intake` record (partial). Without a CLI command, the T0 PASS log (harness.md: "log T0 pre-flight: PASS") has no durable evidence.

### GAP-5: T4 — Reconciliation Verdict
**Gate behavior:** Before Jira close — Delivered vs Requested per AC. Verdict stored as PASS / AMBIGUOUS / FAIL. AMBIGUOUS requires user confirmation; interpretation is recorded in the story.  
**What's missing:** The `story` table has a `status` column but its valid values are `planned|in_progress|implemented|changed|retired` — none of these map to PASS/AMBIGUOUS/FAIL reconciliation verdicts. There is no `t4_verdict` column, no `t4_interpretation` text field, no `t4_recorded_at` timestamp.  
**Needed:** `ALTER TABLE story ADD COLUMN t4_verdict TEXT CHECK(t4_verdict IN ('pass','ambiguous','fail') OR t4_verdict IS NULL)` + `ADD COLUMN t4_notes TEXT` + `ADD COLUMN t4_recorded_at TEXT`. This is a new migration (006 or merged with P1). The `story update` command would need `--t4-verdict` and `--t4-notes` args.

### GAP-6: Lane Classification Checklist
**Gate behavior:** Every story file MUST include a `## Lane Classification` block — 10-item checklist + `Lane: <lane> (N flags)` verdict. T2 blocks if missing.  
**What's missing:** The `intake` table has `risk_lane` and `risk_flags` (JSON array) which captures the classification outcome, but does not store the 10-item checklist rationale per item. There is no structured per-item storage.  
**Needed:** Either a `lane_checklist` table (10 rows per intake: `intake_id`, `item_name`, `flagged` bool, `rationale`) or a JSON column on `intake` (simpler). This is moderate complexity; the existing `notes` field is a weak fallback. Needs a new migration.  
**Verdict:** This is in-CLI-scope — the intake command is the right place. Needs column extension or child table.

### GAP-7: Review Gate — Pass/Fail Verdict Recording
**Gate behavior:** Reviewer issues PASS or FAIL verdict. On FAIL → fix loop, max 1 retry. Reviewer ≠ implementer (Forbidden #2).  
**What's missing:** The `intervention` table captures human/reviewer corrections and overrides, but there is no `review_verdict` concept: no column for PASS/FAIL, no linkage to "reviewer identity is different from implementer", no retry count.  
**Partial coverage:** `intervention add --type approval --source reviewer` can be used as a PASS proxy. `intervention add --type correction --source reviewer` as FAIL proxy. But the CLI has no `review pass` / `review fail` subcommand, and no column enforces reviewer ≠ implementer.  
**Needed:** Either a dedicated `review` table or additional columns on `intervention`/`story`: `review_verdict TEXT CHECK(review_verdict IN ('pass','fail'))`, `reviewer_id TEXT`, `implementer_id TEXT`, `retry_count INTEGER`. Alternatively, the existing `intervention` model is sufficient if the harness agent convention is established — low-priority migration candidate.

---

## 3. Verdict: Automation Coverage

| Category | Gates Covered | Mechanism |
|---|---|---|
| **Fully covered** | T1 (intake/story/lane), T2 (story add + matrix), T3 (story update + verify + score-trace), M2-story (story/decision/backlog), M2-friction (trace/backlog), Tool pre-flight (tool register/check) | Existing CLI + schema |
| **Partially covered** | T0 (tool check covers MCP auth presence; audit surfaces broken tools; no git/branch/AGENTS.md checks), Review gate (intervention table is a proxy; no explicit pass/fail verb) | Extension needed |
| **Not covered** | P1 push-permission log, P2 Jira-write tier log, M1 recall log, T0 git/branch/AGENTS checks, T4 reconciliation verdict columns, Lane checklist per-item rationale | New tables/migrations or out-of-CLI-scope (P1/P2 debatable) |

**Percentage estimate:**

- 9 harness gates total (T0, T1, M1, T2, T3, Review, P1, T4, P2) + M2 (3 sub-writes).
- Fully automatable as-is: **T1, T2, T3, M2** (story/decision/backlog/trace/friction writes + queries) = ~4.5 of 9 gates = **~50%** of gate bookkeeping automated without changes.
- With minor extensions (T4 verdict columns, Review gate verb, T0 preflight command): rises to **~70%**.
- P1, P2, M1 are either out-of-CLI-scope (runtime agent decisions) or need new tables: the CLI can record outcomes but cannot enforce the gate itself. If recording is sufficient, adding 3 log tables reaches **~85%** total bookkeeping coverage.
- The remaining ~15% (lane checklist per-item rationale, Forbidden #2 reviewer-identity enforcement) requires either structured new tables or stays as agent-convention only.

**Bottom line:** The upstream CLI handles the data-persistence half of the harness well (intake, story lifecycle, decision records, backlog, traces, tool registry). The gaps are concentrated in the approval/permission trail (P1, P2), the recall provenance (M1), the pre-flight composite (T0 git/branch checks), and the T4 reconciliation verdict — six targeted extensions (4 new tables + 2 column additions) would bring coverage to ~85%.
