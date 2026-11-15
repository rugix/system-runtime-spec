/** Evaluate health graphs using caller-supplied boot-time evidence. */

import { ValueEvidence } from "./threshold.js";
import type { ValueDefinition, ValueObservation } from "./generated/value.js";

import { HealthStatus, HealthContext, HealthNode } from "./generated/health.js";
import type {
  AggregationRule, HealthNodeId, HealthNodeResult, HealthReport, HealthState, ObservationRule,
} from "./generated/health.js";

/** Validate the graph, then evaluate every node in declaration order. */
export function evaluateHealth(
  nodes: readonly HealthNode[], statuses: readonly HealthStatus[], context: HealthContext,
  values: readonly ValueDefinition[] = [], valueObservations: readonly ValueObservation[] = [],
): HealthReport {
  const evidence = new ValueEvidence(values, valueObservations);
  const clockCheck = HealthContext.check(context);
  if (clockCheck.isErr()) throw new Error(clockCheck.toString());
  const byId = new Map<HealthNodeId, HealthNode>();
  for (const node of nodes) {
    const validation = HealthNode.check(node);
    if (validation.isErr()) throw new Error(validation.toString());
    if (byId.has(node.id)) throw new Error("Duplicate health node ID");
    byId.set(node.id, node);
  }
  const observations = new Map<HealthNodeId, HealthStatus>();
  for (const status of statuses) {
    const validation = HealthStatus.check(status);
    if (validation.isErr()) throw new Error(validation.toString());
    if (observations.has(status.id)) throw new Error("Duplicate health status ID");
    if (!status.maxAgeMs) throw new Error(`Status ${status.id} has zero maxAgeMs`);
    const node = byId.get(status.id);
    if (node !== undefined && node.rule.rule !== "observed") {
      throw new Error(`Status supplied for derived node ${status.id}`);
    }
    observations.set(status.id, status);
  }
  const remaining = new Map<HealthNodeId, number>();
  const dependents = new Map(nodes.map((node) => [node.id, [] as HealthNodeId[]]));
  for (const node of nodes) {
    const rule = node.rule;
    if (rule.rule === "observed") {
      if (rule.maxAgeMs === 0) throw new Error(`Node ${node.id} has zero maxAgeMs`);
      remaining.set(node.id, 0);
    } else if (rule.rule === "threshold") {
      evidence.validate(rule);
      remaining.set(node.id, 0);
    } else {
      const [required, healthy] = counts(rule);
      if (!(1 <= required && required <= healthy && healthy <= rule.inputs.length)) {
        throw new Error(`Invalid thresholds for ${node.id}`);
      }
      if (new Set(rule.inputs.map((input) => input.node)).size !== rule.inputs.length) throw new Error(`Duplicate input for ${node.id}`);
      remaining.set(node.id, rule.inputs.length);
      for (const input of rule.inputs) {
        if (!byId.has(input.node)) throw new Error(`Missing health node ${input.node}`);
        dependents.get(input.node)!.push(node.id);
      }
    }
  }
  const ready = [...remaining].filter(([, count]) => count === 0).map(([id]) => id);
  const results = new Map<HealthNodeId, HealthNodeResult>();
  for (let position = 0; position < ready.length; position++) {
    const id = ready[position];
    const node = byId.get(id)!;
    let result: HealthNodeResult;
    if (node.rule.rule === "observed") {
      result = observe(id, node.rule, observations.get(id), context, node.origin);
    } else if (node.rule.rule === "threshold") {
      result = evidence.evaluate(node, node.rule, context);
    } else {
      result = aggregate(id, node.rule, node.rule.inputs.map((input) => results.get(input.node)!));
    }
    if (node.origin !== undefined) result.origin = node.origin;
    results.set(id, result);
    for (const dependent of dependents.get(id)!) {
      const count = remaining.get(dependent)! - 1;
      remaining.set(dependent, count);
      if (count === 0) ready.push(dependent);
    }
  }
  if (results.size !== nodes.length) throw new Error("Cycle in health graph");
  return { nodes: nodes.map((node) => results.get(node.id)!) };
}

/** Apply freshness and identity constraints to one status. */
function observe(
  id: HealthNodeId, rule: ObservationRule, status: HealthStatus | undefined, context: HealthContext,
  origin: string | undefined,
): HealthNodeResult {
  let reason: string | undefined;
  if (!status) reason = "Missing status";
  else if (status.bootId !== context.bootId) reason = "Status belongs to another boot";
  else if (origin !== undefined && status.origin !== origin) reason = "Status origin does not match";
  else if (rule.expectedTag !== undefined && status.tag !== rule.expectedTag) reason = "Status tag does not match";
  else if (BigInt(status.observedAtMs) > BigInt(context.nowMs)) reason = "Status is in the future";
  else if (BigInt(context.nowMs) - BigInt(status.observedAtMs) >= BigInt(
    Math.min(status.maxAgeMs, rule.maxAgeMs ?? status.maxAgeMs),
  )) reason = "Status has expired";
  if (reason !== undefined) return { id, state: "unknown", inputs: [], message: reason };
  const observation = status!;
  return {
    id, state: observation.state, inputs: [],
    ...(observation.message === undefined ? {} : { message: observation.message }),
  };
}

/** Resolve the minimum and desired counts, defaulting to all inputs healthy. */
function counts(rule: AggregationRule): [number, number] {
  const healthy = rule.healthy ?? rule.inputs.length;
  return [rule.required ?? healthy, healthy];
}

/** Apply mandatory dependency gates and desired and minimum input counts. */
function aggregate(id: HealthNodeId, rule: AggregationRule, inputs: HealthNodeResult[]): HealthNodeResult {
  const [required, desired] = counts(rule);
  const healthy = inputs.filter((input) => input.state === "healthy").length;
  const minimumMet = inputs.filter((input) => input.state === "healthy" || input.state === "degraded").length;
  const possibleMinimum = inputs.filter((input) => input.state !== "unhealthy").length;
  const mandatory = inputs.filter((_, index) => rule.inputs[index].required === true);
  const state: HealthState = possibleMinimum < required || mandatory.some((input) => input.state === "unhealthy")
    ? "unhealthy"
    : minimumMet < required || mandatory.some((input) => input.state === "unknown")
      ? "unknown"
      : healthy >= desired && mandatory.every((input) => input.state === "healthy")
        ? "healthy" : "degraded";
  return { id, state, inputs: rule.inputs.map((input) => input.node) };
}
