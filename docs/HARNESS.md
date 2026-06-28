# Harness

The app is what users touch. The harness is what agents touch.

This harness classifies every change by risk lane, requires a proposal-and-review
cycle for non-trivial changes, and enforces a validation + user-flow test +
review gate before any work is archived. It combines workflow governance with
a durable SQLite layer for operational records.

## Mental Model

```text
┌─────────────────────┐
│   Human intent      │
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│  Feature Intake     │  classify risk → choose lane
│                     │  record with: harness-cli intake
└────────┬────────────┘
         │
         ├── tiny ──► patch + validate
         │
         ▼  normal / high-risk
┌─────────────────────┐
│  Propose            │  openspec new change "<name>" → proposal.md + design.md + tasks.md
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│  Deep-Design        │  spawn deep-design agent → find gaps, ambiguities, risks
│  Gap Analysis       │  (Metis + Oracle in parallel → cross-critique → synthesis)
└────────┬────────────┘
         │
         ├── gaps found ──► revise proposal/design ──► re-run deep-design
         │
         ▼  clean pass

┌─────────────────────┐
│  Specs + Story      │  acceptance criteria per behavior slice
│                     │  story in docs/stories/ (link proposal + issue)
│                     │  record with: harness-cli story add
│                     │  update docs/TEST_MATRIX.md with expected proof
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│  Implement          │  work through tasks list
│                     │  make must stay green
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│  Validate           │  run validation ladder appropriate to lane
└────────┬────────────┘
         │
         ├── fail ────► fix → re-validate (max 2 attempts before consulting Oracle)
         │
         ▼  pass
┌─────────────────────┐
│  User-Flow Test     │  run test through user's entry point matching changed surface
│                     │  Exempt if change type = infra/refactor/docs (see § Change Types)
└────────┬────────────┘
         │
         ├── fail ────► fix → re-test (max 2 attempts)
         │
         ▼  pass
┌─────────────────────┐
│  Review Gate        │  fresh review agent verifies each acceptance criterion
│                     │  Reviewer ≠ implementer. Cite evidence per criterion.
│                     │  record with: harness-cli intervention add
└────────┬────────────┘
         │
         ├── FAIL ────► fix → re-review (max 1 re-review before consulting human)
         │
         ▼  PASS
┌─────────────────────┐
│  PR + Bot Review    │  push branch → open PR
│  Loop               │  automated PR review
│                     │  agent reads PR comments → fix → re-validate → re-test
└────────┬────────────┘
         │
         ├── bot comments ──► triage → fix or justify → push again
         │
         ▼  approved
┌─────────────────────┐
│  Harness Delta      │  merge PR → openspec archive "<name>"
│                     │  update docs/stories/, docs/decisions/, docs/TEST_MATRIX.md
│                     │  record with: harness-cli trace + backlog add (if friction)
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│   Next intent       │
└─────────────────────┘
```

Every task has two possible outputs:

1. **Product delta**: app code, tests, API shape, data model, or product docs.
2. **Harness delta**: docs, templates, validation expectations, backlog items, or
   decision records that make the next task easier.

## Durable Layer

Policy documents describe how to work. The durable layer stores what happened.

Operational data — intake classifications, story status, decision outcomes,
backlog items, and execution traces — lives in a SQLite database (`harness.db`)
managed by the Rust Harness CLI at `scripts/bin/harness-cli`. Agents and humans
should use that binary for Harness work. The database is local to each project
instance and `.gitignore`d. The schema is version-controlled under
`scripts/schema/`.

This separation keeps policy docs stable and human-readable while giving agents
a structured, queryable record of operational state. It also prepares the
harness for future observability and automated evolution without adding more
markdown files.

Initialize the database if it does not exist:

```bash
scripts/bin/harness-cli init
```

Common commands:

