# Handoff: Unified Harness CLI Implementation

## Completed

### Phase 0: Preparation ✅
- Created `crates/harness-cli/src/eval/` module structure
- Added dependencies: serde, serde_json, serde_yaml, chrono, regex
- Created migration `007-eval-harness.sql` with 5 tables:
  - `eval_run` - tracks eval runs
  - `eval_case` - individual case results
  - `eval_baseline` - baseline snapshots
  - `eval_history` - event log
  - `eval_budget` - daily budget tracking
- Ported `pricing.json` to `crates/harness-cli/pricing.json`

### Phase 1: Core Scoring ✅
Implemented in `src/eval/scoring.rs`:
- `run_all_checks()` - runs all checks for a case
- `run_shell_check()` - shell command execution with safety filter
- `run_jq_check()` - jq path validation
- `run_file_exists_check()` - file existence check
- `run_output_contains_check()` - transcript grep
- `run_output_not_contains_check()` - inverse grep
- `run_llm_judge_check()` - placeholder for LLM judge

Safety features:
- `is_dangerous_command()` - rejects rm -rf, curl | sh, command substitution

### Phase 2: Case Management ✅
Implemented in `src/eval/case.rs`:
- `load_case()` - loads YAML case files
- `copy_fixtures()` - copies fixtures with path traversal protection
- `discover_cases()` - discovers all case files for a skill

Path traversal protection:
- Rejects absolute paths
- Rejects `..` in paths
- Canonicalizes and verifies within workdir

### Supporting Modules ✅
- `attribution.rs` - 4 attribution classes (SKILL_CHANGED, FIXTURE_STALE, MODEL_CHANGED, UNKNOWN_DRIFT)
- `diff.rs` - diff markdown rendering
- `stability.rs` - 3-sample byte-identical check
- `pricing.rs` - pricing data loading and staleness check
- `config.rs` - eval config loading
- `registry.rs` - repo registry management
- `manifest.rs` - environment manifest capture
- `lock.rs` - concurrent run locking
- `budget.rs` - daily budget enforcement
- `preflight.rs` - preflight checks
- `spawn.rs` - opencode process spawning
- `stats.rs` - run summary building
- `report.rs` - JUnit XML rendering

## Remaining Work

### Phase 3: Execution Engine (Not Started)
- Wire up spawn.rs to actually run opencode
- Implement proper directory hashing in manifest.rs
- Add timeout handling

### Phase 4: Reporting (Not Started)
- Implement SARIF report format
- Add per-case timing instrumentation
- Add pricing staleness gate

### Phase 5: CLI Commands (Not Started)
- Add `eval run` command to interface.rs
- Add `eval baseline` command
- Add `eval diff` command
- Add `eval status` command
- Add `eval promote` command

### Phase 6: Hooks (Not Started)
- Port pre-push hook
- Port sync-publish hook

### Phase 7: Tests (Not Started)
- Port 51 test suites to Rust #[test]
- Run parallel comparison (bash vs Rust)

### Phase 8: Documentation (Not Started)
- Update README with eval commands
- Create migration guide
- Create benchmark README

## Key Decisions

1. **Module structure**: Each eval-harness lib file maps to a Rust module
2. **Safety first**: All shell commands go through `is_dangerous_command()` filter
3. **Path traversal protection**: All file operations verify within workdir
4. **Backward compatibility**: YAML case format preserved unchanged

## Risks

1. **LLM judge not implemented**: Currently returns placeholder. Needs Anthropic API integration.
2. **Spawn not tested**: `spawn.rs` needs real opencode integration testing.
3. **No tests yet**: Phase 7 is critical for validation.

## Next Steps

1. Continue with Phase 3-8 as outlined in plan
2. Test each phase thoroughly before moving on
3. Run parallel comparison with bash version when Phase 7 complete

## Files Modified

- `crates/harness-cli/Cargo.toml` - added dependencies
- `crates/harness-cli/src/main.rs` - added eval module
- `crates/harness-cli/src/eval/mod.rs` - module declarations
- `crates/harness-cli/src/eval/*.rs` - 15 new files
- `scripts/schema/007-eval-harness.sql` - new migration
- `crates/harness-cli/pricing.json` - ported pricing data
