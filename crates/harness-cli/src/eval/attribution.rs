use serde::{Deserialize, Serialize};

/// Attribution classes for eval-harness
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttributionClass {
    SkillChanged,
    FixtureStale,
    ModelChanged,
    UnknownDrift,
}

/// Attribution result for a failed case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribution {
    pub class: AttributionClass,
    pub details: String,
    pub flaky: bool,
}

/// Compute attribution for a failed case
pub fn compute_attribution(
    skill_sha: &str,
    baseline_skill_sha: &str,
    fixture_sha: &str,
    baseline_fixture_sha: &str,
    model_id: &str,
    baseline_model_id: &str,
    stability_samples: usize,
    byte_identical: bool,
) -> Attribution {
    let flaky = stability_samples > 1 && !byte_identical;

    if skill_sha != baseline_skill_sha {
        Attribution {
            class: AttributionClass::SkillChanged,
            details: format!("Skill SHA changed: {} -> {}", baseline_skill_sha, skill_sha),
            flaky,
        }
    } else if fixture_sha != baseline_fixture_sha {
        Attribution {
            class: AttributionClass::FixtureStale,
            details: format!("Fixture SHA changed: {} -> {}", baseline_fixture_sha, fixture_sha),
            flaky,
        }
    } else if model_id != baseline_model_id {
        Attribution {
            class: AttributionClass::ModelChanged,
            details: format!("Model changed: {} -> {}", baseline_model_id, model_id),
            flaky,
        }
    } else {
        Attribution {
            class: AttributionClass::UnknownDrift,
            details: "No detectable change in skill, fixture, or model".to_string(),
            flaky,
        }
    }
}
