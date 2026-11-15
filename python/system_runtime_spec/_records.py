"""The records of a system and the diagnostics about them."""

from __future__ import annotations

import os
from collections.abc import Iterable
from typing import Any, cast

import pydantic

from . import _compatibility, _evaluation
from ._decoding import (
    KINDS,
    InvalidRecord,
    Json,
    Record,
    RecordModel,
    decode,
    diagnostic,
    to_json,
)
from .models.common import Diagnostic, RecordKind
from .models.component import CompatibilityReport, ComponentDeclaration
from .models.health import HealthContext, HealthNode, HealthReport, HealthStatus
from .models.metric import MetricDefinition, MetricSample

_HEALTH_KINDS: tuple[RecordKind, ...] = (
    "HealthNode",
    "HealthStatus",
    "MetricDefinition",
    "MetricSample",
)


class Records:
    """The records of a system and the diagnostics about them.

    Records are added as models or as JSON values. Adding a record decodes and validates
    it, and an invalid record is skipped with an error diagnostic. Records keep their
    original JSON, so that snapshots contain them unchanged, including fields of newer
    minor versions.
    """

    def __init__(self) -> None:
        self._records: dict[RecordKind, list[Record[Any]]] = {
            kind: [] for kind in KINDS.values() if kind != "DirectoryConfig"
        }
        self._diagnostics: list[Diagnostic] = []

    @classmethod
    def discover(cls, root: str | os.PathLike[str] = "/") -> Records:
        """Read the records of a Linux system whose root directory is `root`.

        Reads the standard directories and the directories that directory
        configurations add, and applies overrides. Files that cannot be read or do not
        contain a valid record are skipped with an error diagnostic.
        """
        from ._discovery import discover

        records = cls()
        discover(root, records)
        return records

    def push(self, record: RecordModel) -> None:
        """Add a record given as a model."""
        self.push_json(type(record), to_json(record))

    def push_json(self, kind: type[RecordModel], value: Json) -> None:
        """Add a record given as JSON, such as a record received from another system.

        `kind` is the model of the record, such as `HealthStatus`.
        """
        record = self._accept(kind, value)
        if record is not None:
            self._add(record)

    def push_diagnostic(self, diagnostic: Diagnostic) -> None:
        """Add a diagnostic, such as one that another reader reported."""
        self._diagnostics.append(diagnostic)

    @property
    def components(self) -> tuple[ComponentDeclaration, ...]:
        """Component declarations, in the order in which they were added."""
        return self._values("ComponentDeclaration")

    @property
    def health_nodes(self) -> tuple[HealthNode, ...]:
        """Health nodes, in the order in which they were added.

        A node with an unknown rule type has no rule, and unknown input modes are
        `Counted`; evaluation reports such nodes as unsupported.
        """
        return self._values("HealthNode")

    @property
    def health_statuses(self) -> tuple[HealthStatus, ...]:
        """Status records, in the order in which they were added.

        Unknown states are `Unknown`; evaluation reports such records as unsupported.
        """
        return self._values("HealthStatus")

    @property
    def metrics(self) -> tuple[MetricDefinition, ...]:
        """Metric definitions, in the order in which they were added."""
        return self._values("MetricDefinition")

    @property
    def metric_samples(self) -> tuple[MetricSample, ...]:
        """Metric samples, in the order in which they were added."""
        return self._values("MetricSample")

    @property
    def diagnostics(self) -> tuple[Diagnostic, ...]:
        """Diagnostics about the records and about directory configurations."""
        return tuple(self._diagnostics)

    def check_compatibility(self) -> CompatibilityReport:
        """Check the composition of the component declarations.

        The report contains the diagnostics about component declarations and directory
        configurations. The composition is consistent if there are no problems and none
        of these diagnostics has the severity `Error`.
        """
        return _compatibility.check(
            self.components,
            self._diagnostics_about("ComponentDeclaration", "DirectoryConfig"),
        )

    def evaluate_health(self, context: HealthContext) -> HealthReport:
        """Evaluate the health records in a health context.

        The report contains one result per node ID, in the order of the node
        definitions, and the diagnostics about health records and directory
        configurations, including those that evaluation adds.
        """
        return _evaluation.evaluate(
            self._records["HealthNode"],
            self._records["HealthStatus"],
            self._records["MetricDefinition"],
            self._records["MetricSample"],
            self._diagnostics_about(*_HEALTH_KINDS, "DirectoryConfig"),
            context,
        )

    def with_update(self, owners: Iterable[str], update: Records) -> Records:
        """Build the candidate composition of an update.

        The update replaces all component declarations of `owners` with the component
        declarations of `update`. A new declaration without a replaced owner is skipped
        with an error diagnostic. The candidate has the current records and
        diagnostics, and the diagnostics of `update` about component declarations.
        """
        replaced = set(owners)
        candidate = self._copy()
        components = candidate._records["ComponentDeclaration"]
        components[:] = [r for r in components if r.value.owner not in replaced]
        candidate._diagnostics.extend(update._diagnostics_about("ComponentDeclaration"))
        for record in update._records["ComponentDeclaration"]:
            if record.value.owner in replaced:
                components.append(record)
            else:
                candidate._diagnostics.append(
                    diagnostic(
                        "Error",
                        "ComponentDeclaration",
                        f"new declaration {record.value.id!r} does not have a "
                        "replaced owner",
                    )
                )
        return candidate

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Records):
            return NotImplemented
        return (self._records, self._diagnostics) == (
            other._records,
            other._diagnostics,
        )

    def __repr__(self) -> str:
        counts = ", ".join(
            f"{kind}: {len(records)}" for kind, records in self._records.items()
        )
        return f"Records({counts}, diagnostics: {len(self._diagnostics)})"

    def _values[M: pydantic.BaseModel](self, kind: RecordKind) -> tuple[M, ...]:
        return tuple(cast(M, record.value) for record in self._records[kind])

    def _diagnostics_about(self, *kinds: RecordKind) -> list[Diagnostic]:
        return [item for item in self._diagnostics if item.kind in kinds]

    def _accept[M: RecordModel](
        self, kind: type[M], value: Json, path: str | None = None
    ) -> Record[M] | None:
        """Decode a record, reporting an error diagnostic if it is invalid."""
        try:
            return decode(kind, value)
        except InvalidRecord as error:
            self._diagnostics.append(diagnostic("Error", KINDS[kind], str(error), path))
            return None

    def _add(self, record: Record[Any]) -> None:
        self._records[KINDS[type(record.value)]].append(record)

    def _copy(self) -> Records:
        result = Records()
        for kind, records in self._records.items():
            result._records[kind] = list(records)
        result._diagnostics = list(self._diagnostics)
        return result

    def _original(self, kind: RecordKind) -> list[Json]:
        return [record.json for record in self._records[kind]]

    def _only(self, *, components: bool, health: bool) -> Records:
        """Copy the records and diagnostics of the given parts."""
        result = self._copy()
        kinds = {"DirectoryConfig"}
        if components:
            kinds.add("ComponentDeclaration")
        else:
            result._records["ComponentDeclaration"] = []
        if health:
            kinds.update(_HEALTH_KINDS)
        else:
            for kind in _HEALTH_KINDS:
                result._records[kind] = []
        result._diagnostics = [d for d in result._diagnostics if d.kind in kinds]
        return result
