use std::path::Path;
use serde::{Deserialize, Serialize};

/// Eval configuration from .opencode/eval-harness.yaml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalConfig {
    pub model: Option<String>,
    pub budget_usd: Option<f64>,
    pub max_seconds: Option<u64>,
    pub llm_judge: Option<LlmJudgeConfig>,
}

/// LLM judge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmJudgeConfig {
    pub model: Option<String>,
}

/// Load eval config from YAML file
pub fn load_config(path: &Path) -> Result<EvalConfig, String> {
    if !path.exists() {
        return Ok(EvalConfig {
            model: None,
            budget_usd: None,
            max_seconds: None,
            llm_judge: None,
        });
    }
    
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse config YAML: {}", e))
}

/// Apply project config to environment
pub fn apply_project_config(config: &EvalConfig) {
    if let Some(model) = &config.model {
        std::env::set_var("EVAL_MODEL", model);
    }
    if let Some(budget) = config.budget_usd {
        std::env::set_var("EVAL_BUDGET_USD", budget.to_string());
    }
    if let Some(max_seconds) = config.max_seconds {
        std::env::set_var("EVAL_MAX_SECONDS", max_seconds.to_string());
    }
}
