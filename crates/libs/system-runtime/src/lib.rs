//! System component compatibility, health graphs, and file discovery.
//!
//! Generated types live in [`component`], [`health`], [`value`], and [`filesystem`].
//! Use [`check_components`] and [`evaluate_health`] with in-memory declarations,
//! or load file contributions with [`RuntimeDirectories`].

mod clock;
mod compatibility;
mod discovery;
mod evaluation;
pub mod generated;
mod node_id;
mod snapshot;
mod threshold;

#[cfg(target_os = "linux")]
pub use clock::current_clock_sample;
pub use clock::status_timestamp;
pub use compatibility::check_components;
pub use discovery::RuntimeDirectories;
#[cfg(target_os = "linux")]
pub use evaluation::current_context;
pub use evaluation::{evaluate_health, evaluate_health_with_values};
pub use generated::{component, filesystem, health, value};
pub use snapshot::without_origin;

/// Failure to validate a specification or read a file contribution.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A declaration cannot be evaluated.
    #[error("invalid specification: {0}")]
    Invalid(String),
    /// A directory or contribution could not be read.
    #[error("failed to read {path}: {source}")]
    Io {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    /// A contribution has invalid syntax or fields.
    #[error("failed to parse {path}: {source}")]
    Decode {
        path: std::path::PathBuf,
        #[source]
        source: sidex::DecodeError,
    },
}
