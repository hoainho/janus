use rusqlite::{params, Connection, OptionalExtension};

use super::stats::RunSummary;
use super::scoring::CaseResult;

pub fn save_eval_run(conn: &Connection, summary: &RunSummary) -> Result<(), String> {
    conn.execute(
        "INSERT INTO eval_run (run_id, skill, trigger_type, verdict, pass_count, total_count, regression_count, total_cost_usd, duration_ms, run_dir)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            summary.run_id,
            summary.skill,
            summary.trigger,
            summary.verdict,
            summary.pass as i64,
            summary.total as i64,
            summary.regression_count as i64,
            summary.total_cost_usd,
            summary.duration_ms.map(|d| d as i64),
            format!("runs/{}", summary.run_id),
        ],
    ).map_err(|e| format!("Failed to save eval run: {}", e))?;
    
    Ok(())
}

pub fn save_eval_case(conn: &Connection, run_id: &str, result: &CaseResult) -> Result<(), String> {
    let checks_json = serde_json::to_string(&result.checks).unwrap_or_default();
    
    conn.execute(
        "INSERT INTO eval_case (run_id, case_id, passed, pass_count, fail_count, duration_ms, checks_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            run_id,
            result.case_id,
            result.passed as i64,
            result.pass_count as i64,
            result.fail_count as i64,
            result.duration_ms.map(|d| d as i64),
            checks_json,
        ],
    ).map_err(|e| format!("Failed to save eval case: {}", e))?;
    
    Ok(())
}

pub fn save_eval_history(conn: &Connection, event: &str, run_id: &str, skill: &str, trigger: &str, verdict: &str, summary_json: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO eval_history (event, run_id, skill, trigger, verdict, summary_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![event, run_id, skill, trigger, verdict, summary_json],
    ).map_err(|e| format!("Failed to save eval history: {}", e))?;
    
    Ok(())
}

pub fn save_eval_budget(conn: &Connection, run_id: &str, cost_usd: f64, model: &str) -> Result<(), String> {
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    
    conn.execute(
        "INSERT INTO eval_budget (date, run_id, cost_usd, model)
         VALUES (?1, ?2, ?3, ?4)",
        params![today, run_id, cost_usd, model],
    ).map_err(|e| format!("Failed to save eval budget: {}", e))?;
    
    Ok(())
}

pub fn get_eval_runs(conn: &Connection, skill: Option<&str>, limit: i64) -> Result<Vec<RunSummary>, String> {
    let mut results = Vec::new();
    
    if let Some(skill_name) = skill {
        let mut stmt = conn.prepare(
            "SELECT run_id, trigger_type, skill, verdict, pass_count, total_count, regression_count, total_cost_usd, duration_ms FROM eval_run WHERE skill = ?1 ORDER BY created_at DESC LIMIT ?2"
        ).map_err(|e| format!("Failed to prepare query: {}", e))?;
        
        let rows = stmt.query_map(params![skill_name, limit], |row| {
            Ok(RunSummary {
                run_id: row.get(0)?,
                trigger: row.get(1)?,
                skill: row.get(2)?,
                verdict: row.get(3)?,
                pass: row.get::<_, i64>(4)? as usize,
                total: row.get::<_, i64>(5)? as usize,
                regression_count: row.get::<_, i64>(6)? as usize,
                regressions: Vec::new(),
                total_cost_usd: row.get(7)?,
                duration_ms: row.get::<_, Option<i64>>(8)?.map(|d| d as u64),
            })
        }).map_err(|e| format!("Failed to query eval runs: {}", e))?;
        
        for row in rows {
            results.push(row.map_err(|e| format!("Failed to read row: {}", e))?);
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT run_id, trigger_type, skill, verdict, pass_count, total_count, regression_count, total_cost_usd, duration_ms FROM eval_run ORDER BY created_at DESC LIMIT ?1"
        ).map_err(|e| format!("Failed to prepare query: {}", e))?;
        
        let rows = stmt.query_map(params![limit], |row| {
            Ok(RunSummary {
                run_id: row.get(0)?,
                trigger: row.get(1)?,
                skill: row.get(2)?,
                verdict: row.get(3)?,
                pass: row.get::<_, i64>(4)? as usize,
                total: row.get::<_, i64>(5)? as usize,
                regression_count: row.get::<_, i64>(6)? as usize,
                regressions: Vec::new(),
                total_cost_usd: row.get(7)?,
                duration_ms: row.get::<_, Option<i64>>(8)?.map(|d| d as u64),
            })
        }).map_err(|e| format!("Failed to query eval runs: {}", e))?;
        
        for row in rows {
            results.push(row.map_err(|e| format!("Failed to read row: {}", e))?);
        }
    }
    
    Ok(results)
}

