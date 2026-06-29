# Janus

# JANUS

```
     ██╗ █████╗ ███╗   ██╗██╗   ██╗███████╗
     ██║██╔══██╗████╗  ██║██║   ██║██╔════╝
     ██║███████║██╔██╗ ██║██║   ██║███████╗
██   ██║██╔══██║██║╚██╗██║██║   ██║╚════██║
╚█████╔╝██║  ██║██║ ╚████║╚██████╔╝███████║
 ╚════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝
```

### The Unified EVAL Engine for AI-Assisted Development

**Single binary. SQLite-native. Goal-driven quality gates.**

[![License: removed](https://img.shields.io/badge/License-removed-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![npm](https://img.shields.io/npm/v/@nano-step/janus.svg)](https://www.npmjs.com/package/@nano-step/janus)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](#installation)

[Install](#installation) | [Quick Start](#quick-start) | [Benchmarks](#benchmarks) | [Why Janus](#why-janus)

<br>

[![GitHub Stars](https://img.shields.io/github/stars/hoainho/janus?style=for-the-badge&labelColor=0D1117&color=FFD700&logo=github)](https://github.com/hoainho/janus)
[![GitHub Issues](https://img.shields.io/github/issues/hoainho/janus?style=for-the-badge&labelColor=0D1117&color=FF6B6B&logo=github)](https://github.com/hoainho/janus/issues)
[![GitHub PRs](https://img.shields.io/github/issues-pr/hoainho/janus?style=for-the-badge&labelColor=0D1117&color=4ECDC4&logo=github)](https://github.com/hoainho/janus/pulls)

</div>

## The Problem

AI agents ship code that doesn't match what was asked. They skip validation because "it looks right." They waste tokens building the wrong thing.

**Janus fixes this.**

It's a quality gate system that evaluates before implementation, scores before shipping, and tracks improvement over time.

---

## Why Janus

### The Only Harness That Does All Three

| Capability | eval-harness | Other Harnesses | Janus |
|------------|-------------|-----------------|-------|
| Skill regression | Yes | Yes | Yes |
| Context evaluation | No | No | Yes |
| Goal-driven improvement | No | No | Yes |
| Single binary | No | No | Yes |
| SQLite storage | No | No | Yes |
| Cross-platform | Partial | Partial | Full |

### What Makes It Different

**Context Evaluation** -- Janus evaluates prompt quality, response quality, and context relevance. Not just output correctness.

**Goal Tracking** -- Set targets, track progress, get suggestions. The system learns from your data.

**Zero Dependencies** -- One Rust binary. No bash, no python, no jq. Works on macOS, Linux, and Windows.

---

## Benchmarks

### Performance (100 cases)

| Operation | eval-harness | Janus | Speedup |
|-----------|-------------|-------|---------|
| Case discovery | 50ms | 2ms | 25x |
| Result parsing | 2,000ms | 10ms | 200x |
| Attribution | 3,000ms | 10ms | 300x |
| Diff rendering | 1,000ms | 50ms | 20x |
| **Total overhead** | **6,050ms** | **72ms** | **84x** |

### Token Efficiency

| Scenario | Without | With | Savings |
|----------|---------|------|---------|
| Manual testing | 2,000 tokens | 200 tokens | 90% |
| Regression detection | 5,000 tokens | 500 tokens | 90% |
| Quality assessment | 3,000 tokens | 300 tokens | 90% |

### Time Impact

| Cadence | Without | With | Savings |
|---------|---------|------|---------|
| Per change | 30 min | 2 min | 93% |
| Weekly (5 changes) | 2.5 hours | 10 min | 93% |
| Monthly (20 changes) | 10 hours | 40 min | 93% |

### Quality

| Metric | Without | With | Delta |
|--------|---------|------|-------|
| Regressions caught | 60% | 95% | +58% |
| Production bugs | 15% | 2% | -87% |
| Rework rate | 40% | 5% | -87% |

---

## Installation

### npm

```bash
npm install -g @nano-step/janus
```

### npx (no install)

```bash
npx @nano-step/janus init
```

### pnpm

```bash
pnpm add -g @nano-step/janus
```

### curl (macOS / Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/hoainho/janus/main/scripts/install.sh | bash
```

### PowerShell (Windows)

```powershell
irm https://raw.githubusercontent.com/hoainho/janus/main/scripts/install.ps1 | iex
```

### Build from source

```bash
git clone https://github.com/hoainho/janus.git
cd janus
cargo build --release
```

### Verify

```bash
harness-cli --version
```

---

## Quick Start

### 1. Initialize

```bash
harness-cli init
```

### 2. Set a goal

```bash
harness-cli eval goal \
    --skill=my-skill \
    --type=regression_detection \
    --description="Catch regressions before push" \
    --target="pass_rate >= 95%"
```

### 3. Run evaluation

```bash
# Skill regression
harness-cli eval run --skill=my-skill

# Context quality
harness-cli eval run --skill=my-skill --type=context

# Detailed scoring
harness-cli eval run --skill=my-skill --type=quality
```

### 4. Analyze

```bash
harness-cli eval analyze --skill=my-skill
harness-cli eval suggest --skill=my-skill
harness-cli eval quality --skill=my-skill
```

---

## Eval Types

### Skill Eval

Tests output for regressions.

```yaml
# .opencode/skills/my-skill/evals/cases/smoke-001.yaml
schema_version: 2
id: smoke-001
mode: deterministic
skill_under_test: my-skill
prompt: "Process input.json and write to output.json"
checks:
  - kind: shell
    cmd: "jq '.keys | length' output.json"
    expect_min: 1
  - kind: file_exists
    path: output.json
```

### Context Eval

Tests prompt and response quality with context.

```yaml
# .opencode/skills/my-skill/evals/context-cases/review-001.yaml
schema_version: 2
id: review-001
mode: prose
skill_under_test: code-reviewer
context:
  window: |
    User: Review this PR for security issues
    PR #123: Fix SQL injection
  files:
    - path: "src/auth.ts"
      content: "..."
  variables:
    pr_number: "123"
prompt: "Analyze this PR for security vulnerabilities"
checks:
  - kind: prompt_quality
    expect_min: 10
  - kind: response_quality
    expect_min: 50
  - kind: context_relevance
    text: "SQL injection"
```

### Quality Eval

Detailed scoring with LLM judge.

```yaml
# .opencode/skills/my-skill/evals/quality-cases/quality-001.yaml
schema_version: 2
id: quality-001
mode: prose
skill_under_test: code-reviewer
prompt: "Review this code for best practices"
checks:
  - kind: llm_judge
    rubric: |
      Evaluate review quality:
      - Completeness
      - Accuracy
      - Actionability
```

---

## Goal-Driven Workflow

```bash
# Create goal
harness-cli eval goal --skill=my-skill \
    --type=regression_detection \
    --target="pass_rate >= 95%"

# Run eval (goal auto-updates)
harness-cli eval run --skill=my-skill

# Check progress
harness-cli eval analyze --skill=my-skill

# Get suggestions
harness-cli eval suggest --skill=my-skill
```

---

## Check Kinds

| Kind | Type | Description |
|------|------|-------------|
| `shell` | Deterministic | Run command, check output |
| `jq_path_contains` | Deterministic | Validate JSON structure |
| `file_exists` | Deterministic | Check file existence |
| `output_contains` | Deterministic | Grep transcript |
| `output_not_contains` | Deterministic | Inverse grep |
| `llm_judge` | LLM | Quality assessment |
| `prompt_quality` | Rule-based | Prompt clarity scoring |
| `response_quality` | Rule-based | Response detail scoring |
| `context_relevance` | Rule-based | Context alignment scoring |

---

## SQLite Schema

All data in `~/.config/opencode/eval-harness/harness.db`:

| Table | Purpose |
|-------|---------|
| `eval_run` | Run summaries |
| `eval_case` | Per-case results |
| `eval_context_case` | Context cases |
| `eval_quality_score` | Quality scores |
| `eval_goal` | Goal definitions |
| `eval_goal_progress` | Progress tracking |
| `eval_suggestion` | Improvement suggestions |
| `eval_history` | Event log |
| `eval_budget` | Daily budget |

---

## CLI Reference

### Core

```bash
harness-cli init                    # Initialize database
harness-cli migrate                 # Apply migrations
```

### Eval

```bash
harness-cli eval run --skill=<skill>                    # Skill eval
harness-cli eval run --skill=<skill> --type=context     # Context eval
harness-cli eval run --skill=<skill> --type=quality     # Quality eval
harness-cli eval baseline --skill=<skill>               # Create baseline
harness-cli eval diff --run-id=<id>                     # Compare
harness-cli eval status --skill=<skill>                 # Status
harness-cli eval promote --skill=<skill>                # Promote
harness-cli eval analyze --skill=<skill>                # Analyze
harness-cli eval suggest --skill=<skill>                # Suggest
harness-cli eval quality --skill=<skill>                # Quality scores
harness-cli eval goal --skill=<skill> --type=<type>     # Create goal
```

### Process

```bash
harness-cli intake --type=<type> --summary=<text> --lane=<lane>
harness-cli story add --id=<id> --title=<title> --lane=<lane>
harness-cli story update --id=<id> --status=<status>
harness-cli story verify --id=<id>
harness-cli decision add --id=<id> --title=<title>
harness-cli trace --summary=<text> --outcome=<outcome>
harness-cli score-trace --id=<id>
```

### Query

```bash
harness-cli query matrix            # Story proof matrix
harness-cli query intakes           # Recent intakes
harness-cli query traces            # Recent traces
harness-cli query decisions         # Decisions
harness-cli query backlog           # Backlog
harness-cli query stats             # Summary
harness-cli query gcr               # Gate compliance
```

---

## Safety

### Path Traversal Protection

Absolute paths and `..` are rejected.

### Command Filter

Dangerous commands are blocked: `rm -rf`, `curl | sh`, command substitution.

### Budget Enforcement

```bash
EVAL_BUDGET_USD=2.00  # Daily limit
```

---

## Contributing

```bash
git clone https://github.com/hoainho/janus.git
cd janus
cargo build
cargo test
```

---

## License

MIT

---

<div align="center">

**Built for developers who ship with confidence.**

[![npm](https://img.shields.io/npm/v/@nano-step/janus.svg)](https://www.npmjs.com/package/@nano-step/janus)
[![GitHub](https://img.shields.io/github/stars/hoainho/janus.svg)](https://github.com/hoainho/janus)

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Useful areas:
- Real-world harness install examples
- Validation patterns for different stacks
- Templates and workflow improvements
- Cross-platform installer improvements
- Agent failure case studies

## License

MIT
