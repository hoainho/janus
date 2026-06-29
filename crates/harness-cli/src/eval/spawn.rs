use std::path::Path;

/// Spawn configuration for opencode
#[derive(Debug, Clone)]
pub struct SpawnConfig {
    pub prompt: String,
    pub workdir: std::path::PathBuf,
    pub sandbox: std::path::PathBuf,
    pub transcript: std::path::PathBuf,
    pub skills_loaded: Vec<String>,
    pub model: Option<String>,
    pub max_seconds: Option<u64>,
    pub temperature: Option<f64>,
}

/// Spawn result
#[derive(Debug)]
pub struct SpawnResult {
    pub exit_code: i32,
    pub transcript_path: std::path::PathBuf,
}

/// Spawn opencode with the given configuration
pub fn spawn_opencode(config: &SpawnConfig) -> Result<SpawnResult, String> {
    std::fs::create_dir_all(&config.workdir)
        .map_err(|e| format!("Failed to create workdir: {}", e))?;
    std::fs::create_dir_all(&config.sandbox)
        .map_err(|e| format!("Failed to create sandbox: {}", e))?;
    
    let mut cmd = std::process::Command::new("opencode");
    cmd.arg("run");
    cmd.arg(&config.prompt);
    cmd.current_dir(&config.workdir);
    
    // Sandbox environment
    cmd.env("HOME", &config.sandbox);
    cmd.env("OPENCODE_CONFIG_DIR", config.sandbox.join("config"));
    
    if let Some(model) = &config.model {
        cmd.env("EVAL_MODEL", model);
    }
    if let Some(temp) = config.temperature {
        cmd.env("EVAL_TEMPERATURE", temp.to_string());
    }
    
    // Skills loaded
    let skills_str = config.skills_loaded.join(",");
    cmd.env("OPENCODE_SKILLS_LOADED", &skills_str);
    
    let output = if let Some(max_seconds) = config.max_seconds {
        // Use timeout command for bounded execution
        let mut timeout_cmd = std::process::Command::new("timeout");
        timeout_cmd.arg(max_seconds.to_string());
        timeout_cmd.arg("opencode");
        timeout_cmd.arg("run");
        timeout_cmd.arg(&config.prompt);
        timeout_cmd.current_dir(&config.workdir);
        timeout_cmd.env("HOME", &config.sandbox);
        timeout_cmd.env("OPENCODE_CONFIG_DIR", config.sandbox.join("config"));
        timeout_cmd.env("OPENCODE_SKILLS_LOADED", &skills_str);
        if let Some(model) = &config.model {
            timeout_cmd.env("EVAL_MODEL", model);
        }
        
        timeout_cmd.output()
            .map_err(|e| format!("Failed to run opencode with timeout: {}", e))?
    } else {
        cmd.output()
            .map_err(|e| format!("Failed to run opencode: {}", e))?
    };
    
    // Write transcript
    std::fs::write(&config.transcript, &output.stdout)
        .map_err(|e| format!("Failed to write transcript: {}", e))?;
    
    // Also write stderr to transcript.err for debugging
    if !output.stderr.is_empty() {
        let err_path = config.transcript.with_extension("jsonl.err");
        let _ = std::fs::write(&err_path, &output.stderr);
    }
    
    let exit_code = output.status.code().unwrap_or(124); // 124 = timeout
    
    Ok(SpawnResult {
        exit_code,
        transcript_path: config.transcript.clone(),
    })
}