pub fn get_eval_cases(conn: &Connection, run_id: &str) -> Result<Vec<CaseResult>, String> {
    let mut stmt = conn.prepare(
        "SELECT case_id, passed, pass_count, fail_count, duration_ms, checks_json FROM eval_case WHERE run_id = ?1 ORDER BY case_id"
    ).map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let rows = stmt.query_map(params![run_id], |row| {
        let checks_json: String = row.get(5)?;
        let checks: Vec<super::scoring::CheckResult> = serde_json::from_str(&checks_json).unwrap_or_default();
        
        Ok(CaseResult {
            case_id: row.get(0)?,
            passed: row.get::<_, bool>(1)?,
            pass_count: row.get::<_, i64>(2)? as usize,
            fail_count: row.get::<_, i64>(3)? as usize,
            duration_ms: row.get::<_, Option<i64>>(4)?.map(|d| d as u64),
            total: checks.len(),
            checks,
        })
    }).map_err(|e| format!("Failed to query eval cases: {}", e))?;
    
    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|e| format!("Failed to read row: {}", e))?);
    }
    
    Ok(results)
}

pub fn get_eval_history(conn: &Connection, skill: Option<&str>, limit: i64) -> Result<Vec<serde_json::Value>, String> {
    let mut results = Vec::new();
    
    if let Some(skill_name) = skill {
        let mut stmt = conn.prepare(
            "SELECT event, run_id, skill, trigger, verdict, summary_json, created_at FROM eval_history WHERE skill = ?1 ORDER BY created_at DESC LIMIT ?2"
        ).map_err(|e| format!("Failed to prepare query: {}", e))?;
        
        let rows = stmt.query_map(params![skill_name, limit], |row| {
            let summary_json: String = row.get(5)?;
            let summary: serde_json::Value = serde_json::from_str(&summary_json).unwrap_or_default();
            
            Ok(serde_json::json!({
                "event": row.get::<_, String>(0)?,
                "run_id": row.get::<_, String>(1)?,
                "skill": row.get::<_, String>(2)?,
                "trigger": row.get::<_, String>(3)?,
                "verdict": row.get::<_, String>(4)?,
                "summary": summary,
                "created_at": row.get::<_, String>(6)?,
            }))
        }).map_err(|e| format!("Failed to query eval history: {}", e))?;
        
        for row in rows {
            results.push(row.map_err(|e| format!("Failed to read row: {}", e))?);
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT event, run_id, skill, trigger, verdict, summary_json, created_at FROM eval_history ORDER BY created_at DESC LIMIT ?1"
        ).map_err(|e| format!("Failed to prepare query: {}", e))?;
        
        let rows = stmt.query_map(params![limit], |row| {
            let summary_json: String = row.get(5)?;
            let summary: serde_json::Value = serde_json::from_str(&summary_json).unwrap_or_default();
            
            Ok(serde_json::json!({
                "event": row.get::<_, String>(0)?,
                "run_id": row.get::<_, String>(1)?,
                "skill": row.get::<_, String>(2)?,
                "trigger": row.get::<_, String>(3)?,
                "verdict": row.get::<_, String>(4)?,
                "summary": summary,
                "created_at": row.get::<_, String>(6)?,
            }))
        }).map_err(|e| format!("Failed to query eval history: {}", e))?;
        
        for row in rows {
            results.push(row.map_err(|e| format!("Failed to read row: {}", e))?);
        }
    }
    
    Ok(results)
}

