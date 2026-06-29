# Project Rules

<!-- HARNESS:BEGIN -->
## Harness

Claude Code loads this file into every session, but it does not auto-load
`AGENTS.md`. The bare `@` lines below import the always-required harness
context (the "Must in all lanes" set from `docs/CONTEXT_RULES.md`) at
context-load time. Never wrap them in backticks; that disables the import.

@AGENTS.md

@docs/FEATURE_INTAKE.md

Also run `scripts/bin/harness-cli query matrix` before starting work.

## Eval Harness

Before any implementation, run eval to catch regressions:

```bash
# Skill eval (output regression)
harness-cli eval run --skill=<skill>

# Context eval (prompt/response quality)
harness-cli eval run --skill=<skill> --type=context

# Analyze trends
harness-cli eval analyze --skill=<skill>
```

Lane-dependent context (`README.md`, `docs/HARNESS.md`, `docs/ARCHITECTURE.md`,
`docs/CONTEXT_RULES.md`, product docs, stories, decisions) is intentionally not
imported — read it per lane, as `docs/CONTEXT_RULES.md` prescribes.
<!-- HARNESS:END -->
