//! System component compatibility, health graphs, and file discovery.
//!
//! Specification models live in [`types`]. Use [`compatibility::check_components`]
//! and [`evaluation::evaluate_health`] with in-memory declarations, or load file
//! contributions with [`discovery::RuntimeDirectories`].

pub mod clock;
pub mod compatibility;
pub mod discovery;
pub mod evaluation;
mod node_id;
pub mod snapshot;
mod threshold;
/// Specification models generated from the shared Sidex schemas.
pub mod types;

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