pub fn create_eval_goal(conn: &Connection, skill: &str, goal_type: &str, description: &str, target_metric: &str) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO eval_goal (skill, goal_type, description, target_metric) VALUES (?1, ?2, ?3, ?4)",
        params![skill, goal_type, description, target_metric],
    ).map_err(|e| format!("Failed to create eval goal: {}", e))?;
    Ok(conn.last_insert_rowid())
}

pub fn get_eval_goals(conn: &Connection, skill: Option<&str>) -> Result<Vec<serde_json::Value>, String> {
    let mut results = Vec::new();
    if let Some(s) = skill {
        let mut stmt = conn.prepare("SELECT id, skill, goal_type, description, target_metric, current_value, created_at, updated_at FROM eval_goal WHERE skill = ?1 ORDER BY created_at DESC")
            .map_err(|e| format!("Failed to prepare: {}", e))?;
        let rows = stmt.query_map(params![s], |row| {
            Ok(serde_json::json!({"id": row.get::<_, i64>(0)?, "skill": row.get::<_, String>(1)?, "goal_type": row.get::<_, String>(2)?, "description": row.get::<_, String>(3)?, "target_metric": row.get::<_, String>(4)?, "current_value": row.get::<_, Option<f64>>(5)?}))
        }).map_err(|e| format!("Failed to query: {}", e))?;
        for row in rows { results.push(row.map_err(|e| format!("Failed to read: {}", e))?); }
    } else {
        let mut stmt = conn.prepare("SELECT id, skill, goal_type, description, target_metric, current_value, created_at, updated_at FROM eval_goal ORDER BY created_at DESC")
            .map_err(|e| format!("Failed to prepare: {}", e))?;
        let rows = stmt.query_map([], |row| {
            Ok(serde_json::json!({"id": row.get::<_, i64>(0)?, "skill": row.get::<_, String>(1)?, "goal_type": row.get::<_, String>(2)?, "description": row.get::<_, String>(3)?, "target_metric": row.get::<_, String>(4)?, "current_value": row.get::<_, Option<f64>>(5)?}))
        }).map_err(|e| format!("Failed to query: {}", e))?;
        for row in rows { results.push(row.map_err(|e| format!("Failed to read: {}", e))?); }
    }
    Ok(results)
}

pub fn update_eval_goal_progress(conn: &Connection, goal_id: i64, run_id: &str, metric_value: f64, notes: &str) -> Result<(), String> {
    conn.execute("INSERT INTO eval_goal_progress (goal_id, run_id, metric_value, notes) VALUES (?1, ?2, ?3, ?4)", params![goal_id, run_id, metric_value, notes])
        .map_err(|e| format!("Failed to update progress: {}", e))?;
    conn.execute("UPDATE eval_goal SET current_value = ?1, updated_at = datetime('now') WHERE id = ?2", params![metric_value, goal_id])
        .map_err(|e| format!("Failed to update goal: {}", e))?;
    Ok(())
}

pub fn get_goal_progress(conn: &Connection, goal_id: i64) -> Result<Vec<serde_json::Value>, String> {
    let mut stmt = conn.prepare("SELECT id, goal_id, run_id, metric_value, notes, created_at FROM eval_goal_progress WHERE goal_id = ?1 ORDER BY created_at DESC")
        .map_err(|e| format!("Failed to prepare: {}", e))?;
    let rows = stmt.query_map(params![goal_id], |row| {
        Ok(serde_json::json!({"id": row.get::<_, i64>(0)?, "goal_id": row.get::<_, i64>(1)?, "run_id": row.get::<_, String>(2)?, "metric_value": row.get::<_, f64>(3)?, "notes": row.get::<_, String>(4)?, "created_at": row.get::<_, String>(5)?}))
    }).map_err(|e| format!("Failed to query: {}", e))?;
    let mut results = Vec::new();
    for row in rows { results.push(row.map_err(|e| format!("Failed to read: {}", e))?); }
    Ok(results)
}

