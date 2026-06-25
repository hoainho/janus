# Task #12 — install.sh + Distribution Evidence

## Install test run

Command:
```
CARGO_TARGET_DIR=<scratchpad>/target-native bash scripts/install.sh
```

Output:
```
Installed: /Users/nhonh/.local/bin/harness-cli
  source:  <scratchpad>/target-native/release/harness-cli
  PATH:    ok — /Users/nhonh/.local/bin is on PATH

Environment contract (set in your shell profile):
  export HARNESS_DB=~/.harness/harness.db
  export HARNESS_REPO_ROOT=/Users/nhonh/Documents/personal/harness-cli

First-time setup after install:
  mkdir -p ~/.harness
  harness-cli init && harness-cli migrate

See docs/MIGRATION.md for moving an existing DB to a new machine.
```

Binary verification: `harness-cli 0.1.10`

## Observations
- Prebuilt binary path (CARGO_TARGET_DIR) was used; cargo build was NOT invoked.
- ~/.local/bin is on PATH.
- install.sh is idempotent: re-running copies the binary again without error.

## Files created
- /Users/nhonh/Documents/personal/harness-cli/scripts/install.sh
- /Users/nhonh/Documents/personal/harness-cli/docs/DISTRIBUTION.md
