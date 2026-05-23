# Review Fixes: 1d30bf62 to main

Base: `1d30bf62a30cd7e65ebcefed765b3f924d381b49`
Starting head: `fd8151968e7e0623ce76beadb2c41641268c0691`
Branch: `review/main-1d30bf62-to-fd81519`
Harness intake: `#34`

## Pass 1

- Status: findings fixed; validation in progress.
- Command: `codex review --base 1d30bf62a30cd7e65ebcefed765b3f924d381b49`
- Findings:
  - P2: `decision verify` ran stored commands from the caller cwd instead of
    the Harness repo root.
  - P3: Rust intake insertion stored absent `--flags` and `--docs` lists as the
    text `"null"` instead of SQL `NULL`.
- Fixes:
  - Set decision verification commands to run with `self.repo_root` as cwd.
  - Store absent intake list fields with `CsvList::as_json_text()` so rusqlite
    binds SQL `NULL`.
  - Added regression coverage for both behaviors.
  - Updated US-002 validation evidence from 9 to 10 Rust tests.
- Validation:
  - `cargo fmt --check`
  - `cargo test --workspace` passed with 10 tests.
  - `bash -n scripts/install-harness.sh scripts/harness scripts/build-harness-cli-release.sh`
  - `git diff --check`
  - `scripts/harness query matrix`

## Pass 2

- Status: finding fixed; validation in progress.
- Command: `codex review --base 1d30bf62a30cd7e65ebcefed765b3f924d381b49`
- Findings:
  - P2: `--refresh-agent-shim` could overwrite an existing
    `$BACKUP_DIR/AGENTS.md` created earlier by `--override` or `--force`.
- Fixes:
  - Made `backup_agent_file` preserve an existing `AGENTS.md` backup instead
    of replacing it during the refresh step.
- Validation:
  - Temp `--override --refresh-agent-shim --yes` install preserved the original
    `AGENTS.md` in `.harness-backup/.../AGENTS.md`.
  - `bash -n scripts/install-harness.sh scripts/harness scripts/build-harness-cli-release.sh`
  - `cargo fmt --check`
  - `cargo test --workspace` passed with 10 tests.
  - `git diff --check`
  - `scripts/harness query matrix`
