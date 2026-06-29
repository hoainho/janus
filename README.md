<div align="center">

# ⚡ Janus

### **The EVAL Engine for AI-Assisted Development**

*Gate evaluation system that ensures quality before code ships. Ask first, build second.*

[![License: removed](https://img.shields.io/badge/License-removed-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-blue.svg)](#installation)

**[Quick Start](#-quick-start)** • **[How It Works](#-how-it-works)** • **[Documentation](#-documentation)**

---

</div>

## 🎯 The Problem Janus Solves

AI agents are powerful but **reckless**. They:
- ❌ Start building without understanding the request
- ❌ Skip validation because "it looks right"
- ❌ Ship code that doesn't match what was asked
- ❌ Waste time building the wrong thing

**Janus changes this.** It's a **quality gate system** that:
- ✅ Asks clarifying questions BEFORE starting
- ✅ Evaluates readiness at every stage
- ✅ Blocks progress if quality criteria aren't met
- ✅ Ensures "done" means "actually done"

---

## 📊 How It Works

```text
User Request
     │
     ▼
┌─────────────────────────────────┐
│  1. EVAL-HARNESS Pre-Check     │  ← Ask questions first
│     • Is request clear?        │
│     • Do we have context?      │
│     • Are prerequisites met?   │
└────────────────┬────────────────┘
                 │
                 ├── < 90% ready → Ask questions → Wait → Re-eval
                 │
                 ▼ READY
┌─────────────────────────────────┐
│  2. Feature Intake             │  ← Classify risk
│     • 10-flag risk checklist   │
│     • Choose lane: tiny/normal/high-risk
└────────────────┬────────────────┘
                 │
                 ▼
┌─────────────────────────────────┐
│  3. Gate Evaluation            │  ← Quality checks
│     • Per-gate rubric scoring  │
│     • Value vs Cost analysis   │
│     • KEEP/FIX/DOWNGRADE/CUT   │
└────────────────┬────────────────┘
                 │
                 ▼
┌─────────────────────────────────┐
│  4. Implementation             │  ← Build with gates
│     • Validation ladder        │
│     • User-flow tests          │
│     • Review gate              │
└────────────────┬────────────────┘
                 │
                 ▼
┌─────────────────────────────────┐
│  5. Archive                    │  ← Capture learnings
│     • Record trace             │
│     • Update decisions         │
│     • Feed back to EVAL        │
└─────────────────────────────────┘
```

---

## 🚀 Quick Start

### One-Line Install

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.ps1 | iex
```

### What Gets Installed

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

### First Task with Janus

```bash
# 1. Initialize database
harness-cli init

# 2. Check readiness (EVAL-HARNESS)
harness-cli eval-harness --request "Add user authentication"

# 3. If ready, record intake
harness-cli intake --type feature --summary "User auth" --lane normal

# 4. Work on task (with gates)

# 5. Record trace
harness-cli trace --summary "Implemented auth" --outcome completed
```

---

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

### With Janus (EVAL First)

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

## 📊 Gate Evaluation Rubric

Every gate is scored on **Value vs Cost**:

| Verdict | Meaning | Action |
|---------|---------|--------|
| **KEEP** | Value > Cost | Run gate |
| **FIX** | Valuable but broken | Fix mechanism |
| **DOWNGRADE** | Cost > Value | Make advisory |
| **CUT** | No value | Remove gate |

See `docs/GATE_EVAL_RUBRIC.md` for per-gate scoring.

---

## 📚 Documentation

| Document | Purpose |
|----------|---------|
| `docs/EVAL_HARNESS.md` | Pre-check gate — **READ FIRST** |
| `docs/HARNESS.md` | Operating model |
| `docs/FEATURE_INTAKE.md` | Risk classification |
| `docs/GATE_EVAL_RUBRIC.md` | Gate evaluation criteria |
| `docs/TEST_MATRIX.md` | Validation matrix |

---

## 🛠️ CLI Reference

```bash
# Database
harness-cli init                    # Initialize database
harness-cli migrate                 # Apply schema migrations

# EVAL-HARNESS (Pre-check)
harness-cli eval-harness            # Check readiness

# Intake
harness-cli intake                  # Classify new task
harness-cli query intakes           # List all intakes

# Stories
harness-cli story add               # Create story
harness-cli story update            # Update story status
harness-cli story verify            # Run validations
harness-cli query stories           # List stories

# Gate Logging
harness-cli gate-log                # Record gate passage
harness-cli query gcr               # Gate compliance rate

# Traces
harness-cli trace                   # Record execution trace
harness-cli query traces            # List traces
harness-cli score-trace             # Score trace quality

# Decisions
harness-cli decision add            # Record decision
harness-cli query decisions         # List decisions
```

---

## 🤝 Contributing

Contributions welcome! See `CONTRIBUTING.md`.

Areas needing help:
- 🐛 Bug reports
- 📝 Documentation improvements
- 🔧 New gate evaluation patterns
- 🎨 Better question templates

---

## 📄 License

© 2025 Hoài Nhớ

---

<div align="center">

**Ready to add quality gates to your AI workflow?**

```bash
curl -fsSL https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh | bash
```

[⭐ Star this repo](https://github.com/hoainho/janus) if you find it useful!

</div>
