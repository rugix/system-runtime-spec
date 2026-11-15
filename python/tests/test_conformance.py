"""Run the conformance suite shared by all implementations."""

import json
import os
import shutil
from pathlib import Path
from typing import Any

import pytest

from system_runtime_spec import (
    Records,
    candidate_composition,
    check_components,
    estimate_timestamp,
    evaluate_health,
    make_snapshot,
    read_records,
    read_snapshot,
)
from system_runtime_spec.models.common import Diagnostic

FIXTURES = Path(__file__).resolve().parents[2] / "tests/fixtures"
CASES = json.loads((FIXTURES / "conformance.json").read_text())


def brief(report: Any) -> dict[str, Any]:
    """Keep the severity and kind of diagnostics, whose messages are not specified."""
    value: dict[str, Any] = json.loads(report.model_dump_json())
    value["diagnostics"] = sorted(
        ({"severity": d["severity"], "kind": d["kind"]} for d in value["diagnostics"]),
        key=lambda d: (d["kind"], d["severity"]),
    )
    return value


def diagnostics(values: list[dict[str, Any]]) -> list[Diagnostic]:
    return [Diagnostic.model_validate(value) for value in values]


@pytest.mark.parametrize("case", CASES["versions"])
def test_versions(case: dict[str, Any]) -> None:
    report = check_components(
        [
            {"id": "provider", "version": case["version"]},
            {
                "id": "consumer",
                "requires": [{"id": "provider", "version": case["requirement"]}],
            },
        ]
    )
    errors = [d for d in report.diagnostics if d.severity.root == "Error"]
    assert bool(errors) == bool(case.get("error"))
    if not case.get("error"):
        assert report.consistent == case["matches"]


@pytest.mark.parametrize("case", CASES["components"], ids=lambda case: case["name"])
def test_components(case: dict[str, Any]) -> None:
    report = check_components(
        case["components"], diagnostics(case.get("diagnostics", []))
    )
    assert brief(report) == case["report"]


@pytest.mark.parametrize("case", CASES["candidates"], ids=lambda case: case["name"])
def test_candidates(case: dict[str, Any]) -> None:
    components, candidate_diagnostics = candidate_composition(
        case["components"],
        diagnostics(case.get("diagnostics", [])),
        case["replace"],
        case["new"],
    )
    assert components == case["candidate"]
    assert brief(check_components(components, candidate_diagnostics)) == case["report"]


@pytest.mark.parametrize("case", CASES["health"], ids=lambda case: case["name"])
def test_health(case: dict[str, Any]) -> None:
    records = case["records"]
    report = evaluate_health(
        Records(
            health_nodes=records["healthNodes"],
            health_statuses=records["healthStatuses"],
            metrics=records.get("metrics", []),
            metric_samples=records.get("metricSamples", []),
            diagnostics=diagnostics(records.get("diagnostics", [])),
        ),
        case["context"],
    )
    assert brief(report) == case["report"]


@pytest.mark.parametrize("case", CASES["snapshots"], ids=lambda case: case["name"])
def test_snapshots(case: dict[str, Any]) -> None:
    if not case["valid"]:
        with pytest.raises(ValueError):
            read_snapshot(case["snapshot"])
        return
    snapshot = read_snapshot(case["snapshot"])
    assert (
        sorted(
            (
                {"severity": d.severity.root, "kind": d.kind.root}
                for d in snapshot.records.diagnostics
            ),
            key=lambda d: (d["kind"], d["severity"]),
        )
        == case["diagnostics"]
    )
    again = read_snapshot(
        make_snapshot(
            snapshot.records,
            snapshot.clock,
            components=snapshot.components,
            health=snapshot.health,
        )
    )
    assert (again.components, again.health) == (snapshot.components, snapshot.health)
    for read in (snapshot, again):
        assert read.records.components == case["snapshot"].get("components", [])
        assert read.records.health_nodes == case["snapshot"].get("healthNodes", [])


@pytest.mark.parametrize("case", CASES["timestamps"], ids=lambda case: case["name"])
def test_timestamps(case: dict[str, Any]) -> None:
    estimate = estimate_timestamp(case["evidence"], case["clock"])
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
    records = read_records(root)
    real = os.path.realpath(root)
    expected = spec["expected"]
    assert records.components == expected["components"]
    assert records.health_nodes == expected["healthNodes"]
    assert records.health_statuses == expected["healthStatuses"]
    assert records.metrics == expected["metrics"]
    assert records.metric_samples == expected["metricSamples"]
    found = sorted(
        (
            {
                "severity": d.severity.root,
                "kind": d.kind.root,
                "path": os.path.relpath(d.path or "", real),
            }
            for d in records.diagnostics
        ),
        key=lambda d: (d["path"], d["kind"], d["severity"]),
    )
    assert found == expected["diagnostics"]
