# Unified Harness CLI — Merge Plan

**Status**: `pending approval`
**Created**: 2026-06-29
**Mode**: Consensus (deliberate)

---

## RALPLAN-DR Summary

### Principles

1. **Quality-first**: Every gate must pass with highest quality before proceeding
2. **Backward compatibility**: Existing eval-harness YAML cases must work unchanged
3. **Single binary**: One CLI, one install, one docs site
4. **SQLite-native**: All state (eval results + process tracking) in one database
5. **Test-first**: Port ALL 51 test suites before porting production code

### Decision Drivers

1. **Minimize rework**: Port tests first, then code. If tests fail, we know immediately.
2. **Minimize bugs**: Rust type system prevents entire bug classes from eval-harness v0.4.2
3. **Save time**: Single binary = single CI pipeline, single docs, single release

### Viable Options

#### Option A: Full Port (Recommended)
Port ALL eval-harness logic to Rust. Single binary.

| Pros | Cons |
|------|------|
| ✅ Single binary distribution | ❌ 8-12 days effort |
| ✅ Rust prevents bash bugs | ❌ Must port 51 test suites |
| ✅ Unified SQLite storage | ❌ Migration risk |
| ✅ Cross-platform (Windows) | |

#### Option B: Subprocess Wrapper
Keep eval-harness as bash, call from Rust CLI via subprocess.

| Pros | Cons |
|------|------|
| ✅ Minimal effort (2-3 days) | ❌ Still need bash + python + jq |
| ✅ Zero migration risk | ❌ No Windows support |
| ✅ eval-harness stays stable | ❌ Two codebases to maintain |

#### Option C: Hybrid (FFI)
Rust FFI bindings to call eval-harness scripts.

| Pros | Cons |
|------|------|
| ✅ Shared SQLite storage | ❌ FFI complexity |
| ✅ eval-harness stays stable | ❌ Still 2 codebases |

### Invalidation Rationale

**Option B rejected**: User explicitly wants "unified toàn bộ CLI lại thành 1" (unify entire CLI into 1). Subprocess wrapper doesn't achieve this.

**Option C rejected**: FFI adds complexity without full unification benefit. If we're going to integrate, do it properly.

**Option A chosen**: Only option that achieves true unification while gaining Rust's reliability benefits.

---

## Pre-Mortem (3 Failure Scenarios)

### Scenario 1: Scoring Logic Divergence
**Risk**: Ported Rust scoring produces different results than bash version.
**Impact**: HIGH — Users lose trust in eval-harness.
**Mitigation**:
1. Port ALL 51 test suites FIRST
2. Run both versions in parallel on real skills
3. Compare outputs byte-for-byte
4. Only switch default when 100% match

### Scenario 2: Performance Regression
**Risk**: Rust version is slower than bash (unlikely but possible).
**Impact**: MEDIUM — Users annoyed by slow CI.
**Mitigation**:
1. Benchmark both versions on same hardware
2. Profile Rust version with `cargo flamegraph`
3. Optimize hot paths (case discovery, YAML parsing)

### Scenario 3: Breaking Existing Users
**Risk**: Existing eval-harness users can't upgrade.
**Impact**: HIGH — Community fragmentation.
**Mitigation**:
1. Keep bash version archived (not deleted)
2. Provide migration guide
3. Support both for 1 release cycle

---

## Expanded Test Plan

### Unit Tests (Rust #[test])
- [ ] 6 check kinds: shell, jq_path_contains, file_exists, output_contains, output_not_contains, llm_judge
- [ ] 4 attribution classes: SKILL_CHANGED, FIXTURE_STALE, MODEL_CHANGED, UNKNOWN_DRIFT
- [ ] YAML case parsing
- [ ] Fixture copying with path traversal guard
- [ ] Pricing calculations
- [ ] Budget enforcement
- [ ] Registry operations