```bash
scripts/bin/harness-cli intake  --type <type> --summary <text> --lane <lane>
scripts/bin/harness-cli story   add --id <id> --title <text> --lane <lane>
scripts/bin/harness-cli story   update --id <id> --status <status>
scripts/bin/harness-cli story   update --id <id> --unit 1 --integration 1 --e2e 0 --platform 0
scripts/bin/harness-cli story   verify <id>
scripts/bin/harness-cli story   verify-all
scripts/bin/harness-cli decision add --id <id> --title <text> --doc docs/decisions/<file>.md
scripts/bin/harness-cli trace   --summary <text> --outcome <outcome>
scripts/bin/harness-cli score-trace
scripts/bin/harness-cli score-context <trace-id>
scripts/bin/harness-cli audit
scripts/bin/harness-cli propose
scripts/bin/harness-cli query   matrix
scripts/bin/harness-cli query   matrix --numeric
scripts/bin/harness-cli query   backlog
scripts/bin/harness-cli query   tools --summary
scripts/bin/harness-cli query   interventions
scripts/bin/harness-cli query   stats
scripts/bin/harness-cli --version
```

## Source Hierarchy

```text
Human intent / prompt
  └── Feature Intake (docs/FEATURE_INTAKE.md)
        └── OpenSpec change proposal (openspec/changes/<name>/)
              ├── proposal.md   — what and why
              ├── design.md     — how (architecture, data model, API shape)
              ├── specs/        — one spec per behavior slice
              └── tasks.md      — implementation checklist
        └── Story packet (docs/stories/<name>.md)
              └── links to OpenSpec change, lists acceptance criteria
        └── docs/TEST_MATRIX.md
              └── maps each story to unit / integration / E2E proof
        └── docs/decisions/
              └── records why contracts or architecture changed
```

Before implementation, product docs and proposal artifacts describe intent.
After implementation, those artifacts plus passing tests are the living contract.

## OpenSpec Integration

OpenSpec is the **proposal and design layer** of this harness. Every normal or
high-risk change must have an OpenSpec change before implementation starts.

### Commands
```bash
openspec new change "<name>"            # scaffold change directory
openspec validate "<name>" --strict     # validate all artifacts
openspec archive "<name>"               # archive after merge
```

## Deep-Design Gap Analysis

After the proposal produces `proposal.md` and `design.md`, run **deep-design**
before locking any spec.

- Spawns Metis (scope/risk) + Oracle (architecture) in parallel
- Cross-critiques their findings
- Produces a confidence-scored synthesis: gaps, ambiguities, hidden risks

### Gate rule

```text
deep-design pass (no blocking gaps)
  → proceed to specs/ + story packet

deep-design finds gaps
  → revise proposal.md or design.md
  → re-run deep-design
  → repeat until clean pass
```

A gap is blocking if it touches: auth, data model, API contract, isolation
boundary, or multi-domain scope. Stylistic gaps are non-blocking.

## Spec Lifecycle

Ongoing work enters the harness as one of these input types:

| Type | What to do |
|---|---|
| New spec | Populate `docs/product/`, create candidate story list, run deep-design on scope |
| Spec slice | Propose → deep-design → specs/ → story → implement |
| Change request | Propose → deep-design (if normal+) → story → implement |
| New initiative | Initiative notes in `docs/stories/` + multiple proposals |
| Maintenance | Story packet only (no proposal required for tiny) |
| Harness improvement | Direct docs update or `harness-cli backlog add` |

Do not extend a monolithic spec. Use change proposals + story packets as the
living surface.

## Growth Rule

The harness grows from friction.

When an agent is confused, repeats manual reasoning, needs a new validation
command, discovers a missing rule, or sees a recurring failure pattern, it must
either improve the harness directly or record the friction:

```bash
scripts/bin/harness-cli backlog add --title "<short name>" --pain "<what was hard>"
```

Use the backlog outcome loop for improvements that are expected to change agent
behavior or validation results:

1. When creating the backlog item, fill `--predicted` with the measurable
   impact expected from the improvement.
2. When closing the item, fill `--outcome` with the actual measured result or
   review evidence.
3. Use `scripts/bin/harness-cli query backlog --open` to review proposed and accepted
   items, and `scripts/bin/harness-cli query backlog --closed` to compare predictions
   with outcomes after implementation.

The `harness_friction` field on traces also captures per-task friction so
patterns can be queried later:

```bash
scripts/bin/harness-cli query friction
```

## Validation Ladder

Run the layers appropriate to the lane. Never claim a layer passes without
running it and seeing exit code 0.

