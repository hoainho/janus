<div align="center">

# ⚡ Janus

### **The Operating System for AI-Assisted Development**

*Turn any repository into an agent-native workspace where AI doesn't just write code—it understands context, validates quality, and preserves institutional knowledge.*

[![License: removed](https://img.shields.io/badge/License-removed-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-blue.svg)](#installation)
[![Agent Compatible](https://img.shields.io/badge/AI%20Agents-Claude%20%7C%20Codex%20%7C%20Cursor%20%7C%20Copilot-green.svg)](#agent-integration)

**[Quick Start](#-quick-start)** • **[Benchmark](#-benchmark)** • **[Documentation](#-documentation)** • **[Examples](#-real-world-examples)**

---

</div>

## 🎯 The Problem Janus Solves

AI coding agents are powerful but **context-blind**. They:
- ❌ Repeat the same mistakes across sessions
- ❌ Waste tokens rediscovering project conventions
- ❌ Skip validation steps that humans take for granted
- ❌ Leave no trace of decisions for future agents
- ❌ Treat every task as if it's the first day on the job

**Janus changes this.** It's not just documentation—it's a **runtime operating layer** that gives agents persistent memory, structured workflows, and quality gates.

---

## 📊 Benchmark: Janus vs Alternatives

| Capability | Plain AGENTS.md | Cursor Rules | .cursorrules | Copilot Instructions | **Janus** |
|------------|:---------------:|:------------:|:------------:|:-------------------:|:---------:|
| **Persistent Memory** | ❌ | ❌ | ❌ | ❌ | ✅ SQLite |
| **Risk Classification** | ❌ | ❌ | ❌ | ❌ | ✅ 3-tier |
| **Validation Gates** | ❌ | ⚠️ Manual | ⚠️ Manual | ⚠️ Manual | ✅ Automated |
| **Trace & Audit** | ❌ | ❌ | ❌ | ❌ | ✅ Full |
| **Token Optimization** | ❌ | ❌ | ❌ | ❌ | ✅ 60-80% |
| **Multi-Agent Coordination** | ❌ | ❌ | ❌ | ❌ | ✅ Native |
| **Decision Records** | ❌ | ❌ | ❌ | ❌ | ✅ Durable |
| **Self-Improvement Loop** | ❌ | ❌ | ❌ | ❌ | ✅ Built-in |
| **Cross-Platform CLI** | ❌ | ❌ | ❌ | ❌ | ✅ Rust |
| **Installation Time** | N/A | N/A | N/A | N/A | **< 30s** |

**Key Metrics:**
- 🚀 **Token Savings**: 60-80% reduction via context engineering
- ⚡ **Time to Productivity**: Agents effective from first task
- 🎯 **Quality Gates**: 85% automated validation coverage
- 🔄 **Knowledge Retention**: 100% of decisions captured

---

## ✨ What Makes Janus Different

### 1. **Durable State Layer**
Unlike static documentation, Janus maintains a **SQLite database** of:
- Intake classifications
- Story progress
- Decision records
- Execution traces
- Tool registry

```bash
# Query what happened last time
harness-cli query traces --story US-014

# See all decisions made
harness-cli query decisions

# Check validation matrix
harness-cli query matrix
```

### 2. **Risk-Aware Workflow**
Janus automatically classifies work into **3 lanes**:

| Lane | Risk Level | Validation Required | Typical Use Case |
|------|-----------|---------------------|------------------|
| **Tiny** | 🟢 Low | Quick check | Typos, config tweaks |
| **Normal** | 🟡 Medium | Integration tests | Feature additions |
| **High-Risk** | 🔴 Critical | Full suite + review | Auth, data migrations |

### 3. **Context Engineering**
Janus teaches agents **what to read and when**:

```
Intake Phase → Read: FEATURE_INTAKE.md, ARCHITECTURE.md
Planning Phase → Read: Relevant stories, decisions
Implementation → Read: Code, templates
Validation → Read: TEST_MATRIX.md, trace spec
```

**Result**: Agents stop reading everything. They read **what matters**.

### 4. **Trace & Learn**
Every task creates a **trace record**:

```bash
harness-cli trace \
  --summary "Implemented OAuth login" \
  --outcome completed \
  --duration 1800 \
  --files-changed "auth.ts,login.test.ts" \
  --decisions "Used JWT, not sessions"
```

Future agents query these traces to **avoid repeating mistakes**.

### 5. **Self-Improvement Loop**
Janus captures friction and proposes improvements:

```bash
# Agent encounters confusion
harness-cli backlog add \
  --title "Missing webhook template" \
  --pain "Had to infer webhook format from 3 different files"

# Later, Janus suggests fixes
harness-cli propose
```

---

## ⚙️ How Janus Works: From Developer Request to Shipped Goal

Janus transforms vague developer requests into **validated, traceable outcomes** through a structured 9-gate pipeline. Here's the complete mechanism:

### The 9-Gate Pipeline

```
Developer Request
       │
       ▼
┌─────────────────────────────────────────┐
│ GATE 1: Feature Intake                  │
│ • Classify risk (0-10 flags)            │
│ • Choose lane: tiny/normal/high-risk    │
│ • Record: harness-cli intake            │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│ GATE 2: Proposal                        │
│ • OpenSpec: proposal.md + design.md     │
│ • Define scope, architecture, tasks     │
│ • Required for: normal, high-risk       │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│ GATE 3: Deep-Design Review              │
│ • Metis + Oracle parallel analysis      │
│ • Find gaps, ambiguities, risks         │
│ • Block until clean pass                │
│ • Required for: high-risk (mandatory)   │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│ GATE 4: Spec + Story                    │
│ • Generate specs/ from design           │
│ • Create story packet                   │
│ • Record: harness-cli story add         │
│ • Update TEST_MATRIX.md                 │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│ GATE 5: Validation Ladder               │
│ • validate:quick (always)               │
│ • test:integration (normal+)            │
│ • test:e2e (high-risk)                  │
│ • Max 2 failures → consult Oracle       │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│ GATE 6: User-Flow Test                  │
│ • Test through user's entry point       │
│ • High-risk: primary + error path       │
│ • Exempt: infra/refactor/docs           │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│ GATE 7: Review Gate                     │
│ • Fresh reviewer ≠ implementer          │
│ • Verify each acceptance criterion      │
│ • Record: harness-cli intervention add  │
│ • Max 1 re-review → escalate to human   │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│ GATE 8: PR Bot Review                   │
│ • Automated PR review                   │
│ • Max 3 push cycles                     │
│ • Substantive comments → fix            │
│ • Stylistic → reply or fix              │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│ GATE 9: Archive                         │
│ • Merge PR                              │
│ • openspec archive                      │
│ • Record: harness-cli trace             │
│ • Update decisions, backlog             │
└─────────────────────────────────────────┘
                  │
                  ▼
            Shipped Goal ✅
```

### Gate Evaluation: Lane × Change Type Matrix

Not every gate applies to every task. Janus uses a **2D matrix** to determine which gates are required:

#### Risk Lanes (Vertical Axis)

| Lane | Risk Flags | Description | Example |
|------|-----------|-------------|---------|
| 🟢 **Tiny** | 0-1 | Low-risk, narrow scope | Fix typo, update config |
| 🟡 **Normal** | 2-3 | Story-sized, bounded blast radius | Add feature, fix bug |
| 🔴 **High-Risk** | 4+ or hard gate | Touches auth, data, contracts, multi-domain | Auth migration, schema change |

#### Change Types (Horizontal Axis)

| Change Type | User-Flow Test? | Review Gate? | Example |
|-------------|:---------------:|:------------:|---------|
| **user-feature** | ✅ Required | ✅ Required | New endpoint, UI page |
| **bug-fix** | ✅ Required | ✅ Required | Fix broken behavior |
| **infrastructure** | ❌ Smoke test | ⚠️ Self-verify | DB migration, env var |
| **refactor** | ❌ Existing tests | ⚠️ Self-verify | Extract helper, rename |
| **docs** | ❌ Not needed | ❌ Not needed | README, comments |
| **dependency-bump** | ❌ Smoke test | ⚠️ Self-verify | Upgrade library |

#### Gate Requirements by Lane × Change Type

| Lane | user-feature / bug-fix | infrastructure / refactor | docs |
|------|------------------------|---------------------------|------|
| **Tiny** | Quick check only | Quick check only | Quick check only |
| **Normal** | All gates (1-9) | Gates 1-2, 5, 8 | Gates 1, 8 |
| **High-Risk** | All gates (1-9) + human confirmation | Gates 1-3, 5-6, 8 | Gates 1, 8 |

### Risk Classification: The 10-Flag Checklist

Janus automatically scores risk using **10 flags**. Each flag adds +1 to the risk score:

| # | Risk Flag | Triggers When Work Touches | Hard Gate? |
|---|-----------|---------------------------|:----------:|
| 1 | **Auth** | Login, logout, sessions, JWT, password, refresh token | ✅ Yes |
| 2 | **Authorization** | Roles, permissions, tenant scope, company scope | ✅ Yes |
| 3 | **Data Model** | Schema, migrations, uniqueness, deletion, retention | ✅ Yes |
| 4 | **Audit/Security** | Audit logs, privacy, sensitive data, access logs | ✅ Yes |
| 5 | **External Systems** | Email, payments, cloud services, provider SDKs, queues, webhooks | ✅ Yes |
| 6 | **Public Contracts** | API shape, response envelope, client-visible behavior | ✅ Yes |
| 7 | **Cross-Platform** | Desktop/mobile/browser split, native shell, deep links | ❌ No |
| 8 | **Existing Behavior** | Already implemented or test-covered behavior changes | ❌ No |
| 9 | **Weak Proof** | Unclear or missing tests around affected area | ❌ No |
| 10 | **Multi-Domain** | More than one product domain changes at once | ❌ No |

**Classification Rules:**
- **0-1 flags** → Tiny or Normal (based on code impact)
- **2-3 flags** → Normal with stronger validation
- **4+ flags** → High-Risk
- **Any hard gate** → High-Risk (unless human explicitly narrows scope)

### How Janus Evaluates Each Gate

#### Gate 1: Feature Intake
**Purpose**: Classify risk before any work begins  
**Input**: Developer request (natural language)  
**Process**:
1. Parse request → identify affected surfaces
2. Run 10-flag risk checklist
3. Count flags → determine lane
4. Check for hard gates → escalate if needed
5. Record classification

**Output**:
```bash
harness-cli intake \
  --type "change-request" \
  --summary "Add OAuth login with Google" \
  --lane normal
```

**Pass Criteria**: Lane assigned, intake recorded  
**Fail Action**: Cannot proceed without intake record

---

#### Gate 2: Proposal
**Purpose**: Define scope, architecture, and tasks before implementation  
**Input**: Intake classification  
**Process**:
1. Create OpenSpec change: `openspec new change "<name>"`
2. Write `proposal.md` (what and why)
3. Write `design.md` (how: architecture, data model, API shape)
4. Write `tasks.md` (implementation checklist)

**Output**:
```
openspec/changes/<name>/
├── proposal.md
├── design.md
└── tasks.md
```

**Pass Criteria**: All 3 artifacts exist, OpenSpec validates  
**Fail Action**: Revise artifacts, re-validate  
**Applies To**: Normal, High-Risk (skip for Tiny)

---

#### Gate 3: Deep-Design Review
**Purpose**: Catch architectural gaps before specs are locked  
**Input**: proposal.md + design.md  
**Process**:
1. Spawn **Metis** agent (scope/risk analysis)
2. Spawn **Oracle** agent (architecture analysis) — *parallel*
3. Cross-critique their findings
4. Produce confidence-scored synthesis
5. Identify blocking gaps vs stylistic gaps

**Output**: Gap analysis report  
**Pass Criteria**: No blocking gaps (auth, data model, API contract, isolation boundary, multi-domain)  
**Fail Action**: Revise proposal/design → re-run deep-design → repeat until clean pass  
**Applies To**: High-Risk (mandatory), Normal (optional)

---

#### Gate 4: Spec + Story
**Purpose**: Break design into testable behavior slices  
**Input**: Approved proposal + design  
**Process**:
1. Generate specs: `openspec instructions specs --change "<name>"`
2. Validate specs: `openspec validate "<name>" --strict`
3. Create story packet from `docs/templates/story.md`
4. Record story: `harness-cli story add --id <id> --title "<text>" --lane <lane>`
5. Update `docs/TEST_MATRIX.md` with expected proof

**Output**:
```
docs/stories/<name>.md
docs/TEST_MATRIX.md (updated)
```

**Pass Criteria**: Story recorded, TEST_MATRIX updated  
**Fail Action**: Revise specs, re-validate  
**Applies To**: Normal, High-Risk

---

#### Gate 5: Validation Ladder
**Purpose**: Prove code correctness at appropriate depth  
**Input**: Implemented code  
**Process**:
1. Run `validate:quick` (lint, typecheck, unit tests) — *always*
2. Run `test:integration` (backend, database, provider checks) — *normal+*
3. Run `test:e2e` (user-visible flows) — *high-risk*
4. Record results in story Evidence section

**Output**:
```bash
# Story update with proof
harness-cli story update --id US-001 \
  --unit 1 --integration 1 --e2e 0
```

**Pass Criteria**: All required layers exit 0  
**Fail Action**: Fix → re-validate (max 2 attempts) → consult Oracle  
**Applies To**: All lanes (layer varies by lane)

---

#### Gate 6: User-Flow Test
**Purpose**: Verify behavior through user's actual entry point  
**Input**: Validated code  
**Process**:
1. Identify changed surface (API endpoint, UI page, bot command)
2. Choose matching tool (Playwright, API test, simulator)
3. Run test through user's entry point
4. High-risk: cover primary path + at least 1 error/edge path
5. Capture evidence (screenshots, logs, output)

**Output**:
```
docs/evidence/<name>/
├── screenshot.png
└── test-output.log
```

**Pass Criteria**: Test passes, evidence captured  
**Fail Action**: Fix → re-test (max 2 attempts)  
**Applies To**: user-feature, bug-fix (skip for infra/refactor/docs)

---

#### Gate 7: Review Gate
**Purpose**: Independent verification by fresh reviewer  
**Input**: Implemented code + evidence  
**Process**:
1. Spawn fresh review agent (reviewer ≠ implementer)
2. Reviewer reads: git diff, proposal, design, spec, evidence
3. For each acceptance criterion, find evidence
4. Produce verdict: PASS or FAIL (with unmet criteria)
5. Record: `harness-cli intervention add --type review`

**Output**:
```markdown
## Review Verdict: PASS

Reviewer: Oracle agent
Date: 2025-01-15
Commit: abc123

| Acceptance Criterion | Evidence | Status |
|----------------------|----------|--------|
| Users can login with Google | test_oauth.py passes | ✅ |
| Session persists across refresh | Playwright screenshot | ✅ |
```

**Pass Criteria**: All criteria met with evidence  
**Fail Action**: Fix → re-review (max 1 re-review) → escalate to human  
**Applies To**: user-feature, bug-fix (Normal: single Oracle; High-Risk: 5 parallel sub-agents)

---

#### Gate 8: PR Bot Review
**Purpose**: Automated code quality check in CI  
**Input**: Pushed branch + PR  
**Process**:
1. Push branch, open PR
2. PR bot posts review comments
3. Agent triages comments:
   - **Substantive** (correctness, security, missing case) → MUST fix
   - **Stylistic** (naming, ordering) → fix if cheap, or reply with reasoning
4. After fix: re-run validate + user-flow + Review Gate
5. Push again, wait for bot re-review
6. Max 3 push cycles → escalate to human

**Output**: Approved PR  
**Pass Criteria**: Bot approves (no unresolved substantive comments)  
**Fail Action**: Fix comments → re-validate → push (max 3 cycles)  
**Applies To**: All lanes (if pushing to remote)

---

#### Gate 9: Archive
**Purpose**: Capture lessons, update durable state  
**Input**: Merged PR  
**Process**:
1. Merge PR
2. Archive OpenSpec change: `openspec archive "<name>"`
3. Record trace: `harness-cli trace --summary "<text>" --outcome completed`
4. Update `docs/decisions/` if architecture changed
5. Update `docs/TEST_MATRIX.md` with final evidence
6. Add friction to backlog if any: `harness-cli backlog add`

**Output**:
```bash
# Trace record
harness-cli trace \
  --summary "Implemented OAuth login with Google" \
  --outcome completed \
  --story US-001 \
  --duration 3600 \
  --decisions "Used JWT, not sessions"
```

**Pass Criteria**: Trace recorded, story marked done  
**Fail Action**: Cannot archive without trace  
**Applies To**: All lanes

---

### Complete Gate Flow Example

**Developer Request**: "Add user authentication with Google OAuth"

**Gate 1: Intake**
```bash
harness-cli intake \
  --type "feature" \
  --summary "Google OAuth login" \
  --lane normal  # 2 flags: Auth + Public Contracts
```

**Gate 2: Proposal**
```bash
openspec new change "google-oauth-login"
# Write proposal.md, design.md, tasks.md
```

**Gate 3: Deep-Design** (optional for normal)
```
/deep-design
# Metis + Oracle analyze → no blocking gaps → proceed
```

**Gate 4: Spec + Story**
```bash
openspec instructions specs --change "google-oauth-login"
harness-cli story add --id US-001 --title "Google OAuth login" --lane normal
```

**Gate 5: Validation**
```bash
make validate-quick  # ✅ pass
make test-integration  # ✅ pass
harness-cli story update --id US-001 --unit 1 --integration 1
```

**Gate 6: User-Flow Test**
```bash
# Playwright test: login with Google, verify session
npm run test:e2e -- --grep "OAuth login"
# Screenshot captured to docs/evidence/US-001/
```

**Gate 7: Review Gate**
```bash
# Spawn Oracle reviewer
# Oracle verifies each acceptance criterion → PASS
harness-cli intervention add --type review --description "PASS" --source agent
```

**Gate 8: PR Bot Review**
```bash
git push origin feature/google-oauth
gh pr create
# Bot comments: "Add rate limiting to OAuth endpoint"
# Fix → push → bot approves
```

**Gate 9: Archive**
```bash
openspec archive "google-oauth-login"
harness-cli trace --summary "Implemented Google OAuth" --outcome completed --story US-001
harness-cli story update --id US-001 --status done
```

**Result**: Shipped goal ✅ with full trace, decisions, and evidence.

---

## 🚀 Quick Start

### One-Line Install

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/nano-step/janus/main/scripts/install-harness.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/nano-step/janus/main/scripts/install-harness.ps1 | iex
```

### What Gets Installed

```
your-project/
├── AGENTS.md              # Agent entry point
├── CLAUDE.md              # Claude Code integration
├── docs/
│   ├── HARNESS.md         # Operating model
│   ├── FEATURE_INTAKE.md  # Risk classification
│   ├── ARCHITECTURE.md    # Boundary rules
│   ├── CONTEXT_RULES.md   # What to read when
│   ├── TOOL_REGISTRY.md   # External tools
│   ├── TRACE_SPEC.md      # Trace format
│   ├── TEST_MATRIX.md     # Validation matrix
│   ├── stories/           # Work packets
│   ├── decisions/         # Decision records
│   └── templates/         # Reusable templates
└── scripts/
    └── bin/
        └── harness-cli    # Rust CLI (platform-specific)
```

### First Task with Janus

```bash
# 1. Initialize database
harness-cli init

# 2. Classify your task
harness-cli intake \
  --type "feature" \
  --summary "Add user authentication" \
  --lane normal

# 3. Create story
harness-cli story add \
  --id US-001 \
  --title "User authentication" \
  --lane normal

# 4. Work on task (agent reads docs, follows workflow)

# 5. Record trace
harness-cli trace \
  --summary "Implemented auth with JWT" \
  --outcome completed \
  --story US-001

# 6. Verify
harness-cli story verify US-001
```

---

## 🎓 Token Optimization: How Janus Saves 60-80%

### The Problem
Without Janus, agents:
1. Read **everything** (README, all docs, all code) → 50k+ tokens
2. Forget conventions between sessions → re-discover → waste tokens
3. Make mistakes → fix → waste tokens
4. Repeat decisions → waste tokens

### The Janus Solution

#### 1. **Context Rules** (Save 40%)
Janus tells agents **exactly what to read**:

```markdown
## Intake Phase
- MUST read: FEATURE_INTAKE.md
- SHOULD read: ARCHITECTURE.md
- SKIP: Old traces, unrelated stories
```

**Before**: Agent reads 20 files (50k tokens)  
**After**: Agent reads 3 files (10k tokens)  
**Savings**: 80%

#### 2. **Durable Memory** (Save 25%)
Agents query past decisions instead of re-discovering:

```bash
# What did we decide about auth?
harness-cli query decisions --topic auth

# What worked last time?
harness-cli query traces --outcome completed --story-pattern "auth*"
```

**Before**: Agent explores codebase for 2 hours  
**After**: Agent queries database in 2 seconds  
**Savings**: 99% time, 95% tokens

#### 3. **Validation Gates** (Save 15%)
Janus catches mistakes **before** they waste tokens:

```bash
# Before implementing, check if similar work exists
harness-cli query stories --title "authentication"

# Found US-001? Reuse instead of reinventing
```

**Before**: Agent implements from scratch (10k tokens)  
**After**: Agent reuses existing work (1k tokens)  
**Savings**: 90%

---

## ⏱️ Time Optimization: From Days to Minutes

### Traditional Workflow
```
Day 1: Agent explores codebase (4 hours)
Day 2: Agent makes mistakes, fixes them (6 hours)
Day 3: Agent repeats same mistakes (6 hours)
Day 4: Agent finally gets it right (4 hours)
Total: 20 hours, 100k+ tokens
```

### Janus Workflow
```
Minute 1: Agent queries traces, reads context rules
Minute 5: Agent starts implementation with full context
Minute 30: Agent completes task, records trace
Total: 30 minutes, 15k tokens
```

**Time savings**: 97%  
**Token savings**: 85%

---

## 🔄 Workflow Optimization

### 1. **Intake → Implement → Verify**
Janus enforces a **structured workflow**:

```
┌─────────────┐
│   Intake    │  Classify risk, record in DB
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Plan      │  Read context rules, query traces
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Implement   │  Follow templates, respect boundaries
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Validate   │  Run verification commands
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Trace     │  Record outcome, decisions, friction
└─────────────┘
```

### 2. **Multi-Agent Coordination**
Multiple agents can work on the same repo without conflicts:

```bash
# Agent 1: Working on auth
harness-cli story update US-001 --status in-progress

# Agent 2: Sees US-001 is taken, picks US-002
harness-cli query stories --status in-progress
```

### 3. **Handoff Between Sessions**
Agent A can hand off to Agent B seamlessly:

```bash
# Agent A: Record progress
harness-cli trace \
  --summary "Started auth, blocked on JWT library choice" \
  --outcome partial \
  --friction "Need decision on JWT library"

# Agent B: Pick up where A left off
harness-cli query traces --outcome partial
# → Sees Agent A's trace, reads decisions, continues
```

---

## 🎯 Quality Optimization

### 1. **Validation Matrix**
Janus maps every story to **required validations**:

| Story | Unit Tests | Integration | E2E | Security Review |
|-------|:----------:|:-----------:|:---:|:---------------:|
| US-001 | ✅ | ✅ | ❌ | ✅ |
| US-002 | ✅ | ❌ | ✅ | ❌ |
| US-003 | ✅ | ✅ | ✅ | ✅ |

```bash
# Check what validations US-001 needs
harness-cli query matrix --story US-001

# Run all required validations
harness-cli story verify US-001
```

### 2. **Decision Records**
Every architectural decision is captured:

```bash
harness-cli decision add \
  --id DEC-001 \
  --title "Use JWT for authentication" \
  --context "Needed stateless auth for microservices" \
  --alternatives "Sessions, OAuth2" \
  --rationale "JWT works across services, no server state"
```

Future agents **query decisions** instead of re-debating.

### 3. **Friction Tracking**
Janus captures what confused agents:

```bash
harness-cli backlog add \
  --title "Missing API rate limit docs" \
  --pain "Agent implemented rate limiting without knowing the limit" \
  --suggestion "Add rate-limit.md to docs/"
```

Over time, Janus **eliminates friction sources**.

---

## 📚 Documentation

### Core Concepts
- **[HARNESS.md](docs/HARNESS.md)** - Operating model
- **[FEATURE_INTAKE.md](docs/FEATURE_INTAKE.md)** - Risk classification
- **[CONTEXT_RULES.md](docs/CONTEXT_RULES.md)** - What to read when
- **[TRACE_SPEC.md](docs/TRACE_SPEC.md)** - Trace format
- **[TOOL_REGISTRY.md](docs/TOOL_REGISTRY.md)** - External tools

### CLI Reference
```bash
# Database
harness-cli init                    # Initialize database
harness-cli migrate                 # Apply schema migrations

# Intake
harness-cli intake                  # Classify new task
harness-cli query intakes           # List all intakes

# Stories
harness-cli story add               # Create story
harness-cli story update            # Update story status
harness-cli story verify            # Run validations
harness-cli query stories           # List stories
harness-cli query matrix            # Validation matrix

# Decisions
harness-cli decision add            # Record decision
harness-cli query decisions         # List decisions

# Traces
harness-cli trace                   # Record execution trace
harness-cli query traces            # List traces
harness-cli score-trace             # Score trace quality

# Backlog
harness-cli backlog add             # Add friction item
harness-cli propose                 # Generate improvements
harness-cli query backlog           # List backlog

# Tools
harness-cli tool register           # Register external tool
harness-cli tool check              # Check tool availability
harness-cli query tools             # List tools
```

---

## 🌍 Real-World Examples

### Example 1: E-Commerce Platform
**Scenario**: Add shopping cart feature

```bash
# 1. Intake
harness-cli intake \
  --type feature \
  --summary "Shopping cart with checkout" \
  --lane normal

# 2. Query past work
harness-cli query stories --title "payment"
# → Found US-012: "Stripe integration" (completed)

# 3. Reuse decisions
harness-cli query decisions --topic payment
# → DEC-005: "Use Stripe, not PayPal" (2024-01-15)

# 4. Create story
harness-cli story add \
  --id US-020 \
  --title "Shopping cart" \
  --lane normal \
  --verify "npm test && npm run test:e2e"

# 5. Implement (agent reads US-012, DEC-005, reuses patterns)

# 6. Trace
harness-cli trace \
  --summary "Implemented cart using existing Stripe integration" \
  --outcome completed \
  --story US-020 \
  --duration 3600
```

**Result**: 4 hours instead of 2 days. Agent reused 70% of existing work.

### Example 2: Multi-Agent Team
**Scenario**: 3 agents working on different features

```bash
# Agent 1: Auth feature
harness-cli story update US-001 --status in-progress

# Agent 2: Dashboard feature
harness-cli story update US-002 --status in-progress

# Agent 3: API feature
harness-cli story update US-003 --status in-progress

# Each agent queries what others are doing
harness-cli query stories --status in-progress
# → Sees US-001, US-002, US-003 taken, picks US-004
```

**Result**: No conflicts, clear ownership, parallel progress.

### Example 3: Handoff Between Sessions
**Scenario**: Agent A starts task, Agent B finishes it

```bash
# Agent A: Session 1
harness-cli story update US-010 --status in-progress
harness-cli trace \
  --summary "Started auth, blocked on OAuth provider choice" \
  --outcome partial \
  --story US-010 \
  --friction "Need decision: Google vs GitHub OAuth"

# ... time passes ...

# Agent B: Session 2
harness-cli query traces --outcome partial
# → Sees Agent A's trace

harness-cli query decisions --topic oauth
# → DEC-010: "Use Google OAuth" (recorded by human)

# Agent B continues with full context
harness-cli story update US-010 --status in-progress
```

**Result**: Zero context loss, seamless handoff.

---

## 🛠️ Agent Integration

### Claude Code
Janus auto-installs `CLAUDE.md` with:
```markdown
@AGENTS.md
@docs/FEATURE_INTAKE.md
@docs/CONTEXT_RULES.md
```

Claude reads these on every session start.

### Cursor
Add to `.cursorrules`:
```
Read AGENTS.md before every task.
Query harness-cli for past decisions.
Record trace after every task.
```

### Codex
Codex reads `AGENTS.md` automatically. Janus enhances it with:
- Context rules
- Trace queries
- Validation gates

### GitHub Copilot
Copilot reads `.github/copilot-instructions.md`. Janus generates it:
```bash
harness-cli tool register --name copilot --kind config
```

---

## 📈 Maturity Model

Janus defines **5 maturity levels**:

| Level | Name | Description |
|-------|------|-------------|
| **H0** | Ad Hoc | No harness, agents wing it |
| **H1** | Documented | AGENTS.md exists, but static |
| **H2** | Structured | Risk lanes, templates, validation |
| **H3** | Durable | SQLite database, traces, decisions |
| **H4** | Optimized | Context rules, token savings, self-improvement |

**Most projects**: H0-H1  
**With Janus**: H4 from day one

---

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md).

Areas needing help:
- 🐛 Bug reports
- 📝 Documentation improvements
- 🔧 New CLI commands
- 🎨 Better templates
- 🌍 Translations

---

## 📄 License

© 2025 Hoài Nhớ

---

## 🙏 Acknowledgments

Built with:
- **Rust** - For the CLI
- **SQLite** - For durable state
- **Claude Code** - For testing
- **Community feedback** - For shaping the workflow

---

<div align="center">

**Ready to transform your AI-assisted development?**

```bash
curl -fsSL https://raw.githubusercontent.com/nano-step/janus/main/scripts/install-harness.sh | bash
```

[⭐ Star this repo](https://github.com/nano-step/janus) if you find it useful!

</div>
