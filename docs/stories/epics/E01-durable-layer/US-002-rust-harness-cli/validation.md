# Validation

## Proof Strategy

Prove parity before replacement. Each migrated command should be tested against
a temporary SQLite database and compared to the current command contract.

The Bash CLI can remain as a reference implementation until the Rust CLI proves
the same durable-layer behavior.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Parse command flags into typed values; reject invalid lanes, statuses, booleans, and missing required flags. |
| Integration | Create a temp database, apply schema, run migrated use cases, and verify rows with SQLite queries. |
| E2E | Install Harness into a temp target, download or locate the prebuilt CLI, run `scripts/harness init`, `intake`, `query intakes`, and `trace`. |
| Platform | Verify supported macOS and Linux binary selection, checksum validation, and clear unsupported-platform errors. |
| Performance | Query commands should remain fast on small local databases; no benchmark gate until larger trace volumes exist. |
| Logs/Audit | Trace writes remain available through `scripts/harness trace` and `scripts/harness query traces`. |

## Fixtures

- Temporary target project with no existing Harness files.
- Temporary target project with existing Harness files for merge behavior.
- Temporary SQLite database seeded with `scripts/schema/001-init.sql`.
- Release-artifact fixture or local file server for installer download tests.

## Commands

```bash
cargo fmt --check
cargo test --workspace
bash -n scripts/harness scripts/install-harness.sh
scripts/harness query stats
tmpdir=$(mktemp -d)
HARNESS_DB="$tmpdir/harness.db" scripts/harness init
HARNESS_DB="$tmpdir/harness.db" scripts/harness intake --type "Harness improvement" --summary "Rust delegated intake smoke" --lane high-risk --flags "public contracts" --docs "docs/decisions/0005-prebuilt-rust-harness-cli" --story US-002
HARNESS_DB="$tmpdir/harness.db" scripts/harness query intakes
rm -rf "$tmpdir"
```

## Acceptance Evidence

- `cargo fmt --check`: passed.
- `cargo test --workspace`: passed, 6 tests.
- `bash -n scripts/harness scripts/install-harness.sh`: passed.
- `scripts/harness query stats`: passed through the Rust delegated `query
  stats` slice.
- Temporary database smoke: `scripts/harness init`, `scripts/harness intake`,
  and `scripts/harness query intakes` passed through the Rust delegated slice.

Remaining evidence needed before story completion:

- Prebuilt release artifact generation.
- Installer platform detection and binary download.
- Checksum verification.
- Parity for the remaining Bash command groups.
