# Task #11 Evidence — `query gcr`

## Files changed

- `crates/harness-cli/src/domain.rs` — added `GcrRecord` struct (story_id, lane, gates_recorded, gates_expected, gcr f64, rag String)
- `crates/harness-cli/src/infrastructure.rs` — added `GcrRecord` import; added `query_gcr(&self) -> Result<Vec<GcrRecord>>` to `HarnessRepository` trait; added `query_gcr` impl on `SqliteHarnessRepository` using the full CTE SQL from §2.4 of the spec
- `crates/harness-cli/src/application.rs` — added `GcrRecord` import; added `query_gcr` passthrough on `HarnessService`
- `crates/harness-cli/src/interface.rs` — added `GcrRecord` import; added `Gcr` variant to `QueryView` enum; wired handler in `run()`; added `print_gcr` function (table per story + overall average row)

## Build

```
cargo build --release   # exit 0, 0 warnings
```

## Sample output (`query gcr` on seeded DB)

Seed: intake + story add + story update (--unit 1 --evidence x) + story t4 --verdict pass + gate-log --gate P1 --decision approved + gate-log --gate review --decision pass + trace --story WIN-9001 --outcome completed --changed <files>

```
story_id  lane    recorded  expected  GCR%   rag  
--------  ------  --------  --------  -----  -----
WIN-9001  normal  8         9         88.9%  green

Overall: 8/9 gates recorded — avg GCR 88.9% (green)
```

Gates recorded for WIN-9001 (normal lane, push happened):
- T0: intake row exists (1)
- T1: intake row exists (1)
- M1: 0 (trace.files_read has no ws-memories/nano-brain mention)
- T2: story row exists (1)
- T3: unit_proof=1 (1)
- Review: gate_log gate='review' decision='pass' (1)
- P1: gate_log gate='P1' decision='approved' (1)
- T4: t4_verdict='pass' (1)
- M2: 0 (trace outcome=completed but standard tier check requires agent+actions_taken+files_read via intake join; trace was not linked to intake by id)

gates_expected = 2 (T0+T1) + 5 (M1,T2,T3,Review,T4) + 1 (P1 conditional) + 1 (M2) = 9
gates_recorded = 8 → GCR = 8/9 = 0.889 → green
