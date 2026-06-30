use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// Context for window-based evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalContext {
    pub window: Option<String>,
    pub files: Option<Vec<ContextFile>>,
    pub variables: Option<std::collections::HashMap<String, String>>,
}

/// File embedded in context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFile {
    pub path: String,
    pub content: String,
}

/// Eval case definition from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalCase {
    pub schema_version: u32,
    pub id: String,
    #[serde(default = "default_mode")]
    pub mode: String,
    pub skill_under_test: String,
    pub skills_loaded: Vec<String>,
    pub description: Option<String>,
    pub model: Option<String>,
    pub setup: Option<CaseSetup>,
    pub prompt: String,
    pub budget: Option<CaseBudget>,
    pub checks: Vec<CaseCheck>,
    pub samples: Option<u32>,
    pub pass_threshold: Option<u32>,
    pub temperature: Option<f64>,
    pub context: Option<EvalContext>,
}

fn default_mode() -> String {
    "deterministic".to_string()
}

/// Case setup with fixtures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseSetup {
    pub fixtures: Option<std::collections::HashMap<String, String>>,
}

/// Case budget constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseBudget {
    pub max_tokens: Option<u64>,
    pub max_seconds: Option<u64>,
}

/// Individual check in a case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseCheck {
    pub kind: String,
    pub cmd: Option<String>,
    pub expect_regex: Option<String>,
    pub expect_min: Option<u64>,
    pub expect_exact: Option<String>,
    pub file: Option<String>,
    pub path: Option<String>,
    pub contains: Option<Vec<String>>,
    pub text: Option<String>,
    pub target_file: Option<String>,
    pub samples: Option<u32>,
    pub judge_model: Option<String>,
    pub rubric: Option<String>,
    pub normalize: Option<Vec<String>>,
}

/// Load eval case from YAML file
pub fn load_case(path: &Path) -> Result<EvalCase, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read case file: {}", e))?;
    serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse case YAML: {}", e))
}

/// Copy fixtures to workdir with path traversal protection
pub fn copy_fixtures(
    case: &EvalCase,
    evals_dir: &Path,
    workdir: &Path,
) -> Result<(), String> {
    let setup = match &case.setup {
        Some(s) => s,
        None => return Ok(()),
    };
    
    let fixtures = match &setup.fixtures {
        Some(f) => f,
        None => return Ok(()),
    };
    
    for (dest, src) in fixtures {
        // Reject absolute paths and path traversal
        if dest.starts_with("/") || dest.contains("..") {
            return Err(format!("Rejected fixture dest='{}' (absolute or contains '..')", dest));
        }
        
        let src_path = if src.starts_with("/") {
            PathBuf::from(src)
        } else {
            evals_dir.join(src)
        };
        
        let full_dest = workdir.join(dest);
        
        // Canonicalize and verify within workdir
        let canonical_dest = full_dest.canonicalize()
            .unwrap_or(full_dest.clone());
        let canonical_workdir = workdir.canonicalize()
            .unwrap_or(workdir.to_path_buf());
        
        if !canonical_dest.starts_with(&canonical_workdir) {
            return Err(format!("Rejected fixture dest='{}' — resolves outside workdir", dest));
        }
        
        // Create parent directory
        if let Some(parent) = full_dest.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("mkdir failed for {}: {}", parent.display(), e))?;
        }
        
        // Copy file
        if src_path.exists() {
            std::fs::copy(&src_path, &full_dest)
                .map_err(|e| format!("cp failed: {} -> {}: {}", src_path.display(), full_dest.display(), e))?;
        } else {
            return Err(format!("Fixture source missing: {}", src_path.display()));
        }
    }
    
    Ok(())
}

/// Discover all case files for a skill
pub fn discover_cases(cases_dir: &Path) -> Result<Vec<PathBuf>, String> {
    if !cases_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut cases: Vec<PathBuf> = std::fs::read_dir(cases_dir)
        .map_err(|e| format!("Failed to read cases dir: {}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "yaml"))
        .collect();
    
    cases.sort();
    Ok(cases)
}