pub fn create_eval_suggestion(conn: &Connection, skill: &str, suggestion_type: &str, priority: &str, title: &str, description: &str, expected_impact: &str) -> Result<i64, String> {
    conn.execute("INSERT INTO eval_suggestion (skill, suggestion_type, priority, title, description, expected_impact) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", params![skill, suggestion_type, priority, title, description, expected_impact])
        .map_err(|e| format!("Failed to create suggestion: {}", e))?;
    Ok(conn.last_insert_rowid())
}

pub fn get_eval_suggestions(conn: &Connection, skill: Option<&str>, status: Option<&str>) -> Result<Vec<serde_json::Value>, String> {
    let mut results = Vec::new();
    let make_json = |row: &rusqlite::Row| -> rusqlite::Result<serde_json::Value> {
        Ok(serde_json::json!({"id": row.get::<_, i64>(0)?, "skill": row.get::<_, String>(1)?, "suggestion_type": row.get::<_, String>(2)?, "priority": row.get::<_, String>(3)?, "title": row.get::<_, String>(4)?, "status": row.get::<_, String>(7)?}))
    };
    match (skill, status) {
        (Some(s), Some(st)) => {
            let mut stmt = conn.prepare("SELECT id, skill, suggestion_type, priority, title, description, expected_impact, status, created_at FROM eval_suggestion WHERE skill = ?1 AND status = ?2 ORDER BY priority, created_at DESC").map_err(|e| format!("Failed: {}", e))?;
            for row in stmt.query_map(params![s, st], make_json).map_err(|e| format!("Failed: {}", e))? {
                results.push(row.map_err(|e| format!("Failed: {}", e))?);
            }
        }
        (Some(s), None) => {
            let mut stmt = conn.prepare("SELECT id, skill, suggestion_type, priority, title, description, expected_impact, status, created_at FROM eval_suggestion WHERE skill = ?1 ORDER BY priority, created_at DESC").map_err(|e| format!("Failed: {}", e))?;
            for row in stmt.query_map(params![s], make_json).map_err(|e| format!("Failed: {}", e))? {
                results.push(row.map_err(|e| format!("Failed: {}", e))?);
            }
        }
        (None, Some(st)) => {
            let mut stmt = conn.prepare("SELECT id, skill, suggestion_type, priority, title, description, expected_impact, status, created_at FROM eval_suggestion WHERE status = ?1 ORDER BY priority, created_at DESC").map_err(|e| format!("Failed: {}", e))?;
            for row in stmt.query_map(params![st], make_json).map_err(|e| format!("Failed: {}", e))? {
                results.push(row.map_err(|e| format!("Failed: {}", e))?);
            }
        }
        (None, None) => {
            let mut stmt = conn.prepare("SELECT id, skill, suggestion_type, priority, title, description, expected_impact, status, created_at FROM eval_suggestion ORDER BY priority, created_at DESC").map_err(|e| format!("Failed: {}", e))?;
            for row in stmt.query_map([], make_json).map_err(|e| format!("Failed: {}", e))? {
                results.push(row.map_err(|e| format!("Failed: {}", e))?);
            }
        }
    };
    Ok(results)
}

