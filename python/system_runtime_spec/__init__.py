"""Compatibility checking, health evaluation, and discovery of runtime records."""

from ._records import SPEC_VERSION, Records
from .clock import current_clock_sample, estimate_timestamp
from .components import candidate_composition, check_components
from .filesystem import read_records
from .health import current_context, evaluate_health
from .models import common, component, filesystem, health, metric
from .snapshot import Snapshot, make_snapshot, read_snapshot

__all__ = [
    "SPEC_VERSION",
    "Records",
    "Snapshot",
    "candidate_composition",
    "check_components",
    "common",
    "component",
    "current_clock_sample",
    "current_context",
    "estimate_timestamp",
    "evaluate_health",
    "filesystem",
    "health",
    "make_snapshot",
    "metric",
    "read_records",
    "read_snapshot",
]
