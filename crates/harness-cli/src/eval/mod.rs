// eval module — Port of eval-harness (bash) to Rust
// Provides behavior-regression eval harness for opencode skills.
//
// The eval harness exposes a complete API surface; not every type/function is
// wired into the CLI yet, so allow the corresponding lints at the module root.
#![allow(dead_code)]
#![allow(clippy::too_many_arguments, clippy::module_inception)]

pub mod attribution;
pub mod budget;
pub mod case;
pub mod config;
pub mod context;
pub mod diff;
pub mod lock;
pub mod manifest;
pub mod preflight;
pub mod pricing;
pub mod quality;
pub mod registry;
pub mod report;
pub mod scoring;
pub mod spawn;
pub mod stability;
pub mod stats;
pub mod storage;

#[cfg(test)]
mod tests;

// Re-export main types
