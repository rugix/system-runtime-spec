from collections.abc import Sequence

from ._decoding import Record, diagnostic
from .models.common import Diagnostic, DiagnosticKind, Severity
from .models.health import (
    Answer,
    HealthContext,
    HealthNode,
    HealthNodeResult,
    HealthReport,
    HealthRule_Aggregate,
    HealthRule_Observed,
    HealthRule_Threshold,
    HealthStatus,
    InputMode,
    Reason,
)
from .models.metric import MetricDefinition, MetricRange, MetricSample

type Answers = tuple[Answer, Answer]

_UNKNOWN: Answers = ("Unknown", "Unknown")


def evaluate(
    nodes: Sequence[Record[HealthNode]],
    statuses: Sequence[Record[HealthStatus]],
    metrics: Sequence[Record[MetricDefinition]],
    samples: Sequence[Record[MetricSample]],
    diagnostics: Sequence[Diagnostic],
    context: HealthContext,
) -> HealthReport:
    return _Evaluation(nodes, statuses, metrics, samples, diagnostics, context).report()


def _group[T: HealthNode | HealthStatus | MetricDefinition | MetricSample](
    records: Sequence[Record[T]],
) -> dict[str, list[Record[T]]]:
    groups: dict[str, list[Record[T]]] = {}
    for record in records:
        groups.setdefault(record.value.id, []).append(record)
    return groups


def _conjunction(answers: list[Answer]) -> Answer:
    if "No" in answers:
        return "No"
    if "Unknown" in answers:
        return "Unknown"
    return "Yes"


def _count_answer(minimum: int, answers: list[Answer]) -> Answer:
    yes = answers.count("Yes")
    if yes >= minimum:
        return "Yes"
    if yes + answers.count("Unknown") < minimum:
        return "No"
    return "Unknown"


def _contains(range_: MetricRange, value: float) -> bool:
    lower, upper = range_.lower, range_.upper
    return (
        lower is None
        or value > lower.value
        or (value == lower.value and lower.inclusive)
    ) and (
        upper is None
        or value < upper.value
        or (value == upper.value and upper.inclusive)
    )


