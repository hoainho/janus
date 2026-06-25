# SQLite Harness Schema — Full Data Model, Dialect Table, and Postgres OLAP Projection Sketch

Generated: 2026-06-25. Source: `scripts/schema/001-init.sql` through `005-tool-extensions.sql`.

---

## 1. Full Data Model

### `schema_version`
Tracks applied migrations. No FK relationships.

| Column | SQLite Type | Constraints |
|--------|------------|-------------|
| `version` | INTEGER | PRIMARY KEY |
| `applied_at` | TEXT | NOT NULL, DEFAULT `datetime('now')` |

Seed rows: `(1)`, `(2)`, `(3)`, `(4)`, `(5)` — one per migration file.

---

### `intake`
Classifies incoming work before a story is created.

| Column | SQLite Type | Constraints / Notes |
|--------|------------|---------------------|
| `id` | INTEGER | PRIMARY KEY AUTOINCREMENT |
| `created_at` | TEXT | NOT NULL, DEFAULT `datetime('now')` |
| `input_type` | TEXT | NOT NULL, CHECK IN `('new_spec','spec_slice','change_request','new_initiative','maintenance','harness_improvement')` |
| `summary` | TEXT | NOT NULL |
| `risk_lane` | TEXT | NOT NULL, CHECK IN `('tiny','normal','high_risk')` |
| `risk_flags` | TEXT | JSON array (e.g. `["auth","data_model"]`), nullable |
| `affected_docs` | TEXT | JSON array of doc paths, nullable |
| `story_id` | TEXT | Soft FK → `story.id` (no REFERENCES constraint declared), nullable |
| `notes` | TEXT | nullable |

---

### `story`
Work packets representing a unit of validated delivery.

**After migration 001:**

| Column | SQLite Type | Constraints / Notes |
|--------|------------|---------------------|
| `id` | TEXT | PRIMARY KEY (e.g. `US-001`) |
| `title` | TEXT | NOT NULL |
| `created_at` | TEXT | NOT NULL, DEFAULT `datetime('now')` |
| `risk_lane` | TEXT | NOT NULL, CHECK IN `('tiny','normal','high_risk')` |
| `contract_doc` | TEXT | path to product doc, nullable |
| `status` | TEXT | NOT NULL, DEFAULT `'planned'`, CHECK IN `('planned','in_progress','implemented','changed','retired')` |
| `unit_proof` | INTEGER | NOT NULL DEFAULT 0 — **integer boolean** (0/1) |
| `integration_proof` | INTEGER | NOT NULL DEFAULT 0 — **integer boolean** (0/1) |
| `e2e_proof` | INTEGER | NOT NULL DEFAULT 0 — **integer boolean** (0/1) |
| `platform_proof` | INTEGER | NOT NULL DEFAULT 0 — **integer boolean** (0/1) |
| `evidence` | TEXT | nullable (likely JSON or path) |
| `notes` | TEXT | nullable |

**Added by migration 002:**

| Column | SQLite Type | Constraints / Notes |
|--------|------------|---------------------|
| `verify_command` | TEXT | nullable |
| `last_verified_at` | TEXT | nullable |
| `last_verified_result` | TEXT | CHECK IN `('pass','fail')` OR NULL |

---

### `decision`
Durable ADR-style records, optionally re-verifiable.

| Column | SQLite Type | Constraints / Notes |
|--------|------------|---------------------|
| `id` | TEXT | PRIMARY KEY (e.g. `0001`) |
| `title` | TEXT | NOT NULL |
| `created_at` | TEXT | NOT NULL, DEFAULT `datetime('now')` |
| `status` | TEXT | NOT NULL, DEFAULT `'proposed'`, CHECK IN `('proposed','accepted','superseded','rejected')` |
| `doc_path` | TEXT | path to ADR markdown, nullable |
| `verify_command` | TEXT | nullable |
| `last_verified_at` | TEXT | nullable |
| `last_verified_result` | TEXT | CHECK IN `('pass','fail')` OR NULL |
| `predicted_impact` | TEXT | nullable |
| `actual_outcome` | TEXT | nullable |
| `notes` | TEXT | nullable |

---

### `backlog`
Harness improvement proposals with lifecycle tracking.

