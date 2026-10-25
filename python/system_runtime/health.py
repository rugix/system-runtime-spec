"""Evaluate health graphs using caller-supplied boot-time evidence."""

import time
from collections import deque
from collections.abc import Sequence
from pathlib import Path
from typing import Literal

from .models.health import (
    HealthContext,
    HealthNode,
    HealthNodeId,
    HealthNodeResult,
    HealthReport,
    HealthRule_Observed,
    HealthRule_Threshold,
    HealthState,
    HealthState_Degraded,
    HealthState_Healthy,
    HealthState_Unhealthy,
    HealthState_Unknown,
    HealthStatus,
)
from .models.value import ValueDefinition, ValueObservation
from .threshold import ValueEvidence

StateName = Literal["healthy", "degraded", "unhealthy", "unknown"]


def current_context() -> HealthContext:
    """Read the Linux boot identifier and CLOCK_BOOTTIME in milliseconds."""
    return HealthContext(
        boot_id=Path("/proc/sys/kernel/random/boot_id").read_text().strip(),
        now_ms=time.clock_gettime_ns(time.CLOCK_BOOTTIME) // 1_000_000,
    )


def evaluate_health(
    nodes: Sequence[HealthNode],
    statuses: Sequence[HealthStatus],
    context: HealthContext,
    *,
    values: Sequence[ValueDefinition] = (),
    value_observations: Sequence[ValueObservation] = (),
) -> HealthReport:
    """Validate the graph, then evaluate every node in declaration order.

    Malformed graphs and duplicate observations raise ValueError. Missing, expired,
    future, wrong-boot, or wrong-tag evidence yields unknown health.
    """
    evidence = ValueEvidence(values, value_observations)
    by_id = {node.id: node for node in nodes}
    if len(by_id) != len(nodes):
        raise ValueError("Duplicate health node ID")
    observations = {status.id: status for status in statuses}
    if len(observations) != len(statuses):
        raise ValueError("Duplicate health status ID")
    for status in statuses:
        if status.max_age_ms == 0:
            raise ValueError(f"Status {status.id!r} has zero maxAgeMs")
        if status.id in by_id and not isinstance(
            by_id[status.id].rule, HealthRule_Observed
        ):
            raise ValueError(f"Status supplied for derived node {status.id!r}")
    remaining: dict[HealthNodeId, int] = {}
    dependents: dict[HealthNodeId, list[HealthNodeId]] = {node.id: [] for node in nodes}
    for node in nodes:
        rule = node.rule
        if isinstance(rule, HealthRule_Observed):
            if rule.max_age_ms == 0:
                raise ValueError(f"Node {node.id!r} has zero maxAgeMs")
            remaining[node.id] = 0
        elif isinstance(rule, HealthRule_Threshold):
            evidence.validate(rule)
            remaining[node.id] = 0
        else:
            if not 1 <= rule.required <= rule.healthy <= len(rule.inputs):
                raise ValueError(f"Invalid thresholds for {node.id!r}")
            if len(set(rule.inputs)) != len(rule.inputs):
                raise ValueError(f"Duplicate input for {node.id!r}")
            remaining[node.id] = len(rule.inputs)
            for input_id in rule.inputs:
                if input_id not in by_id:
                    raise ValueError(f"Missing health node {input_id!r}")
                dependents[input_id].append(node.id)
    ready = deque(id for id, count in remaining.items() if count == 0)
    results: dict[HealthNodeId, HealthNodeResult] = {}
    while ready:
        id = ready.popleft()
        node = by_id[id]
        if isinstance(node.rule, HealthRule_Observed):
            results[id] = _observe(node, observations.get(id), context)
        elif isinstance(node.rule, HealthRule_Threshold):
            results[id] = evidence.evaluate(node, context)
        else:
            inputs = [results[id] for id in node.rule.inputs]
            healthy = sum(result.state.root == "healthy" for result in inputs)
            minimum_met = sum(
                result.state.root in ("healthy", "degraded") for result in inputs
            )
            possible_minimum = sum(
                result.state.root != "unhealthy" for result in inputs
            )
            state: StateName
            if healthy >= node.rule.healthy:
                state = "healthy"
            elif minimum_met >= node.rule.required:
                state = "degraded"
            elif possible_minimum < node.rule.required:
                state = "unhealthy"
            else:
                state = "unknown"
            results[id] = HealthNodeResult(
                id=id,
                origin=node.origin,
                state=_state(state),
                inputs=node.rule.inputs,
            )
        for dependent in dependents[id]:
            remaining[dependent] -= 1
            if remaining[dependent] == 0:
                ready.append(dependent)
    if len(results) != len(nodes):
        raise ValueError("Cycle in health graph")
    return HealthReport(nodes=[results[node.id] for node in nodes])


def _observe(
    node: HealthNode, status: HealthStatus | None, context: HealthContext
) -> HealthNodeResult:
    """Apply freshness and identity constraints to one observed node."""
    rule = node.rule
    assert isinstance(rule, HealthRule_Observed)
    reason: str | None = None
    if status is None:
        reason = "Missing status"
    elif status.boot_id != context.boot_id:
        reason = "Status belongs to another boot"
    elif node.origin is not None and status.origin != node.origin:
        reason = "Status origin does not match"
    elif rule.expected_tag is not None and status.tag != rule.expected_tag:
        reason = "Status tag does not match"
    elif status.observed_at_ms > context.now_ms:
        reason = "Status is in the future"
    elif context.now_ms - status.observed_at_ms >= min(
        status.max_age_ms,
        rule.max_age_ms if rule.max_age_ms is not None else status.max_age_ms,
    ):
        reason = "Status has expired"
    if reason is not None:
        return HealthNodeResult(
            id=node.id,
            origin=node.origin,
            state=_state("unknown"),
            inputs=[],
            message=reason,
        )
    assert status is not None
    return HealthNodeResult(
        id=node.id,
        origin=node.origin,
        state=status.state,
        inputs=[],
        message=status.message,
    )


def _state(name: StateName) -> HealthState:
    """Construct the generated externally tagged state model."""
    match name:
        case "healthy":
            return HealthState_Healthy(name)
        case "degraded":
            return HealthState_Degraded(name)
        case "unhealthy":
            return HealthState_Unhealthy(name)
        case "unknown":
            return HealthState_Unknown(name)
