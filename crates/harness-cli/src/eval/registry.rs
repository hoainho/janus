use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Registry entry for a repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub repo_name: String,
    pub enabled: bool,
    pub enabled_at: String,
}

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub entries: Vec<RegistryEntry>,
}

/// Load registry from file
pub fn load_registry(path: &Path) -> Result<Registry, String> {
    if !path.exists() {
        return Ok(Registry {
            entries: Vec::new(),
        });
    }
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read registry: {}", e))?;
    serde_yaml::from_str(&content).map_err(|e| format!("Failed to parse registry: {}", e))
}

/// Save registry to file
pub fn save_registry(path: &Path, registry: &Registry) -> Result<(), String> {
    let content = serde_yaml::to_string(registry)
        .map_err(|e| format!("Failed to serialize registry: {}", e))?;
    fs::write(path, content).map_err(|e| format!("Failed to write registry: {}", e))
}

/// Check if a repo is enabled in the registry
pub fn is_enabled(registry: &Registry, repo_name: &str) -> bool {
    registry
        .entries
        .iter()
        .any(|e| e.repo_name == repo_name && e.enabled)
}

/// Enable a repo in the registry
pub fn enable(registry: &mut Registry, repo_name: &str) {
    if let Some(entry) = registry
        .entries
        .iter_mut()
        .find(|e| e.repo_name == repo_name)
    {
        entry.enabled = true;
        entry.enabled_at = chrono::Utc::now().to_rfc3339();
    } else {
        registry.entries.push(RegistryEntry {
            repo_name: repo_name.to_string(),
            enabled: true,
            enabled_at: chrono::Utc::now().to_rfc3339(),
        });
    }
}

/// Disable a repo in the registry
pub fn disable(registry: &mut Registry, repo_name: &str) {
    if let Some(entry) = registry
        .entries
        .iter_mut()
        .find(|e| e.repo_name == repo_name)
    {
        entry.enabled = false;
    }
}

/// Get repo name from path
pub fn repo_name_from_path(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string()
}
