//! Compatibility checking, health evaluation, and discovery for the
//! [System Runtime Specification](https://github.com/rugix/system-runtime-spec/blob/main/SPEC.md).
//!
//! Records are JSON values ([`JsonValue`](sidex::types::JsonValue)), so that snapshots carry
//! them unchanged and records of newer minor versions keep working. The typed models in
//! [`types`] describe their structure; [`records::to_json`] converts them.
//!
//! - [`compatibility::check_components`] checks a composition, and
//!   [`compatibility::candidate_composition`] predicts the composition after an update.
//! - [`evaluation::evaluate_health`] evaluates health in a health context.
//! - [`discovery::read_records`] reads the records of a Linux system.
//! - [`snapshot::make_snapshot`] and [`snapshot::read_snapshot`] send records to another
//!   system, and [`clock::estimate_timestamp`] places evidence in wall-clock time.

pub mod clock;
pub mod compatibility;
#[cfg(target_os = "linux")]
pub mod discovery;
pub mod evaluation;
pub mod records;
pub mod snapshot;
/// Models generated from the Sidex schemas.
pub mod types;

/// Failure to read the system's clock or a snapshot.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A snapshot is not valid.
    #[error("invalid snapshot: {0}")]
    Invalid(String),
    /// A file could not be read.
    #[error("failed to read {path}: {source}")]
    Io {
        /// Path of the file.
        path: std::path::PathBuf,
        /// Underlying error.
        #[source]
        source: std::io::Error,
    },
}
