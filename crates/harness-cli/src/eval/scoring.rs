use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};

/// Check kinds supported by eval-harness
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckKind {
    Shell,
    JqPathContains,
    FileExists,
    OutputContains,
    OutputNotContains,
    LlmJudge,
    PromptQuality,
    ResponseQuality,
    ContextRelevance,
}

/// Result of a single check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub kind: CheckKind,
    pub passed: bool,
    pub failed_check_id: Option<String>,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub diff_hint: Option<String>,
    pub error: Option<bool>,
}

/// Result of a complete case evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseResult {
    pub case_id: String,
    pub passed: bool,
    pub total: usize,
    pub pass_count: usize,
    pub fail_count: usize,
    pub checks: Vec<CheckResult>,
    pub duration_ms: Option<u64>,
}

/// Run all checks for a case
pub fn run_all_checks(
    case: &super::case::EvalCase,
    workdir: &Path,
    transcript: &Path,
) -> CaseResult {
    let start = std::time::Instant::now();
    let mut checks = Vec::new();
    
    for check in &case.checks {
        let result = match check.kind.as_str() {
            "shell" => run_shell_check(check, workdir),
            "jq_path_contains" => run_jq_check(check, workdir),
            "file_exists" => run_file_exists_check(check, workdir),
            "output_contains" => run_output_contains_check(check, transcript),
            "output_not_contains" => run_output_not_contains_check(check, transcript),
            "llm_judge" => run_llm_judge_check(check, workdir, transcript),
            "prompt_quality" => run_prompt_quality_check(check, transcript),
            "response_quality" => run_response_quality_check(check, transcript),
            "context_relevance" => run_context_relevance_check(check, transcript),
            _ => CheckResult {
                kind: CheckKind::Shell,
                passed: false,
                failed_check_id: Some(format!("unknown_kind:{}", check.kind)),
                expected: Some("known check kind".to_string()),
                actual: Some(check.kind.clone()),
                diff_hint: Some("Unknown check kind".to_string()),
                error: Some(true),
            },
        };
        checks.push(result);
    }
    
    let pass_count = checks.iter().filter(|c| c.passed).count();
    let fail_count = checks.len() - pass_count;
    
    CaseResult {
        case_id: case.id.clone(),
        passed: fail_count == 0,
        total: checks.len(),
        pass_count,
        fail_count,
        checks,
        duration_ms: Some(start.elapsed().as_millis() as u64),
    }
}

/// Run a shell command check
fn run_shell_check(check: &super::case::CaseCheck, workdir: &Path) -> CheckResult {
    let cmd = check.cmd.as_deref().unwrap_or("");
    
    // Safety: reject dangerous commands
    if is_dangerous_command(cmd) {
        return CheckResult {
            kind: CheckKind::Shell,
            passed: false,
            failed_check_id: Some("shell_safety".to_string()),
            expected: Some("safe command".to_string()),
            actual: Some(cmd.to_string()),
            diff_hint: Some("Command rejected by safety filter".to_string()),
            error: Some(true),
        };
    }
    
    // Run command
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .current_dir(workdir)
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            // Check expectations
            if let Some(expect_exact) = &check.expect_exact {
                let passed = stdout.trim() == expect_exact.trim();
                CheckResult {
                    kind: CheckKind::Shell,
                    passed,
                    failed_check_id: if passed { None } else { Some("shell_exact".to_string()) },
                    expected: Some(expect_exact.clone()),
                    actual: Some(stdout.trim().to_string()),
                    diff_hint: if passed { None } else { Some("Output doesn't match expected".to_string()) },
                    error: None,
                }
            } else if let Some(expect_min) = check.expect_min {
                let count: u64 = stdout.trim().parse().unwrap_or(0);
                let passed = count >= expect_min;
                CheckResult {
                    kind: CheckKind::Shell,
                    passed,
                    failed_check_id: if passed { None } else { Some("shell_min".to_string()) },
                    expected: Some(format!(">={}", expect_min)),
                    actual: Some(count.to_string()),
                    diff_hint: if passed { None } else { Some("Count below minimum".to_string()) },
                    error: None,
                }
            } else if let Some(expect_regex) = &check.expect_regex {
                let re = regex::Regex::new(expect_regex);
                match re {
                    Ok(re) => {
                        let passed = re.is_match(&stdout);
                        CheckResult {
                            kind: CheckKind::Shell,
                            passed,
                            failed_check_id: if passed { None } else { Some("shell_regex".to_string()) },
                            expected: Some(expect_regex.clone()),
                            actual: Some(stdout.trim().to_string()),
                            diff_hint: if passed { None } else { Some("Output doesn't match regex".to_string()) },
                            error: None,
                        }
                    }
                    Err(e) => CheckResult {
                        kind: CheckKind::Shell,
                        passed: false,
                        failed_check_id: Some("shell_regex_invalid".to_string()),
                        expected: Some(expect_regex.clone()),
                        actual: Some(format!("Invalid regex: {}", e)),
                        diff_hint: Some("Invalid regex pattern".to_string()),
                        error: Some(true),
                    },
                }
            } else {
                // Just check exit code
                CheckResult {
                    kind: CheckKind::Shell,
                    passed: output.status.success(),
                    failed_check_id: if output.status.success() { None } else { Some("shell_exit".to_string()) },
                    expected: Some("exit 0".to_string()),
                    actual: Some(format!("exit {}", output.status.code().unwrap_or(-1))),
                    diff_hint: if output.status.success() { None } else { Some(stderr.trim().to_string()) },
                    error: None,
                }
            }
        }
        Err(e) => CheckResult {
            kind: CheckKind::Shell,
            passed: false,
            failed_check_id: Some("shell_spawn".to_string()),
            expected: Some("command runs".to_string()),
            actual: Some(format!("Failed to spawn: {}", e)),
            diff_hint: Some("Failed to execute command".to_string()),
            error: Some(true),
        },
    }
}

