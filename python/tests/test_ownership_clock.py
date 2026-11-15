"""Owner-based update candidates and transferable health clock correlation."""

import json
from datetime import UTC, datetime
from pathlib import Path

import pytest

from system_runtime import (
    check_components,
    current_clock_sample,
    evaluate_health,
    status_timestamp,
    without_owner,
)
from system_runtime.models.component import ComponentDeclaration
from system_runtime.models.filesystem import RuntimeSnapshot
from system_runtime.models.health import (
    HealthClockSample,
    HealthSnapshot,
    HealthStatus,
)

FIXTURES = Path(__file__).resolve().parents[2] / "tests/fixtures"
CASES = json.loads((FIXTURES / "ownership-clock.json").read_text())


@pytest.mark.parametrize("case", CASES["owners"], ids=lambda case: case["name"])
def test_owner_update(case):
    """Remove obsolete IDs by owner, preserving sibling owners and constraints."""
    original = RuntimeSnapshot.model_validate(case["snapshot"])
    candidate = without_owner(original, case["owner"])
    assert json.loads(candidate.model_dump_json()) == case["expected"]
    assert check_components(candidate.components).consistent == case["consistent"]
    if "replacement" in case:
        candidate.components.append(
            ComponentDeclaration.model_validate(case["replacement"])
        )
        assert check_components(candidate.components).consistent
    assert json.loads(original.model_dump_json()) == case["snapshot"]


@pytest.mark.parametrize("case", CASES["timestamps"], ids=lambda case: case["name"])
def test_timestamp_correlation(case):
    """Map large boot-time values to UTC without renewing evidence."""
    status = HealthStatus.model_validate(case["status"])
    clock = HealthClockSample.model_validate(case["clock"])
    expected = datetime.fromisoformat(case["expected"]) if case["expected"] else None
    assert status_timestamp(status, clock) == expected


def test_health_transport():
    """Round-trip an owned health graph while preserving expiry and observation time."""
    source = RuntimeSnapshot.model_validate(CASES["owners"][0]["snapshot"])
    clock = HealthClockSample.model_validate(CASES["timestamps"][0]["clock"])
    snapshot = HealthSnapshot(
        clock=clock,
        nodes=source.health_nodes,
        statuses=source.health_statuses,
        values=source.values,
        value_observations=source.value_observations,
    )
    received = HealthSnapshot.model_validate_json(snapshot.model_dump_json())
    assert received == snapshot
    report = evaluate_health(received.nodes, received.statuses, received.clock.context)
    assert report.nodes[1].owner == "rugix.apps.example"
    assert report.nodes[1].state.root == "healthy"
    assert status_timestamp(received.statuses[1], received.clock) == datetime(
        2026, 10, 14, 11, 59, 58, 623000, tzinfo=UTC
    )
    received.clock.context.now_ms = 5500
    assert (
        evaluate_health(received.nodes, received.statuses, received.clock.context)
        .nodes[1]
        .state.root
        == "unknown"
    )
    assert received.statuses[1].observed_at_ms == 3500


def test_current_clock_sample():
    """Capture a valid UTC and Linux boot-time correlation for transport."""
    clock = current_clock_sample()
    assert clock.context.boot_id
    assert clock.context.now_ms >= 0
    assert clock.timestamp.utcoffset().total_seconds() == 0
    assert HealthClockSample.model_validate_json(clock.model_dump_json()) == clock
