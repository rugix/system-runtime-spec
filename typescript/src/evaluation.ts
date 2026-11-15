import { builtins } from "@sidex/types";
import type { Diagnostic, RecordKind, Severity } from "./generated/common.js";
import type {
  AggregationRule, Answer, HealthContext, HealthNode, HealthNodeId, HealthNodeResult, HealthReport, HealthState,
  HealthStatus, InputMode, ObservationRule, Reason, ThresholdRule,
} from "./generated/health.js";
import type { MetricDefinition, MetricRange, MetricSample } from "./generated/metric.js";
import { type Record, diagnostic } from "./decoding.js";

type Answers = [Answer, Answer];

const UNKNOWN: Answers = ["Unknown", "Unknown"];
const STATE_ANSWERS: { [S in HealthState]: Answers } = {
  Healthy: ["Yes", "Yes"],
  Degraded: ["Yes", "No"],
  Unhealthy: ["No", "No"],
  Unknown: UNKNOWN,
};
const { U64, F64 } = builtins;

function conjunction(answers: Answer[]): Answer {
  if (answers.includes("No")) return "No";
  if (answers.includes("Unknown")) return "Unknown";
  return "Yes";
}

function countAnswer(minimum: bigint, answers: Answer[]): Answer {
  const yes = BigInt(answers.filter((a) => a === "Yes").length);
  const unknown = BigInt(answers.filter((a) => a === "Unknown").length);
  if (yes >= minimum) return "Yes";
  if (yes + unknown < minimum) return "No";
  return "Unknown";
}

function contains(range: MetricRange, value: number): boolean {
  const { lower, upper } = range;
  const bound = (b: { value: builtins.F64 }) => F64.toNumber(b.value);
  return (lower === undefined || value > bound(lower) || (value === bound(lower) && lower.inclusive))
    && (upper === undefined || value < bound(upper) || (value === bound(upper) && upper.inclusive));
}

function group<T extends { id: string }>(records: readonly Record<T>[]): Map<string, Record<T>[]> {
  const groups = new Map<string, Record<T>[]>();
  for (const record of records) {
    groups.set(record.value.id, [...(groups.get(record.value.id) ?? []), record]);
  }
  return groups;
}

export function evaluate(
  nodes: readonly Record<HealthNode>[],
  statuses: readonly Record<HealthStatus>[],
  metrics: readonly Record<MetricDefinition>[],
  samples: readonly Record<MetricSample>[],
  diagnostics: readonly Diagnostic[],
  context: HealthContext,
): HealthReport {
  return new Evaluation(nodes, statuses, metrics, samples, diagnostics, context).report();
}

class Evaluation {
  private readonly diagnostics: Diagnostic[];
  private readonly reported = new Set<string>();
  private readonly order: string[] = [];
  private readonly nodes = new Map<string, Record<HealthNode>>();
  private readonly conflicts = new Set<string>();
  private readonly statuses: Map<string, Record<HealthStatus>[]>;
  private readonly metrics: Map<string, Record<MetricDefinition>[]>;
  private readonly samples: Map<string, Record<MetricSample>[]>;
  private readonly inputs = new Map<string, [string, InputMode][]>();
  private readonly unsupported = new Set<string>();
  private readonly onCycle: Set<string>;
  private readonly results = new Map<string, HealthNodeResult>();

  constructor(
    nodes: readonly Record<HealthNode>[],
    statuses: readonly Record<HealthStatus>[],
    metrics: readonly Record<MetricDefinition>[],
    samples: readonly Record<MetricSample>[],
    diagnostics: readonly Diagnostic[],
    private readonly context: HealthContext,
  ) {
    this.diagnostics = [...diagnostics];
    this.statuses = group(statuses);
    this.metrics = group(metrics);
    this.samples = group(samples);
    for (const [id, nodesWithId] of group(nodes)) {
      this.order.push(id);
      if (nodesWithId.length > 1) {
        this.diagnose("Error", "HealthNode", `several nodes have the ID ${JSON.stringify(id)}`);
        this.conflicts.add(id);
      } else {
        this.nodes.set(id, nodesWithId[0]);
        if (nodesWithId[0].unsupported) this.unsupported.add(id);
      }
    }
    this.resolveInputs();
    this.onCycle = this.findCycles();
  }

  private diagnose(severity: Severity, kind: RecordKind, message: string): void {
    if (this.reported.has(message)) return;
    this.reported.add(message);
    this.diagnostics.push(diagnostic(severity, kind, message));
  }

  private aggregateRule(id: string): AggregationRule | undefined {
    const rule = this.nodes.get(id)?.value.rule;
    return rule?.type === "Aggregate" ? rule : undefined;
  }

