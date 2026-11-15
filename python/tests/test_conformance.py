"""Run the conformance suite shared by all implementations."""

import json
import os
import shutil
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

import pydantic
import pytest

from system_runtime_spec import (
    RecordModel,
    Records,
    Snapshot,
    SnapshotError,
    estimate_timestamp,
)
from system_runtime_spec.models.common import Diagnostic
from system_runtime_spec.models.component import ComponentDeclaration
from system_runtime_spec.models.health import (
    HealthClockSample,
    HealthContext,
    HealthNode,
    HealthStatus,
)
from system_runtime_spec.models.metric import MetricDefinition, MetricSample

FIXTURES = Path(__file__).resolve().parents[2] / "tests/fixtures"
CASES = json.loads((FIXTURES / "conformance.json").read_text())
LISTS: dict[str, type[RecordModel]] = {
    "components": ComponentDeclaration,
    "healthNodes": HealthNode,
    "healthStatuses": HealthStatus,
    "metrics": MetricDefinition,
    "metricSamples": MetricSample,
}
CLOCK = HealthClockSample(
    context=HealthContext(boot_id="boot", now_ms=0),
    timestamp=datetime(2026, 1, 1, tzinfo=UTC),
)


def records(value: dict[str, Any]) -> Records:
    """Read the records and diagnostics of the lists of a fixture object."""
    result = Records()
    for item in value.get("diagnostics", []):
        result.push_diagnostic(Diagnostic.model_validate(item))
    for name, model in LISTS.items():
        for item in value.get(name, []):
            result.push_json(model, item)
    return result


def originals(value: Records) -> dict[str, Any]:
    """The original JSON of records, as a snapshot contains them."""
    return Snapshot(value, CLOCK).to_json()


def brief_diagnostics(values: list[Diagnostic]) -> list[dict[str, Any]]:
    """Keep the severity and kind of diagnostics, whose messages are not specified."""
    return sorted(
        ({"severity": d.severity, "kind": d.kind} for d in values),
        key=lambda d: (d["kind"], d["severity"]),
    )


def brief(report: pydantic.BaseModel) -> dict[str, Any]:
    value: dict[str, Any] = json.loads(report.model_dump_json())
    value["diagnostics"] = brief_diagnostics(getattr(report, "diagnostics"))
    return value


@pytest.mark.parametrize("case", CASES["versions"])
def test_versions(case: dict[str, Any]) -> None:
    report = records(
        {
            "components": [
                {"id": "provider", "version": case["version"]},
                {
                    "id": "consumer",
                    "requires": [{"id": "provider", "version": case["requirement"]}],
                },
            ]
        }
    ).check_compatibility()
    errors = [d for d in report.diagnostics if d.severity == "Error"]
    assert bool(errors) == bool(case.get("error"))
    if not case.get("error"):
        assert report.consistent == case["matches"]


@pytest.mark.parametrize("case", CASES["components"], ids=lambda case: case["name"])
def test_components(case: dict[str, Any]) -> None:
    report = records(case).check_compatibility()
    assert brief(report) == case["report"]


def test_typed_records() -> None:
    result = Records()
    result.push(ComponentDeclaration(id="os", owner="rugix.system", version="2.1"))
    result.push(ComponentDeclaration(id="not an ID"))
    assert [c.id for c in result.components] == ["os"]
    assert [d.severity for d in result.diagnostics] == ["Error"]
    assert originals(result)["components"] == [
        {"id": "os", "owner": "rugix.system", "version": "2.1"}
    ]


@pytest.mark.parametrize("case", CASES["candidates"], ids=lambda case: case["name"])
def test_candidates(case: dict[str, Any]) -> None:
    update = records({"components": case["new"]})
    candidate = records(case).with_update(case["replace"], update)
    assert originals(candidate)["components"] == case["candidate"]
    assert brief(candidate.check_compatibility()) == case["report"]


@pytest.mark.parametrize("case", CASES["health"], ids=lambda case: case["name"])
def test_health(case: dict[str, Any]) -> None:
    context = HealthContext.model_validate(case["context"])
    report = records(case["records"]).evaluate_health(context)
    assert brief(report) == case["report"]


@pytest.mark.parametrize("case", CASES["snapshots"], ids=lambda case: case["name"])
def test_snapshots(case: dict[str, Any]) -> None:
    original = case["snapshot"]
    if not case["valid"]:
        with pytest.raises(SnapshotError):
            Snapshot.from_json(original)
        return
    snapshot = Snapshot.from_json(original)
    value = snapshot.to_json()
    assert Snapshot.from_json(value) == snapshot
    for name in LISTS:
        assert value.get(name) == original.get(name)
    assert brief_diagnostics(list(snapshot.records.diagnostics)) == case["diagnostics"]


@pytest.mark.parametrize("case", CASES["timestamps"], ids=lambda case: case["name"])
def test_timestamps(case: dict[str, Any]) -> None:
    estimate = estimate_timestamp(
        HealthClockSample.model_validate(case["clock"]),
        HealthStatus.model_validate(case["evidence"]),
    )
    text = (
        None
        if estimate is None
        else estimate.isoformat(timespec="milliseconds").replace("+00:00", "Z")
    )
    assert text == case["expected"]


def test_discovery(tmp_path: Path) -> None:
    fixture = FIXTURES / "discovery"
    spec = json.loads((fixture / "discovery.json").read_text())
    root = tmp_path / "root"
    shutil.copytree(fixture / "root", root)
    for link in spec["symlinks"]:
        (root / link["path"]).parent.mkdir(parents=True, exist_ok=True)
        os.symlink(link["target"], root / link["path"])
    discovered = Records.discover(root)
    expected = spec["expected"]
    value = originals(discovered)
    for name in LISTS:
        assert value[name] == expected[name]
    real = os.path.realpath(root)
    found = sorted(
        (
            {
                "severity": d.severity,
                "kind": d.kind,
                "path": os.path.relpath(d.path or "", real),
            }
            for d in discovered.diagnostics
        ),
        key=lambda d: (d["path"], d["kind"], d["severity"]),
    )
    assert found == expected["diagnostics"]
