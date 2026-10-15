//! Select contributions by their ownership origin before evaluating a candidate.

use crate::filesystem::RuntimeSnapshot;

/// Return a snapshot without records with this exact origin.
///
/// Untagged records and other origins are retained. References and requirements
/// are preserved so callers can check the resulting composition.
#[tracing::instrument(level = "debug", skip_all)]
pub fn without_origin(snapshot: &RuntimeSnapshot, origin: &str) -> RuntimeSnapshot {
    RuntimeSnapshot {
        components: snapshot
            .components
            .iter()
            .filter(|record| record.origin.as_deref() != Some(origin))
            .cloned()
            .collect(),
        health_nodes: snapshot
            .health_nodes
            .iter()
            .filter(|record| record.origin.as_deref() != Some(origin))
            .cloned()
            .collect(),
        health_statuses: snapshot
            .health_statuses
            .iter()
            .filter(|record| record.origin.as_deref() != Some(origin))
            .cloned()
            .collect(),
    }
}
