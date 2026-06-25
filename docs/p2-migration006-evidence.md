# Migration 006 — Evidence Report

## Files Changed

- `scripts/schema/006-gate-audit.sql` — new migration file
- `crates/harness-cli/src/domain.rs` — added `GateLogRecord` struct
- `crates/harness-cli/src/application.rs` — added `GateLogInput`, `SetT4VerdictInput`; extended `IntakeInput` with `lane_checklist`; added `record_gate_log`, `query_gate_log`, `set_t4_verdict` service methods
- `crates/harness-cli/src/infrastructure.rs` — updated imports; added 3 trait methods to `HarnessRepository`; implemented `record_gate_log`, `query_gate_log`, `set_t4_verdict`; updated `record_intake` to persist `lane_checklist`
- `crates/harness-cli/src/interface.rs` — updated imports; added `GateLog` command with `record` subcommand; added `story t4` subcommand; added `--lane-checklist` flag to intake; added `query gate-log` view; added `print_gate_log` function

## Build

Exit code: 0 (clean, no warnings suppressed)

```
Compiling harness-cli v0.1.10
Finished `release` profile [optimized] target(s) in 3.19s
```

## Schema Version

```
=== migrate ===
Current schema version: 6
Already up to date.
```

## New Command Outputs

### gate-log record + query gate-log

```
=== gate-log record ===
Gate log #1 recorded.
=== query gate-log ===
id  created_at           gate  story   action  decision  source  detail
--  -------------------  ----  ------  ------  --------  ------  --------------
1   2026-06-25 12:55:46  P1    US-001  push    yes       agent   branch x->main
```

### story t4 + query sql verification

```
=== story add ===
Story US-001 added.
=== story t4 ===
Story US-001 T4 verdict recorded.
=== verify via query sql ===
id      t4_verdict  t4_notes
------  ----------  ---------------
US-001  ambiguous   interpreted AC2
```

### intake --lane-checklist + query sql verification

```
=== intake with lane_checklist ===
Intake #1 recorded.
=== verify lane_checklist via query sql ===
lane_checklist
--------------
{"auth":"n/a"}
```

## Regression

```
=== regression: query stats ===
=== Harness Stats ===
intakes  stories  decisions  backlog_items  traces
-------  -------  ---------  -------------  ------
1        1        0          0              0

=== regression: backlog add ===
Backlog #1 added.
```

## gitignore

Root `.gitignore` already covers `target/`. Build artifacts placed in scratchpad CARGO_TARGET_DIR, not in repo tree.