| Column | SQLite Type | Constraints / Notes |
|--------|------------|---------------------|
| `id` | INTEGER | PRIMARY KEY AUTOINCREMENT |
| `created_at` | TEXT | NOT NULL, DEFAULT `datetime('now')` |
| `title` | TEXT | NOT NULL |
| `discovered_while` | TEXT | nullable |
| `current_pain` | TEXT | nullable |
| `suggested_improvement` | TEXT | nullable |
| `risk` | TEXT | CHECK IN `('tiny','normal','high_risk')`, nullable |
| `status` | TEXT | NOT NULL, DEFAULT `'proposed'`, CHECK IN `('proposed','accepted','implemented','rejected')` |
| `predicted_impact` | TEXT | nullable |
| `actual_outcome` | TEXT | nullable |
| `implemented_at` | TEXT | nullable |
| `notes` | TEXT | nullable |

---

### `trace`
Agent task execution records — primary observability table.

| Column | SQLite Type | Constraints / Notes |
|--------|------------|---------------------|
| `id` | INTEGER | PRIMARY KEY AUTOINCREMENT |
| `created_at` | TEXT | NOT NULL, DEFAULT `datetime('now')` |
| `task_summary` | TEXT | NOT NULL |
| `intake_id` | INTEGER | REFERENCES `intake(id)`, nullable |
| `story_id` | TEXT | REFERENCES `story(id)`, nullable |
| `agent` | TEXT | nullable |
| `actions_taken` | TEXT | **JSON array** stored as TEXT, nullable |
| `files_read` | TEXT | **JSON array** stored as TEXT, nullable |
| `files_changed` | TEXT | **JSON array** stored as TEXT, nullable |
| `decisions_made` | TEXT | **JSON array** stored as TEXT, nullable |
| `errors` | TEXT | **JSON array** stored as TEXT, nullable |
| `outcome` | TEXT | CHECK IN `('completed','blocked','partial','failed')`, nullable |
| `duration_seconds` | INTEGER | nullable |
| `token_estimate` | INTEGER | nullable |
| `harness_friction` | TEXT | nullable |
| `notes` | TEXT | nullable |

FK chain: `trace.intake_id → intake.id`, `trace.story_id → story.id`.

---

### `intervention`
Separates review/override/escalation events from normal trace records.

| Column | SQLite Type | Constraints / Notes |
|--------|------------|---------------------|
| `id` | INTEGER | PRIMARY KEY AUTOINCREMENT |
| `created_at` | TEXT | NOT NULL, DEFAULT `datetime('now')` |
| `trace_id` | INTEGER | REFERENCES `trace(id)`, nullable |
| `story_id` | TEXT | nullable (no REFERENCES — soft FK) |
| `type` | TEXT | NOT NULL, CHECK IN `('correction','override','escalation','approval')` |
| `description` | TEXT | NOT NULL |
| `source` | TEXT | NOT NULL, CHECK IN `('human','reviewer','ci','agent')` |
| `impact` | TEXT | nullable |

---

### `tool`
Machine-readable registry of project tools.

**After migration 003:**

| Column | SQLite Type | Constraints / Notes |
|--------|------------|---------------------|
| `name` | TEXT | PRIMARY KEY |
| `created_at` | TEXT | NOT NULL, DEFAULT `datetime('now')` |
| `provider` | TEXT | NOT NULL DEFAULT `'custom'` |
| `command` | TEXT | NOT NULL |
| `description` | TEXT | NOT NULL |
| `args` | TEXT | nullable |
| `responsibility` | TEXT | NOT NULL |
| `since` | TEXT | NOT NULL DEFAULT `'registered'` |

**Added by migration 005 (with backfill UPDATE):**

| Column | SQLite Type | Constraints / Notes |
|--------|------------|---------------------|
| `kind` | TEXT | NOT NULL DEFAULT `'cli'`; backfilled: `'mcp'` if `command LIKE 'mcp:%'`, `'skill'` if `command LIKE 'skill:%'` |
| `capability` | TEXT | nullable |
| `scan_target` | TEXT | nullable |
| `status` | TEXT | NOT NULL DEFAULT `'unknown'` |
| `checked_at` | TEXT | nullable |

---

### Migration Evolution Summary

| Migration | Changes |
|-----------|---------|
| 001 | Creates `schema_version`, `intake`, `story`, `decision`, `backlog`, `trace` |
| 002 | Adds `verify_command`, `last_verified_at`, `last_verified_result` to `story` |
| 003 | Creates `tool` (basic registry: name, command, description, responsibility) |
| 004 | Creates `intervention` (review/override/escalation events linked to trace) |
| 005 | Extends `tool` with `kind`, `capability`, `scan_target`, `status`, `checked_at`; backfills `kind` from command prefix |

