#!/usr/bin/env bash
# install.sh — install harness-cli to ~/.local/bin/harness-cli
#
# Resolution order for a prebuilt binary:
#   1. $CARGO_TARGET_DIR/release/harness-cli   (set by the build worker or CI)
#   2. <repo-root>/target/release/harness-cli  (default cargo output location)
#   3. <repo-root>/crates/harness-cli/target/release/harness-cli (crate-local override)
#   4. If none of the above exist AND cargo is on PATH: build from source (slow path).
#      The build path is intentionally last; it is for machines that have no prebuilt
#      binary and want a fully self-contained install. It is never reached when any of
#      the paths 1-3 exist.
#
# Idempotent: safe to re-run; re-copies the binary unconditionally so an updated
# build always lands at the destination.
#
# Environment contract printed on every run:
#   HARNESS_DB          path to the SQLite file  (e.g. ~/.harness/harness.db)
#   HARNESS_REPO_ROOT   repo root — locates scripts/schema/*.sql for init/migrate
set -euo pipefail

DEST="$HOME/.local/bin/harness-cli"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

# ── Locate a prebuilt binary ────────────────────────────────────────────────

BINARY=""

try_path() {
  if [ -f "$1" ] && [ -x "$1" ]; then
    BINARY="$1"
  fi
}

# 1. Explicit Cargo output dir override
[ -n "${CARGO_TARGET_DIR:-}" ] && try_path "$CARGO_TARGET_DIR/release/harness-cli"

# 2. Default cargo workspace target
[ -z "$BINARY" ] && try_path "$REPO_ROOT/target/release/harness-cli"

# 3. Crate-local target (less common, but present on some setups)
[ -z "$BINARY" ] && try_path "$REPO_ROOT/crates/harness-cli/target/release/harness-cli"

# ── Build from source if no prebuilt binary found ───────────────────────────
# This branch runs ONLY when none of the paths above exist.
# It requires `cargo` to be on PATH (install via rustup: https://rustup.rs).

if [ -z "$BINARY" ]; then
  if command -v cargo >/dev/null 2>&1; then
    printf 'No prebuilt binary found. Building from source (this may take ~40s)...\n'
    (
      cd "$REPO_ROOT"
      cargo build --package harness-cli --release --quiet
    )
    # After a successful build the binary lands in the default location.
    try_path "$REPO_ROOT/target/release/harness-cli"
    [ -n "$BINARY" ] || { printf 'Error: build succeeded but binary not found at expected path.\n' >&2; exit 1; }
  else
    printf 'Error: no prebuilt binary found and `cargo` is not on PATH.\n' >&2
    printf 'Install Rust via rustup (https://rustup.rs) and re-run, or\n' >&2
    printf 'copy a prebuilt binary to %s/target/release/harness-cli first.\n' "$REPO_ROOT" >&2
    exit 1
  fi
fi

# ── Install ──────────────────────────────────────────────────────────────────

mkdir -p "$(dirname "$DEST")"
cp "$BINARY" "$DEST"
chmod +x "$DEST"

# ── Report ───────────────────────────────────────────────────────────────────

printf '\nInstalled: %s\n' "$DEST"
printf '  source:  %s\n' "$BINARY"

# PATH check
case ":${PATH}:" in
  *":$(dirname "$DEST"):"*)
    printf '  PATH:    ok — %s is on PATH\n' "$(dirname "$DEST")"
    ;;
  *)
    printf '  PATH:    %s is NOT on PATH\n' "$(dirname "$DEST")"
    printf '           Add this line to your shell profile (~/.zshrc, ~/.bashrc, etc.):\n'
    printf '             export PATH="%s:$PATH"\n' "$(dirname "$DEST")"
    ;;
esac

printf '\nEnvironment contract (set in your shell profile):\n'
printf '  export HARNESS_DB=~/.harness/harness.db\n'
printf '  export HARNESS_REPO_ROOT=%s\n' "$REPO_ROOT"
printf '\nFirst-time setup after install:\n'
printf '  mkdir -p ~/.harness\n'
printf '  harness-cli init && harness-cli migrate\n'
printf '\nSee docs/MIGRATION.md for moving an existing DB to a new machine.\n'
