use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttributionClass {
    SkillChanged,
    FixtureStale,
    ModelChanged,
    CrossSkillChange,
    UnknownDrift,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribution {
    pub class: AttributionClass,
    pub details: String,
    pub flaky: bool,
    pub suspected_skills: Option<Vec<String>>,
}

pub fn compute_attribution(
    skill_sha: &str,
    baseline_skill_sha: &str,
    fixture_sha: &str,
    baseline_fixture_sha: &str,
    model_id: &str,
    baseline_model_id: &str,
    stability_samples: usize,
    byte_identical: bool,
    other_skill_shas: &[(String, String, String)],
) -> Attribution {
    let flaky = stability_samples > 1 && !byte_identical;

    if skill_sha != baseline_skill_sha {
        Attribution {
            class: AttributionClass::SkillChanged,
            details: format!("Skill SHA changed: {} -> {}", baseline_skill_sha, skill_sha),
            flaky,
            suspected_skills: None,
        }
    } else if fixture_sha != baseline_fixture_sha {
        Attribution {
            class: AttributionClass::FixtureStale,
            details: format!(
                "Fixture SHA changed: {} -> {}",
                baseline_fixture_sha, fixture_sha
            ),
            flaky,
            suspected_skills: None,
        }
    } else if model_id != baseline_model_id {
        Attribution {
            class: AttributionClass::ModelChanged,
            details: format!("Model changed: {} -> {}", baseline_model_id, model_id),
            flaky,
            suspected_skills: None,
        }
    } else {
        let changed: Vec<String> = other_skill_shas
            .iter()
            .filter(|(_, current, baseline)| current != baseline)
            .map(|(name, _, _)| name.clone())
            .collect();

        if !changed.is_empty() {
            Attribution {
                class: AttributionClass::CrossSkillChange,
                details: format!("Other skills changed: {}", changed.join(", ")),
                flaky,
                suspected_skills: Some(changed),
            }
        } else {
            Attribution {
                class: AttributionClass::UnknownDrift,
                details: "No detectable change in skill, fixture, or model".to_string(),
                flaky,
                suspected_skills: None,
            }
        }
    }
}
