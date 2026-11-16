//! The `health` commands, which show the evaluated health of a system and check whether a
//! node is healthy or usable.

use std::collections::{BTreeMap, BTreeSet};

use clap::Subcommand;
use reportify::{Report, ResultExt, bail};
use system_runtime_spec::types::health::{
    Answer, HealthContext, HealthNodeResult, HealthProblem, HealthReport,
};

use super::output::{print, print_diagnostics, print_json, table};
use super::{CliError, Options, Outcome, discover};

#[derive(Subcommand)]
pub enum Command {
    /// Show the health of all nodes, or of one node and its inputs.
    Show {
        /// ID of the node to show with its inputs.
        node: Option<String>,
    },
    /// Check whether a node is healthy; exits with 0 for yes, 1 for no, and 2 for unknown.
    Check {
        /// ID of the node to check.
        node: String,
        /// Check whether the node is usable instead.
        #[arg(long)]
        usable: bool,
    },
}

/// Run a `health` command on the records below the root directory.
#[tracing::instrument(level = "debug", skip_all, fields(root = %options.root.display()))]
pub fn run(command: Command, options: &Options) -> Result<Outcome, Report<CliError>> {
    let context =
        HealthContext::current().whatever("failed to read the boot ID and boot-time clock")?;
    let report = discover(&options.root)?.evaluate_health(&context);
    match command {
        Command::Show { node } => show(report, node.as_deref(), options),
        Command::Check { node, usable } => check(&report, &node, usable, options),
    }
}

fn show(
    mut report: HealthReport,
    node: Option<&str>,
    options: &Options,
) -> Result<Outcome, Report<CliError>> {
    let tree = Tree::new(&report);
    let roots = match node {
        Some(node) => vec![tree.find(node)?],
        None => tree.roots(),
    };
    if options.json {
        let shown: BTreeSet<String> = tree.reached(&roots).into_iter().map(String::from).collect();
        report
            .results
            .retain(|result| shown.contains(AsRef::<str>::as_ref(&result.id)));
        print_json(&report)?;
    } else {
        print_diagnostics(&report.diagnostics);
        let rows: Vec<[String; 3]> = tree
            .lines(&roots)
            .iter()
            .map(|line| {
                [
                    answer(line.result.usable).to_owned(),
                    answer(line.result.healthy).to_owned(),
                    format!("{}{}", line.prefix, node_details(line.result)),
                ]
            })
            .collect();
        print(&table(["USABLE", "HEALTHY", "NODE"], &rows))?;
    }
    Ok(Outcome::Positive)
}

fn check(
    report: &HealthReport,
    node: &str,
    usable: bool,
    options: &Options,
) -> Result<Outcome, Report<CliError>> {
    let result = Tree::new(report).find(node)?;
    let value = if usable {
        result.usable
    } else {
        result.healthy
    };
    print_diagnostics(&report.diagnostics);
    if options.json {
        print_json(result)?;
    } else {
        print(&format!("{}\n", answer(value)))?;
    }
    Ok(match value {
        Answer::Yes => Outcome::Positive,
        Answer::No => Outcome::Negative,
        Answer::Unknown => Outcome::Unknown,
    })
}

/// Results of a health report, linked by the inputs of aggregate nodes.
struct Tree<'r> {
    results: Vec<&'r HealthNodeResult>,
    by_id: BTreeMap<&'r str, &'r HealthNodeResult>,
}

/// One line of the rendered tree.
struct Line<'r> {
    prefix: String,
    result: &'r HealthNodeResult,
}

impl<'r> Tree<'r> {
    fn new(report: &'r HealthReport) -> Self {
        Self {
            results: report.results.iter().collect(),
            by_id: report
                .results
                .iter()
                .map(|result| (result.id.as_ref(), result))
                .collect(),
        }
    }

    fn find(&self, node: &str) -> Result<&'r HealthNodeResult, Report<CliError>> {
        match self.by_id.get(node) {
            Some(result) => Ok(result),
            None => bail!("health node {node:?} does not exist"),
        }
    }

    /// Nodes that are not inputs of other nodes, followed by those only reachable on cycles.
    fn roots(&self) -> Vec<&'r HealthNodeResult> {
        let inputs: BTreeSet<&str> = self
            .results
            .iter()
            .flat_map(|result| result.inputs.iter().map(AsRef::as_ref))
            .collect();
        let mut roots: Vec<_> = self
            .results
            .iter()
            .copied()
            .filter(|result| !inputs.contains(result.id.as_ref()))
            .collect();
        let mut reached = self.reached(&roots);
        for result in &self.results {
            if !reached.contains(result.id.as_ref()) {
                roots.push(result);
                reached.extend(self.reached(&[result]));
            }
        }
        roots
    }

    /// IDs of the nodes in the trees below `roots`.
    fn reached(&self, roots: &[&'r HealthNodeResult]) -> BTreeSet<&'r str> {
        self.lines(roots)
            .iter()
            .map(|line| line.result.id.as_ref())
            .collect()
    }

    /// Lines of the trees below `roots`, without the inputs of a node that repeats on its
    /// path.
    fn lines(&self, roots: &[&'r HealthNodeResult]) -> Vec<Line<'r>> {
        let mut lines = Vec::new();
        for root in roots {
            self.push_lines(
                &mut lines,
                root,
                String::new(),
                String::new(),
                &mut Vec::new(),
            );
        }
        lines
    }

    fn push_lines(
        &self,
        lines: &mut Vec<Line<'r>>,
        result: &'r HealthNodeResult,
        prefix: String,
        indent: String,
        ancestors: &mut Vec<&'r str>,
    ) {
        lines.push(Line { prefix, result });
        if ancestors.contains(&result.id.as_ref()) {
            return;
        }
        ancestors.push(result.id.as_ref());
        let inputs: Vec<_> = result
            .inputs
            .iter()
            .filter_map(|input| self.by_id.get(input.as_ref()).copied())
            .collect();
        for (index, input) in inputs.iter().enumerate() {
            let (branch, continuation) = if index + 1 == inputs.len() {
                ("└── ", "    ")
            } else {
                ("├── ", "│   ")
            };
            self.push_lines(
                lines,
                input,
                format!("{indent}{branch}"),
                format!("{indent}{continuation}"),
                ancestors,
            );
        }
        ancestors.pop();
    }
}

/// Node ID, followed by its problem and the first line of its message.
fn node_details(result: &HealthNodeResult) -> String {
    let mut details = result.id.to_string();
    if let Some(problem) = result.problem {
        details.push_str(&format!(" ({})", problem_name(problem)));
    }
    if let Some(message) = result.message.as_deref().and_then(|m| m.lines().next()) {
        details.push_str(&format!(": {message}"));
    }
    details
}

fn answer(answer: Answer) -> &'static str {
    match answer {
        Answer::Yes => "yes",
        Answer::No => "no",
        Answer::Unknown => "unknown",
    }
}

fn problem_name(problem: HealthProblem) -> &'static str {
    match problem {
        HealthProblem::Missing => "missing",
        HealthProblem::OtherBoot => "other boot",
        HealthProblem::OwnerMismatch => "owner mismatch",
        HealthProblem::TagMismatch => "tag mismatch",
        HealthProblem::Future => "future",
        HealthProblem::Expired => "expired",
        HealthProblem::Conflict => "conflict",
        HealthProblem::Cycle => "cycle",
        HealthProblem::Unsupported => "unsupported",
    }
}