```text
validate:quick   (always — every lane)
  make validate-quick

test:integration   (normal + high-risk)
  make test-integration

test:e2e   (high-risk or when UI behavior changes)
  make test-e2e

test:release   (before deploy)
  make test-release
```

**Lane → required layers:**

| Lane | validate:quick | test:integration | test:e2e |
|------|:-:|:-:|:-:|
| tiny | ✓ | — | — |
| normal | ✓ | ✓ | — |
| high-risk | ✓ | ✓ | ✓ |

Agents must not claim a layer passes until it has been run and output verified.

## Change Types

The validation ladder is necessary but not sufficient. The **change type**
determines whether user-flow testing and review gate apply.

| Change type | E2E required? | Review gate? | Example |
|-------------|:-:|:-:|---|
| **user-feature** (new behavior, new surface) | ✅ | ✅ | new endpoint, new UI page |
| **bug-fix** (user-visible defect) | ✅ | ✅ | "OTP not arriving", broken response |
| **infrastructure** (migrations, config, deploy) | ❌ smoke test sufficient | ⚠️ self-verify | DB migration, env var change |
| **refactor** (same I/O) | ❌ existing tests pass | ⚠️ self-verify | extract helper, rename internal symbol |
| **docs** (markdown / comments only) | ❌ | ❌ | README, ADR write-up |
| **dependency-bump** | ❌ smoke test | ⚠️ self-verify | upgrade library version |

**Combined gate:** Lane × Change Type. Both must pass to proceed.

For change types marked **❌ smoke test** instead of E2E:
- Run a deterministic check that exercises the changed surface (e.g.
  `alembic upgrade head` for migrations, `import <app>` for refactors).
- Paste the output in story Evidence section.
- No user-flow test required — there is no user surface to test.

For change types marked **⚠️ self-verify**:
- Implementing agent runs the validation ladder and pastes output.
- No independent review agent required.
- Still subject to PR bot review (see below).

## User-Flow Testing

After validation ladder passes, run at least one test that exercises the
changed behavior through the **user's actual entry point**. Choose the tool
that matches the changed surface:

| Changed surface | Tool | Command |
|---|---|---|
| Bot / chat handler | Command simulator | `# project-specific` |
| Web UI | Playwright / Cypress | `npm run test:e2e` |
| REST API | API integration test | `npm run test:integration` |
| Backend-only (no user surface) | Existing integration tests | `npm run test:integration` |
| LLM / external service call | Live smoke script | `# project-specific` |

**Lane × user-flow requirement:**

| Lane | User-flow test required? |
|------|:-:|
| tiny | No (escalate to normal if user-visible behavior changes) |
| normal | Yes — at least 1 test covering the primary changed behavior |
| high-risk | Yes — cover primary + at least 1 error/edge path |

**E2E not applicable**: If change type is infra/refactor/docs/deps, write
"E2E: not applicable — [reason]" in the story Evidence section. The review
gate validates this justification.

**Happy-path-only is insufficient for high-risk**: at minimum cover one
error/edge path (auth fail, rate limit, malformed input, etc.).

## Review Gate

After user-flow tests pass, a **fresh review agent** verifies the implementation.
The reviewer **must not be** the implementing agent.

**What the reviewer checks:**
1. Read `git diff <default-branch>` + the proposal, design, and spec.
2. For each acceptance criterion, find evidence (test output, screenshot,
   command result) that it is satisfied.
3. Produce a verdict: **PASS** (all criteria met with evidence) or **FAIL**
   (list unmet criteria + missing evidence).

**Lane × Change Type → review requirement:**

| Lane | user-feature / bug-fix | infra / refactor / deps | docs |
|------|---|---|---|
| tiny | n/a (escalate if user-visible) | self-verify | none |
| normal | Single Oracle review | self-verify | none |
| high-risk | Full review-work skill (5 parallel sub-agents) | single Oracle | n/a |

**Review output format:**

```text
## Review Verdict: PASS | FAIL

Reviewer: <agent name>
Date: YYYY-MM-DD
Commit: <sha>

| Acceptance Criterion | Evidence | Status |
|---|---|---|
| "Users can upload receipt photo" | test_receipt_upload.py passes (output below) | ✓ |
| "Items appear in inventory" | simulator output shows items listed | ✓ |

Unmet criteria (if FAIL):
- [criterion] — missing [evidence type]
```