/// Check if a command is dangerous
pub(crate) fn is_dangerous_command(cmd: &str) -> bool {
    let dangerous_patterns = [
        "rm -rf",
        "rm -r /",
        "$(",
        "`",
    ];
    
    for pattern in &dangerous_patterns {
        if cmd.contains(pattern) {
            return true;
        }
    }
    
    let cmd_lower = cmd.to_lowercase();
    if (cmd_lower.contains("curl") || cmd_lower.contains("wget")) 
        && cmd_lower.contains("|") 
        && (cmd_lower.contains("sh") || cmd_lower.contains("bash")) {
        return true;
    }
    
    false
}

/// Run a jq path check
fn run_jq_check(check: &super::case::CaseCheck, workdir: &Path) -> CheckResult {
    let file = check.file.as_deref().unwrap_or("");
    let path = check.path.as_deref().unwrap_or(".");
    let contains = check.contains.as_deref().unwrap_or(&[]);
    
    let file_path = workdir.join(file);
    if !file_path.exists() {
        return CheckResult {
            kind: CheckKind::JqPathContains,
            passed: false,
            failed_check_id: Some("jq_file_missing".to_string()),
            expected: Some(format!("{} exists", file)),
            actual: Some("file not found".to_string()),
            diff_hint: Some("File not found".to_string()),
            error: Some(true),
        };
    }
    
    // Run jq
    let output = Command::new("jq")
        .arg("-r")
        .arg(path)
        .arg(&file_path)
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let values: Vec<&str> = stdout.lines().collect();
            
            let missing: Vec<String> = contains.iter()
                .filter(|c| !values.contains(&c.as_str()))
                .cloned()
                .collect();
            
            let passed = missing.is_empty();
            CheckResult {
                kind: CheckKind::JqPathContains,
                passed,
                failed_check_id: if passed { None } else { Some("jq_missing_values".to_string()) },
                expected: Some(format!("{:?}", contains)),
                actual: Some(format!("{:?}", values)),
                diff_hint: if passed { None } else { Some(format!("Missing: {:?}", missing)) },
                error: None,
            }
        }
        Err(e) => CheckResult {
            kind: CheckKind::JqPathContains,
            passed: false,
            failed_check_id: Some("jq_spawn".to_string()),
            expected: Some("jq runs".to_string()),
            actual: Some(format!("Failed to spawn jq: {}", e)),
            diff_hint: Some("Failed to execute jq".to_string()),
            error: Some(true),
        },
    }
}

/// Run a file exists check
fn run_file_exists_check(check: &super::case::CaseCheck, workdir: &Path) -> CheckResult {
    let path = check.path.as_deref().unwrap_or("");
    let file_path = workdir.join(path);
    
    let passed = file_path.exists();
    CheckResult {
        kind: CheckKind::FileExists,
        passed,
        failed_check_id: if passed { None } else { Some("file_missing".to_string()) },
        expected: Some(format!("{} exists", path)),
        actual: Some(if passed { "exists" } else { "not found" }.to_string()),
        diff_hint: if passed { None } else { Some("File not found".to_string()) },
        error: None,
    }
}

/// Run an output contains check
fn run_output_contains_check(check: &super::case::CaseCheck, transcript: &Path) -> CheckResult {
    let text = check.text.as_deref().unwrap_or("");
    
    let content = std::fs::read_to_string(transcript).unwrap_or_default();
    let passed = content.contains(text);
    
    CheckResult {
        kind: CheckKind::OutputContains,
        passed,
        failed_check_id: if passed { None } else { Some("output_missing".to_string()) },
        expected: Some(format!("output contains '{}'", text)),
        actual: Some(if passed { "found" } else { "not found" }.to_string()),
        diff_hint: if passed { None } else { Some("Text not found in transcript".to_string()) },
        error: None,
    }
}

