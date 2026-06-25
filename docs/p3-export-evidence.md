# Task #5 — `export` command evidence

## Files changed

- `crates/harness-cli/src/domain.rs` — added `StoryExportRecord` struct (id, title, lane, status, proof flags, last_verified_result, t4_verdict, t4_notes, evidence)
- `crates/harness-cli/src/infrastructure.rs` — added `StoryExportRecord` import; added `query_export_matrix` and `query_export_story` to the `HarnessRepository` trait and `SqliteHarnessRepository` impl
- `crates/harness-cli/src/application.rs` — added `StoryExportRecord` import; added `query_export_matrix` and `query_export_story` pass-through methods to `HarnessService`
- `crates/harness-cli/src/interface.rs` — added `StoryExportRecord` import; added `Export(ExportArgs)` top-level command variant with `ExportView::{Matrix, Story}` subcommands; added `ExportArgs`, `ExportView`, `ExportStoryArgs` structs; added `print_export_matrix_md` and `print_export_story_md` helpers; added `md_proof` and `md_opt` helpers

## Build

```
cargo build --release   # exit 0, 0 warnings
```

## Sample: `export matrix`

```
| ID | Lane | Status | unit | integration | e2e | platform | verify | t4_verdict | evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| US-001 | normal | in_progress | yes | yes | no | no |  | pass |  |
| US-002 | high_risk | planned | no | no | no | no |  |  |  |
```

## Sample: `export story --id US-001`

```
## US-001 — Login flow

- **lane**: normal
- **status**: in_progress
- **proof**: unit=yes integration=yes e2e=no platform=no
- **verify result**: 
- **t4_verdict**: pass
```

## Not-found error (exit 1)

```
error: story update: story 'US-999' not found
```
