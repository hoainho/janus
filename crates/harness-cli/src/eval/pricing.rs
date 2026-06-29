use std::path::Path;
use serde::{Deserialize, Serialize};

/// Pricing data from pricing.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingData {
    pub schema_version: u32,
    pub as_of: String,
    pub stale_after_days: u32,
    pub source: String,
    pub models: std::collections::HashMap<String, ModelPricing>,
}

/// Per-model pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    pub input_per_mtok_usd: f64,
    pub output_per_mtok_usd: f64,
}

/// Pricing staleness status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingStaleness {
    pub status: String, // FRESH, STALE, MISSING, INVALID
    pub message: Option<String>,
}

/// Load pricing data from file
pub fn load_pricing(path: &Path) -> Result<PricingData, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read pricing.json: {}", e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse pricing.json: {}", e))
}

/// Check if pricing data is stale
pub fn check_staleness(pricing: &PricingData) -> PricingStaleness {
    let as_of = chrono::NaiveDate::parse_from_str(&pricing.as_of, "%Y-%m-%d");
    match as_of {
        Ok(date) => {
            let now = chrono::Utc::now().date_naive();
            let days_old = (now - date).num_days();
            if days_old > pricing.stale_after_days as i64 {
                PricingStaleness {
                    status: "STALE".to_string(),
                    message: Some(format!(
                        "Pricing data is {} days old (stale after {} days)",
                        days_old, pricing.stale_after_days
                    )),
                }
            } else {
                PricingStaleness {
                    status: "FRESH".to_string(),
                    message: None,
                }
            }
        }
        Err(_) => PricingStaleness {
            status: "INVALID".to_string(),
            message: Some("Invalid date format in pricing.json".to_string()),
        },
    }
}

/// Calculate cost for a model given token counts
pub fn calculate_cost(
    pricing: &PricingData,
    model: &str,
    input_tokens: u64,
    output_tokens: u64,
) -> Option<f64> {
    pricing.models.get(model).map(|m| {
        let input_cost = (input_tokens as f64 / 1_000_000.0) * m.input_per_mtok_usd;
        let output_cost = (output_tokens as f64 / 1_000_000.0) * m.output_per_mtok_usd;
        input_cost + output_cost
    })
}
