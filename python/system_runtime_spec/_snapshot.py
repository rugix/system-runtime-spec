from __future__ import annotations

import re
from collections.abc import Mapping
from dataclasses import dataclass

import pydantic

from ._decoding import (
    KINDS,
    SPEC_VERSION,
    SUPPORTED_MAJOR,
    Json,
    RecordModel,
    parse_spec_version,
    to_json,
)
from ._records import Records
from .models.common import Diagnostic
from .models.component import ComponentDeclaration
from .models.health import HealthClockSample, HealthNode, HealthStatus
from .models.metric import MetricDefinition, MetricSample

_TIMESTAMP = re.compile(
    r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:([0-9]{2})(\.[0-9]+)?Z"
)
_LISTS: tuple[tuple[str, type[RecordModel]], ...] = (
    ("components", ComponentDeclaration),
    ("healthNodes", HealthNode),
    ("healthStatuses", HealthStatus),
    ("metrics", MetricDefinition),
    ("metricSamples", MetricSample),
)
_HEALTH_FIELDS = ("healthNodes", "healthStatuses", "metrics", "metricSamples")


@dataclass(frozen=True)
class Parts:
    """Parts of the specification whose records a snapshot includes."""

    components: bool = True
    """Whether the snapshot includes component declarations."""
    health: bool = True
    """Whether the snapshot includes health nodes, status records, and metrics."""


class SnapshotError(ValueError):
    """A snapshot of an unsupported version or with an invalid structure."""


class Snapshot:
    """Records of a system with a clock sample, for sending them to another system.

    A snapshot contains the original records, so that the receiving system evaluates
    them as the sending system read them, including fields of newer minor versions.
    """

    def __init__(
        self,
        records: Records,
        clock: HealthClockSample,
        *,
        components: bool = True,
        health: bool = True,
    ) -> None:
        """Create a snapshot of the records of the given parts and their diagnostics.

        Take the clock sample after reading the records.
        """
        self._records = records._only(components=components, health=health)
        self._clock = clock
        self._parts = Parts(components, health)

    @property
    def records(self) -> Records:
        """Records of the snapshot; the lists of parts it does not include are empty."""
        return self._records

    @property
    def clock(self) -> HealthClockSample:
        """Correlation of the boot-time clock with wall-clock time."""
        return self._clock

    @property
    def parts(self) -> Parts:
        """Parts whose records the snapshot includes."""
        return self._parts

    def to_json(self) -> dict[str, Json]:
        """Encode the snapshot as a `RuntimeSnapshot`."""
        snapshot: dict[str, Json] = {
            "specVersion": SPEC_VERSION,
            "clock": to_json(self._clock),
        }
        for name, model in _LISTS:
            if self._parts.components if name == "components" else self._parts.health:
                snapshot[name] = self._records._original(KINDS[model])
        if self._records.diagnostics:
            snapshot["diagnostics"] = [to_json(d) for d in self._records.diagnostics]
        return snapshot

    @classmethod
    def from_json(cls, value: Mapping[str, Json]) -> Snapshot:
        """Decode a `RuntimeSnapshot`, applying the fallbacks to its diagnostics.

        Raise SnapshotError for snapshots of an unsupported version or with an invalid
        structure. Invalid records are skipped with an error diagnostic, as when they
        are read from files.
        """
        version = parse_spec_version(value.get("specVersion"))
        if version is None or version[0] != SUPPORTED_MAJOR:
            raise SnapshotError(
                f"unsupported snapshot version {value.get('specVersion')!r}"
            )
        for name in (*(name for name, _ in _LISTS), "diagnostics"):
            if name in value and not isinstance(value[name], list):
                raise SnapshotError(f"{name} is not a list")
        clock = value.get("clock")
        timestamp = clock.get("timestamp") if isinstance(clock, dict) else None
        match = _TIMESTAMP.fullmatch(timestamp) if isinstance(timestamp, str) else None
        if match is None or match[1] == "60":
            raise SnapshotError(f"invalid clock timestamp {timestamp!r}")
        try:
            sample = HealthClockSample.model_validate(clock)
        except pydantic.ValidationError as error:
            raise SnapshotError(f"invalid clock sample: {error}") from None
        present = [name in value for name in _HEALTH_FIELDS]
        health = all(present)
        if any(present) and not health:
            raise SnapshotError("snapshot includes only some lists of the health part")
        records = Records()
        for item in value.get("diagnostics", []):
            records.push_diagnostic(_read_diagnostic(item))
        for name, model in _LISTS:
            for record in value.get(name, []):
                records.push_json(model, record)
        # Keep all received diagnostics, also those about parts the snapshot omits.
        snapshot = cls(records, sample)
        snapshot._parts = Parts("components" in value, health)
        return snapshot

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Snapshot):
            return NotImplemented
        return (self._records, self._clock, self._parts) == (
            other._records,
            other._clock,
            other._parts,
        )

    def __repr__(self) -> str:
        return f"Snapshot({self._records!r}, {self._parts!r})"


def _read_diagnostic(item: Json) -> Diagnostic:
    """Decode a diagnostic, treating unknown kinds as `DirectoryConfig` and unknown
    severities as `Error`."""
    if not isinstance(item, dict):
        raise SnapshotError("diagnostic is not an object")
    item = dict(item)
    for name in ("kind", "severity"):
        if not isinstance(item.get(name), str):
            raise SnapshotError(f"diagnostic {name} is not a string")
    if item["kind"] not in KINDS.values():
        item["kind"] = "DirectoryConfig"
    if item["severity"] != "Warning":
        item["severity"] = "Error"
    try:
        return Diagnostic.model_validate(item)
    except pydantic.ValidationError as error:
        raise SnapshotError(f"invalid diagnostic: {error}") from None
