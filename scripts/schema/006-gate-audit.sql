-- Harness v0 schema - migration 006
-- Gate audit: T4 verdict on story, lane checklist on intake, unified gate-passage log.

ALTER TABLE story ADD COLUMN t4_verdict TEXT
    CHECK(t4_verdict IN ('pass','ambiguous','fail') OR t4_verdict IS NULL);
ALTER TABLE story ADD COLUMN t4_notes TEXT;
ALTER TABLE story ADD COLUMN t4_recorded_at TEXT;

ALTER TABLE intake ADD COLUMN lane_checklist TEXT;  -- JSON: 10-item flag->rationale

CREATE TABLE gate_log (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    gate        TEXT NOT NULL CHECK(gate IN ('T0','M1','P1','P2','review')),
    story_id    TEXT,
    action      TEXT NOT NULL,
    decision    TEXT,
    detail      TEXT,
    source      TEXT NOT NULL DEFAULT 'agent'
                CHECK(source IN ('human','reviewer','ci','agent'))
);

INSERT INTO schema_version (version) VALUES (6);