---

## 2. Dialect Table: SQLite-ism → Postgres Equivalent

| SQLite construct | Postgres equivalent | Notes / Non-trivial flags |
|-----------------|---------------------|---------------------------|
| `PRAGMA journal_mode = WAL` | n/a — WAL is Postgres' default mode | Drop entirely on migration; Postgres uses WAL-based MVCC natively |
| `PRAGMA foreign_keys = ON` | n/a — Postgres enforces FKs by default | Drop entirely |
| `INTEGER PRIMARY KEY AUTOINCREMENT` | `BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY` or `BIGSERIAL PRIMARY KEY` | Prefer `GENERATED ALWAYS AS IDENTITY` (SQL standard). `AUTOINCREMENT` in SQLite guarantees monotonic non-reuse; Postgres sequences can gap on rollback but do not reuse — semantically equivalent for harness use. |
| `TEXT PRIMARY KEY` (story.id, decision.id, tool.name) | `TEXT PRIMARY KEY` | Direct mapping; no change needed |
| `DEFAULT (datetime('now'))` | `DEFAULT now()` | SQLite returns an ISO-8601 string; Postgres `now()` returns `timestamptz`. On projection, parse the TEXT column as `::timestamptz` |
| `TEXT` columns storing timestamps (`created_at`, `last_verified_at`, etc.) | `TIMESTAMPTZ` | **Non-trivial**: existing SQLite rows hold strings. ETL must cast `TEXT → TIMESTAMPTZ`; rows with non-conforming values will error — add `NULLIF` guard on projection job |
| Integer boolean columns (`unit_proof`, `integration_proof`, `e2e_proof`, `platform_proof`) | `BOOLEAN` | SQLite stores 0/1; cast `CASE WHEN col = 1 THEN TRUE ELSE FALSE END` in projection SELECT, or `col::boolean` after confirming only 0/1 values |
| `CHECK(col IN (...))` | `CHECK(col IN (...))` or `CREATE TYPE ... AS ENUM` | Direct CHECK constraint works. PG enums are more type-safe but add migration cost; for a read-only OLAP projection, CHECK constraints on TEXT columns are sufficient and simpler |
| `TEXT` columns storing JSON arrays (`risk_flags`, `affected_docs`, `actions_taken`, `files_read`, `files_changed`, `decisions_made`, `errors`) | `JSONB` | **Non-trivial**: projection job should cast `col::jsonb` (will error on malformed JSON — add `CASE WHEN col IS NULL OR col = '' THEN NULL ELSE col::jsonb END`). JSONB enables `@>`, `->`, `->>` operators and GIN indexing on Postgres for analytics queries |
| `ON CONFLICT` (used implicitly by AUTOINCREMENT uniqueness) | `ON CONFLICT DO NOTHING` / `ON CONFLICT DO UPDATE` | SQLite's `INSERT OR IGNORE` / `INSERT OR REPLACE` map to `INSERT ... ON CONFLICT ... DO NOTHING` / `DO UPDATE SET ...` respectively. The projection job uses INSERT-only or upsert; no replace semantics needed for read-derived flow |
| `REFERENCES table(col)` without `ON DELETE` | `REFERENCES table(col)` | SQLite only enforces if `PRAGMA foreign_keys = ON`; Postgres always enforces. For the OLAP projection, define FKs as `DEFERRABLE INITIALLY DEFERRED` to tolerate bulk-load ordering issues |
| Soft FK (`intake.story_id TEXT` with no REFERENCES) | `TEXT` column or add FK `REFERENCES story(id)` | Non-trivial: the soft FK in `intake` means rows can exist with `story_id` that has no matching story. On Postgres projection, either keep as `TEXT` (safe) or enforce FK with `NOT VALID` to skip historical orphans |
| `LIKE 'mcp:%'` in UPDATE (005 backfill) | Identical in Postgres | `LIKE` is compatible; the backfill is a one-time migration step |

---

## 3. Postgres OLAP Projection Sketch

### 3.1 Why one-way READ-DERIVED projection, not dual-write

