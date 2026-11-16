//! The `components` commands, which list component declarations and check their
//! compatibility.

use std::path::PathBuf;

use clap::{Args, Subcommand};
use reportify::Report;
use system_runtime_spec::Records;
use system_runtime_spec::types::common::{DiagnosticKind, Severity};
use system_runtime_spec::types::component::{
    CapabilityDeclaration, CapabilitySelectorDeclaration, CompatibilityProblem,
    CompatibilityReport, ComponentDeclaration,
};

use super::output::{print, print_diagnostics, print_json, table};
use super::{CliError, Options, Outcome, discover};

#[derive(Subcommand)]
pub enum Command {
    /// List the component declarations.
    List,
    /// Check the compatibility of the components; exits with 1 if they are inconsistent.
    Check(CheckArgs),
}

#[derive(Args)]
pub struct CheckArgs {
    /// Check the composition after an update with the records below this root directory.
    #[arg(long, value_name = "DIR", requires = "owners")]
    update: Option<PathBuf>,
    /// Owner whose component declarations the update replaces; can be repeated.
    #[arg(long = "owner", value_name = "OWNER", requires = "update")]
    owners: Vec<String>,
}

/// Run a `components` command on the records below the root directory.
#[tracing::instrument(level = "debug", skip_all, fields(root = %options.root.display()))]
pub fn run(command: Command, options: &Options) -> Result<Outcome, Report<CliError>> {
    let records = discover(&options.root)?;
    match command {
        Command::List => list(&records, options),
        Command::Check(args) => {
            let report = match &args.update {
                Some(update) => {
                    let owners: Vec<&str> = args.owners.iter().map(String::as_str).collect();
                    records
                        .with_update(&owners, &discover(update)?)
                        .check_compatibility()
                }
                None => records.check_compatibility(),
            };
            check(&report, options)
        }
    }
}

fn list(records: &Records, options: &Options) -> Result<Outcome, Report<CliError>> {
    print_diagnostics(records.diagnostics().iter().filter(|diagnostic| {
        matches!(
            diagnostic.kind,
            DiagnosticKind::ComponentDeclaration | DiagnosticKind::DirectoryConfig
        )
    }));
    let components: Vec<ComponentDeclaration> = records.components().cloned().collect();
    if options.json {
        print_json(&components)?;
    } else {
        let rows: Vec<[String; 5]> = components.iter().map(row).collect();
        print(&table(
            ["ID", "VERSION", "OWNER", "PROVIDES", "REQUIRES"],
            &rows,
        ))?;
    }
    Ok(Outcome::Positive)
}

fn check(report: &CompatibilityReport, options: &Options) -> Result<Outcome, Report<CliError>> {
    if options.json {
        print_json(report)?;
    } else {
        print_diagnostics(&report.diagnostics);
        let mut text = String::from(if report.consistent {
            "The components are consistent.\n"
        } else {
            "The components are inconsistent.\n"
        });
        for problem in &report.problems {
            text.push_str(&format!("- {}\n", describe(problem)));
        }
        if report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == Severity::Error)
        {
            text.push_str("- records were skipped because of errors\n");
        }
        print(&text)?;
    }
    Ok(if report.consistent {
        Outcome::Positive
    } else {
        Outcome::Negative
    })
}

/// Columns of a component in the list.
fn row(component: &ComponentDeclaration) -> [String; 5] {
    let list = |items: Vec<String>| {
        if items.is_empty() {
            "-".to_owned()
        } else {
            items.join(", ")
        }
    };
    [
        component.id.clone(),
        component.version.clone().unwrap_or_else(|| "-".to_owned()),
        component.owner.clone().unwrap_or_else(|| "-".to_owned()),
        list(
            component
                .provides
                .iter()
                .flatten()
                .map(capability)
                .collect(),
        ),
        list(component.requires.iter().flatten().map(selector).collect()),
    ]
}

fn describe(problem: &CompatibilityProblem) -> String {
    match problem {
        CompatibilityProblem::DuplicateComponent(problem) => {
            format!("more than one component has the ID {}", problem.id)
        }
        CompatibilityProblem::DuplicateClaim(problem) => format!(
            "more than one component claims {}: {}",
            problem.id,
            problem.components.join(", ")
        ),
        CompatibilityProblem::UnsatisfiedRequirement(problem) => format!(
            "{} requires {}, which no component provides",
            problem.component,
            selector(&problem.selector)
        ),
        CompatibilityProblem::Conflict(problem) => format!(
            "{} conflicts with {}, which {} provides as {}",
            problem.component,
            selector(&problem.selector),
            problem.provider,
            capability(&problem.capability)
        ),
    }
}

fn capability(capability: &CapabilityDeclaration) -> String {
    qualified(&capability.id, &capability.version, &capability.value)
}

fn selector(selector: &CapabilitySelectorDeclaration) -> String {
    qualified(&selector.id, &selector.version, &selector.value)
}

/// Format a capability or selector as `id version`, `id=value`, or `id`.
fn qualified(id: &str, version: &Option<String>, value: &Option<String>) -> String {
    match (version, value) {
        (Some(version), _) => format!("{id} {version}"),
        (None, Some(value)) => format!("{id}={value}"),
        (None, None) => id.to_owned(),
    }
}
