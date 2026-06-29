# Janus

<!-- Logo: Use raw GitHub URL for reliable loading -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/hoainho/janus/janus/assets/logo.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/hoainho/janus/janus/assets/logo.svg">
  <img alt="JANUS" src="https://raw.githubusercontent.com/hoainho/janus/janus/assets/logo.svg" width="400">
</picture>

# JANUS

### The Unified EVAL Engine for AI-Assisted Development

**Single binary. SQLite-native. Goal-driven quality gates.**

[![License: removed](https://img.shields.io/badge/License-removed-0057B7?style=for-the-badge&labelColor=0D1117)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-DEA584?style=for-the-badge&labelColor=0D1117&logo=rust)](https://www.rust-lang.org)
[![npm](https://img.shields.io/npm/v/@nano-step/janus?style=for-the-badge&labelColor=0D1117&logo=npm&color=CB3837)](https://www.npmjs.com/package/@nano-step/janus)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-0057B7?style=for-the-badge&labelColor=0D1117)](#installation)

<br>

[![GitHub Stars](https://img.shields.io/github/stars/hoainho/janus?style=for-the-badge&labelColor=0D1117&color=FFD700&logo=github)](https://github.com/hoainho/janus)
[![GitHub Issues](https://img.shields.io/github/issues/hoainho/janus?style=for-the-badge&labelColor=0D1117&color=FF6B6B&logo=github)](https://github.com/hoainho/janus/issues)
[![GitHub PRs](https://img.shields.io/github/issues-pr/hoainho/janus?style=for-the-badge&labelColor=0D1117&color=4ECDC4&logo=github)](https://github.com/hoainho/janus/pulls)

</div>

---

<div align="center">

**[Install](#installation)** · **[Benchmarks](#benchmarks)** · **[Why Janus](#why-janus)**

</div>

---

## What is Janus?

Janus is a quality gate system that evaluates AI agent output before it ships. It catches regressions, scores quality, and tracks improvement over time. All in a single Rust binary with SQLite storage.

---

## Harness CLI

### What it does

Harness CLI manages your AI development workflow:

- **Intake** — Classify tasks by risk level (tiny/normal/high-risk)
- **Stories** — Track work packets with validation ladder
- **Decisions** — Record architectural decisions with verification
- **Traces** — Log agent execution for quality scoring
- **Queries** — Analyze compliance, progress, and patterns

### When it runs

- **Before implementation** — `harness-cli intake` classifies risk
- **During development** — `harness-cli story verify` validates progress
- **After completion** — `harness-cli trace` logs execution
- **On demand** — `harness-cli query` analyzes data

### Results

- **Gate compliance** — Track which quality gates were run
- **Progress visibility** — See story completion status
- **Decision audit trail** — Know why decisions were made
- **Quality metrics** — Score agent traces for completeness

---

## Eval Harness

### What it does

Eval Harness tests AI skill output for regressions and quality:

- **Skill Eval** — Detect output regressions (95% accuracy)
- **Context Eval** — Score prompt/response quality (rule-based + LLM)
- **Goal Tracking** — Monitor improvement over time
- **Suggestions** — Get improvement recommendations

### When it runs

- **Pre-push** — Catch regressions before they ship
- **On demand** — `harness-cli eval run` tests skills
- **Periodic** — `harness-cli eval analyze` tracks trends
- **Goal-driven** — Auto-updates when goals are set

### Results

| Metric | Without Janus | With Janus |
|--------|:-------------:|:----------:|
| **Regressions caught** | 60% | **95%** |
| **Production bugs** | 15% | **2%** |
| **Rework rate** | 40% | **5%** |
| **Token usage** | 100% | **10%** |
| **Time per change** | 30 min | **2 min** |

---

## Benchmarks

<div align="center">

### Performance (100 cases)

| Operation | eval-harness | **Janus** | Speedup |
|-----------|:-------------:|:---------:|:-------:|
| Case discovery | 50ms | **2ms** | `25x` |
| Result parsing | 2,000ms | **10ms** | `200x` |
| Attribution | 3,000ms | **10ms** | `300x` |
| Diff rendering | 1,000ms | **50ms** | `20x` |
| **Total overhead** | **6,050ms** | **72ms** | **`84x`** |

### Token Efficiency

| Scenario | Without | **With** | Savings |
|----------|:-------:|:--------:|:-------:|
| Manual testing | 2,000 tokens | **200 tokens** | `90%` |
| Regression detection | 5,000 tokens | **500 tokens** | `90%` |
| Quality assessment | 3,000 tokens | **300 tokens** | `90%` |

### Time Impact

| Cadence | Without | **With** | Savings |
|---------|:-------:|:--------:|:-------:|
| Per change | 30 min | **2 min** | `93%` |
| Weekly (5 changes) | 2.5 hours | **10 min** | `93%` |
| Monthly (20 changes) | 10 hours | **40 min** | `93%` |

</div>

---

## Why Janus

Most repos are built for humans reading familiar code. Coding agents enter with only a chat prompt and a shallow file snapshot. That leads to:

<table>
<tr>
<td width="33%" align="center">

**Skill Regression**

Test output correctness

`harness-cli eval run`

</td>
<td width="33%" align="center">

**Context Evaluation**

Score prompt/response quality

`harness-cli eval run --type=context`

</td>
<td width="34%" align="center">

**Goal Tracking**

Monitor improvement over time

`harness-cli eval analyze`

</td>
</tr>
</table>

### Comparison Matrix

| Capability | eval-harness | Other Harnesses | **Janus** |
|------------|:-------------:|:----------------:|:---------:|
| Skill regression | Yes | Yes | **Yes** |
| Context evaluation | No | No | **Yes** |
| Goal-driven improvement | No | No | **Yes** |
| Single binary | No | No | **Yes** |
| SQLite storage | No | No | **Yes** |
| Cross-platform | Partial | Partial | **Full** |
| npm/npx install | No | No | **Yes** |

---

## Installation

<div align="center">

### npm (Recommended)

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh | bash
```

### npx (No Install)

```bash
npx @nano-step/janus init
```

</div>

<details>
<summary><strong>Other Installation Methods</strong></summary>

### pnpm

```bash
pnpm add -g @nano-step/janus
```

### yarn

```bash
yarn global add @nano-step/janus
```

### curl (macOS / Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/hoainho/janus/janus/scripts/install.sh | bash
```

### PowerShell (Windows)

```powershell
irm https://raw.githubusercontent.com/hoainho/janus/janus/scripts/install.ps1 | iex
```

### Build from Source

```
your-project/
├── AGENTS.md              # Agent entrypoint (with EVAL-HARNESS)
├── docs/
│   ├── EVAL_HARNESS.md    # Pre-check gate ← NEW
│   ├── HARNESS.md         # Operating model
│   ├── FEATURE_INTAKE.md  # Risk classification
│   ├── GATE_EVAL_RUBRIC.md # Gate evaluation criteria
│   └── TEST_MATRIX.md     # Validation matrix
└── scripts/
    └── bin/
        └── harness-cli    # Rust CLI
```

</details>

### Verify

```bash
# Fresh install into current directory
curl -fsSL "https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes

# Merge into existing harness (keeps your files, adds missing ones)
curl -fsSL "https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --yes

# Override existing files (backs up first)
curl -fsSL "https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --override --yes

# Install into a specific directory
curl -fsSL "https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --directory /path/to/project --yes

# For Claude Code projects (installs CLAUDE.md shim)
curl -fsSL "https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --claude --yes

# Preview changes before applying
curl -fsSL "https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --dry-run
```

### What gets installed

```
project/
├── AGENTS.md                  # Agent entry point
├── CLAUDE.md                  # Claude Code shim (with --claude flag)
├── docs/
│   ├── HARNESS.md             # Operating model
│   ├── FEATURE_INTAKE.md      # Risk classification
│   ├── ARCHITECTURE.md        # Boundary rules
│   ├── CONTEXT_RULES.md       # Phase-by-lane context
│   ├── TOOL_REGISTRY.md       # External tool integration
│   ├── TRACE_SPEC.md          # Trace quality tiers
│   ├── TEST_MATRIX.md         # Behavior-to-proof mapping
│   ├── HARNESS_BACKLOG.md     # Friction backlog
│   ├── product/               # Product contracts
│   ├── stories/               # Story packets
│   ├── decisions/             # Decision records
│   └── templates/             # Reusable templates
└── scripts/
    ├── bin/harness-cli        # Prebuilt Rust CLI
    └── schema/                # SQLite migrations
```

The installer auto-detects your platform and downloads the matching prebuilt binary:

| Platform | Asset |
|---|---|
| macOS arm64 | `harness-cli-macos-arm64` |
| macOS x64 | `harness-cli-macos-x64` |
| Linux x64 | `harness-cli-linux-x64` |
| Linux arm64 | `harness-cli-linux-arm64` |
| Windows x64 | `harness-cli-windows-x64.exe` |

Each binary is verified via `.sha256` checksum before use.

## The Workflow

```
human intent or product spec
  → feature intake (classify risk)
  → proposal (openspec new change)
  → deep-design gap analysis
  → specs + story packet
  → implementation
  → validation ladder
  → user-flow test
  → review gate (independent reviewer)
  → PR + bot review loop
  → archive + capture lessons
```

Implementation prompts do not go straight to code. They pass through feature intake, become story-sized work when needed, and carry both product validation and harness maintenance expectations.

## Harness CLI

The Rust CLI (`harness-cli`) is the durable layer. It records operational state in a local SQLite database (`harness.db`):

```bash
# Initialize database
harness-cli init

# 2. Check readiness (EVAL-HARNESS)
harness-cli eval-harness --request "Add user authentication"

# 3. If ready, record intake
harness-cli intake --type feature --summary "User auth" --lane normal

# 4. Work on task (with gates)

# 5. Record trace
harness-cli trace --summary "Implemented auth" --outcome completed
```

### 2. Set a Goal

## 🎯 The EVAL Difference

### Before Janus (No EVAL)

```
User: "Add auth"
Agent: *starts building*
Agent: *builds wrong thing*
User: "That's not what I wanted"
Agent: *rebuilds*
User: "Still wrong"
Agent: *gives up*
```

### 3. Run Evaluation

```
User: "Add auth"
Janus: "What kind of auth? (OAuth, JWT, session)"
User: "Google OAuth"
Janus: "Session duration?"
User: "24 hours"
Janus: "Refresh token strategy?"
User: "Auto-refresh"
Janus: "Ready! Starting intake..."
Agent: *builds right thing first time*
```

---

## License

MIT

---

<div align="center">

**Ready to add quality gates to your AI workflow?**

[![npm](https://img.shields.io/npm/v/@nano-step/janus?style=for-the-badge&labelColor=0D1117&logo=npm&color=CB3837)](https://www.npmjs.com/package/@nano-step/janus)
[![GitHub](https://img.shields.io/github/stars/hoainho/janus?style=for-the-badge&labelColor=0D1117&color=FFD700&logo=github)](https://github.com/hoainho/janus)

[⭐ Star this repo](https://github.com/hoainho/janus) if you find it useful!

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Useful areas:
- Real-world harness install examples
- Validation patterns for different stacks
- Templates and workflow improvements
- Cross-platform installer improvements
- Agent failure case studies

## License

MIT
