"""Decode and validate records, applying the fallbacks for unknown values."""

import copy
import math
import re
from collections.abc import Callable, Mapping
from dataclasses import dataclass, field
from typing import Any

import pydantic

from ._version import Version, parse_requirement
from .models.common import Diagnostic, RecordKind, Severity
from .models.component import ComponentDeclaration
from .models.filesystem import DirectoryConfig
from .models.health import (
    HealthNode,
    HealthRule_Aggregate,
    HealthRule_Observed,
    HealthRule_Threshold,
    HealthStatus,
)
from .models.metric import MetricDefinition, MetricRange, MetricSample

SPEC_VERSION = "1.0"
SUPPORTED_MAJOR = 1
SUPPORTED_MINOR = 0

type RecordModel = (
    ComponentDeclaration | HealthNode | HealthStatus | MetricDefinition | MetricSample
)
"""Model of a kind of record that `Records` holds."""

type Json = Any

KINDS: dict[type[pydantic.BaseModel], RecordKind] = {
    ComponentDeclaration: "ComponentDeclaration",
    HealthNode: "HealthNode",
    HealthStatus: "HealthStatus",
    MetricDefinition: "MetricDefinition",
    MetricSample: "MetricSample",
    DirectoryConfig: "DirectoryConfig",
}

_SPEC_VERSION = re.compile(r"(0|[1-9][0-9]{0,19})\.(0|[1-9][0-9]{0,19})")
_ID = re.compile(r"[!-~]+")
_U64_MAX = 2**64 - 1

_RULE_TYPES = ("Observed", "Threshold", "Aggregate")
_STATES = ("Healthy", "Degraded", "Unhealthy", "Unknown")
_MODES = ("Counted", "Mandatory", "Optional")


class InvalidRecord(ValueError):
    """A record that is not valid and is skipped with an error diagnostic."""


@dataclass(frozen=True)
class Record[M: pydantic.BaseModel]:
    """A valid record with its original JSON and its model, normalized for evaluation.

    Unknown rule types are removed, and unknown states and modes are replaced; the flags
    record where the fallbacks apply.
    """

    json: Json
    value: M
    newer: bool = False
    unsupported: bool = False
    unsupported_attachments: frozenset[str] = field(default_factory=frozenset)


def to_json(model: pydantic.BaseModel) -> Json:
    """Encode a model as JSON."""
    return model.model_dump(mode="json")


def diagnostic(
    severity: Severity, kind: RecordKind, message: str, path: str | None = None
) -> Diagnostic:
    return Diagnostic(severity=severity, kind=kind, message=message, path=path)


def decode[M: pydantic.BaseModel](model: type[M], value: Json) -> Record[M]:
    """Decode and validate a record; raise InvalidRecord if it is not valid."""
    if not isinstance(value, Mapping):
        raise InvalidRecord("record is not a JSON object")
    original = copy.deepcopy(dict(value))
    data = _integral(original)
    kind = KINDS[model]
    newer = _check_version(kind, data)
    unsupported = False
    attachments: set[str] = set()
    if model is HealthNode:
        unsupported = _node_fallbacks(data, attachments)
    elif model is HealthStatus:
        state = data.get("state")
        if state not in _STATES:
            if not isinstance(state, str):
                raise InvalidRecord("state is not a string")
            unsupported = True
            data["state"] = "Unknown"
    try:
        instance = model.model_validate(data)
    except pydantic.ValidationError as error:
        detail = error.errors()[0]
        location = ".".join(str(part) for part in detail["loc"])
        raise InvalidRecord(
            f"{location}: {detail['msg']}" if location else detail["msg"]
        ) from None
    _VALIDATORS[kind](instance)
    return Record(original, instance, newer, unsupported, frozenset(attachments))


def _integral(value: Json) -> Json:
    """Read integral JSON numbers, such as `1.0`, as integers."""
    if isinstance(value, float) and value.is_integer():
        return int(value)
    if isinstance(value, dict):
        return {key: _integral(item) for key, item in value.items()}
    if isinstance(value, list):
        return [_integral(item) for item in value]
    return value


def parse_spec_version(text: object) -> tuple[int, int] | None:
    if not isinstance(text, str):
        return None
    match = _SPEC_VERSION.fullmatch(text)
    if match is None:
        return None
    major, minor = int(match[1]), int(match[2])
    if major > _U64_MAX or minor > _U64_MAX:
        return None
    return major, minor


def _check_version(kind: RecordKind, record: dict[str, Json]) -> bool:
    """Check a record's version; return whether it is a newer minor version."""
    if "specVersion" not in record:
        return False
    version = parse_spec_version(record["specVersion"])
    if version is None:
        raise InvalidRecord(f"invalid specVersion {record['specVersion']!r}")
    major, minor = version
    if major != SUPPORTED_MAJOR:
        raise InvalidRecord(f"unsupported specVersion {record['specVersion']}")
    if minor <= SUPPORTED_MINOR:
        return False
    if kind == "ComponentDeclaration":
        raise InvalidRecord(
            f"component declaration of newer specVersion {record['specVersion']}"
        )
    return True


def _node_fallbacks(record: dict[str, Json], attachments: set[str]) -> bool:
    """Apply the fallbacks of a health node; return whether its rule is unsupported."""
    unsupported = False
    rule = record.get("rule")
    if isinstance(rule, dict):
        rule_type = rule.get("type")
        if not isinstance(rule_type, str):
            raise InvalidRecord("rule.type is not a string")
        if rule_type not in _RULE_TYPES:
            unsupported = True
            del record["rule"]
        elif rule_type == "Aggregate":
            inputs = rule.get("inputs")
            for entry in inputs if isinstance(inputs, list) else []:
                if isinstance(entry, dict) and _mode_fallback(entry):
                    unsupported = True
    part_of = record.get("partOf")
    for entry in part_of if isinstance(part_of, list) else []:
        if isinstance(entry, dict) and _mode_fallback(entry):
            if isinstance(entry.get("node"), str):
                attachments.add(entry["node"])
    return unsupported


