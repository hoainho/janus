use std::path::Path;
use std::collections::BTreeMap;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvManifest {
    pub skill_bundle_sha: String,
    pub skill_sha: String,
    pub fixture_sha: String,
    pub model_id: String,
    pub opencode_version: String,
    pub platform: String,
    pub timestamp: String,
}

pub fn capture_manifest(
    skill_dir: &Path,
    fixtures_dir: &Path,
    model_id: &str,
) -> EnvManifest {
    let skill_sha = hash_directory(skill_dir);
    let fixture_sha = hash_directory(fixtures_dir);
    
    EnvManifest {
        skill_bundle_sha: skill_sha.clone(),
        skill_sha,
        fixture_sha,
        model_id: model_id.to_string(),
        opencode_version: get_opencode_version(),
        platform: std::env::consts::OS.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

fn hash_directory(path: &Path) -> String {
    if !path.exists() {
        return "sha256:empty".to_string();
    }
    
    let mut files = BTreeMap::new();
    collect_files(path, path, &mut files);
    
    let mut hasher = Sha256::new();
    for (relative_path, full_path) in &files {
        hasher.update(relative_path.as_bytes());
        if let Ok(contents) = std::fs::read(full_path) {
            hasher.update(&contents);
        }
    }
    
    let result = hasher.finalize();
    format!("sha256:{:x}", result)
}

fn collect_files(base: &Path, current: &Path, files: &mut BTreeMap<String, std::path::PathBuf>) {
    if !current.is_dir() {
        return;
    }
    
    if let Ok(entries) = std::fs::read_dir(current) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(relative) = path.strip_prefix(base) {
                    files.insert(relative.to_string_lossy().to_string(), path);
                }
            } else if path.is_dir() {
                collect_files(base, &path, files);
            }
        }
    }
}

fn get_opencode_version() -> String {
    std::process::Command::new("opencode")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
