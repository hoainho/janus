// eval module — Port of eval-harness (bash) to Rust
// Provides behavior-regression eval harness for opencode skills.

pub mod scoring;
pub mod attribution;
pub mod diff;
pub mod stability;
pub mod pricing;
pub mod config;
pub mod registry;
pub mod case;
pub mod manifest;
pub mod lock;
pub mod budget;
pub mod preflight;
pub mod spawn;
pub mod stats;
pub mod report;
pub mod storage;
pub mod context;
pub mod quality;

#[cfg(test)]
mod tests;

// Re-export main types
pub use scoring::{CheckKind, CheckResult, CaseResult};
pub use attribution::{AttributionClass, Attribution};
pub use pricing::PricingData;
pub use case::EvalCase;
pub use config::EvalConfig;
