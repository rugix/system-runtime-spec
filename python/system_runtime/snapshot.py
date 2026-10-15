"""Select contributions by their ownership origin before evaluating a candidate."""

from .models.filesystem import RuntimeSnapshot


def without_origin(snapshot: RuntimeSnapshot, origin: str) -> RuntimeSnapshot:
    """Remove an exact origin from all collections, preserving other constraints."""
    return RuntimeSnapshot(
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
