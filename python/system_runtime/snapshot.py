"""Select contributions by their ownership origin before evaluating a candidate."""

from .models.filesystem import RuntimeSnapshot


def without_origin(snapshot: RuntimeSnapshot, origin: str) -> RuntimeSnapshot:
    """Remove an exact origin from all collections, preserving other constraints."""
    return RuntimeSnapshot(
        values=None
        if snapshot.values is None
        else [r for r in snapshot.values if r.origin != origin],
        value_observations=None
        if snapshot.value_observations is None
        else [r for r in snapshot.value_observations if r.origin != origin],
        components=[
            record for record in snapshot.components if record.origin != origin
        ],
        health_nodes=[
            record for record in snapshot.health_nodes if record.origin != origin
        ],
        health_statuses=[
            record for record in snapshot.health_statuses if record.origin != origin
        ],
    )