Dual-write (writing the same record to both SQLite and Postgres simultaneously) introduces:
- **Atomicity gap**: SQLite and Postgres transactions are independent; a crash between the two writes leaves them inconsistent with no rollback path.
- **Latency coupling**: every harness agent write would block on a network call to Azure PG.
- **Divergence surface**: any code path that only updates one store (a missed call site, a bulk operation, a migration rollback) silently diverges the two.

One-way projection eliminates all three risks. SQLite is the single writer and source of truth. A periodic job (or CDC-style tail) reads from SQLite and upserts into Postgres. The projection can be torn down, rebuilt from scratch, or paused without touching the OLTP store. Postgres becomes a queryable snapshot — stale by at most one projection interval, but never corrupted by write conflicts.

---

### 3.2 Tables that feed analytics

| Table | Analytics value | Projection priority |
|-------|----------------|---------------------|
| `trace` | Core observability: agent identity, task outcomes, duration, token spend, files touched, error patterns | **P0 — primary fact table** |
| `story` | Proof-gate completion rates, lane distribution, verification coverage, status lifecycle | **P0 — dimension + fact** |
| `intake` | Work classification trends, risk flag frequency, lane assignment accuracy | **P1** |
| `intervention` | Review/override/escalation rates, source breakdown (human vs CI vs agent), impact patterns | **P1** |
| `decision` | ADR lifecycle, verify-command pass/fail trends, outcome accuracy (predicted vs actual) | **P2** |
| `backlog` | Harness improvement velocity, pain-to-implementation lag | **P2** |
| `tool` | Tool availability/reliability over time (status + checked_at), kind distribution | **P3 — supplementary** |
| `schema_version` | Audit only; not projected | **skip** |

---

### 3.3 Target schema shape on Azure PostgreSQL

```sql
-- harness_projection schema on Azure PG (read-only, rebuilt from SQLite)

CREATE SCHEMA IF NOT EXISTS harness;

-- Dimension: story with proof gates
CREATE TABLE harness.story (
    id                   TEXT PRIMARY KEY,
    title                TEXT NOT NULL,
    created_at           TIMESTAMPTZ,
    risk_lane            TEXT CHECK(risk_lane IN ('tiny','normal','high_risk')),
    contract_doc         TEXT,
    status               TEXT CHECK(status IN ('planned','in_progress','implemented','changed','retired')),
    unit_proof           BOOLEAN NOT NULL DEFAULT FALSE,
    integration_proof    BOOLEAN NOT NULL DEFAULT FALSE,
    e2e_proof            BOOLEAN NOT NULL DEFAULT FALSE,
    platform_proof       BOOLEAN NOT NULL DEFAULT FALSE,
    evidence             TEXT,
    notes                TEXT,
    verify_command       TEXT,
    last_verified_at     TIMESTAMPTZ,
    last_verified_result TEXT CHECK(last_verified_result IN ('pass','fail') OR last_verified_result IS NULL),
    _projected_at        TIMESTAMPTZ NOT NULL DEFAULT now()  -- projection metadata
);

-- Dimension: intake classification
CREATE TABLE harness.intake (
    id              BIGINT PRIMARY KEY,
    created_at      TIMESTAMPTZ,
    input_type      TEXT,
    summary         TEXT,
    risk_lane       TEXT,
    risk_flags      JSONB,       -- cast from TEXT JSON array
    affected_docs   JSONB,       -- cast from TEXT JSON array
    story_id        TEXT,        -- soft FK; not enforced
    notes           TEXT,
    _projected_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Fact: agent execution traces
CREATE TABLE harness.trace (
    id               BIGINT PRIMARY KEY,
    created_at       TIMESTAMPTZ,
    task_summary     TEXT,
    intake_id        BIGINT REFERENCES harness.intake(id) DEFERRABLE INITIALLY DEFERRED,
    story_id         TEXT   REFERENCES harness.story(id)  DEFERRABLE INITIALLY DEFERRED,
    agent            TEXT,
    actions_taken    JSONB,      -- cast from TEXT JSON array
    files_read       JSONB,      -- cast from TEXT JSON array
    files_changed    JSONB,      -- cast from TEXT JSON array
    decisions_made   JSONB,      -- cast from TEXT JSON array
    errors           JSONB,      -- cast from TEXT JSON array
    outcome          TEXT,
    duration_seconds INTEGER,
    token_estimate   INTEGER,
    harness_friction TEXT,
    notes            TEXT,
    _projected_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX ON harness.trace(story_id);
CREATE INDEX ON harness.trace(agent);
CREATE INDEX ON harness.trace(created_at);
CREATE INDEX ON harness.trace USING GIN(errors);       -- query error patterns
CREATE INDEX ON harness.trace USING GIN(files_changed); -- query churn by file

-- Fact: interventions
CREATE TABLE harness.intervention (
    id           BIGINT PRIMARY KEY,
    created_at   TIMESTAMPTZ,
    trace_id     BIGINT REFERENCES harness.trace(id) DEFERRABLE INITIALLY DEFERRED,
    story_id     TEXT,
    type         TEXT,
    description  TEXT,
    source       TEXT,
    impact       TEXT,
    _projected_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX ON harness.intervention(trace_id);
CREATE INDEX ON harness.intervention(story_id);

-- Supplementary: decisions
CREATE TABLE harness.decision (
    id                   TEXT PRIMARY KEY,
    title                TEXT,
    created_at           TIMESTAMPTZ,
    status               TEXT,
    doc_path             TEXT,
    verify_command       TEXT,
    last_verified_at     TIMESTAMPTZ,
    last_verified_result TEXT,
    predicted_impact     TEXT,
    actual_outcome       TEXT,
    notes                TEXT,
    _projected_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

---

### 3.4 Projection job design

**Mode**: periodic batch upsert (not dual-write, not streaming CDC).

**Pseudocode:**
```
for each table in [intake, story, trace, intervention, decision]:
    SELECT * FROM sqlite_db.table WHERE id > last_projected_id
    for each row:
        cast TEXT timestamps → TIMESTAMPTZ (nullify on parse failure)
        cast integer booleans → BOOLEAN
        cast JSON TEXT columns → JSONB (nullify on parse failure)
    UPSERT INTO harness.schema.table ON CONFLICT (id) DO UPDATE SET ...
    persist last_projected_id to projection_state table