pub fn save_context_case(conn: &rusqlite::Connection, case_id: &str, skill: &str, context: &super::case::EvalContext) -> Result<i64, String> {
    let files_json = serde_json::to_string(&context.files).unwrap_or_default();
    let variables_json = serde_json::to_string(&context.variables).unwrap_or_default();
    
    conn.execute(
        "INSERT INTO eval_context_case (case_id, skill, window_context, files_json, variables_json) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![case_id, skill, context.window, files_json, variables_json],
    ).map_err(|e| format!("Failed to save context case: {}", e))?;
    
    Ok(conn.last_insert_rowid())
}

pub fn get_context_cases(conn: &rusqlite::Connection, skill: Option<&str>) -> Result<Vec<serde_json::Value>, String> {
    let mut results = Vec::new();
    
    if let Some(s) = skill {
        let mut stmt = conn.prepare("SELECT id, case_id, skill, window_context, files_json, variables_json, created_at FROM eval_context_case WHERE skill = ?1 ORDER BY created_at DESC")
            .map_err(|e| format!("Failed to prepare: {}", e))?;
        let rows = stmt.query_map(rusqlite::params![s], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i64>(0)?,
                "case_id": row.get::<_, String>(1)?,
                "skill": row.get::<_, String>(2)?,
                "window_context": row.get::<_, Option<String>>(3)?,
                "files_json": row.get::<_, Option<String>>(4)?,
                "variables_json": row.get::<_, Option<String>>(5)?,
                "created_at": row.get::<_, String>(6)?,
            }))
        }).map_err(|e| format!("Failed to query: {}", e))?;
        for row in rows { results.push(row.map_err(|e| format!("Failed to read: {}", e))?); }
    } else {
        let mut stmt = conn.prepare("SELECT id, case_id, skill, window_context, files_json, variables_json, created_at FROM eval_context_case ORDER BY created_at DESC")
            .map_err(|e| format!("Failed to prepare: {}", e))?;
        let rows = stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i64>(0)?,
                "case_id": row.get::<_, String>(1)?,
                "skill": row.get::<_, String>(2)?,
                "window_context": row.get::<_, Option<String>>(3)?,
                "files_json": row.get::<_, Option<String>>(4)?,
                "variables_json": row.get::<_, Option<String>>(5)?,
                "created_at": row.get::<_, String>(6)?,
            }))
        }).map_err(|e| format!("Failed to query: {}", e))?;
        for row in rows { results.push(row.map_err(|e| format!("Failed to read: {}", e))?); }
    }
    
    Ok(results)
}

pub fn save_quality_score(conn: &rusqlite::Connection, run_id: &str, case_id: &str, prompt_score: f64, response_score: f64, context_score: f64, overall_score: f64, feedback: &str) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO eval_quality_score (run_id, case_id, prompt_score, response_score, context_score, overall_score, feedback) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![run_id, case_id, prompt_score, response_score, context_score, overall_score, feedback],
    ).map_err(|e| format!("Failed to save quality score: {}", e))?;
    
    Ok(conn.last_insert_rowid())
}

// === BASELINE FUNCTIONS ===

pub fn save_eval_baseline(conn: &Connection, skill: &str, case_id: &str, checks_json: &str, env_manifest: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO eval_baseline (skill, case_id, checks_json, env_manifest)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(skill, case_id) DO UPDATE SET
           checks_json = excluded.checks_json,
           env_manifest = excluded.env_manifest,
           created_at = datetime('now')",
        params![skill, case_id, checks_json, env_manifest],
    ).map_err(|e| format!("Failed to save eval baseline: {}", e))?;
    Ok(())
}

pub fn get_eval_baseline(conn: &Connection, skill: &str, case_id: &str) -> Result<Option<serde_json::Value>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, skill, case_id, created_at, checks_json, env_manifest FROM eval_baseline WHERE skill = ?1 AND case_id = ?2"
    ).map_err(|e| format!("Failed to prepare: {}", e))?;
    
    let result = stmt.query_row(params![skill, case_id], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "skill": row.get::<_, String>(1)?,
            "case_id": row.get::<_, String>(2)?,
            "created_at": row.get::<_, String>(3)?,
            "checks_json": row.get::<_, Option<String>>(4)?,
            "env_manifest": row.get::<_, Option<String>>(5)?,
        }))
    }).optional().map_err(|e| format!("Failed to query baseline: {}", e))?;
    
    Ok(result)
}

