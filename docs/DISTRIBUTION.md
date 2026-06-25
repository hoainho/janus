# harness-cli — Distribution & Install Guide

This document covers getting `harness-cli` onto a new machine, configuring the
environment contract, and initialising or migrating the database.

For moving an existing database between machines see **[docs/MIGRATION.md](./MIGRATION.md)**.

---

## Prerequisites

| Requirement | Notes |
|---|---|
| macOS arm64 / x86_64, or Linux x86_64 / arm64 | Windows is not supported by the prebuilt binary |
| `~/.local/bin` on `PATH` (or another writable `PATH` directory) | `install.sh` reports the exact `export` line if it is missing |
| `bash` 3.2+ | Ships with macOS; standard on Linux |

---

## Option A — Build from source (rustup + cargo)

Use this when you want to compile the binary natively (e.g. first machine, CI,
or you changed the source).

```sh
# 1. Install Rust (once per machine)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Follow the prompts; restart your shell or run: source "$HOME/.cargo/env"

# 2. Clone the repo (adjust the remote once one is chosen)
git clone <remote-url> harness-cli
cd harness-cli

# 3. Build and install
bash scripts/install.sh
```

`install.sh` detects that no prebuilt binary exists and runs
`cargo build --package harness-cli --release` automatically. The resulting
binary is copied to `~/.local/bin/harness-cli`.

> **Note on build time:** an initial build takes roughly 40 seconds on Apple
> Silicon. Subsequent incremental builds are faster. The binary is ~3 MB.

---

## Option B — Copy a prebuilt binary

Use this on a second (or CI/server) machine where you already have the binary
from a previous build or from a future release artefact.

```sh
# 1. Place the prebuilt binary somewhere install.sh can find it.
#    The recommended location is the default cargo output dir:
mkdir -p /path/to/harness-cli/target/release
cp harness-cli-macos-arm64 /path/to/harness-cli/target/release/harness-cli
chmod +x /path/to/harness-cli/target/release/harness-cli

# 2. Run the installer
cd /path/to/harness-cli
bash scripts/install.sh
```

Alternatively, set `CARGO_TARGET_DIR` to point directly at the directory that
contains `release/harness-cli`:

```sh
CARGO_TARGET_DIR=/tmp/my-prebuilt bash scripts/install.sh
```

`install.sh` resolution order:

1. `$CARGO_TARGET_DIR/release/harness-cli`
2. `<repo-root>/target/release/harness-cli`
3. `<repo-root>/crates/harness-cli/target/release/harness-cli`
4. Falls back to `cargo build` (slow path) only if none of the above exist and
   `cargo` is on `PATH`.

---

## Environment contract

Add these two lines to your shell profile (`~/.zshrc`, `~/.bashrc`, etc.):

```sh
export HARNESS_DB=~/.harness/harness.db
export HARNESS_REPO_ROOT=/path/to/harness-cli   # absolute path to the repo root
```

| Variable | Purpose |
|---|---|
| `HARNESS_DB` | Path to the SQLite database file. Created on `init`. |
| `HARNESS_REPO_ROOT` | Locates `scripts/schema/*.sql` (needed by `init`, `migrate`, `import`). |

The `install.sh` script prints the exact `export` lines with values filled in.

---

## First-time database initialisation

After install, initialise the database:

```sh
mkdir -p ~/.harness
harness-cli init && harness-cli migrate
```

`init` creates the database file; `migrate` applies all schema migrations up to
the current version. Both commands are idempotent — safe to re-run.

To optionally seed the markdown-backed tables (stories, decisions, backlog) from
the repository's markdown files:

```sh
harness-cli import
```

`import` rebuilds only the markdown-backed tables. It does **not** restore
operational/telemetry tables (`intake`, `trace`, `intervention`, `tool`). Use the
migration path in `docs/MIGRATION.md` if you need to carry those across.

---

## Moving the database to a new machine

See **[docs/MIGRATION.md](./MIGRATION.md)** for the full WAL-safe runbook.

The short version:

```sh
# On the old machine — produce a consistent single-file snapshot
sqlite3 "$HARNESS_DB" ".backup '/tmp/harness-migrated.db'"

# Copy harness-migrated.db to the new machine, then:
export HARNESS_DB=~/.harness/harness.db
cp /path/to/harness-migrated.db "$HARNESS_DB"
harness-cli migrate   # no-op if schema is already current
harness-cli query stats
```

---

## Automated CI releases — DEFERRED

Automated release builds (GitHub Actions, artifact upload, checksummed downloads)
are **deferred** until a git remote is chosen. The repository is currently
local-only with no remote.

Once a remote is set up, the release workflow template is already present at
`scripts/build-harness-cli-release.sh`, which produces platform-specific
artefacts and `.sha256` checksums. At that point, Option B above becomes a
simple `curl` download with no local build required.

The existing `scripts/install-harness.sh` (which installs the harness scaffold
into a target project) already contains a `download_file` path referencing
GitHub Releases; the binary installer can follow the same pattern once a remote
is chosen.