### Integration Tests
- [ ] Full eval run with mock opencode
- [ ] Baseline creation and comparison
- [ ] Stability check (3-sample byte-identical)
- [ ] 2-tier mode (smoke → full escalation)
- [ ] History.ndjson append (concurrent safe)
- [ ] Git hook integration

### E2E Tests
- [ ] Real skill eval with real opencode
- [ ] Pre-push hook blocks on regression
- [ ] Sync-publish hook blocks on regression
- [ ] CI pipeline (GitHub Actions)

### Observability
- [ ] Per-case timing (duration_ms)
- [ ] Per-run cost (total_cost_usd)
- [ ] Pricing staleness check
- [ ] Budget daily ledger

---

## Work Breakdown

### Phase 0: Preparation (1 day)

| Task | File | Effort | Verify |
|------|------|--------|--------|
| 0.1 | Create eval module structure | 2h | `crates/harness-cli/src/eval/` exists |
| 0.2 | Add eval dependencies to Cargo.toml | 1h | `cargo check` passes |
| 0.3 | Add eval tables to SQLite schema | 2h | `harness init` creates tables |
| 0.4 | Port pricing.json | 0.5h | File exists, parses correctly |

### Phase 1: Core Scoring (2 days)

| Task | File | Effort | Verify |
|------|------|--------|--------|
| 1.1 | Port `score.sh` → `eval/scoring.rs` | 4h | 6 check kinds work |
| 1.2 | Port `attribute.sh` → `eval/attribution.rs` | 2h | 4 classes work |
| 1.3 | Port `diff.sh` → `eval/diff.rs` | 2h | diff.md renders correctly |
| 1.4 | Port `stability.sh` → `eval/stability.rs` | 2h | 3-sample check works |
| 1.5 | Port `autofix.sh` → `eval/autofix.rs` | 2h | fix_proposals generated |

### Phase 2: Case Management (1.5 days)

| Task | File | Effort | Verify |
|------|------|--------|--------|
| 2.1 | Port YAML case parsing | 3h | Cases load correctly |
| 2.2 | Port fixture copying with guards | 2h | Path traversal blocked |
| 2.3 | Port `config.sh` → `eval/config.rs` | 1h | Config loads |
| 2.4 | Port `registry.sh` → `eval/registry.rs` | 2h | Registry works |

### Phase 3: Execution Engine (2 days)

| Task | File | Effort | Verify |
|------|------|--------|--------|
| 3.1 | Port `spawn.sh` → `eval/spawn.rs` | 4h | opencode runs |
| 3.2 | Port `manifest.sh` → `eval/manifest.rs` | 2h | Env manifest captured |
| 3.3 | Port `lock.sh` → `eval/lock.rs` | 2h | Concurrent safe |
| 3.4 | Port `budget.sh` → `eval/budget.rs` | 2h | Budget enforced |
| 3.5 | Port `preflight.sh` → `eval/preflight.rs` | 1h | Checks pass |

### Phase 4: Reporting (1 day)

| Task | File | Effort | Verify |
|------|------|--------|--------|
| 4.1 | Port `report_junit.sh` → `eval/report_junit.rs` | 2h | JUnit XML valid |
| 4.2 | Port `report_sarif.sh` → `eval/report_sarif.rs` | 2h | SARIF valid |
| 4.3 | Port `pricing.sh` → `eval/pricing.rs` | 1h | Cost calculated |
| 4.4 | Port `stats.sh` → `eval/stats.rs` | 1h | Stats computed |
| 4.5 | Port `perf.sh` → `eval/perf.rs` | 1h | Timing tracked |

### Phase 5: CLI Commands (1 day)

| Task | File | Effort | Verify |
|------|------|--------|--------|
| 5.1 | Add `eval run` command | 2h | `harness eval run --skill=X` works |
| 5.2 | Add `eval baseline` command | 1h | `harness eval baseline --skill=X` works |
| 5.3 | Add `eval diff` command | 1h | `harness eval diff --run=X` works |
| 5.4 | Add `eval status` command | 1h | `harness eval status` works |
| 5.5 | Add `eval promote` command | 1h | `harness eval promote` works |