pub fn get_eval_baselines(conn: &Connection, skill: &str) -> Result<Vec<serde_json::Value>, String> {
    let mut results = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT id, skill, case_id, created_at, checks_json, env_manifest FROM eval_baseline WHERE skill = ?1 ORDER BY case_id"
    ).map_err(|e| format!("Failed to prepare: {}", e))?;
    
    let rows = stmt.query_map(params![skill], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "skill": row.get::<_, String>(1)?,
            "case_id": row.get::<_, String>(2)?,
            "created_at": row.get::<_, String>(3)?,
            "checks_json": row.get::<_, Option<String>>(4)?,
            "env_manifest": row.get::<_, Option<String>>(5)?,
        }))
    }).map_err(|e| format!("Failed to query baselines: {}", e))?;
    
    for row in rows { results.push(row.map_err(|e| format!("Failed to read: {}", e))?); }
    Ok(results)
}

pub fn delete_eval_baselines(conn: &Connection, skill: &str) -> Result<(), String> {
    conn.execute("DELETE FROM eval_baseline WHERE skill = ?1", params![skill])
        .map_err(|e| format!("Failed to delete baselines: {}", e))?;
    Ok(())
}

// === PROMOTION FUNCTIONS ===

pub fn get_promoted_status(conn: &Connection, skill: &str) -> Result<bool, String> {
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM eval_promoted WHERE skill = ?1"
    ).map_err(|e| format!("Failed to prepare: {}", e))?;
    
    let count: i64 = stmt.query_row(params![skill], |row| row.get(0))
        .map_err(|e| format!("Failed to query: {}", e))?;
    
    Ok(count > 0)
}

pub fn set_promoted_status(conn: &Connection, skill: &str, promoted: bool) -> Result<(), String> {
    if promoted {
        conn.execute(
            "INSERT OR IGNORE INTO eval_promoted (skill, promoted_at) VALUES (?1, datetime('now'))",
            params![skill],
        ).map_err(|e| format!("Failed to promote: {}", e))?;
    } else {
        conn.execute(
            "DELETE FROM eval_promoted WHERE skill = ?1",
            params![skill],
        ).map_err(|e| format!("Failed to demote: {}", e))?;
    }
    Ok(())
}

pub fn get_green_days(conn: &Connection, skill: &str) -> Result<i64, String> {
    let mut stmt = conn.prepare(
        "SELECT COUNT(DISTINCT date(created_at)) FROM eval_run WHERE skill = ?1 AND verdict = 'PASS'"
    ).map_err(|e| format!("Failed to prepare: {}", e))?;
    
    let count: i64 = stmt.query_row(params![skill], |row| row.get(0))
        .map_err(|e| format!("Failed to query: {}", e))?;
    
    Ok(count)
}

