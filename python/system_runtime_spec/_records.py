"""Decode and validate records, applying the fallbacks for unknown values."""

import math
import re
from collections.abc import Iterable, Mapping
from dataclasses import dataclass, field
from typing import Any

import pydantic

from ._version import Version, parse_requirement
from .models.common import Diagnostic
from .models.component import ComponentDeclaration
from .models.filesystem import DirectoryConfig
from .models.health import HealthNode, HealthStatus
from .models.metric import MetricDefinition, MetricSample

SPEC_VERSION = "1.0"
SUPPORTED_MAJOR = 1
SUPPORTED_MINOR = 0

type Json = Any
type JsonObject = dict[str, Any]

_SPEC_VERSION = re.compile(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)")
_ID = re.compile(r"[!-~]+")
_U64_MAX = 2**64 - 1

RULE_TYPES = ("Observed", "Threshold", "Aggregate")
STATES = ("Healthy", "Degraded", "Unhealthy", "Unknown")
MODES = ("Counted", "Mandatory", "Optional")
KINDS = (
    "ComponentDeclaration",
    "HealthNode",
    "HealthStatus",
    "MetricDefinition",
    "MetricSample",
    "DirectoryConfig",
)

_MODELS: dict[str, type[pydantic.BaseModel]] = {
    "ComponentDeclaration": ComponentDeclaration,
    "HealthNode": HealthNode,
    "HealthStatus": HealthStatus,
    "MetricDefinition": MetricDefinition,
    "MetricSample": MetricSample,
    "DirectoryConfig": DirectoryConfig,
}


class InvalidRecord(ValueError):
    """A record that is not valid and is skipped with an error diagnostic."""


@dataclass
class Records:
    """Records of all kinds, as JSON objects, with the diagnostics about them.

    Records keep their original JSON, so snapshots contain them unchanged.
    """

    components: list[JsonObject] = field(default_factory=list)
    health_nodes: list[JsonObject] = field(default_factory=list)
    health_statuses: list[JsonObject] = field(default_factory=list)
    metrics: list[JsonObject] = field(default_factory=list)
    metric_samples: list[JsonObject] = field(default_factory=list)
    diagnostics: list[Diagnostic] = field(default_factory=list)


@dataclass
class Decoded:
    """A valid record, normalized for evaluation, with the values that need a fallback.

    Unknown rule types are removed, and unknown states and modes are replaced; the flags
    record where the fallbacks apply.
    """

    record: JsonObject
    newer: bool = False
    unsupported: bool = False
    unsupported_attachments: set[str] = field(default_factory=set)


def to_json(value: Json) -> Json:
    """Convert a model to its JSON value; other values pass through."""
    if isinstance(value, pydantic.BaseModel):
        return value.model_dump(mode="json", by_alias=True, exclude_none=True)
    return value


def diagnostic(
    severity: str, kind: str, message: str, path: str | None = None
) -> Diagnostic:
    value: JsonObject = {"severity": severity, "kind": kind, "message": message}
    if path is not None:
        value["path"] = path
    return Diagnostic.model_validate(value)


def decode(kind: str, value: Json) -> Decoded:
    """Validate a record of a kind and determine its fallbacks.

    Raise InvalidRecord for records that are not valid.
    """
    if not isinstance(value, Mapping):
        raise InvalidRecord("record is not a JSON object")
    record = _integral(dict(value))
    newer = _check_version(kind, record)
    result = Decoded(record=record, newer=newer)
    if kind == "HealthNode":
        _health_node_fallbacks(record, result)
    elif kind == "HealthStatus":
        if record.get("state") not in STATES:
            if not isinstance(record.get("state"), str):
                raise InvalidRecord("state is not a string")
            result.unsupported = True
            record["state"] = "Unknown"
    try:
        _MODELS[kind].model_validate(record)
    except pydantic.ValidationError as error:
        raise InvalidRecord(_first_error(error)) from None
    _VALIDATORS[kind](record)
    return result


def _first_error(error: pydantic.ValidationError) -> str:
    detail = error.errors()[0]
    location = ".".join(str(part) for part in detail["loc"])
    return f"{location}: {detail['msg']}" if location else detail["msg"]


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


def _check_version(kind: str, record: JsonObject) -> bool:
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


def _health_node_fallbacks(record: JsonObject, result: Decoded) -> None:
    rule = record.get("rule")
    if isinstance(rule, dict):
        rule_type = rule.get("type")
        if not isinstance(rule_type, str):
            raise InvalidRecord("rule.type is not a string")
        if rule_type not in RULE_TYPES:
            result.unsupported = True
            del record["rule"]
        elif rule_type == "Aggregate":
            for entry in rule.get("inputs") or []:
                if isinstance(entry, dict) and _unknown_mode(entry):
                    result.unsupported = True
    for entry in record.get("partOf") or []:
        if isinstance(entry, dict) and _unknown_mode(entry):
            if isinstance(entry.get("node"), str):
                result.unsupported_attachments.add(entry["node"])


def _unknown_mode(entry: JsonObject) -> bool:
    """Replace an unknown input mode with `Counted`; return whether it was unknown."""
    mode = entry.get("mode")
    if mode is None or mode in MODES:
        return False
    if not isinstance(mode, str):
        raise InvalidRecord("mode is not a string")
    entry["mode"] = "Counted"
    return True