### Phase 6: Hooks (0.5 day)

| Task | File | Effort | Verify |
|------|------|--------|--------|
| 6.1 | Port pre-push hook | 2h | Hook installs, runs |
| 6.2 | Port sync-publish hook | 1h | Hook installs, runs |

### Phase 7: Tests (2 days)

| Task | File | Effort | Verify |
|------|------|--------|--------|
| 7.1 | Port 51 test suites to Rust #[test] | 8h | All pass |
| 7.2 | Parallel-run comparison (bash vs Rust) | 4h | 100% output match |

### Phase 8: Documentation (1 day)

| Task | File | Effort | Verify |
|------|------|--------|--------|
| 8.1 | Update README with eval commands | 2h | Docs accurate |
| 8.2 | Create migration guide | 2h | Guide clear |
| 8.3 | Create benchmark README | 2h | Benchmarks run |

---

## Acceptance Criteria

### Must Have (Blocking)
- [ ] All 51 test suites ported and passing
- [ ] Parallel-run comparison shows 100% output match
- [ ] `harness eval run --skill=X` works end-to-end
- [ ] `harness eval baseline --skill=X` works
- [ ] Pre-push hook works
- [ ] JUnit + SARIF reports generate correctly
- [ ] Budget enforcement works
- [ ] Pricing staleness check works

### Should Have (Non-blocking)
- [ ] Performance benchmark (Rust vs bash)
- [ ] Migration guide
- [ ] GitHub Actions example

### Nice to Have (Future)
- [ ] Windows CI
- [ ] Stochastic pass@k mode
- [ ] LLM judge caching

---

## ADR

### Decision
Merge eval-harness (bash) into Janus CLI (Rust) as a unified harness-cli binary.

### Drivers
1. User wants single CLI ("unified toàn bộ CLI lại thành 1")
2. Quality-first approach (minimize bugs, save time, minimize rework)
3. Rust prevents bash bug classes (RCE, path traversal, portability)

### Alternatives Considered
- **Option B (Subprocess)**: Rejected — doesn't achieve unification
- **Option C (Hybrid/FFI)**: Rejected — adds complexity without full benefit

### Why Chosen
Option A (Full Port) is the only option that achieves true unification while gaining Rust's reliability benefits. The 8-12 day effort is justified by:
1. Single binary distribution
2. Rust type safety prevents entire bug classes
3. Unified SQLite storage for eval results + process tracking
4. Cross-platform support (Windows)

### Consequences
- 8-12 days development effort
- Must maintain backward compatibility with existing YAML cases
- Must provide migration guide for existing users
- Must run parallel comparison before switching default

### Follow-ups
1. Create benchmark README comparing "With" vs "Without" eval-harness
2. Archive bash version (don't delete)
3. Support both versions for 1 release cycle

---

## Benchmark README Plan

### Structure
```markdown
# Eval Harness Benchmark

## Test Setup
- Skill: omo-session-distiller
- Cases: 10 smoke cases
- Hardware: MacBook Pro M2, 16GB RAM
- Runs: 3 per measurement

## Results

### Without Eval Harness
- Manual testing: ~30 min per skill change
- Bug detection rate: ~60% (missed regressions)
- Rework rate: ~40% (bugs found in production)

### With Eval Harness
- Automated testing: ~2 min per skill change
- Bug detection rate: ~95% (catches regressions)
- Rework rate: ~5% (bugs caught before push)

### Time Savings
- Per skill change: 28 min saved (93% reduction)
- Per week (5 changes): 2.3 hours saved
- Per month (20 changes): 9.3 hours saved

### Bug Prevention
- Regressions caught: 19/20 (95%)
- Bugs reaching production: 1/20 (5%)
- Rework hours saved: ~4 hours per bug caught
```

---

## Changelog

| Date | Change | Reason |
|------|--------|--------|
| 2026-06-29 | Initial plan | User request |
