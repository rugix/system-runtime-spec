"""Select contributions by their ownership owner before evaluating a candidate."""

from .models.filesystem import RuntimeSnapshot


def without_owner(snapshot: RuntimeSnapshot, owner: str) -> RuntimeSnapshot:
    """Remove an exact owner from all collections, preserving other constraints."""
    return RuntimeSnapshot(
        metrics=None
        if snapshot.metrics is None
        else [r for r in snapshot.metrics if r.owner != owner],
        metric_samples=None
        if snapshot.metric_samples is None
        else [r for r in snapshot.metric_samples if r.owner != owner],
        components=[record for record in snapshot.components if record.owner != owner],
        health_nodes=[
            record for record in snapshot.health_nodes if record.owner != owner
        ],
        health_statuses=[
            record for record in snapshot.health_statuses if record.owner != owner
        ],
    )
