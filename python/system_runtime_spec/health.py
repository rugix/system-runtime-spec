"""Evaluate health in a health context."""

import time
from collections.abc import Callable
from pathlib import Path

from ._records import (
    Decoded,
    Json,
    JsonObject,
    Records,
    decode_all,
    diagnostic,
    to_json,
)
from .models.common import Diagnostic
from .models.health import HealthContext, HealthReport

type Answers = tuple[str, str]

_STATE_ANSWERS: dict[str, Answers] = {
    "Healthy": ("Yes", "Yes"),
    "Degraded": ("Yes", "No"),
    "Unhealthy": ("No", "No"),
    "Unknown": ("Unknown", "Unknown"),
}
_UNKNOWN: Answers = ("Unknown", "Unknown")


def current_context() -> HealthContext:
    """Read the Linux boot ID and the boot-time clock in milliseconds."""
    return HealthContext(
        boot_id=Path("/proc/sys/kernel/random/boot_id").read_text().strip(),
        now_ms=time.clock_gettime_ns(time.CLOCK_BOOTTIME) // 1_000_000,
    )


def evaluate_health(records: Records, context: HealthContext | Json) -> HealthReport:
    """Evaluate the health records of `records` in a health context.

    Invalid records are skipped with an error diagnostic. The report contains one result
    per node ID, in the order of the node definitions, and all diagnostics.
    """
    return _Evaluation(records, to_json(context)).report()


def _group(decoded: list[Decoded]) -> dict[str, list[Decoded]]:
    groups: dict[str, list[Decoded]] = {}
    for item in decoded:
        groups.setdefault(item.record["id"], []).append(item)
    return groups


def _conjunction(answers: list[str]) -> str:
    if "No" in answers:
        return "No"
    if "Unknown" in answers:
        return "Unknown"
    return "Yes"


def _count(minimum: int, answers: list[str]) -> str:
    yes = answers.count("Yes")
    if yes >= minimum:
        return "Yes"
    if yes + answers.count("Unknown") < minimum:
        return "No"
    return "Unknown"


def _contains(range_: JsonObject, value: float) -> bool:
    lower, upper = range_.get("lower"), range_.get("upper")
    return (
        lower is None
        or value > lower["value"]
        or (value == lower["value"] and lower["inclusive"])
    ) and (
        upper is None
        or value < upper["value"]
        or (value == upper["value"] and upper["inclusive"])
    )