  private resolveInputs(): void {
    const attached = new Map<string, [string, InputMode][]>();
    for (const [id, node] of this.nodes) {
      for (const attachment of node.value.partOf ?? []) {
        const parent = attachment.node;
        if (this.aggregateRule(parent) === undefined) continue;
        if (node.unsupportedAttachments.has(parent)) this.unsupported.add(parent);
        attached.set(parent, [...(attached.get(parent) ?? []), [id, attachment.mode ?? "Counted"]]);
      }
    }
    for (const id of this.nodes.keys()) {
      const rule = this.aggregateRule(id);
      if (rule === undefined) continue;
      const inputs = new Map<string, InputMode>();
      for (const entry of rule.inputs ?? []) {
        const child = entry.node;
        if (!this.nodes.has(child) && !this.conflicts.has(child)) {
          this.diagnose("Warning", "HealthNode", `input ${JSON.stringify(child)} of ${JSON.stringify(id)} does not exist`);
        }
        if (!inputs.has(child)) inputs.set(child, entry.mode ?? "Counted");
      }
      for (const [child, mode] of attached.get(id) ?? []) {
        if (!inputs.has(child)) inputs.set(child, mode);
      }
      this.inputs.set(id, [...inputs]);
    }
  }

  /** Find the nodes on a cycle with Tarjan's algorithm. */
  private findCycles(): Set<string> {
    const edges = (id: string) => (this.inputs.get(id) ?? []).map(([child]) => child).filter((c) => this.nodes.has(c));
    const index = new Map<string, number>();
    const low = new Map<string, number>();
    const stack: string[] = [];
    const onStack = new Set<string>();
    const onCycle = new Set<string>();
    const enter = (id: string) => {
      index.set(id, index.size);
      low.set(id, index.get(id) as number);
      stack.push(id);
      onStack.add(id);
    };
    for (const start of this.inputs.keys()) {
      if (index.has(start)) continue;
      const work: [string, number][] = [[start, 0]];
      enter(start);
      while (work.length > 0) {
        const [id, position] = work[work.length - 1];
        const children = edges(id);
        if (position < children.length) {
          work[work.length - 1] = [id, position + 1];
          const child = children[position];
          if (!index.has(child)) {
            enter(child);
            work.push([child, 0]);
          } else if (onStack.has(child)) {
            low.set(id, Math.min(low.get(id) as number, index.get(child) as number));
          }
          continue;
        }
        work.pop();
        if (work.length > 0) {
          const parent = work[work.length - 1][0];
          low.set(parent, Math.min(low.get(parent) as number, low.get(id) as number));
        }
        if (low.get(id) === index.get(id)) {
          const component: string[] = [];
          for (;;) {
            const member = stack.pop() as string;
            onStack.delete(member);
            component.push(member);
            if (member === id) break;
          }
          if (component.length > 1 || children.includes(id)) {
            for (const member of component) onCycle.add(member);
          }
        }
      }
    }
    return onCycle;
  }

  report(): HealthReport {
    const nodes = this.order.map((id): HealthNodeResult =>
      this.conflicts.has(id)
        ? { id: id as HealthNodeId, usable: "Unknown", healthy: "Unknown", inputs: [], reason: "Conflict" }
        : this.result(id));
    return { nodes, diagnostics: this.diagnostics };
  }

  private answers(id: string): Answers {
    if (!this.nodes.has(id)) return UNKNOWN;
    const result = this.result(id);
    return [result.usable, result.healthy];
  }

  private result(id: string): HealthNodeResult {
    const cached = this.results.get(id);
    if (cached !== undefined) return cached;
    const node = (this.nodes.get(id) as Record<HealthNode>).value;
    const result: HealthNodeResult = {
      id: node.id,
      ...(node.owner === undefined ? {} : { owner: node.owner }),
      usable: "Unknown",
      healthy: "Unknown",
      inputs: (this.inputs.get(id) ?? []).map(([child]) => child as HealthNodeId),
    };
    const rule = node.rule ?? { type: "Observed" };
    if (this.onCycle.has(id)) result.reason = "Cycle";
    else if (this.unsupported.has(id)) result.reason = "Unsupported";
    else if (rule.type === "Observed") this.observe(result, node, rule);
    else if (rule.type === "Threshold") this.threshold(result, node, rule);
    else this.aggregate(result, rule);
    this.results.set(id, result);
    return result;
  }

