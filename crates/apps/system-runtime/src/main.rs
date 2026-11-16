//! Command-line tool for the [System Runtime Specification](https://github.com/rugix/system-runtime-spec).
//!
//! The tool reads the records of a Linux system below a root directory, `/` by default:
//!
//! - `system-runtime components` lists the component declarations and checks their
//!   compatibility, also for the composition that an update would produce.
//! - `system-runtime health` shows the evaluated health of the system and answers whether a
//!   node is healthy or usable.
//!
//! With `--json`, the commands print the reports that the specification's schemas define.
//! The exit code reports the result of a check, see `cli::Outcome`.

use std::process::ExitCode;

// Discovery and the health context read Linux interfaces.
#[cfg(target_os = "linux")]
mod cli;

#[cfg(target_os = "linux")]
fn main() -> ExitCode {
    cli::main()
}

#[cfg(not(target_os = "linux"))]
fn main() -> ExitCode {
    eprintln!("system-runtime reads the records of Linux systems and only runs on Linux");
    ExitCode::FAILURE
}
