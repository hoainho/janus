# Janus

**Agent-ready engineering harness for AI-assisted development.**

Janus turns any software repository into a structured workspace where coding agents (Claude Code, Codex, Cursor, Copilot, and others) can classify risk, validate work, and preserve institutional knowledge — without relying on chat history.

> The app is what users touch. The harness is what agents touch.

**Author:** [Hoài Nhớ](https://github.com/hoainho)

---

## Why Janus

Most repos are built for humans reading familiar code. Coding agents enter with only a chat prompt and a shallow file snapshot. That leads to:

- Agents editing code before understanding product intent
- Constraints living only in chat history
- Vague validation expectations discovered too late
- Architecture tradeoffs repeated instead of inherited
- Large requests not broken into reviewable story-sized work

Janus solves this by giving every repo a **structured operating layer** that answers:

- What should I read first?
- How risky is this change?
- What proof shows the work is done?
- What decisions should future agents inherit?

## Features

| Capability | Description |
|---|---|
| **Risk Classification** | Automatic tiny / normal / high-risk lane assignment via 10-flag checklist |
| **Proposal Layer** | OpenSpec integration for structured proposal → design → specs flow |
| **Deep-Design Review** | Parallel multi-agent gap analysis before specs are locked |
| **Validation Ladder** | Lane-appropriate test requirements (quick → integration → E2E) |
| **Review Gate** | Independent reviewer verification with evidence per criterion |
| **PR Bot Loop** | Automated PR review with max 3 push cycles |
| **Durable State** | SQLite-backed operational records via Rust CLI (`harness-cli`) |
| **Trace System** | 3-tier quality scoring (Minimal / Standard / Detailed) |
| **Tool Registry** | Capability-based external tool integration with degrade ladder |
| **Maturity Model** | H0 → H5 verifiable maturity ladder |
| **Self-Improvement** | Friction → audit → propose → outcome feedback loop |
| **Cross-Platform** | macOS (arm64/x64), Linux (x64/arm64), Windows (x64) |

## Quick Start

### One-line install (macOS / Linux)

```bash
curl -fsSL "https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```

### One-line install (Windows PowerShell)

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

### Install options

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

# Record intake classification
harness-cli intake --type "change-request" --summary "Add OAuth login" --lane normal

# Create story
harness-cli story add --id US-014 --title "OAuth login flow" --lane normal

# Run story verification
harness-cli story verify US-014

# Record execution trace
harness-cli trace --summary "Implemented OAuth login" --outcome completed

# Query proof matrix
harness-cli query matrix

# Record friction
harness-cli backlog add --title "Missing webhook template" --pain "Had to infer webhook format"

# Generate improvement proposals
harness-cli propose

# Run drift audit
harness-cli audit
```

## Tool Registry

Register external tools as capability providers. The harness adapts to what is equipped — absent tools are clean skips, never failures.

```bash
# Register a tool
harness-cli tool register --name deploy-check --kind cli \
  --capability deploy-verification --command ./scripts/deploy-check.sh \
  --responsibility Verification --description "Verify deploy health"

# Scan presence
harness-cli tool check

# Look up by capability
harness-cli query tools --capability deploy-verification --status present
```

Supported tool kinds: `cli`, `binary`, `mcp`, `skill`, `http`.

## Maturity Levels

| Level | Name | Description |
|---|---|---|
| H0 | Bare Environment | No harness present |
| H1 | Scaffolding & Policy | Static instructions, templates, risk lanes |
| H2 | Durable State | SQLite records, trace spec, context rules |
| H3 | Active Observability | Trace scoring, friction classification, backlog loop |
| H4 | Automated Verification | Story verify commands, trace-time warnings |
| H5 | Self-Improving | Audit, propose, outcome comparison |

## Repository Structure

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

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Useful areas:
- Real-world harness install examples
- Validation patterns for different stacks
- Templates and workflow improvements
- Cross-platform installer improvements
- Agent failure case studies

## License

MIT
