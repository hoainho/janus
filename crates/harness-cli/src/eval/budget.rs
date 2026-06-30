use std::io::Write;

/// Budget manager for daily spend tracking
pub struct BudgetManager {
    daily_limit_usd: f64,
    ledger_path: std::path::PathBuf,
}

impl BudgetManager {
    pub fn new(ledger_path: std::path::PathBuf, daily_limit_usd: f64) -> Self {
        Self {
            daily_limit_usd,
            ledger_path,
        }
    }

    /// Check if we can spend the given amount based on today's ledger
    pub fn can_spend(&self, amount_usd: f64) -> bool {
        let today_total = self.today_spend();
        today_total + amount_usd <= self.daily_limit_usd
    }

    /// Get today's total spend from ledger
    fn today_spend(&self) -> f64 {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

        if !self.ledger_path.exists() {
            return 0.0;
        }

        let content = std::fs::read_to_string(&self.ledger_path).unwrap_or_default();
        content
            .lines()
            .filter_map(|line| {
                let entry: serde_json::Value = serde_json::from_str(line).ok()?;
                let ts = entry.get("timestamp")?.as_str()?;
                let cost = entry.get("cost_usd")?.as_f64()?;
                if ts.starts_with(&today) {
                    Some(cost)
                } else {
                    None
                }
            })
            .sum()
    }

    /// Record a spend in the ledger
    pub fn record_spend(&self, run_id: &str, cost_usd: f64, model: &str) -> Result<(), String> {
        let entry = serde_json::json!({
            "run_id": run_id,
            "cost_usd": cost_usd,
            "model": model,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        let line = format!("{}\n", entry);
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ledger_path)
            .map_err(|e| format!("Failed to open ledger: {}", e))?;
        file.write_all(line.as_bytes())
            .map_err(|e| format!("Failed to write ledger: {}", e))?;

        Ok(())
    }

    /// Get remaining budget for today
    pub fn remaining(&self) -> f64 {
        self.daily_limit_usd - self.today_spend()
    }
}
