use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{Args, Parser, Subcommand};
use thiserror::Error;

use crate::application::{HarnessContext, HarnessService, InitResult, IntakeInput};
use crate::domain::{CsvList, HarnessStats, InputType, IntakeRecord, RiskLane};

#[derive(Parser, Debug)]
#[command(name = "harness")]
#[command(about = "durable layer for the project harness", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create the harness database if it does not already exist.
    Init,
    /// Record a feature intake classification.
    Intake(IntakeArgs),
    /// Query harness data.
    Query(QueryArgs),
}

#[derive(Args, Debug)]
struct IntakeArgs {
    #[arg(long = "type")]
    input_type: String,
    #[arg(long)]
    summary: String,
    #[arg(long)]
    lane: String,
    #[arg(long)]
    flags: Option<String>,
    #[arg(long)]
    docs: Option<String>,
    #[arg(long)]
    story: Option<String>,
    #[arg(long)]
    notes: Option<String>,
}

#[derive(Args, Debug)]
struct QueryArgs {
    #[command(subcommand)]
    view: QueryView,
}

#[derive(Subcommand, Debug)]
enum QueryView {
    /// Recent intake classifications.
    Intakes,
    /// Summary counts.
    Stats,
}

#[derive(Debug, Error)]
pub enum InterfaceError {
    #[error("{0}")]
    ParseHarnessValue(#[from] crate::domain::ParseHarnessValueError),
    #[error("{0}")]
    Infrastructure(#[from] crate::infrastructure::HarnessInfraError),
    #[error("could not determine current directory: {0}")]
    CurrentDir(std::io::Error),
}

pub fn run(cli: Cli) -> Result<(), InterfaceError> {
    let service = HarnessService::new(resolve_context()?);

    match cli.command {
        Command::Init => match service.init()? {
            InitResult::Created { db_path } => {
                println!("Creating harness database at {}", db_path.display());
                println!("Schema version 1 applied.");
            }
            InitResult::Existing { db_path, version } => {
                println!("Database already exists at {}", db_path.display());
                println!("Current schema version: {version}");
            }
            InitResult::MigratedExisting { db_path } => {
                println!("Database already exists at {}", db_path.display());
                println!("No schema version found. Applying schema version 1.");
                println!("Schema version 1 applied.");
            }
        },
        Command::Intake(args) => {
            let id = service.record_intake(IntakeInput {
                input_type: InputType::from_str(&args.input_type)?,
                summary: args.summary,
                risk_lane: RiskLane::from_str(&args.lane)?,
                risk_flags: CsvList::from_optional(args.flags),
                affected_docs: CsvList::from_optional(args.docs),
                story_id: args.story,
                notes: args.notes,
            })?;
            println!("Intake #{id} recorded.");
        }
        Command::Query(args) => match args.view {
            QueryView::Intakes => print_intakes(&service.query_intakes()?),
            QueryView::Stats => print_stats(&service.query_stats()?),
        },
    }

    Ok(())
}

fn resolve_context() -> Result<HarnessContext, InterfaceError> {
    let repo_root = match env::var_os("HARNESS_REPO_ROOT") {
        Some(path) => PathBuf::from(path),
        None => env::current_dir().map_err(InterfaceError::CurrentDir)?,
    };
    let db_path = env::var_os("HARNESS_DB")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root.join("harness.db"));

    Ok(HarnessContext {
        db_path,
        schema_dir: repo_root.join("scripts/schema"),
    })
}

fn print_intakes(records: &[IntakeRecord]) {
    let headers = ["id", "created_at", "input_type", "risk_lane", "summary"];
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.created_at.clone(),
                record.input_type.clone(),
                record.risk_lane.clone(),
                record.summary.clone(),
            ]
        })
        .collect::<Vec<_>>();

    print_table(&headers, &rows);
}

fn print_stats(stats: &HarnessStats) {
    println!("=== Harness Stats ===");
    print_table(
        &["intakes", "stories", "decisions", "backlog_items", "traces"],
        &[vec![
            stats.intakes.to_string(),
            stats.stories.to_string(),
            stats.decisions.to_string(),
            stats.backlog_items.to_string(),
            stats.traces.to_string(),
        ]],
    );
}

fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    let widths = headers
        .iter()
        .enumerate()
        .map(|(index, header)| {
            rows.iter()
                .filter_map(|row| row.get(index))
                .map(String::len)
                .chain(std::iter::once(header.len()))
                .max()
                .unwrap_or(header.len())
        })
        .collect::<Vec<_>>();

    print_row(
        &headers
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>(),
        &widths,
    );
    print_row(
        &widths
            .iter()
            .map(|width| "-".repeat(*width))
            .collect::<Vec<_>>(),
        &widths,
    );
    for row in rows {
        print_row(row, &widths);
    }
}

fn print_row(values: &[String], widths: &[usize]) {
    for (index, width) in widths.iter().enumerate() {
        if index > 0 {
            print!("  ");
        }
        let value = values.get(index).map(String::as_str).unwrap_or("");
        print!("{value:<width$}");
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }
}