  /** Return why evidence is not fresh, or undefined if it is fresh. */
  private freshnessReason(
    evidence: HealthStatus | MetricSample,
    owner: string | undefined,
    tag: string | undefined,
    maxAges: (builtins.U64 | undefined)[],
  ): Reason | undefined {
    if (evidence.bootId !== this.context.bootId) return "OtherBoot";
    if (owner !== undefined && evidence.owner !== owner) return "OwnerMismatch";
    if (tag !== undefined && evidence.tag !== tag) return "TagMismatch";
    const now = U64.toBigInt(this.context.nowMs);
    const observed = U64.toBigInt(evidence.observedAtMs);
    if (observed > now) return "Future";
    const limits = [evidence.maxAgeMs, ...maxAges]
      .filter((age): age is builtins.U64 => age !== undefined)
      .map((age) => U64.toBigInt(age));
    if (limits.some((limit) => now - observed >= limit)) return "Expired";
    return undefined;
  }

  private carry(result: HealthNodeResult, evidence: HealthStatus | MetricSample): void {
    if (evidence.message !== undefined) result.message = evidence.message;
    if (evidence.details !== undefined) result.details = evidence.details;
  }

  private observe(result: HealthNodeResult, node: HealthNode, rule: ObservationRule): void {
    const statuses = this.statuses.get(node.id) ?? [];
    if (statuses.length > 1) {
      this.diagnose("Warning", "HealthStatus", `several status records for ${JSON.stringify(node.id)}`);
      result.reason = "Conflict";
      return;
    }
    if (statuses.length === 0) {
      result.reason = "Missing";
      return;
    }
    const [status] = statuses;
    const reason = this.freshnessReason(status.value, node.owner, rule.expectedTag, [rule.maxAgeMs]);
    if (reason !== undefined) {
      result.reason = reason;
      return;
    }
    if (status.unsupported) result.reason = "Unsupported";
    else [result.usable, result.healthy] = STATE_ANSWERS[status.value.state];
    this.carry(result, status.value);
  }

  private threshold(result: HealthNodeResult, node: HealthNode, rule: ThresholdRule): void {
    const metric = rule.metric;
    const definitions = this.metrics.get(metric) ?? [];
    const samples = this.samples.get(metric) ?? [];
    if (definitions.length === 0) {
      result.reason = "Missing";
      return;
    }
    const definition = definitions[0].value;
    const conflicts: [RecordKind, string][] = [];
    if (definitions.length > 1) conflicts.push(["MetricDefinition", `several definitions of ${JSON.stringify(metric)}`]);
    if (samples.length > 1) conflicts.push(["MetricSample", `several samples of ${JSON.stringify(metric)}`]);
    if (rule.unit !== undefined && rule.unit !== definition.unit) {
      conflicts.push(["HealthNode", `unit of ${JSON.stringify(node.id)} differs from ${JSON.stringify(metric)}`]);
    }
    if (conflicts.length > 0) {
      for (const [kind, message] of conflicts) this.diagnose("Warning", kind, message);
      result.reason = "Conflict";
      return;
    }
    if (samples.length === 0) {
      result.reason = "Missing";
      return;
    }
    const sample = samples[0].value;
    const reason = this.freshnessReason(sample, definition.owner, definition.expectedTag, [definition.maxAgeMs, rule.maxAgeMs]);
    if (reason !== undefined) {
      result.reason = reason;
      return;
    }
    const value = F64.toNumber(sample.value);
    [result.usable, result.healthy] = contains(rule.healthy, value)
      ? ["Yes", "Yes"]
      : contains(rule.usable, value) ? ["Yes", "No"] : ["No", "No"];
    this.carry(result, sample);
  }

  private aggregate(result: HealthNodeResult, rule: AggregationRule): void {
    const inputs = (this.inputs.get(result.id) as [string, InputMode][])
      .map(([child, mode]): [InputMode, Answers] => [mode, this.answers(child)]);
    const counted = inputs.filter(([mode]) => mode !== "Optional").map(([, answers]) => answers);
    const withMode = (wanted: InputMode) => inputs.filter(([mode]) => mode === wanted).map(([, answers]) => answers);
    const minHealthy = rule.minHealthy === undefined ? BigInt(counted.length) : U64.toBigInt(rule.minHealthy);
    const minUsable = rule.minUsable === undefined ? minHealthy : U64.toBigInt(rule.minUsable);
    const usable = conjunction([
      countAnswer(minUsable, counted.map(([u]) => u)),
      ...withMode("Mandatory").map(([u]) => u),
    ]);
    const healthy = conjunction([
      usable,
      countAnswer(minHealthy, counted.map(([, h]) => h)),
      ...withMode("Mandatory").map(([, h]) => h),
      ...withMode("Optional").map(([, h]) => h),
    ]);
    [result.usable, result.healthy] = [usable, healthy];
  }
}