pub fn get_quality_scores(conn: &rusqlite::Connection, run_id: Option<&str>, case_id: Option<&str>) -> Result<Vec<serde_json::Value>, String> {
    let mut results = Vec::new();
    
    match (run_id, case_id) {
        (Some(r), Some(c)) => {
            let mut stmt = conn.prepare("SELECT id, run_id, case_id, prompt_score, response_score, context_score, overall_score, feedback, created_at FROM eval_quality_score WHERE run_id = ?1 AND case_id = ?2 ORDER BY created_at DESC")
                .map_err(|e| format!("Failed to prepare: {}", e))?;
            let rows = stmt.query_map(rusqlite::params![r, c], |row| {
                Ok(serde_json::json!({
                    "id": row.get::<_, i64>(0)?,
                    "run_id": row.get::<_, String>(1)?,
                    "case_id": row.get::<_, String>(2)?,
                    "prompt_score": row.get::<_, f64>(3)?,
                    "response_score": row.get::<_, f64>(4)?,
                    "context_score": row.get::<_, f64>(5)?,
                    "overall_score": row.get::<_, f64>(6)?,
                    "feedback": row.get::<_, String>(7)?,
                    "created_at": row.get::<_, String>(8)?,
                }))
            }).map_err(|e| format!("Failed to query: {}", e))?;
            for row in rows { results.push(row.map_err(|e| format!("Failed to read: {}", e))?); }
        }
        (Some(r), None) => {
            let mut stmt = conn.prepare("SELECT id, run_id, case_id, prompt_score, response_score, context_score, overall_score, feedback, created_at FROM eval_quality_score WHERE run_id = ?1 ORDER BY created_at DESC")
                .map_err(|e| format!("Failed to prepare: {}", e))?;
            let rows = stmt.query_map(rusqlite::params![r], |row| {
                Ok(serde_json::json!({
                    "id": row.get::<_, i64>(0)?,
                    "run_id": row.get::<_, String>(1)?,
                    "case_id": row.get::<_, String>(2)?,
                    "prompt_score": row.get::<_, f64>(3)?,
                    "response_score": row.get::<_, f64>(4)?,
                    "context_score": row.get::<_, f64>(5)?,
                    "overall_score": row.get::<_, f64>(6)?,
                    "feedback": row.get::<_, String>(7)?,
                    "created_at": row.get::<_, String>(8)?,
                }))
            }).map_err(|e| format!("Failed to query: {}", e))?;
            for row in rows { results.push(row.map_err(|e| format!("Failed to read: {}", e))?); }
        }
        (None, Some(c)) => {
            let mut stmt = conn.prepare("SELECT id, run_id, case_id, prompt_score, response_score, context_score, overall_score, feedback, created_at FROM eval_quality_score WHERE case_id = ?1 ORDER BY created_at DESC")
                .map_err(|e| format!("Failed to prepare: {}", e))?;
            let rows = stmt.query_map(rusqlite::params![c], |row| {
                Ok(serde_json::json!({
                    "id": row.get::<_, i64>(0)?,
                    "run_id": row.get::<_, String>(1)?,
                    "case_id": row.get::<_, String>(2)?,
                    "prompt_score": row.get::<_, f64>(3)?,
                    "response_score": row.get::<_, f64>(4)?,
                    "context_score": row.get::<_, f64>(5)?,
                    "overall_score": row.get::<_, f64>(6)?,
                    "feedback": row.get::<_, String>(7)?,
                    "created_at": row.get::<_, String>(8)?,
                }))
            }).map_err(|e| format!("Failed to query: {}", e))?;
            for row in rows { results.push(row.map_err(|e| format!("Failed to read: {}", e))?); }
        }
        (None, None) => {
            let mut stmt = conn.prepare("SELECT id, run_id, case_id, prompt_score, response_score, context_score, overall_score, feedback, created_at FROM eval_quality_score ORDER BY created_at DESC")
                .map_err(|e| format!("Failed to prepare: {}", e))?;
            let rows = stmt.query_map([], |row| {
                Ok(serde_json::json!({
                    "id": row.get::<_, i64>(0)?,
                    "run_id": row.get::<_, String>(1)?,
                    "case_id": row.get::<_, String>(2)?,
                    "prompt_score": row.get::<_, f64>(3)?,
                    "response_score": row.get::<_, f64>(4)?,
                    "context_score": row.get::<_, f64>(5)?,
                    "overall_score": row.get::<_, f64>(6)?,
                    "feedback": row.get::<_, String>(7)?,
                    "created_at": row.get::<_, String>(8)?,
                }))
            }).map_err(|e| format!("Failed to query: {}", e))?;
            for row in rows { results.push(row.map_err(|e| format!("Failed to read: {}", e))?); }
        }
    }
    
    Ok(results)
}
