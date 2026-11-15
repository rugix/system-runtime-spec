//! Select contributions by their ownership owner before evaluating a candidate.

use crate::types::filesystem::RuntimeSnapshot;

/// Return a snapshot without records with this exact owner.
///
/// Untagged records and other owners are retained. References and requirements
/// are preserved so callers can check the resulting composition.
#[tracing::instrument(level = "debug", skip_all)]
pub fn without_owner(snapshot: &RuntimeSnapshot, owner: &str) -> RuntimeSnapshot {
    RuntimeSnapshot {
        values: snapshot.values.as_ref().map(|records| {
            records
                .iter()
                .filter(|r| r.owner.as_deref() != Some(owner))
                .cloned()
                .collect()
        }),
        value_observations: snapshot.value_observations.as_ref().map(|records| {
            records
                .iter()
                .filter(|r| r.owner.as_deref() != Some(owner))
                .cloned()
                .collect()
        }),
        components: snapshot
            .components
            .iter()
            .filter(|record| record.owner.as_deref() != Some(owner))
            .cloned()
            .collect(),
        health_nodes: snapshot
            .health_nodes
            .iter()
            .filter(|record| record.owner.as_deref() != Some(owner))
            .cloned()
            .collect(),
        health_statuses: snapshot
            .health_statuses
            .iter()
            .filter(|record| record.owner.as_deref() != Some(owner))
            .cloned()
            .collect(),
    }
}
