//! Compatibility checking, health evaluation, and discovery for the
//! [System Runtime Specification](https://github.com/rugix/system-runtime-spec/blob/main/SPEC.md).
//!
//! [`Records`] holds the records of a system: component declarations, health nodes, status
//! records, metric definitions, and metric samples, together with the diagnostics about them.
//! The models in [`types`] are generated from the specification's schemas.
//!
//! - [`Records::discover`] reads the records of a Linux system.
//! - [`Records::check_compatibility`] checks the composition, and [`Records::with_update`]
//!   builds the composition after an update.
//! - [`Records::evaluate_health`] evaluates health in a
//!   [`HealthContext`](types::health::HealthContext).
//! - [`Snapshot`] sends records to another system.
//!
//! ```no_run
//! use system_runtime_spec::Records;
//! use system_runtime_spec::types::health::HealthContext;
//!
//! let records = Records::discover("/");
//! let compatibility = records.check_compatibility();
//! let health = records.evaluate_health(&HealthContext::current()?);
//! println!("consistent: {}", compatibility.consistent);
//! for node in &health.nodes {
//!     println!("{}: usable {:?}, healthy {:?}", node.id, node.usable, node.healthy);
//! }
//! # Ok::<(), std::io::Error>(())
//! ```

mod clock;
mod compatibility;
#[cfg(target_os = "linux")]
mod discovery;
mod evaluation;
mod records;
mod snapshot;
/// Models generated from the specification's schemas.
pub mod types;

pub use clock::Evidence;
pub use records::{RecordModel, Records, SPEC_VERSION};
pub use snapshot::{Parts, Snapshot, SnapshotError};

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
