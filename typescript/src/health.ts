/** Evaluate health in a health context. */

import type { Diagnostic, RecordKind, Severity } from "./generated/common.js";
import type {
  Answer, HealthContext, HealthNodeId, HealthNodeResult, HealthReport, InputMode, Reason,
} from "./generated/health.js";
import { type Decoded, type Range, type JsonObject, type Records, decodeAll, diagnostic } from "./records.js";

type Answers = [Answer, Answer];

const UNKNOWN: Answers = ["Unknown", "Unknown"];
const STATE_ANSWERS: Record<string, Answers> = {
  Healthy: ["Yes", "Yes"],
  Degraded: ["Yes", "No"],
  Unhealthy: ["No", "No"],
  Unknown: UNKNOWN,
};

function conjunction(answers: Answer[]): Answer {
  if (answers.includes("No")) return "No";
  if (answers.includes("Unknown")) return "Unknown";
  return "Yes";
}

function count(minimum: bigint, answers: Answer[]): Answer {
  const yes = BigInt(answers.filter((a) => a === "Yes").length);
  const unknown = BigInt(answers.filter((a) => a === "Unknown").length);
  if (yes >= minimum) return "Yes";
  if (yes + unknown < minimum) return "No";
  return "Unknown";
}

function contains(range: Range, value: number): boolean {
  const { lower, upper } = range;
  return (lower === undefined || value > lower.value || (value === lower.value && lower.inclusive))
    && (upper === undefined || value < upper.value || (value === upper.value && upper.inclusive));
}

function group(items: Decoded[]): Map<string, Decoded[]> {
  const groups = new Map<string, Decoded[]>();
  for (const item of items) {
    const id = item.record.id as string;
    groups.set(id, [...(groups.get(id) ?? []), item]);
  }
  return groups;
}

function int(value: unknown): bigint {
  return BigInt(value as string | number);
}

/**
 * Evaluate the health records of `records` in a health context.
 *
 * Invalid records are skipped with an error diagnostic. The report contains one result per node
 * ID, in the order of the node definitions, and all diagnostics.
 */
export function evaluateHealth(records: Records, context: HealthContext): HealthReport {
  return new Evaluation(records, context).report();
}

class Evaluation {
  private readonly diagnostics: Diagnostic[];
  private readonly reported = new Set<string>();
  private readonly order: string[] = [];
  private readonly nodes = new Map<string, Decoded>();
  private readonly conflicts = new Set<string>();
  private readonly statuses: Map<string, Decoded[]>;
  private readonly metrics: Map<string, Decoded[]>;
  private readonly samples: Map<string, Decoded[]>;
  private readonly inputs = new Map<string, [string, InputMode][]>();
  private readonly unsupported = new Set<string>();
  private readonly onCycle: Set<string>;
  private readonly results = new Map<string, HealthNodeResult>();

