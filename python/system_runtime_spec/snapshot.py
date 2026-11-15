"""Select contributions by their ownership owner before evaluating a candidate."""

from .models.filesystem import RuntimeSnapshot


def without_owner(snapshot: RuntimeSnapshot, owner: str) -> RuntimeSnapshot:
    """Remove an exact owner from all collections, preserving other constraints."""
    return RuntimeSnapshot(
        values=None
        if snapshot.values is None
        else [r for r in snapshot.values if r.owner != owner],
        value_observations=None
        if snapshot.value_observations is None
        else [r for r in snapshot.value_observations if r.owner != owner],
        components=[record for record in snapshot.components if record.owner != owner],
        health_nodes=[
            record for record in snapshot.health_nodes if record.owner != owner
        ],
        health_statuses=[
            record for record in snapshot.health_statuses if record.owner != owner
        ],
    )
