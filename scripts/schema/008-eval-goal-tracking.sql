-- Goal tracking tables — migration 008
-- Adds goal-driven improvement tracking for eval harness.

INSERT INTO schema_version (version) VALUES (8);

----------------------------------------------------------------------
-- eval_goal: Goal tracking for eval harness
----------------------------------------------------------------------
CREATE TABLE eval_goal (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    skill           TEXT NOT NULL,
    goal_type       TEXT NOT NULL
                    CHECK(goal_type IN (
                        'regression_detection',
                        'quality_assessment',
                        'performance_comparison',
                        'model_compatibility',
                        'custom'
                    )),
    description     TEXT,
    target_metric   TEXT,
    current_value   REAL,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

----------------------------------------------------------------------
-- eval_goal_progress: Track goal progress over time
----------------------------------------------------------------------
CREATE TABLE eval_goal_progress (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    goal_id         INTEGER NOT NULL REFERENCES eval_goal(id),
    run_id          TEXT NOT NULL,
    metric_value    REAL,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

----------------------------------------------------------------------
-- eval_suggestion: Improvement suggestions based on eval data
----------------------------------------------------------------------
CREATE TABLE eval_suggestion (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    skill           TEXT NOT NULL,
    suggestion_type TEXT NOT NULL
                    CHECK(suggestion_type IN ('fix', 'add', 'remove', 'optimize', 'upgrade')),
    priority        TEXT NOT NULL
                    CHECK(priority IN ('high', 'medium', 'low')),
    title           TEXT NOT NULL,
    description     TEXT,
    expected_impact TEXT,
    status          TEXT NOT NULL DEFAULT 'pending'
                    CHECK(status IN ('pending', 'accepted', 'rejected', 'implemented')),
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    implemented_at  TEXT
);

----------------------------------------------------------------------
-- Indexes
----------------------------------------------------------------------
CREATE INDEX idx_eval_goal_skill ON eval_goal(skill);
CREATE INDEX idx_eval_goal_progress_goal ON eval_goal_progress(goal_id);
CREATE INDEX idx_eval_goal_progress_run ON eval_goal_progress(run_id);
CREATE INDEX idx_eval_suggestion_skill ON eval_suggestion(skill);
CREATE INDEX idx_eval_suggestion_status ON eval_suggestion(status);