class _Evaluation:
    def __init__(self, records: Records, context: JsonObject) -> None:
        self.context = context
        self.reported: set[str] = set()
        self.diagnostics: list[Diagnostic] = list(records.diagnostics)
        self.nodes = decode_all("HealthNode", records.health_nodes, self.diagnostics)
        self.statuses = _group(
            decode_all("HealthStatus", records.health_statuses, self.diagnostics)
        )
        self.metrics = _group(
            decode_all("MetricDefinition", records.metrics, self.diagnostics)
        )
        self.samples = _group(
            decode_all("MetricSample", records.metric_samples, self.diagnostics)
        )
        groups = _group(self.nodes)
        self.conflicts = {id for id, group in groups.items() if len(group) > 1}
        for id in sorted(self.conflicts):
            self._diagnose("Error", "HealthNode", f"several nodes have the ID {id!r}")
        self.unique = {id: group[0] for id, group in groups.items() if len(group) == 1}
        self.inputs: dict[str, list[tuple[str, str]]] = {}
        self.unsupported = {id for id, node in self.unique.items() if node.unsupported}
        self._resolve_inputs()
        self.on_cycle = self._find_cycles()
        self.results: dict[str, JsonObject] = {}

    def _diagnose(self, severity: str, kind: str, message: str) -> None:
        if message not in self.reported:
            self.reported.add(message)
            self.diagnostics.append(diagnostic(severity, kind, message))

    def _is_aggregate(self, id: str) -> bool:
        node = self.unique.get(id)
        return (
            node is not None and node.record.get("rule", {}).get("type") == "Aggregate"
        )

    def _resolve_inputs(self) -> None:
        attached: dict[str, list[tuple[str, str]]] = {}
        for id, node in self.unique.items():
            for attachment in node.record.get("partOf") or []:
                parent = attachment["node"]
                if not self._is_aggregate(parent):
                    continue
                if parent in node.unsupported_attachments:
                    self.unsupported.add(parent)
                attached.setdefault(parent, []).append(
                    (id, attachment.get("mode", "Counted"))
                )
        for id, node in self.unique.items():
            if not self._is_aggregate(id):
                continue
            inputs: dict[str, str] = {}
            for entry in node.record["rule"].get("inputs") or []:
                if (
                    entry["node"] not in self.unique
                    and entry["node"] not in self.conflicts
                ):
                    self._diagnose(
                        "Warning",
                        "HealthNode",
                        f"input {entry['node']!r} of {id!r} does not exist",
                    )
                inputs.setdefault(entry["node"], entry.get("mode", "Counted"))
            for child, mode in attached.get(id, []):
                inputs.setdefault(child, mode)
            self.inputs[id] = list(inputs.items())

    def _find_cycles(self) -> set[str]:
        """Find the nodes on a cycle with Tarjan's algorithm."""
        index: dict[str, int] = {}
        low: dict[str, int] = {}
        stack: list[str] = []
        on_stack: set[str] = set()
        on_cycle: set[str] = set()

        def edges(id: str) -> list[str]:
            return [
                child for child, _ in self.inputs.get(id, []) if child in self.unique
            ]

        for start in self.inputs:
            if start in index:
                continue
            work = [(start, 0)]
            index[start] = low[start] = len(index)
            stack.append(start)
            on_stack.add(start)
            while work:
                id, position = work[-1]
                children = edges(id)
                if position < len(children):
                    work[-1] = (id, position + 1)
                    child = children[position]
                    if child not in index:
                        index[child] = low[child] = len(index)
                        stack.append(child)
                        on_stack.add(child)
                        work.append((child, 0))
                    elif child in on_stack:
                        low[id] = min(low[id], index[child])
                    continue
                work.pop()
                if work:
                    parent = work[-1][0]
                    low[parent] = min(low[parent], low[id])
                if low[id] == index[id]:
                    component = []
                    while True:
                        member = stack.pop()
                        on_stack.discard(member)
                        component.append(member)
                        if member == id:
                            break
                    if len(component) > 1 or id in edges(id):
                        on_cycle.update(component)
        return on_cycle

    def report(self) -> HealthReport:
        nodes = []
        emitted = set()
        for node in self.nodes:
            id = node.record["id"]
            if id in emitted:
                continue
            emitted.add(id)
            if id in self.conflicts:
                nodes.append(
                    {
                        "id": id,
                        "usable": "Unknown",
                        "healthy": "Unknown",
                        "inputs": [],
                        "reason": "Conflict",
                    }
                )
            else:
                nodes.append(self._result(id))
        return HealthReport.model_validate(
            {
                "nodes": nodes,
                "diagnostics": [to_json(item) for item in self.diagnostics],
            }
        )

    def _answers(self, id: str) -> Answers:
        if id in self.unique:
            result = self._result(id)
            return result["usable"], result["healthy"]
        return _UNKNOWN

    def _result(self, id: str) -> JsonObject:
        if id in self.results:
            return self.results[id]
        node = self.unique[id].record
        result: JsonObject = {"id": id}
        if "owner" in node:
            result["owner"] = node["owner"]
        result["inputs"] = [child for child, _ in self.inputs.get(id, [])]
        rule = node.get("rule", {"type": "Observed"})
        if id in self.on_cycle:
            self._set(result, _UNKNOWN, "Cycle")
        elif id in self.unsupported:
            self._set(result, _UNKNOWN, "Unsupported")
        elif rule["type"] == "Observed":
            self._observe(result, node, rule)
        elif rule["type"] == "Threshold":
            self._threshold(result, node, rule)
        else:
            self._aggregate(result, node, rule)
        self.results[id] = result
        return result

    @staticmethod
    def _set(result: JsonObject, answers: Answers, reason: str | None = None) -> None:
        result["usable"], result["healthy"] = answers
        if reason is not None:
            result["reason"] = reason

    def _fresh(
        self,
        evidence: JsonObject,
        owner: str | None,
        tag: str | None,
        max_ages: list[int | None],
    ) -> str | None:
        """Return why evidence is not fresh, or None if it is fresh."""
        if evidence["bootId"] != self.context["bootId"]:
            return "OtherBoot"
        if owner is not None and evidence.get("owner") != owner:
            return "OwnerMismatch"
        if tag is not None and evidence.get("tag") != tag:
            return "TagMismatch"
        now = int(self.context["nowMs"])
        observed = int(evidence["observedAtMs"])
        if observed > now:
            return "Future"
        limits = [evidence["maxAgeMs"], *(age for age in max_ages if age is not None)]
        if any(now - observed >= int(limit) for limit in limits):
            return "Expired"
        return None

    def _evidence(
        self,
        result: JsonObject,
        evidence: JsonObject,
        reason: str | None,
        interpret: Callable[[JsonObject], Answers | str],
    ) -> None:
        if reason is not None:
            self._set(result, _UNKNOWN, reason)
            return
        answers = interpret(evidence)
        if isinstance(answers, str):
            self._set(result, _UNKNOWN, answers)
        else:
            self._set(result, answers)
        for name in ("message", "details"):
            if name in evidence:
                result[name] = evidence[name]

    def _observe(self, result: JsonObject, node: JsonObject, rule: JsonObject) -> None:
        statuses = self.statuses.get(node["id"], [])
        if len(statuses) > 1:
            self._diagnose(
                "Warning", "HealthStatus", f"several status records for {node['id']!r}"
            )
            self._set(result, _UNKNOWN, "Conflict")
            return
        if not statuses:
            self._set(result, _UNKNOWN, "Missing")
            return
        status = statuses[0]
        reason = self._fresh(
            status.record,
            node.get("owner"),
            rule.get("expectedTag"),
            [rule.get("maxAgeMs")],
        )
        self._evidence(
            result,
            status.record,
            reason,
            lambda record: (
                "Unsupported" if status.unsupported else _STATE_ANSWERS[record["state"]]
            ),
        )

    def _threshold(
        self, result: JsonObject, node: JsonObject, rule: JsonObject
    ) -> None:
        metric = rule["metric"]
        definitions = self.metrics.get(metric, [])
        samples = self.samples.get(metric, [])
        if not definitions:
            self._set(result, _UNKNOWN, "Missing")
            return
        definition = definitions[0].record
        conflicts = []
        if len(definitions) > 1:
            conflicts.append(("MetricDefinition", f"several definitions of {metric!r}"))
        if len(samples) > 1:
            conflicts.append(("MetricSample", f"several samples of {metric!r}"))
        if "unit" in rule and rule["unit"] != definition["unit"]:
            conflicts.append(
                ("HealthNode", f"unit of {node['id']!r} differs from {metric!r}")
            )
        if conflicts:
            for kind, message in conflicts:
                self._diagnose("Warning", kind, message)
            self._set(result, _UNKNOWN, "Conflict")
            return
        if not samples:
            self._set(result, _UNKNOWN, "Missing")
            return
        sample = samples[0].record
        reason = self._fresh(
            sample,
            definition.get("owner"),
            definition.get("expectedTag"),
            [definition.get("maxAgeMs"), rule.get("maxAgeMs")],
        )

        def interpret(record: JsonObject) -> Answers:
            if _contains(rule["healthy"], record["value"]):
                return "Yes", "Yes"
            if _contains(rule["usable"], record["value"]):
                return "Yes", "No"
            return "No", "No"

        self._evidence(result, sample, reason, interpret)

    def _aggregate(
        self, result: JsonObject, node: JsonObject, rule: JsonObject
    ) -> None:
        inputs = self.inputs[node["id"]]
        answers = {child: self._answers(child) for child, _ in inputs}
        counted = [answers[child] for child, mode in inputs if mode != "Optional"]
        mandatory = [answers[child] for child, mode in inputs if mode == "Mandatory"]
        optional = [answers[child] for child, mode in inputs if mode == "Optional"]
        min_healthy = int(rule.get("minHealthy", len(counted)))
        min_usable = int(rule.get("minUsable", min_healthy))
        usable = _conjunction(
            [
                _count(min_usable, [usable for usable, _ in counted]),
                *(usable for usable, _ in mandatory),
            ]
        )
        healthy = _conjunction(
            [
                usable,
                _count(min_healthy, [healthy for _, healthy in counted]),
                *(healthy for _, healthy in mandatory),
                *(healthy for _, healthy in optional),
            ]
        )
        self._set(result, (usable, healthy))
