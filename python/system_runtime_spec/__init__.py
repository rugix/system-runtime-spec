"""Compatibility checking, health evaluation, and discovery of runtime records.

`Records` holds the records of a system and the diagnostics about them. The models in
`system_runtime_spec.models` are generated from the specification's schemas.
"""

from ._clock import current_clock_sample, current_context, estimate_timestamp
from ._decoding import SPEC_VERSION, RecordModel
from ._records import Records
from ._snapshot import Parts, Snapshot, SnapshotError

__all__ = [
    "SPEC_VERSION",
    "Parts",
    "RecordModel",
    "Records",
    "Snapshot",
    "SnapshotError",
    "current_clock_sample",
    "current_context",
    "estimate_timestamp",
]