/// Run an output not contains check
fn run_output_not_contains_check(check: &super::case::CaseCheck, transcript: &Path) -> CheckResult {
    let text = check.text.as_deref().unwrap_or("");
    
    let content = std::fs::read_to_string(transcript).unwrap_or_default();
    let passed = !content.contains(text);
    
    CheckResult {
        kind: CheckKind::OutputNotContains,
        passed,
        failed_check_id: if passed { None } else { Some("output_unwanted".to_string()) },
        expected: Some(format!("output does NOT contain '{}'", text)),
        actual: Some(if passed { "not found" } else { "found" }.to_string()),
        diff_hint: if passed { None } else { Some("Unwanted text found in transcript".to_string()) },
        error: None,
    }
}

/// Run an LLM judge check
fn run_llm_judge_check(check: &super::case::CaseCheck, workdir: &Path, transcript: &Path) -> CheckResult {
    let target_file = check.target_file.as_deref().unwrap_or("");
    let rubric = check.rubric.as_deref().unwrap_or("");
    let samples = check.samples.unwrap_or(3);
    let judge_model = check.judge_model.as_deref().unwrap_or("anthropic/claude-sonnet-4-6");
    
    let target_path = workdir.join(target_file);
    if !target_path.exists() {
        return CheckResult {
            kind: CheckKind::LlmJudge,
            passed: false,
            failed_check_id: Some("judge_target_missing".to_string()),
            expected: Some(format!("{} exists", target_file)),
            actual: Some("file not found".to_string()),
            diff_hint: Some("Target file not found".to_string()),
            error: Some(true),
        };
    }
    
    let target_content = std::fs::read_to_string(&target_path).unwrap_or_default();
    
    // TODO: Implement actual LLM judge API call
    // For now, return a placeholder
    CheckResult {
        kind: CheckKind::LlmJudge,
        passed: false,
        failed_check_id: Some("judge_not_implemented".to_string()),
        expected: Some("LLM judge evaluates rubric".to_string()),
        actual: Some("not implemented".to_string()),
        diff_hint: Some("LLM judge not yet implemented".to_string()),
        error: Some(true),
    }
}

fn run_prompt_quality_check(check: &super::case::CaseCheck, transcript: &Path) -> CheckResult {
    let content = std::fs::read_to_string(transcript).unwrap_or_default();
    let min_length = check.expect_min.unwrap_or(10);
    
    let prompt_lines: Vec<&str> = content.lines().collect();
    let prompt_text = prompt_lines.join(" ");
    let word_count = prompt_text.split_whitespace().count();
    
    let passed = word_count >= min_length as usize;
    
    CheckResult {
        kind: CheckKind::PromptQuality,
        passed,
        failed_check_id: if passed { None } else { Some("prompt_too_short".to_string()) },
        expected: Some(format!(">={} words", min_length)),
        actual: Some(format!("{} words", word_count)),
        diff_hint: if passed { None } else { Some("Prompt is too short, provide more context".to_string()) },
        error: None,
    }
}

fn run_response_quality_check(check: &super::case::CaseCheck, transcript: &Path) -> CheckResult {
    let content = std::fs::read_to_string(transcript).unwrap_or_default();
    let min_length = check.expect_min.unwrap_or(50);
    
    let response_lines: Vec<&str> = content.lines().collect();
    let response_text = response_lines.join(" ");
    let word_count = response_text.split_whitespace().count();
    
    let passed = word_count >= min_length as usize;
    
    CheckResult {
        kind: CheckKind::ResponseQuality,
        passed,
        failed_check_id: if passed { None } else { Some("response_too_short".to_string()) },
        expected: Some(format!(">={} words", min_length)),
        actual: Some(format!("{} words", word_count)),
        diff_hint: if passed { None } else { Some("Response is too short, provide more detail".to_string()) },
        error: None,
    }
}

fn run_context_relevance_check(check: &super::case::CaseCheck, transcript: &Path) -> CheckResult {
    let content = std::fs::read_to_string(transcript).unwrap_or_default();
    let required_text = check.text.as_deref().unwrap_or("");
    
    let passed = content.contains(required_text);
    
    CheckResult {
        kind: CheckKind::ContextRelevance,
        passed,
        failed_check_id: if passed { None } else { Some("context_not_relevant".to_string()) },
        expected: Some(format!("contains '{}'", required_text)),
        actual: Some(if passed { "found" } else { "not found" }.to_string()),
        diff_hint: if passed { None } else { Some("Response does not contain required context".to_string()) },
        error: None,
    }
}
