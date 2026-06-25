# harness-cli — Reinstall & Machine-Migration Runbook (VERIFIED)

> Status: verified via round-trip test 2026-06-25 (see "Proof" below). SQLite store.
> Graduate this file to `docs/` when the CLI is wired into the live harness.

## TL;DR
- The SQLite store is a **single portable file**, cross-OS/arch (SQLite format is endian-independent).
- **Full migration** (keep everything incl. traces): `sqlite3 <db> ".backup <dest>"` → copy `<dest>` → point `HARNESS_DB` at it. WAL-safe.
- **Fresh reinstall**: install binary → `harness-cli init && migrate`. Optionally `import` to reseed the markdown-backed tables.
- No daemon, no server. The only thing per-machine is the **binary** (and two env vars).

## Configuration (per machine)
| Env var | Meaning | Note |
|---|---|---|
| `HARNESS_DB` | path to the SQLite file | e.g. `~/.harness/harness.db`; set once in shell profile |
| `HARNESS_REPO_ROOT` | repo root (locates `scripts/schema/*.sql` + markdown for `import`) | needed by `init`/`migrate`/`import` |

## A. Install the binary on a new machine
Pick ONE (do NOT install rust just to *run*):
1. **Download prebuilt release** (recommended): grab the macOS-arm64 (or matching) artifact from the CI release (`.github/workflows/harness-cli-release.yml`) / your dotfiles, drop on `PATH`. Zero build.
2. **Build once natively** (where you have rust): `cargo build --release` (~40s) → copy the binary out. Apply `busy_timeout.patch` first (or use an upstreamed release).
> Run-mode decision: prefer a **native binary on PATH** (zero-daemon, instant). Avoid a Docker-wrapper for daily use — it re-introduces a daemon dependency + per-call latency + SQLite-over-bind-mount lock fragility.

## B. Full-fidelity migration (carry ALL state, incl. traces)
On the OLD machine:
```sh
# WAL-safe consistent snapshot to a single file (works even if a process is mid-write)
sqlite3 "$HARNESS_DB" ".backup '/tmp/harness-migrated.db'"
sqlite3 /tmp/harness-migrated.db "PRAGMA integrity_check;"   # expect: ok
```
Copy `harness-migrated.db` to the NEW machine, then:
```sh
export HARNESS_DB=~/.harness/harness.db
cp /path/to/harness-migrated.db "$HARNESS_DB"
harness-cli migrate    # expect: "Current schema version: 5 / Already up to date."
harness-cli query stats
```
**Why `.backup` and not `cp harness.db`:** in WAL mode, recent writes may live in the `-wal` sidecar. `cp` of just the `.db` while a process is mid-write loses them. `.backup` (online backup API) always produces a consistent single file. (If no process is running, rusqlite checkpoints on close and a plain `cp` of the whole DB *directory* — `.db` + `.db-wal` + `.db-shm` — is also safe. `.backup` is the no-think-safe default.)

## C. Fresh reinstall (no carry-over)
```sh
export HARNESS_DB=~/.harness/harness.db HARNESS_REPO_ROOT=/path/to/repo
harness-cli init && harness-cli migrate
harness-cli import          # OPTIONAL: reseeds stories/decisions/backlog from markdown
```
**`import` coverage (important):** it rebuilds only the **markdown-backed** tables — `story` (TEST_MATRIX), `decision`, `backlog`. It does **NOT** restore `intake`, `trace`, `intervention`, `tool`. Those are operational/telemetry; if you need them, use path **B** (file migration) instead. Since markdown/Jira/git are the systems of record, `import` is the canonical "rebuild from truth" path; trace history is regenerated going forward.

## D. Cross-arch / OS
- Same arch (e.g. Apple Silicon → Apple Silicon): one binary works everywhere; the DB file is portable.
- Different arch/OS: ship the matching binary (CI matrix); the **DB file itself is still portable** across OS/arch.

## Proof (round-trip executed 2026-06-25, Docker rust:1-slim + host macOS sqlite3)
- Machine A populated: `intake=2, backlog=3, trace=5`.
- Host `sqlite3` (macOS) read the Linux-written DB: `integrity_check = ok`; `.backup` → clean 61KB single file.
- Machine B opened the backup: `intake=2, backlog=3, trace=5` — **exact match** (trace preserved — `import` would not have).
- Fresh `init`+`migrate` on empty path: schema v5, all-zero stats.
