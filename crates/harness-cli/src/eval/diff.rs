use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Diff result for comparing baseline vs current
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    pub case_id: String,
    pub baseline_passed: bool,
    pub current_passed: bool,
    pub regression: bool,
    pub fix_proposal: Option<String>,
}

/// Render diff markdown from run results
pub fn render_diff_md(
    run_id: &str,
    skill: &str,
    case_results: &[super::scoring::CaseResult],
    baselines: &HashMap<String, super::scoring::CaseResult>,
) -> String {
    let mut md = String::new();
    md.push_str(&format!("# Eval Harness Diff — {}\n\n", run_id));
    md.push_str(&format!("**Skill**: {}\n", skill));
    md.push_str(&format!("**Cases**: {}\n\n", case_results.len()));

    // Summary
    let pass = case_results.iter().filter(|r| r.passed).count();
    let total = case_results.len();
    let regressions = case_results.iter().filter(|r| !r.passed).count();

    md.push_str("## Summary\n\n");
    md.push_str(&format!("- **Pass**: {}/{}\n", pass, total));
    md.push_str(&format!("- **Regressions**: {}\n\n", regressions));

    // Case details
    md.push_str("## Cases\n\n");
    for result in case_results {
        let status = if result.passed { "PASS" } else { "FAIL" };
        let baseline_status = baselines
            .get(&result.case_id)
            .map(|b| if b.passed { "PASS" } else { "FAIL" })
            .unwrap_or("N/A");

        md.push_str(&format!("### {} — {}\n\n", result.case_id, status));
        md.push_str(&format!("- **Baseline**: {}\n", baseline_status));
        md.push_str(&format!("- **Current**: {}\n", status));

        if !result.passed {
            md.push_str("- **Regression**: YES\n");
            // Add fix proposal if available
            for check in &result.checks {
                if let Some(hint) = &check.diff_hint {
                    md.push_str(&format!("- **Fix Proposal**: {}\n", hint));
                }
            }
        }
        md.push('\n');
    }

    md
}
