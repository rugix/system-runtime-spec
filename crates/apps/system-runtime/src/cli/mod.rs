//! Command-line interface: argument parsing, logging, and exit codes.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use reportify::render::RenderOptions;
use reportify::{Report, ResultExt, bail};
use system_runtime_spec::Records;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;

mod components;
mod health;
mod output;

reportify::new_whatever_type! {
    /// Failure of a command.
    pub CliError
}

/// Version of the release that built the tool, or `dev` for other builds.
const VERSION: &str = match option_env!("SYSTEM_RUNTIME_VERSION") {
    Some(version) => version,
    None => "dev",
};

/// Exit code of failures, including invalid arguments.
const FAILURE: u8 = 3;

/// Result of a command, reported as its exit code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The command succeeded and its check passed.
    Positive = 0,
    /// The check failed: the components are inconsistent, or the node is not healthy.
    Negative = 1,
    /// The evidence does not determine whether the node is healthy.
    Unknown = 2,
}

/// Check the components and health of a system according to the System Runtime
/// Specification.
#[derive(Parser)]
#[command(name = "system-runtime", version = VERSION)]
struct Cli {
    #[command(flatten)]
    options: Options,
    #[command(subcommand)]
    command: Command,
}

/// Options of all commands.
#[derive(Args)]
pub struct Options {
    /// Root directory of the system whose records to read.
    #[arg(long, global = true, value_name = "DIR", default_value = "/")]
    pub root: PathBuf,
    /// Print the report that the specification's schemas define, as JSON.
    #[arg(long, global = true)]
    pub json: bool,
}

#[derive(Subcommand)]
enum Command {
    /// List the components and check their compatibility.
    #[command(subcommand)]
    Components(components::Command),
    /// Show the health of the system and check whether a node is healthy.
    #[command(subcommand)]
    Health(health::Command),
}

/// Run the command of the arguments and return its exit code.
pub fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_env_var("SYSTEM_RUNTIME_LOG")
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy(),
        )
        .with_writer(std::io::stderr)
        .init();
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            if let Err(print_error) = error.print() {
                tracing::error!("failed to print the usage: {print_error}");
                return ExitCode::from(FAILURE);
            }
            // Help and version requests are not errors.
            return ExitCode::from(if error.use_stderr() { FAILURE } else { 0 });
        }
    };
    let result = match cli.command {
        Command::Components(command) => components::run(command, &cli.options),
        Command::Health(command) => health::run(command, &cli.options),
    };
    match result {
        Ok(outcome) => ExitCode::from(outcome as u8),
        Err(report) => {
            let options = RenderOptions::new().compact().for_stderr();
            eprintln!("error: {}", report.render(options));
            ExitCode::from(FAILURE)
        }
    }
}

/// Read the records below `root`, which must be a directory.
///
/// Discovery treats missing record directories as empty, so a mistyped root would
/// otherwise read as a system without records.
fn discover(root: &Path) -> Result<Records, Report<CliError>> {
    let metadata = std::fs::metadata(root)
        .whatever_with(|_| format!("failed to read root directory {}", root.display()))?;
    if !metadata.is_dir() {
        bail!("root {} is not a directory", root.display());
    }
    Ok(Records::discover(root))
}
