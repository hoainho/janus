/// Preflight check results
#[derive(Debug)]
pub struct PreflightResult {
    pub opencode_available: bool,
    pub api_key_available: bool,
    pub yq_available: bool,
    pub python_available: bool,
}

/// Run preflight checks
pub fn preflight_check() -> PreflightResult {
    PreflightResult {
        opencode_available: check_command("opencode"),
        api_key_available: check_api_key(),
        yq_available: check_command("yq"),
        python_available: check_command("python3"),
    }
}

/// Check if a command is available
fn check_command(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Check if API key is available
fn check_api_key() -> bool {
    std::env::var("ANTHROPIC_API_KEY").is_ok()
}

/// Format preflight results for display
pub fn format_preflight(result: &PreflightResult) -> String {
    let mut lines = Vec::new();
    lines.push("Preflight checks:".to_string());
    lines.push(format!(
        "  opencode: {}",
        if result.opencode_available {
            "✓"
        } else {
            "✗"
        }
    ));
    lines.push(format!(
        "  API key: {}",
        if result.api_key_available {
            "✓"
        } else {
            "✗"
        }
    ));
    lines.push(format!(
        "  yq: {}",
        if result.yq_available { "✓" } else { "✗" }
    ));
    lines.push(format!(
        "  python3: {}",
        if result.python_available {
            "✓"
        } else {
            "✗"
        }
    ));
    lines.join("\n")
}