```

**Trigger interval**: run on-demand or on a slow cron (hourly or post-sprint).
No real-time requirement — analytics lag of minutes to hours is acceptable.

**Error handling**: malformed JSON or unparseable timestamps → project the row with `NULL` in the affected column + log to a `harness.projection_errors` sidecar table. Never skip the row silently.

---

### 3.5 Joins with BI / App Insights that become possible

| Query | Tables joined | Enabled by |
|-------|--------------|------------|
| Token spend per agent per sprint | `trace JOIN story` on `story_id` | `trace.token_estimate` + `story.created_at` |
| Proof-gate coverage by risk lane | `story` (aggregation) | `unit_proof`/`integration_proof`/`e2e_proof` as BOOLEAN, `risk_lane` |
| Error pattern frequency across stories | `trace` (GIN index on `errors` JSONB) | `jsonb_array_elements(errors)` + GROUP BY |
| Override/escalation rate per story | `intervention JOIN trace JOIN story` | `intervention.type`, `intervention.source` |
| Files with highest churn | `trace` (GIN index on `files_changed`) | `jsonb_array_elements_text(files_changed)` + COUNT |
| Risk-flag → outcome correlation | `intake JOIN trace` on `intake_id` | `intake.risk_flags` JSONB + `trace.outcome` |
| App Insights correlation | `trace.agent` + `trace.created_at` → join to App Insights `customDimensions` on session/agent ID | Requires App Insights export to same Azure PG or Log Analytics workspace; `trace.created_at` is the time-align key |
| Sprint velocity (stories closed per week) | `story` WHERE `status = 'implemented'` + DATE_TRUNC on `last_verified_at` | Direct once timestamps are TIMESTAMPTZ |

---

### 3.6 Trigger condition for turning on the projection

**Turn on the projection when ALL of the following are true:**

1. `trace` table has accumulated ≥ 500 rows across ≥ 2 sprint cycles — enough volume for aggregate queries to be meaningful rather than noise.
2. At least one concrete analytics question has been articulated that cannot be answered by a direct SQLite query (i.e., a cross-dataset join with App Insights, BI events, or a dashboard that requires SQL-over-HTTP).
3. The projection job has been dry-run successfully against a copy of the SQLite file with zero projection errors on all 5 tables.
4. Azure PG connection string + credentials are stored in Key Vault (never in the projection job source).
5. The `_projected_at` column is queryable and the freshness SLA (acceptable lag) is documented — prevents "is this data current?" confusion in dashboards.

Until those conditions are met, direct SQLite queries (`sqlite3 harness.db "SELECT ..."`) cover all operational needs at zero infrastructure cost.