**Rule:** `openspec archive "<name>"` is forbidden until Review Verdict = PASS.

Record the review with:
```bash
scripts/bin/harness-cli intervention add --type review --description "Review verdict: PASS" --source agent
```

## PR + Bot Review Loop

After the local Review Gate passes, push branch and open a PR. The PR triggers your configured automated reviewer.

```text
1. Push branch + open PR
        │
        ▼
2. PR bot posts review comments
        │
        ├── comments substantive ──► agent reads → fix → push
        │                            │
        │                            ▼
        │                   re-run validate + user-flow test
        │                            │
        │                            ▼
        │                   if substantive impl change → re-run Review Gate
        │                            │
        │                            ▼
        │                   wait for bot re-review
        │
        ├── comments stylistic only ─► address inline or reply with reason
        │
        ▼
3. Bot approves → merge → openspec archive "<name>"
```

**Rules for handling PR comments:**

- **Read every comment.** Do not collapse / dismiss without action or reasoned reply.
- **Substantive comment** (correctness, security, missing case): MUST fix.
  After fix, re-run validate + user-flow + Review Gate before pushing.
- **Stylistic comment** (naming, ordering, preference): fix if cheap, or reply
  with reasoning and tag for human review.
- **Disagreement**: do NOT silently dismiss. Reply with rationale; tag human.
- **Loop limit**: max 3 push cycles per PR. After 3, escalate to human review.
- **Never**: force-push to bypass bot, dismiss without reading, or merge
  without bot approval (unless human override documented in PR).

The PR review loop is not optional. It is the final correctness gate before
the change becomes part of the trunk.

## Forbidden Practices

1. **Claiming "tests pass" without output.** Paste the command and its exit code.
   A claim without evidence is not a claim.
2. **Self-review.** The implementing agent must not perform its own Review Gate.
   Use review-work skill or spawn a fresh review agent.
3. **Skipping user-flow tests for "refactors."** If the refactor changes
   observable behavior (response shape, timing, error messages, side effects),
   it needs a user-flow test. Only pure internal refactors (identical I/O)
   qualify as "E2E not applicable."
4. **Happy-path-only E2E for high-risk changes.** High-risk must cover at least
   one error or edge path.
5. **Archiving without review verdict.** openspec archive "<name>" is blocked until
   the story shows Review Verdict = PASS with per-criterion evidence.
6. **Backdating evidence.** Evidence must reference the current implementation
   commit, not a previous passing run.
7. **Force-pushing to bypass PR bot review.** PR bot must approve or be
   overridden by documented human decision.
8. **Dismissing PR comments without action or reasoned reply.** Every
   substantive comment requires a fix or a documented disagreement.

## Story Verification

Stories may carry a mechanical proof command:

```bash
scripts/bin/harness-cli story add --id US-012 --title "Story verification" --lane normal --verify "cargo test --workspace"
scripts/bin/harness-cli story update --id US-012 --verify "cargo test --workspace"
scripts/bin/harness-cli story verify US-012
```

`story verify` runs the command from the repository root, records
`last_verified_at` and `last_verified_result`, and exits 0 on pass or 1 on fail.
When `trace --story <id>` links to a story whose verification command has never
passed, the trace still records but prints an advisory warning before close.

Use `story verify-all` before merges, maturity claims, and benchmark runs. It
runs every configured story verification command, prints one result per story,
skips stories without `verify_command`, and exits 1 if any configured story
fails.

## Phase 5 Evolution Commands

Tool discovery:

```bash
scripts/bin/harness-cli query tools --summary
scripts/bin/harness-cli query tools --json
scripts/bin/harness-cli tool register --name <name> --command <cmd> --description <text> --responsibility Verification
```

Context and drift checks:

```bash
scripts/bin/harness-cli score-context <trace-id>
scripts/bin/harness-cli audit
```

`score-context` is advisory; it reports context-rule coverage without changing
the trace. `audit` reports drift categories and an entropy score documented in
`docs/HARNESS_AUDIT.md`.

Interventions are separate from traces:

```bash
scripts/bin/harness-cli intervention add --trace <id> --type correction --description <text> --source human
scripts/bin/harness-cli query interventions --story US-024
```

Record an intervention when a human, reviewer, CI system, or another agent
corrects, overrides, escalates, or approves work.

Improvement proposals:

```bash
scripts/bin/harness-cli propose
scripts/bin/harness-cli propose --commit
```

`propose` prints deterministic proposals from repeated friction, interventions,
and audit drift. `--commit` creates proposed backlog items only; it does not
edit policy docs or approve the proposal.

## Decision Records

High-risk work needs durable decisions when it changes behavior or architecture.
For auth, authorization, data ownership, API shape, audit/security, or
validation changes, record the decision in both places:

1. Add a markdown file under `docs/decisions/` from
   `docs/templates/decision.md`.
2. Add or refresh the durable record:

```bash
scripts/bin/harness-cli decision add \
  --id 0008-auth-boundary \
  --title "Auth Boundary" \
  --doc docs/decisions/0008-auth-boundary.md \
  --notes "Accepted during T4 authentication work."
```

The trace `--decisions` field is useful evidence, but it is not the decision
log. Do not treat decision text in a trace as satisfying the durable decision
record requirement.

## Harness Change Policy

Agents may update directly:

- Story status and evidence via `scripts/bin/harness-cli story update`.
- Test matrix rows via `scripts/bin/harness-cli story add` and
  `scripts/bin/harness-cli story update`.
- Links from story packets to product docs.
- Validation notes and reports.
- Small clarifications tied to the current task.
- Intake records, traces, and backlog items via `scripts/bin/harness-cli`.

Agents should ask for human confirmation before:

- Changing architecture direction.
- Removing validation requirements.
- Changing the source-of-truth hierarchy.
- Changing risk classification rules.
- Replacing the feature workflow.

## Done Definition

A task is done only when:

- The requested change is completed or the blocker is documented.
- Relevant docs, stories, and test matrix entries remain current.
- Validation commands were run when they exist.
- A trace has been recorded with `scripts/bin/harness-cli trace`.
- Missing harness capabilities were recorded with
  `scripts/bin/harness-cli backlog add`.
- The final response says what changed and what was not attempted.

## Consumption Model

The personal workspace has **one canonical harness** at Janus and its
companions. Sub-repos consume the harness via one of three mechanisms:

### 1. In-workspace (current, default)

Each onboarded sub-repo has relative symlinks in its `docs/` directory
pointing back at the workspace root. To verify a repo is wired correctly:

```bash
ls -la <repo>/docs/HARNESS.md
# Should show: docs/HARNESS.md -> ../../docs/HARNESS.md
```

Each repo also has a regular `docs/HARNESS.local.md` (NOT a symlink) explaining
how to recover the harness if the repo is cloned standalone.

### 2. Via installer (recommended for external repos)

The Janus installer bootstraps the harness into any project:

```bash
curl -fsSL "https://raw.githubusercontent.com/nano-step/janus/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```

The installer downloads the prebuilt Harness CLI, verifies its checksum,
and sets up the complete harness structure.

### 3. Manual re-init (escape hatch)

If symlinks break or you need a standalone copy, use the harness-init skill:

```bash
bash ~/.config/opencode/skills/harness-init/scripts/install.sh \
  --target "$PWD" \
  --config /tmp/answers.yaml
```

## How to onboard a new sub-repo

```bash
cd /Users/nhonh/Documents/personal/<new-repo>
mkdir -p docs/templates docs/evidence
ln -sfn ../../docs/HARNESS.md           docs/HARNESS.md
ln -sfn ../../docs/FEATURE_INTAKE.md    docs/FEATURE_INTAKE.md
ln -sfn ../../docs/HARNESS_BACKLOG.md   docs/HARNESS_BACKLOG.md
ln -sfn ../../../docs/templates/story.md docs/templates/story.md
ln -sfn ../../../docs/evidence/README.md docs/evidence/README.md
# then write a docs/HARNESS.local.md pointer (see existing repos for template)
```

To use the full Janus installer instead:

```bash
curl -fsSL "https://raw.githubusercontent.com/nano-step/janus/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```