  constructor(records: Records, private readonly context: HealthContext) {
    this.diagnostics = [...(records.diagnostics ?? [])];
    const nodes = decodeAll("HealthNode", records.healthNodes, this.diagnostics);
    this.statuses = group(decodeAll("HealthStatus", records.healthStatuses, this.diagnostics));
    this.metrics = group(decodeAll("MetricDefinition", records.metrics, this.diagnostics));
    this.samples = group(decodeAll("MetricSample", records.metricSamples, this.diagnostics));
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

  private aggregateRule(id: string): JsonObject | undefined {
    const rule = this.nodes.get(id)?.record.rule as JsonObject | undefined;
    return rule?.type === "Aggregate" ? rule : undefined;
  }

  private resolveInputs(): void {
    const attached = new Map<string, [string, InputMode][]>();
    for (const [id, node] of this.nodes) {
      for (const attachment of (node.record.partOf as JsonObject[] | undefined) ?? []) {
        const parent = attachment.node as string;
        if (this.aggregateRule(parent) === undefined) continue;
        if (node.unsupportedAttachments.has(parent)) this.unsupported.add(parent);
        attached.set(parent, [...(attached.get(parent) ?? []), [id, (attachment.mode as InputMode) ?? "Counted"]]);
      }
    }
    for (const id of this.nodes.keys()) {
      const rule = this.aggregateRule(id);
      if (rule === undefined) continue;
      const inputs = new Map<string, InputMode>();
      for (const entry of (rule.inputs as JsonObject[] | undefined) ?? []) {
        const child = entry.node as string;
        if (!this.nodes.has(child) && !this.conflicts.has(child)) {
          this.diagnose("Warning", "HealthNode", `input ${JSON.stringify(child)} of ${JSON.stringify(id)} does not exist`);
        }
        if (!inputs.has(child)) inputs.set(child, (entry.mode as InputMode) ?? "Counted");
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
    const node = (this.nodes.get(id) as Decoded).record;
    const result: HealthNodeResult = { id: id as HealthNodeId, usable: "Unknown", healthy: "Unknown", inputs: [] };
    if (node.owner !== undefined) result.owner = node.owner as string;
    result.inputs = (this.inputs.get(id) ?? []).map(([child]) => child as HealthNodeId);
    const rule = (node.rule as JsonObject | undefined) ?? { type: "Observed" };
    if (this.onCycle.has(id)) result.reason = "Cycle";
    else if (this.unsupported.has(id)) result.reason = "Unsupported";
    else if (rule.type === "Observed") this.observe(result, node, rule);
    else if (rule.type === "Threshold") this.threshold(result, node, rule);
    else this.aggregate(result, rule);
    this.results.set(id, result);
    return result;
  }

  /** Return why evidence is not fresh, or undefined if it is fresh. */
  private fresh(
    evidence: JsonObject, owner: unknown, tag: unknown, maxAges: unknown[],
  ): Reason | undefined {
    if (evidence.bootId !== this.context.bootId) return "OtherBoot";
    if (owner !== undefined && evidence.owner !== owner) return "OwnerMismatch";
    if (tag !== undefined && evidence.tag !== tag) return "TagMismatch";
    const now = int(this.context.nowMs);
    const observed = int(evidence.observedAtMs);
    if (observed > now) return "Future";
    const limits = [evidence.maxAgeMs, ...maxAges.filter((age) => age !== undefined)].map(int);
    if (limits.some((limit) => now - observed >= limit)) return "Expired";
    return undefined;
  }

  private carry(result: HealthNodeResult, evidence: JsonObject): void {
    if (evidence.message !== undefined) result.message = evidence.message as string;
    if (evidence.details !== undefined) result.details = evidence.details as HealthNodeResult["details"];
  }

  private observe(result: HealthNodeResult, node: JsonObject, rule: JsonObject): void {
    const id = node.id as string;
    const statuses = this.statuses.get(id) ?? [];
    if (statuses.length > 1) {
      this.diagnose("Warning", "HealthStatus", `several status records for ${JSON.stringify(id)}`);
      result.reason = "Conflict";
      return;
    }
    if (statuses.length === 0) {
      result.reason = "Missing";
      return;
    }
    const [status] = statuses;
    const reason = this.fresh(status.record, node.owner, rule.expectedTag, [rule.maxAgeMs]);
    if (reason !== undefined) {
      result.reason = reason;
      return;
    }
    if (status.unsupported) result.reason = "Unsupported";
    else [result.usable, result.healthy] = STATE_ANSWERS[status.record.state as string];
    this.carry(result, status.record);
  }

  private threshold(result: HealthNodeResult, node: JsonObject, rule: JsonObject): void {
    const metric = rule.metric as string;
    const definitions = this.metrics.get(metric) ?? [];
    const samples = this.samples.get(metric) ?? [];
    if (definitions.length === 0) {
      result.reason = "Missing";
      return;
    }
    const definition = definitions[0].record;
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
    const sample = samples[0].record;
    const reason = this.fresh(sample, definition.owner, definition.expectedTag, [definition.maxAgeMs, rule.maxAgeMs]);
    if (reason !== undefined) {
      result.reason = reason;
      return;
    }
    const value = sample.value as number;
    [result.usable, result.healthy] = contains(rule.healthy as Range, value)
      ? ["Yes", "Yes"]
      : contains(rule.usable as Range, value) ? ["Yes", "No"] : ["No", "No"];
    this.carry(result, sample);
  }

  private aggregate(result: HealthNodeResult, rule: JsonObject): void {
    const inputs = (this.inputs.get(result.id) as [string, InputMode][]).map(([child, mode]): [InputMode, Answers] => [mode, this.answers(child)]);
    const counted = inputs.filter(([mode]) => mode !== "Optional").map(([, answers]) => answers);
    const withMode = (wanted: InputMode) => inputs.filter(([mode]) => mode === wanted).map(([, answers]) => answers);
    const minHealthy = rule.minHealthy === undefined ? BigInt(counted.length) : int(rule.minHealthy);
    const minUsable = rule.minUsable === undefined ? minHealthy : int(rule.minUsable);
    const usable = conjunction([
      count(minUsable, counted.map(([u]) => u)),
      ...withMode("Mandatory").map(([u]) => u),
    ]);
    const healthy = conjunction([
      usable,
      count(minHealthy, counted.map(([, h]) => h)),
      ...withMode("Mandatory").map(([, h]) => h),
      ...withMode("Optional").map(([, h]) => h),
    ]);
    [result.usable, result.healthy] = [usable, healthy];
  }
}