def _mode_fallback(entry: dict[str, Json]) -> bool:
    """Replace an unknown input mode with `Counted`; return whether it was unknown."""
    mode = entry.get("mode")
    if mode is None or mode in _MODES:
        return False
    if not isinstance(mode, str):
        raise InvalidRecord("mode is not a string")
    entry["mode"] = "Counted"
    return True


def _check_id(value: str | None, what: str) -> None:
    if value is not None and _ID.fullmatch(value) is None:
        raise InvalidRecord(f"{what} is not a valid ID: {value!r}")


def _check_positive(value: int | None, what: str) -> None:
    if value == 0:
        raise InvalidRecord(f"{what} is zero")


def _check_finite(value: float, what: str) -> None:
    if not math.isfinite(value):
        raise InvalidRecord(f"{what} is not finite")


def _parse(parser: Callable[[str], object], text: str) -> None:
    try:
        parser(text)
    except ValueError as error:
        raise InvalidRecord(str(error)) from None


def _validate_component(record: ComponentDeclaration) -> None:
    _check_id(record.id, "id")
    _check_id(record.owner, "owner")
    if record.version is not None:
        _parse(Version.parse, record.version)
    for claim in record.claims or []:
        _check_id(claim.id, "claim ID")
    for capability in record.provides or []:
        _check_capability(capability.id, capability.version, capability.value)
        if capability.version is not None:
            _parse(Version.parse, capability.version)
    for selector in [*(record.requires or []), *(record.conflicts or [])]:
        _check_capability(selector.id, selector.version, selector.value)
        if selector.version is not None:
            _parse(parse_requirement, selector.version)


def _check_capability(id: str, version: str | None, value: str | None) -> None:
    _check_id(id, "capability ID")
    if version is not None and value is not None:
        raise InvalidRecord(f"capability {id!r} has both version and value")


def _validate_node(record: HealthNode) -> None:
    _check_id(record.id, "id")
    _check_id(record.owner, "owner")
    for component in record.components or []:
        _check_id(component, "component ID")
    for attachment in record.part_of or []:
        _check_id(attachment.node, "attachment node")
    rule = record.rule
    if rule is None:
        return
    if isinstance(rule, HealthRule_Observed | HealthRule_Threshold):
        _check_positive(rule.max_age_ms, "maxAgeMs")
    if isinstance(rule, HealthRule_Threshold):
        _check_id(rule.metric, "metric")
        _check_ranges(rule.healthy, rule.usable)
    elif isinstance(rule, HealthRule_Aggregate):
        for entry in rule.inputs or []:
            _check_id(entry.node, "input node")


def _check_ranges(healthy: MetricRange, usable: MetricRange) -> None:
    for range_ in (healthy, usable):
        for bound in (range_.lower, range_.upper):
            if bound is not None:
                _check_finite(bound.value, "range bound")
        lower, upper = range_.lower, range_.upper
        if lower is not None and upper is not None:
            if lower.value > upper.value or (
                lower.value == upper.value and not (lower.inclusive and upper.inclusive)
            ):
                raise InvalidRecord("range is empty")
    for is_lower, inner, outer in (
        (True, healthy.lower, usable.lower),
        (False, healthy.upper, usable.upper),
    ):
        if outer is None:
            continue
        if inner is None:
            raise InvalidRecord("healthy range is not a subset of the usable range")
        beyond = inner.value < outer.value if is_lower else inner.value > outer.value
        if beyond or (
            inner.value == outer.value and inner.inclusive and not outer.inclusive
        ):
            raise InvalidRecord("healthy range is not a subset of the usable range")


def _validate_status(record: HealthStatus) -> None:
    _check_id(record.id, "id")
    _check_id(record.owner, "owner")
    _check_positive(record.max_age_ms, "maxAgeMs")


def _validate_metric(record: MetricDefinition) -> None:
    _check_id(record.id, "id")
    _check_id(record.owner, "owner")
    for component in record.components or []:
        _check_id(component, "component ID")
    _check_positive(record.max_age_ms, "maxAgeMs")


def _validate_sample(record: MetricSample) -> None:
    _check_id(record.id, "id")
    _check_id(record.owner, "owner")
    _check_finite(record.value, "value")
    _check_positive(record.max_age_ms, "maxAgeMs")


def _validate_directories(record: DirectoryConfig) -> None:
    for paths in (
        record.components,
        record.health_nodes,
        record.health_statuses,
        record.metrics,
        record.metric_samples,
    ):
        for path in paths or []:
            check_directory_path(path)


def check_directory_path(path: str) -> None:
    if not path.startswith("/"):
        raise InvalidRecord(f"path is not absolute: {path!r}")
    if "\0" in path:
        raise InvalidRecord(f"path contains a NUL character: {path!r}")
    for segment in path.split("/"):
        if segment == "..":
            raise InvalidRecord(f"path contains '..': {path!r}")
        if "*" in segment and segment != "*":
            raise InvalidRecord(f"path contains a partial wildcard: {path!r}")


_VALIDATORS: dict[RecordKind, Callable[[Any], None]] = {
    "ComponentDeclaration": _validate_component,
    "HealthNode": _validate_node,
    "HealthStatus": _validate_status,
    "MetricDefinition": _validate_metric,
    "MetricSample": _validate_sample,
    "DirectoryConfig": _validate_directories,
}