class _Evaluation:
    def __init__(
        self,
        nodes: Sequence[Record[HealthNode]],
        statuses: Sequence[Record[HealthStatus]],
        metrics: Sequence[Record[MetricDefinition]],
        samples: Sequence[Record[MetricSample]],
        diagnostics: Sequence[Diagnostic],
        context: HealthContext,
    ) -> None:
        self.context = context
        self.reported: set[str] = set()
        self.diagnostics = list(diagnostics)
        self.order = list(dict.fromkeys(node.value.id for node in nodes))
        self.statuses = _group(statuses)
        self.metrics = _group(metrics)
        self.samples = _group(samples)
        groups = _group(nodes)
        self.conflicts = {id for id, group in groups.items() if len(group) > 1}
        for id in sorted(self.conflicts):
            self._diagnose("Error", "HealthNode", f"several nodes have the ID {id!r}")
        self.nodes = {id: group[0] for id, group in groups.items() if len(group) == 1}
        self.unsupported = {id for id, node in self.nodes.items() if node.unsupported}
        self.inputs: dict[str, list[tuple[str, InputMode]]] = {}
        self._resolve_inputs()
        self.on_cycle = self._find_cycles()
        for id in self.order:
            if id in self.on_cycle:
                self._diagnose("Warning", "HealthNode", f"node {id!r} is on a cycle")
        self.results: dict[str, HealthNodeResult] = {}

    def _diagnose(self, severity: Severity, kind: DiagnosticKind, message: str) -> None:
        if message not in self.reported:
            self.reported.add(message)
            self.diagnostics.append(diagnostic(severity, kind, message))

    def _aggregate_rule(self, id: str) -> HealthRule_Aggregate | None:
        node = self.nodes.get(id)
        rule = node.value.rule if node is not None else None
        return rule if isinstance(rule, HealthRule_Aggregate) else None

    def _resolve_inputs(self) -> None:
        attached: dict[str, list[tuple[str, InputMode]]] = {}
        for id in self.order:
            node = self.nodes.get(id)
            if node is None:
                continue
            for attachment in node.value.part_of or []:
                parent = attachment.node
                if self._aggregate_rule(parent) is None:
                    continue
                if parent in node.unsupported_parents:
                    self.unsupported.add(parent)
                attached.setdefault(parent, []).append(
                    (id, attachment.mode or "Counted")
                )
        for id in self.order:
            rule = self._aggregate_rule(id)
            if rule is None:
                continue
            inputs: dict[str, InputMode] = {}
            for entry in rule.inputs or []:
                if entry.node not in self.nodes and entry.node not in self.conflicts:
                    self._diagnose(
                        "Warning",
                        "HealthNode",
                        f"input {entry.node!r} of {id!r} does not exist",
                    )
                inputs.setdefault(entry.node, entry.mode or "Counted")
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
                child for child, _ in self.inputs.get(id, []) if child in self.nodes
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
        nodes = [
            HealthNodeResult(
                id=id,
                usable="Unknown",
                healthy="Unknown",
                inputs=[],
                reason="Conflict",
            )
            if id in self.conflicts
            else self._result(id)
            for id in self.order
        ]
        return HealthReport(nodes=nodes, diagnostics=self.diagnostics)

    def _answers(self, id: str) -> Answers:
        if id in self.nodes:
            result = self._result(id)
            return result.usable, result.healthy
        return _UNKNOWN

    def _result(self, id: str) -> HealthNodeResult:
        if id in self.results:
            return self.results[id]
        node = self.nodes[id].value
        result = HealthNodeResult(
            id=id,
            owner=node.owner,
            usable="Unknown",
            healthy="Unknown",
            inputs=[child for child, _ in self.inputs.get(id, [])],
        )
        rule = node.rule
        if id in self.on_cycle:
            result.reason = "Cycle"
        elif id in self.unsupported:
            result.reason = "Unsupported"
        elif isinstance(rule, HealthRule_Threshold):
            self._threshold(result, node, rule)
        elif isinstance(rule, HealthRule_Aggregate):
            self._aggregate(result, rule)
        else:
            self._observe(result, node, rule or HealthRule_Observed())
        self.results[id] = result
        return result

    def _freshness_reason(
        self,
        evidence: HealthStatus | MetricSample,
        owner: str | None,
        tag: str | None,
        max_ages: list[int | None],
    ) -> Reason | None:
        """Return why evidence is not fresh, or None if it is fresh."""
        if evidence.boot_id != self.context.boot_id:
            return "OtherBoot"
        if owner is not None and evidence.owner != owner:
            return "OwnerMismatch"
        if tag is not None and evidence.tag != tag:
            return "TagMismatch"
        now = self.context.now_ms
        if evidence.observed_at_ms > now:
            return "Future"
        limits = [evidence.max_age_ms, *(age for age in max_ages if age is not None)]
        if any(now - evidence.observed_at_ms >= limit for limit in limits):
            return "Expired"
        return None

    def _observe(
        self, result: HealthNodeResult, node: HealthNode, rule: HealthRule_Observed
    ) -> None:
        statuses = self.statuses.get(node.id, [])
        if len(statuses) > 1:
            self._diagnose(
                "Warning", "HealthStatus", f"several status records for {node.id!r}"
            )
            result.reason = "Conflict"
            return
        if not statuses:
            result.reason = "Missing"
            return
        status = statuses[0]
        reason = self._freshness_reason(
            status.value, node.owner, rule.expected_tag, [rule.max_age_ms]
        )
        if reason is not None:
            result.reason = reason
            return
        if status.unsupported:
            result.reason = "Unsupported"
        else:
            result.usable, result.healthy = status.value.usable, status.value.healthy
        result.message = status.value.message

    def _threshold(
        self, result: HealthNodeResult, node: HealthNode, rule: HealthRule_Threshold
    ) -> None:
        metric = rule.metric
        definitions = self.metrics.get(metric, [])
        samples = self.samples.get(metric, [])
        if not definitions:
            result.reason = "Missing"
            return
        definition = definitions[0].value
        conflicts: list[tuple[DiagnosticKind, str]] = []
        if len(definitions) > 1:
            conflicts.append(("MetricDefinition", f"several definitions of {metric!r}"))
        if len(samples) > 1:
            conflicts.append(("MetricSample", f"several samples of {metric!r}"))
        if rule.unit is not None and rule.unit != definition.unit:
            conflicts.append(
                ("HealthNode", f"unit of {node.id!r} differs from {metric!r}")
            )
        if conflicts:
            for kind, message in conflicts:
                self._diagnose("Warning", kind, message)
            result.reason = "Conflict"
            return
        if not samples:
            result.reason = "Missing"
            return
        sample = samples[0].value
        reason = self._freshness_reason(
            sample,
            definition.owner,
            definition.expected_tag,
            [definition.max_age_ms, rule.max_age_ms],
        )
        if reason is not None:
            result.reason = reason
            return
        if _contains(rule.healthy, sample.value):
            result.usable, result.healthy = "Yes", "Yes"
        elif _contains(rule.usable, sample.value):
            result.usable, result.healthy = "Yes", "No"
        else:
            result.usable, result.healthy = "No", "No"
        result.message = sample.message

    def _aggregate(self, result: HealthNodeResult, rule: HealthRule_Aggregate) -> None:
        inputs = self.inputs[result.id]
        answers = {child: self._answers(child) for child, _ in inputs}
        counted = [answers[child] for child, mode in inputs if mode != "Optional"]
        mandatory = [answers[child] for child, mode in inputs if mode == "Mandatory"]
        optional = [answers[child] for child, mode in inputs if mode == "Optional"]
        min_healthy = rule.min_healthy if rule.min_healthy is not None else len(counted)
        min_usable = rule.min_usable if rule.min_usable is not None else min_healthy
        usable = _conjunction(
            [
                _count_answer(min_usable, [usable for usable, _ in counted]),
                *(usable for usable, _ in mandatory),
            ]
        )
        healthy = _conjunction(
            [
                usable,
                _count_answer(min_healthy, [healthy for _, healthy in counted]),
                *(healthy for _, healthy in mandatory),
                *(healthy for _, healthy in optional),
            ]
        )
        result.usable, result.healthy = usable, healthy
