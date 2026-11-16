//! Printing of reports as text or JSON, and of diagnostics.

use std::io::Write;

use reportify::{Report, ResultExt};
use system_runtime_spec::types::common::{Diagnostic, Severity};

use super::CliError;

/// Print `text` to standard output.
pub fn print(text: &str) -> Result<(), Report<CliError>> {
    std::io::stdout()
        .lock()
        .write_all(text.as_bytes())
        .whatever("failed to write to standard output")
}

/// Print `value` as pretty JSON to standard output.
pub fn print_json<T: sidex::Encode + ?Sized>(value: &T) -> Result<(), Report<CliError>> {
    let json = sidex::formats::json::to_string_pretty(value)
        .expect("models of the specification encode as JSON");
    print(&format!("{json}\n"))
}

/// Print diagnostics to standard error, one per line.
pub fn print_diagnostics<'d>(diagnostics: impl IntoIterator<Item = &'d Diagnostic>) {
    for diagnostic in diagnostics {
        let severity = match diagnostic.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        match &diagnostic.path {
            Some(path) => eprintln!("{severity}: {path}: {}", diagnostic.message),
            None => eprintln!("{severity}: {}", diagnostic.message),
        }
    }
}

/// Format rows as a table with a header and columns separated by two spaces.
pub fn table<const N: usize>(header: [&str; N], rows: &[[String; N]]) -> String {
    let mut widths = header.map(str::len);
    for row in rows {
        for (width, cell) in widths.iter_mut().zip(row) {
            *width = (*width).max(cell.chars().count());
        }
    }
    let mut text = String::new();
    for row in std::iter::once(header.map(String::from)).chain(rows.iter().cloned()) {
        let line = row
            .iter()
            .zip(widths)
            .map(|(cell, width)| format!("{cell:width$}"))
            .collect::<Vec<_>>()
            .join("  ");
        text.push_str(line.trim_end());
        text.push('\n');
    }
    text
}
