<div align="center">

<img src="assets/logo.svg" alt="JANUS — The Unified Eval Engine" width="460">

<br>

### The quality gate for AI‑assisted development — in a single Rust binary.

**Evaluate _before_ you build. Score _before_ you ship. Catch regressions _before_ they ship for you.**

<br>

[![License: MIT](https://img.shields.io/badge/License-MIT-8B5CF6.svg?style=flat-square&labelColor=0B0E14)](LICENSE)
[![Rust](https://img.shields.io/badge/Built_with-Rust-22D3EE.svg?style=flat-square&labelColor=0B0E14&logo=rust)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/macOS_·_Linux_·_Windows-6366F1.svg?style=flat-square&labelColor=0B0E14)](#-install)
[![npm](https://img.shields.io/npm/v/@nano-step/janus.svg?style=flat-square&labelColor=0B0E14&color=10B981&label=npm)](https://www.npmjs.com/package/@nano-step/janus)
[![Single binary](https://img.shields.io/badge/Single_binary-5.4_MB-F43F5E.svg?style=flat-square&labelColor=0B0E14)](#-install)

[**Install**](#-install) · [**Quick start**](#-quick-start) · [**How it works**](#-how-it-works) · [**Eval harness**](#-eval-harness) · [**Metrics**](#-metrics--impact) · [**CLI**](#-cli-reference)

</div>

---

## Why Janus exists

AI agents are fast — and confidently wrong. They ship code that doesn't match the
ask, skip validation because "it looks right," and burn tokens building the wrong
thing. The cost isn't the model call; it's the rework, the missed regression, the
silent drift between "what was requested" and "what was delivered."

**Janus is a quality gate that sits between intent and merge.** It classifies risk
at intake, forces acceptance criteria before code, proves each criterion with
evidence, and runs behavior‑regression evals on your skills — all from one binary,
all backed by one SQLite database.

> Named after the Roman god of gates and transitions — two faces, one looking to
> the **past** (your baseline), one to the **future** (your current run). Janus
> guards the doorway between them.

---

## ✨ What you get

| | |
|---|---|
| 🛡️ **9 quality gates** | Pre‑flight → AC extraction → recall → matrix → evidence → review → push permission → Jira tier → reconciliation, scaled by risk lane. |
| 🎯 **14 eval commands** | `run · baseline · diff · status · promote · trend · accept · apply · ab · rebaseline · analyze · suggest · goal · quality`. |
| 🔬 **9 check kinds** | Shell, JSON‑path, file, transcript grep (±), LLM judge, prompt/response quality, context relevance. |
| 🧭 **5 attribution classes** | When an eval regresses, Janus tells you *why*: skill, fixture, model, cross‑skill, or unknown drift. |
| ⚡ **Single Rust binary** | 5.4 MB, **zero runtime dependencies** — no bash, no jq, no python. Cold start ≈ 10 ms. |
| 🗃️ **SQLite‑native** | Every intake, story, trace, decision, eval run and baseline lives in one queryable DB. |
| 💰 **Cost‑aware** | Per‑case token & time caps, a daily spend budget, and model pricing baked in. |

<sub>All counts above are verifiable from the source tree and `harness-cli --help`. See [Metrics & impact](#-metrics--impact).</sub>

---

## 📦 Install

```bash
# One‑liner — installs the binary + git hooks into your project
cd ~/your-project
curl -fsSL https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh | bash -s -- --yes

# Initialize the SQLite state database
scripts/bin/harness-cli init
```

<details>
<summary><b>Other ways to install</b></summary>

```bash
# npm (cross-platform, picks the right prebuilt binary)
npm install -g @nano-step/janus

# From a local clone of Janus
bash ~/janus/scripts/install-harness.sh --yes

# Build from source
git clone https://github.com/hoainho/janus.git
cd janus && cargo build --release
```

</details>

---

## 🚀 Quick start

```bash
# 1 — Record what you're about to build (Janus classifies the risk lane)
harness-cli intake --type feature --summary "Add OAuth login" --lane normal

# 2 — Open a story to track acceptance criteria
harness-cli story add --id US-001 --title "OAuth login flow" --lane normal

# 3 — ...implement...

# 4 — Verify the story against its evidence, then record a trace
harness-cli story verify US-001
harness-cli trace --summary "Implemented OAuth login" --outcome pass --story US-001

# 5 — See the whole proof matrix at a glance
harness-cli query matrix
```

Guarding a skill against regressions is just as direct:

```bash
harness-cli eval baseline --skill=code-reviewer     # snapshot today's behavior
harness-cli eval run --skill=code-reviewer --mode=2tier   # smoke first, escalate failures
harness-cli eval diff                                 # what changed vs baseline?
```

---

## 🧭 How it works

Janus runs two cooperating engines over one database: the **Harness** (governs how
a change moves from intent to merge) and the **Eval Harness** (governs whether your
skills still behave as they did).

### The Harness flow

Every change enters one gate and is routed to a **risk lane** — `tiny`, `normal`,
or `high-risk`. The lane decides which of the 9 gates apply.

```mermaid
flowchart TD
    START([User intent]) --> INTAKE[Feature intake]
    INTAKE -->|classify risk| LANE{Risk lane?}
    LANE -->|tiny| PATCH[Patch and validate]
    LANE -->|normal or high-risk| PROPOSE[Propose then deep-design]
    PROPOSE --> T1[T1 Acceptance criteria]
    T1 --> M1[M1 Recall prior fixes]
    M1 --> T2[T2 AC to test matrix]
    T2 --> IMPL[Implement]
    IMPL --> T3[T3 Evidence per AC]
    T3 --> REVIEW[Review gate reviewer not implementer]
    REVIEW --> T4[T4 Reconciliation]
    T4 --> M2[M2 Persist for recall]
    PATCH --> M2
    M2 --> ARCHIVE([Archive])

    style START fill:#6366F1,color:#fff
    style ARCHIVE fill:#10B981,color:#fff
    style REVIEW fill:#8B5CF6,color:#fff
```

### The 9 gates × risk lanes

```mermaid
flowchart LR
    subgraph Pre
        T0[T0 Pre-flight]
        T1[T1 AC extraction]
        M1[M1 Recall]
    end
    subgraph During
        T2[T2 Matrix]
        T3[T3 Evidence]
        REV[Review]
    end
    subgraph Post
        P1[P1 Push permission]
        P2[P2 Jira tier]
        T4[T4 Reconciliation]
    end
    subgraph Persist
        M2[M2 Persist]
    end
    T0 --> T1 --> M1 --> T2 --> T3 --> REV --> P1 --> P2 --> T4 --> M2
```

| Gate | Purpose | tiny | normal | high‑risk |
|------|---------|:----:|:------:|:---------:|
| **T0** Pre‑flight | Block a bad start (dirty tree, wrong base) | ✅ | ✅ | ✅ |
| **T1** AC extraction | Acceptance criteria, verbatim from the ask | — | ✅ | ✅ |
| **M1** Recall | Surface relevant prior fixes | — | ✅ | ✅ |
| **T2** Matrix | Map every AC to a test tier | — | ✅ | ✅ |
| **T3** Evidence | Per‑AC proof on disk | — | ✅ | ✅ |
| **Review** | Reviewer ≠ implementer, cites evidence | — | ✅ | ✅ |
| **P1** Push permission | Stop an unwanted push / history rewrite | — | — | ✅ |
| **P2** Jira tier | No surprise teammate‑visible writes | — | — | ✅ |
| **T4** Reconciliation | Delivered == requested, per AC | — | ✅ | ✅ |
| **M2** Persist | Make the work recallable next session | ✅ | ✅ | ✅ |

---

## 🔬 Eval harness

The eval harness answers one question on every push: **does this skill still do
what it did at baseline?** Pick a mode, run the checks, and on a regression Janus
attributes the likely cause instead of just turning red.

```mermaid
flowchart TD
    RUN[harness-cli eval run] --> MODE{Mode}
    MODE -->|smoke| S[1 sample / case]
    MODE -->|full| F[3 samples / case]
    MODE -->|2tier| TT[smoke first, escalate failures]
    S --> CHK[Run 9 check kinds]
    F --> CHK
    TT --> CHK
    CHK --> Q{All pass?}
    Q -->|yes| PASS[VERDICT: PASS]
    Q -->|no| ATTR[Attribute the cause]
    ATTR --> CLASS{Class}
    CLASS -->|SKILL_CHANGED| R[VERDICT: REGRESSION]
    CLASS -->|FIXTURE_STALE| R
    CLASS -->|MODEL_CHANGED| R
    CLASS -->|CROSS_SKILL_CHANGE| R
    CLASS -->|UNKNOWN_DRIFT| R
    PASS --> GATE{Promoted?}
    R --> GATE
    GATE -->|warn-only| W[exit 0]
    GATE -->|blocking and regression| B[exit 12 push blocked]

    style PASS fill:#10B981,color:#fff
    style R fill:#F43F5E,color:#fff
    style B fill:#F43F5E,color:#fff
    style W fill:#F59E0B,color:#fff
```

### Three modes

```bash
harness-cli eval run --skill=my-skill --mode=smoke   # fast — 1 sample
harness-cli eval run --skill=my-skill --mode=full    # thorough — 3 samples
harness-cli eval run --skill=my-skill --mode=2tier   # smoke, then escalate only failures
```

### Nine check kinds

| Kind | What it asserts |
|------|------------------|
| `shell` | Run a command; check regex / min / exact output |
| `jq_path_contains` | A JSON path holds an expected value |
| `file_exists` | An artifact was produced |
| `output_contains` | The transcript contains text (optional normalization) |
| `output_not_contains` | The transcript does **not** contain text |
| `llm_judge` | Your current model judges an artifact, majority vote over N samples |
| `prompt_quality` | Prompt meets a minimum substance bar |
| `response_quality` | Response meets a minimum substance bar |
| `context_relevance` | Injected context contains the text it should |

### Five attribution classes

When something regresses, the diff alone rarely tells you *why*. Janus does:

`SKILL_CHANGED` · `FIXTURE_STALE` · `MODEL_CHANGED` · `CROSS_SKILL_CHANGE` · `UNKNOWN_DRIFT`

<details>
<summary><b>Example eval case (<code>schema_version: 2</code>)</b></summary>

```yaml
schema_version: 2
id: review-must-flag-sql-injection
mode: deterministic
skill_under_test: code-reviewer
skills_loaded: [code-reviewer]
description: "Reviewer must flag a raw SQL string interpolation"

budget:
  max_tokens: 50000
  max_seconds: 180

prompt: "Review src/db.ts and report security issues."

checks:
  - kind: file_exists
    path: review.md
  - kind: output_contains
    text: "sql injection"
    normalize: [whitespace, case]
  - kind: llm_judge
    target_file: review.md
    rubric: "Must identify the unsanitized query as a security risk"
    samples: 3
```

</details>

### Git hooks

```bash
cp crates/harness-cli/hooks/pre-push     .git/hooks/   # auto-eval changed skills on push
cp crates/harness-cli/hooks/sync-publish .git/hooks/   # block publish on regression
cp crates/harness-cli/hooks/opencode-stop.sh .git/hooks/   # eval when a session ends
```

---

## 📊 Metrics & impact

### Measured facts

These come straight from the build and the test suite — reproduce them yourself:

| Property | Value | How to verify |
|----------|-------|---------------|
| Binary size | **5.4 MB**, single file | `ls -lh scripts/bin/harness-cli` |
| Runtime deps | **0** (no bash/jq/python) | `otool -L` / `ldd` |
| Cold start | **≈ 10 ms** (`--help`) | `time harness-cli --help` |
| State engine | **SQLite**, 8 migrations | `harness-cli init` |
| Test suite | **58 tests passing** | `cargo test -p harness-cli` |
| Surface | **9** gates · **14** eval commands · **9** check kinds · **5** attribution classes · **3** modes | `harness-cli eval --help` |

### Cost & time control (built in)

Evals are real model calls, so Janus treats spend as a first‑class concern:

- **Per‑case caps** — `budget.max_tokens` and `budget.max_seconds` on every case.
- **Daily budget** — `EVAL_BUDGET_USD` (default **$2.00**) with a running ledger.
- **Pricing aware** — token cost computed from a built‑in model price table.
- **Per‑run accounting** — duration and cost recorded per run in SQLite for `eval trend`.

### Illustrative impact

> ⚠️ **The numbers below are an illustrative projection of a typical team workflow,
> not measured benchmarks of this repo.** They show the *kind* of payoff a quality
> gate is designed to produce; your mileage will vary. Only the table above is
> measured.

| Metric (illustrative) | Without a gate | With Janus |
|------------------------|:--------------:|:----------:|
| Skill check time | ~30 min manual | ~2 min automated |
| Regressions caught before push | ~60% | ~95% |
| Rework rate | ~40% | ~5% |

---

## 📟 CLI reference

<details>
<summary><b>Harness commands</b></summary>

```bash
harness-cli init                                       # initialize the database
harness-cli intake --type=X --summary=Y --lane=Z       # record + classify work
harness-cli story add --id=X --title=Y --lane=Z
harness-cli story update --id=X --status=Y
harness-cli story verify X                              # verify one story
harness-cli story verify-all
harness-cli decision add --id=X --title=Y
harness-cli trace --summary=X --outcome=Y [--story=Z]
harness-cli score-trace
harness-cli backlog add --title=X --pain=Y
harness-cli propose                                    # suggest harness improvements
harness-cli audit                                      # detect doc/state drift
```

</details>

<details>
<summary><b>Query commands</b></summary>

```bash
harness-cli query matrix        # story proof matrix
harness-cli query intakes
harness-cli query traces
harness-cli query decisions
harness-cli query backlog
harness-cli query stats
harness-cli query gcr           # gate-compliance metric
harness-cli query friction
```

</details>

<details>
<summary><b>Eval commands (all 14)</b></summary>

```bash
harness-cli eval run --skill=X [--mode=smoke|full|2tier] [--strict]
harness-cli eval baseline --skill=X
harness-cli eval diff
harness-cli eval status [--skill=X]
harness-cli eval promote --skill=X            # warn-only -> blocking
harness-cli eval trend --skill=X [--last=N]
harness-cli eval accept --skill=X --case=Y
harness-cli eval apply --skill=X              # show fix proposals
harness-cli eval ab --skill-a=X --skill-b=Y
harness-cli eval rebaseline --skill=X
harness-cli eval analyze --skill=X
harness-cli eval suggest --skill=X
harness-cli eval goal --skill=X --type=Y --target=Z
harness-cli eval quality --skill=X
```

</details>

<details>
<summary><b>Configuration (environment variables)</b></summary>

| Variable | Default | Description |
|----------|---------|-------------|
| `OPENCODE_SKILLS_ROOT` | auto‑detect | Skills directory |
| `EVAL_STATE_DIR` | `~/.config/opencode/eval-harness` | State directory |
| `EVAL_BUDGET_USD` | `2.00` | Daily spend limit |
| `EVAL_MAX_SECONDS` | `180` | Per‑case timeout |
| `EVAL_MODEL` | `anthropic/claude-haiku-3-5` | Default judge model |
| `EVAL_STRICT` | `0` | Block on regression |
| `EVAL_BYPASS` | `0` | Skip evals |

</details>

---

## 🏗️ Architecture

```text
harness-cli  (single Rust binary)
├── domain.rs / application.rs / infrastructure.rs / interface.rs
├── eval/
│   ├── scoring.rs       # 9 check kinds + normalization
│   ├── attribution.rs   # 5 attribution classes
│   ├── case.rs          # YAML parsing + fixtures (path-traversal guarded)
│   ├── spawn.rs         # runs the skill under test
│   ├── budget.rs · pricing.rs   # daily budget + cost accounting
│   ├── storage.rs       # SQLite: runs, baselines, goals, suggestions
│   ├── quality.rs · context.rs  # prompt/response/context scoring
│   └── stability.rs · diff.rs · report.rs (JUnit/SARIF) · ...
├── hooks/               # pre-push · sync-publish · opencode-stop
└── scripts/
    ├── install-harness.sh
    └── schema/          # 8 SQL migrations
```

---

## 🤝 Contributing

```bash
git clone https://github.com/hoainho/janus.git
cd janus
cargo build
cargo test          # 58 tests
```

See [`CONTRIBUTING.md`](CONTRIBUTING.md). Issues and PRs welcome at
[github.com/hoainho/janus](https://github.com/hoainho/janus).

---

## 📄 License

[MIT](LICENSE) © 2025–2026 **Hoài Nhớ**.

<div align="center">
<br>
<sub><b>Built for developers who ship with confidence.</b></sub>
<br>
<sub>Two faces. One gate. Zero regressions slip through.</sub>
</div>