def _check_id(value: object, what: str) -> None:
    if not isinstance(value, str) or _ID.fullmatch(value) is None:
        raise InvalidRecord(f"{what} is not a valid ID: {value!r}")


def _check_ids(record: JsonObject, *fields: str) -> None:
    for name in fields:
        if name in record:
            _check_id(record[name], name)


def _check_positive(record: JsonObject, name: str) -> None:
    if name in record and record[name] == 0:
        raise InvalidRecord(f"{name} is zero")


def _check_finite(value: Json, what: str) -> None:
    if not isinstance(value, (int, float)) or not math.isfinite(value):
        raise InvalidRecord(f"{what} is not finite")


def _validate_component(record: JsonObject) -> None:
    _check_ids(record, "id", "owner")
    for capability in record.get("provides") or []:
        _check_capability(capability, version=True)
    for claim in record.get("claims") or []:
        _check_id(claim["id"], "claim ID")
    for selector in [*(record.get("requires") or []), *(record.get("conflicts") or [])]:
        _check_capability(selector, version=False)
    if "version" in record:
        _parse(Version.parse, record["version"])


def _check_capability(value: JsonObject, *, version: bool) -> None:
    _check_id(value["id"], "capability ID")
    if "version" in value and "value" in value:
        raise InvalidRecord(f"capability {value['id']!r} has both version and value")
    if "version" in value:
        _parse(Version.parse if version else parse_requirement, value["version"])


def _parse(parser: Any, text: str) -> None:
    try:
        parser(text)
    except ValueError as error:
        raise InvalidRecord(str(error)) from None


def _validate_node(record: JsonObject) -> None:
    _check_ids(record, "id", "owner")
    for component in record.get("components") or []:
        _check_id(component, "component ID")
    for attachment in record.get("partOf") or []:
        _check_id(attachment["node"], "attachment node")
    rule = record.get("rule")
    if rule is None:
        return
    _check_positive(rule, "maxAgeMs")
    if rule["type"] == "Threshold":
        _check_id(rule["metric"], "metric")
        _check_ranges(rule["healthy"], rule["usable"])
    elif rule["type"] == "Aggregate":
        for entry in rule.get("inputs") or []:
            _check_id(entry["node"], "input node")


def _check_ranges(healthy: JsonObject, usable: JsonObject) -> None:
    for range_ in (healthy, usable):
        for bound in (range_.get("lower"), range_.get("upper")):
            if bound is not None:
                _check_finite(bound["value"], "range bound")
        lower, upper = range_.get("lower"), range_.get("upper")
        if lower is not None and upper is not None:
            if lower["value"] > upper["value"] or (
                lower["value"] == upper["value"]
                and not (lower["inclusive"] and upper["inclusive"])
            ):
                raise InvalidRecord("range is empty")
    for side, inner, outer in (
        ("lower", healthy.get("lower"), usable.get("lower")),
        ("upper", healthy.get("upper"), usable.get("upper")),
    ):
        if outer is None:
            continue
        if inner is None:
            raise InvalidRecord("healthy range is not a subset of the usable range")
        beyond = (
            inner["value"] < outer["value"]
            if side == "lower"
            else inner["value"] > outer["value"]
        )
        if beyond or (
            inner["value"] == outer["value"]
            and inner["inclusive"]
            and not outer["inclusive"]
        ):
            raise InvalidRecord("healthy range is not a subset of the usable range")


def _validate_status(record: JsonObject) -> None:
    _check_ids(record, "id", "owner")
    _check_positive(record, "maxAgeMs")


def _validate_metric(record: JsonObject) -> None:
    _check_ids(record, "id", "owner")
    for component in record.get("components") or []:
        _check_id(component, "component ID")
    _check_positive(record, "maxAgeMs")


def _validate_sample(record: JsonObject) -> None:
    _check_ids(record, "id", "owner")
    _check_finite(record["value"], "value")
    _check_positive(record, "maxAgeMs")


def _validate_directories(record: JsonObject) -> None:
    for name in (
        "components",
        "healthNodes",
        "healthStatuses",
        "metrics",
        "metricSamples",
    ):
        for path in record.get(name) or []:
            check_directory_path(path)


def check_directory_path(path: str) -> None:
    if not path.startswith("/"):
        raise InvalidRecord(f"path is not absolute: {path!r}")
    for segment in path.split("/"):
        if segment == "..":
            raise InvalidRecord(f"path contains '..': {path!r}")
        if "*" in segment and segment != "*":
            raise InvalidRecord(f"path contains a partial wildcard: {path!r}")


_VALIDATORS = {
    "ComponentDeclaration": _validate_component,
    "HealthNode": _validate_node,
    "HealthStatus": _validate_status,
    "MetricDefinition": _validate_metric,
    "MetricSample": _validate_sample,
    "DirectoryConfig": _validate_directories,
}


def decode_all(
    kind: str, values: Iterable[Json], diagnostics: list[Diagnostic]
) -> list[Decoded]:
    """Decode records, skipping invalid ones with an error diagnostic."""
    result = []
    for value in values:
        try:
            result.append(decode(kind, to_json(value)))
        except InvalidRecord as error:
            diagnostics.append(diagnostic("Error", kind, str(error)))
    return result
