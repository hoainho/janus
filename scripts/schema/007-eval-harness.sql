-- Eval harness tables — migration 007
-- Adds tables for behavior-regression eval harness.

INSERT INTO schema_version (version) VALUES (7);

----------------------------------------------------------------------
-- EvalRun: tracks eval harness runs
----------------------------------------------------------------------
CREATE TABLE eval_run (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id          TEXT    NOT NULL UNIQUE,
    created_at      TEXT    NOT NULL DEFAULT (datetime('now')),
    skill           TEXT    NOT NULL,
    trigger         TEXT    NOT NULL
                    CHECK(trigger IN ('manual', 'pre-push', 'sync-publish', 'stop-hook')),
    verdict         TEXT    NOT NULL
                    CHECK(verdict IN ('PASS', 'REGRESSION', 'FAIL', 'ERROR')),
    pass_count      INTEGER NOT NULL DEFAULT 0,
    total_count     INTEGER NOT NULL DEFAULT 0,
    regression_count INTEGER NOT NULL DEFAULT 0,
    total_cost_usd  REAL,
    duration_ms     INTEGER,
    run_dir         TEXT    NOT NULL
);

----------------------------------------------------------------------
-- EvalCase: individual case results within a run
----------------------------------------------------------------------
CREATE TABLE eval_case (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id          TEXT    NOT NULL REFERENCES eval_run(run_id),
    case_id         TEXT    NOT NULL,
    passed          INTEGER NOT NULL DEFAULT 0,
    pass_count      INTEGER NOT NULL DEFAULT 0,
    fail_count      INTEGER NOT NULL DEFAULT 0,
    duration_ms     INTEGER,
    checks_json    TEXT,  -- JSON array of check results
    stability_json TEXT,  -- JSON stability check result
    UNIQUE(run_id, case_id)
);

----------------------------------------------------------------------
-- EvalBaseline: baseline snapshots for comparison
----------------------------------------------------------------------
CREATE TABLE eval_baseline (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    skill           TEXT    NOT NULL,
    case_id         TEXT    NOT NULL,
    created_at      TEXT    NOT NULL DEFAULT (datetime('now')),
    checks_json    TEXT,  -- JSON array of check results
    env_manifest   TEXT,  -- JSON environment manifest
    UNIQUE(skill, case_id)
);

----------------------------------------------------------------------
-- EvalHistory: event log for trend analysis
----------------------------------------------------------------------
CREATE TABLE eval_history (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at      TEXT    NOT NULL DEFAULT (datetime('now')),
    event           TEXT    NOT NULL,
    run_id          TEXT,
    skill           TEXT,
    trigger         TEXT,
    verdict         TEXT,
    summary_json    TEXT
);

----------------------------------------------------------------------
-- EvalBudget: daily budget tracking
----------------------------------------------------------------------
CREATE TABLE eval_budget (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    date            TEXT    NOT NULL,
    run_id          TEXT    NOT NULL,
    cost_usd        REAL    NOT NULL,
    model           TEXT,
    created_at      TEXT    NOT NULL DEFAULT (datetime('now'))
);

----------------------------------------------------------------------
-- Indexes for eval tables
----------------------------------------------------------------------
CREATE INDEX idx_eval_run_skill ON eval_run(skill);
CREATE INDEX idx_eval_run_created ON eval_run(created_at);
CREATE INDEX idx_eval_case_run ON eval_case(run_id);
CREATE INDEX idx_eval_baseline_skill ON eval_baseline(skill);
CREATE INDEX idx_eval_history_created ON eval_history(created_at);
CREATE INDEX idx_eval_budget_date ON eval_budget(date);
