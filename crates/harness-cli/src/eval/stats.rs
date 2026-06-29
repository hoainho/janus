use serde::{Deserialize, Serialize};

/// Run summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSummary {
    pub run_id: String,
    pub trigger: String,
    pub skill: String,
    pub verdict: String,
    pub pass: usize,
    pub total: usize,
    pub regression_count: usize,
    pub regressions: Vec<String>,
    pub total_cost_usd: Option<f64>,
    pub duration_ms: Option<u64>,
}

/// Build run summary from case results
pub fn build_run_summary(
    run_id: &str,
    trigger: &str,
    skill: &str,
    case_results: &[crate::eval::scoring::CaseResult],
) -> RunSummary {
    let pass = case_results.iter().filter(|r| r.passed).count();
    let total = case_results.len();
    let regressions: Vec<String> = case_results.iter()
        .filter(|r| !r.passed)
        .map(|r| r.case_id.clone())
        .collect();
    
    let verdict = if regressions.is_empty() {
        "PASS".to_string()
    } else {
        "REGRESSION".to_string()
    };
    
    RunSummary {
        run_id: run_id.to_string(),
        trigger: trigger.to_string(),
        skill: skill.to_string(),
        verdict,
        pass,
        total,
        regression_count: regressions.len(),
        regressions,
        total_cost_usd: None,
        duration_ms: None,
    }
}
