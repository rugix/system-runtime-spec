"""Send records to another system as snapshots."""

import re
from collections.abc import Mapping
from dataclasses import dataclass

from ._records import (
    KINDS,
    SPEC_VERSION,
    SUPPORTED_MAJOR,
    Json,
    JsonObject,
    Records,
    parse_spec_version,
    to_json,
)
from .models.common import Diagnostic
from .models.health import HealthClockSample

_TIMESTAMP = re.compile(
    r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:([0-9]{2})(\.[0-9]+)?Z"
)
_HEALTH = ("healthNodes", "healthStatuses", "metrics", "metricSamples")


@dataclass
class Snapshot:
    """A received snapshot.

    Omitted lists are empty in `records`; `components` and `health` state whether the
    snapshot includes the records of the components part and of the health part.
    """

    records: Records
    clock: HealthClockSample
    components: bool
    health: bool


def make_snapshot(
    records: Records,
    clock: HealthClockSample | Json,
    *,
    components: bool = True,
    health: bool = True,
) -> JsonObject:
    """Create a snapshot with the original records and their diagnostics.

    Take the clock sample after reading the records.
    """
    snapshot: JsonObject = {"specVersion": SPEC_VERSION, "clock": to_json(clock)}
    if components:
        snapshot["components"] = list(records.components)
    if health:
        snapshot["healthNodes"] = list(records.health_nodes)
        snapshot["healthStatuses"] = list(records.health_statuses)
        snapshot["metrics"] = list(records.metrics)
        snapshot["metricSamples"] = list(records.metric_samples)
    included = {"DirectoryConfig"}
    if components:
        included.add("ComponentDeclaration")
    if health:
        included.update(
            ("HealthNode", "HealthStatus", "MetricDefinition", "MetricSample")
        )
    diagnostics = [
        to_json(item)
        for item in records.diagnostics
        if to_json(item)["kind"] in included
    ]
    if diagnostics:
        snapshot["diagnostics"] = diagnostics
    return snapshot


def read_snapshot(value: Mapping[str, Json]) -> Snapshot:
    """Read a snapshot, applying the fallbacks to its diagnostics.

    Raise ValueError for snapshots of an unsupported version or an invalid structure.
    """
    version = parse_spec_version(value.get("specVersion"))
    if version is None or version[0] != SUPPORTED_MAJOR:
        raise ValueError(f"unsupported snapshot version {value.get('specVersion')!r}")
    timestamp = value.get("clock", {}).get("timestamp")
    match = _TIMESTAMP.fullmatch(timestamp) if isinstance(timestamp, str) else None
    if match is None or match[1] == "60":
        raise ValueError(f"invalid clock timestamp {timestamp!r}")
    clock = HealthClockSample.model_validate(value["clock"])
    present = [name in value for name in _HEALTH]
    if any(present) and not all(present):
        raise ValueError("snapshot includes only some lists of the health part")
    diagnostics = []
    for item in value.get("diagnostics") or []:
        item = dict(item)
        if item.get("kind") not in KINDS:
            item["kind"] = "DirectoryConfig"
        if item.get("severity") not in ("Error", "Warning"):
            item["severity"] = "Error"
        diagnostics.append(Diagnostic.model_validate(item))
    records = Records(
        components=list(value.get("components") or []),
        health_nodes=list(value.get("healthNodes") or []),
        health_statuses=list(value.get("healthStatuses") or []),
        metrics=list(value.get("metrics") or []),
        metric_samples=list(value.get("metricSamples") or []),
        diagnostics=diagnostics,
    )
    return Snapshot(records, clock, "components" in value, all(present))
