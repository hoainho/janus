use serde::{Deserialize, Serialize};

/// Stability check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityResult {
    pub samples: usize,
    pub byte_identical: bool,
    pub hashes: Vec<String>,
    pub performed: bool,
}

/// Perform stability check by running case multiple times
pub fn check_stability(
    _case_id: &str,
    workdir: &std::path::Path,
    _prompt: &str,
    _skills_loaded: &[String],
    samples: usize,
) -> StabilityResult {
    if samples <= 1 {
        return StabilityResult {
            samples: 1,
            byte_identical: true,
            hashes: Vec::new(),
            performed: false,
        };
    }

    let mut hashes = Vec::new();

    for i in 0..samples {
        // Create sample directory
        let sample_dir = workdir.join(format!("stability/sample-{}", i + 1));
        std::fs::create_dir_all(&sample_dir).unwrap_or_default();

        // Run case
        let _transcript = sample_dir.join("transcript.jsonl");
        // TODO: Actually run the case and hash results
        let hash = format!("hash_{}", i);
        hashes.push(hash);
    }

    // Check if all hashes are identical
    let first = &hashes[0];
    let byte_identical = hashes.iter().all(|h| h == first);

    StabilityResult {
        samples,
        byte_identical,
        hashes,
        performed: true,
    }
}

/// Hash checks.json for comparison
pub fn hash_checks(checks_path: &std::path::Path) -> String {
    // TODO: Implement proper hashing
    // For now, return a placeholder
    format!("hash_{}", checks_path.display())
}
