# harness-cli (geargames fork) — Implementation Status

_Round: **Foundation-only** (no live-harness edits). Updated 2026-06-25._

## Purpose
Re-introduce the durable-layer automation (`harness-cli` + SQLite store) that the
geargames 9-gate harness needs to stop hand-transcribing markdown at every gate
(fixes backlog item **HB-003**). Forked from MIT upstream `hoangnb24/repository-harness`.

## Locked decisions
- **Storage:** SQLite for OLTP gate-writes (zero-daemon = the HB-003 fix). Postgres deferred to a future one-way OLAP projection onto Azure PG (only on confirmed team-shared need). See `docs/p1-schema-and-projection.md`.
- **Run-mode:** native binary on PATH (not Docker-wrapper, which would re-add a daemon dependency).
- **Code home:** this local repo, **no remote** (`origin` renamed to `upstream`). Headed to a personal repo — final repo TBD by owner. **Nothing committed/pushed.**

## Done & verified this round
| Item | Evidence |
|---|---|
| Rust toolchain installed (host, brew) | cargo 1.96, `aarch64-apple-darwin` |
| `busy_timeout(10s)` patch (both `Connection::open` sites) | `docs/busy_timeout.patch`; uncommitted in `infrastructure.rs` |
| Native build | Mach-O arm64, `cargo build --release` ~27s |
| Migration/portability (reinstall + machine-move) | `docs/MIGRATION.md` (round-trip proven; `.backup` WAL-safe; cross-OS file) |
| Concurrency (AC4) | Docker 750-write 0 BUSY; native 80/80 0 BUSY/0 crash; `docs/p1-concurrency-proof.md` |
| **Migration 006** — `gate_log` table + `story.t4_*` + `intake.lane_checklist` | schema **v6**; `gate-log record`, `story t4`, `query gate-log`, `--lane-checklist` all verified; `docs/p2-migration006-evidence.md` |

## Gate coverage (see `docs/p1-gate-mapping.md`)
- As-is upstream: ~50% (T1/T2/T3/M2 sub-writes — the HB-003 transcription pain).
- After migration 006: ~85% (adds P1/P2/M1/T0/Review audit via `gate_log`, T4 verdict, lane checklist).
- Remaining ~15%: reviewer-identity enforcement + lane-checklist per-item rationale — agent-runtime, not pure CLI.

## NOT done — deferred to next increments
1. **Live-harness wiring** (edit `.claude/rules/harness.md` + `docs/HARNESS.md` so agents actually call `harness-cli`). Reviewed, one gate at a time, starting T2/T3/M2. This is what proves **AC1 (zero hand-transcription)** — not yet proven.
2. **Native binary distribution** for machine-switching (CI release artifact or dotfiles) — design in `docs/MIGRATION.md`.
3. **Postgres OLAP projection** — design only; activate on team-shared trigger.
4. **Upstream the `busy_timeout` patch** (PR) vs maintain as fork diff.

## Build & run
```sh
export PATH="/opt/homebrew/bin:$PATH"
cd crates/harness-cli && cargo build --release    # -> target/release/harness-cli
export HARNESS_REPO_ROOT=/Users/nhonh/Documents/personal/harness-cli HARNESS_DB=~/.harness/harness.db
harness-cli init && harness-cli migrate           # schema v6
```
