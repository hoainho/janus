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

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/hoainho/janus/main/scripts/install-harness.ps1"))) -Yes
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

```
janus/
├── crates/harness-cli/        # Rust CLI source
│   └── src/
│       ├── main.rs
│       ├── application.rs
│       ├── domain.rs
│       ├── infrastructure.rs
│       └── interface.rs
├── docs/                      # Harness documentation
├── scripts/
│   ├── install-harness.sh     # Bash installer
│   ├── install-harness.ps1    # PowerShell installer
│   └── schema/                # SQLite migrations
├── .github/workflows/         # CI/CD
├── Cargo.toml                 # Rust workspace
├── AGENTS.md                  # Agent entry point
└── README.md
```

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Useful areas:
- Real-world harness install examples
- Validation patterns for different stacks
- Templates and workflow improvements
- Cross-platform installer improvements
- Agent failure case studies

## License

MIT
