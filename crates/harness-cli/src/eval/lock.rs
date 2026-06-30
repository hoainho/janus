use std::fs;

/// Lock manager for concurrent eval runs
pub struct LockManager {
    lock_dir: std::path::PathBuf,
}

impl LockManager {
    pub fn new(lock_dir: std::path::PathBuf) -> Self {
        Self { lock_dir }
    }

    /// Acquire a lock for a skill:case:trigger combination
    pub fn acquire(
        &self,
        skill: &str,
        case: &str,
        trigger: &str,
        timeout_secs: u64,
    ) -> Result<Lock, String> {
        let lock_key = format!("{}:{}:{}", skill, case, trigger);
        let lock_file = self.lock_dir.join(format!("{}.lock", lock_key));
        let mkdir_lock = self.lock_dir.join(format!("{}.lock.d", lock_key));

        fs::create_dir_all(&self.lock_dir)
            .map_err(|e| format!("Failed to create lock dir: {}", e))?;

        let start = std::time::Instant::now();
        loop {
            // Try mkdir-based lock (atomic on most filesystems)
            if fs::create_dir(&mkdir_lock).is_ok() {
                return Ok(Lock {
                    path: lock_file,
                    mkdir_path: Some(mkdir_lock),
                });
            }

            if start.elapsed().as_secs() >= timeout_secs {
                return Err(format!(
                    "Lock timeout after {} seconds for {}",
                    timeout_secs, lock_key
                ));
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

/// A held lock
pub struct Lock {
    path: std::path::PathBuf,
    mkdir_path: Option<std::path::PathBuf>,
}

impl Drop for Lock {
    fn drop(&mut self) {
        if let Some(mkdir_path) = &self.mkdir_path {
            let _ = fs::remove_dir(mkdir_path);
        }
        let _ = fs::remove_file(&self.path);
    }
}
