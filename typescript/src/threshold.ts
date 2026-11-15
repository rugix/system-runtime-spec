/** Validate numeric evidence and derive health from declared ranges. */

import type { builtins } from "@sidex/types";

import type { HealthContext, HealthNode, HealthNodeResult, ThresholdRule } from "./generated/health.js";
import { MetricDefinition, MetricSample } from "./generated/metric.js";
import type { MetricId, MetricRange } from "./generated/metric.js";

/** Validated definitions and measurements shared by threshold nodes. */
export class MetricEvidence {
  private definitions = new Map<MetricId, MetricDefinition>();
  private observations = new Map<MetricId, MetricSample>();

  constructor(definitions: readonly MetricDefinition[], observations: readonly MetricSample[]) {
    for (const definition of definitions) {
      const check = MetricDefinition.check(definition);
      if (check.isErr()) throw new Error(check.toString());
      if (this.definitions.has(definition.id)) throw new Error("Duplicate metric ID");
      if (definition.maxAgeMs === 0) throw new Error("Value has zero maxAgeMs");
      this.definitions.set(definition.id, definition);
    }
    for (const observation of observations) {
      const check = MetricSample.check(observation);
      if (check.isErr()) throw new Error(check.toString());
      if (this.observations.has(observation.id)) throw new Error("Duplicate metric sample ID");
      if (observation.maxAgeMs === 0) throw new Error("Metric sample has zero maxAgeMs");
      number(observation.value);
      this.observations.set(observation.id, observation);
    }
  }

  /** Require a metric definition and nested, nonempty ranges. */
  validate(rule: ThresholdRule): void {
    if (!this.definitions.has(rule.metric)) throw new Error(`Missing metric ${rule.metric}`);
    if (rule.maxAgeMs === 0) throw new Error("Threshold has zero maxAgeMs");
    for (const range of [rule.healthy, rule.usable]) {
      for (const bound of [range.lower, range.upper]) {
        if (bound !== undefined) number(bound.value);
      }
      if (range.lower !== undefined && range.upper !== undefined) {
        const lower = number(range.lower.value), upper = number(range.upper.value);
        if (lower > upper || (lower === upper && !(range.lower.inclusive && range.upper.inclusive))) {
          throw new Error("Empty or reversed value range");
        }
      }
    }
    for (const [inner, outer, lower] of [
      [rule.healthy.lower, rule.usable.lower, true],
      [rule.healthy.upper, rule.usable.upper, false],
    ] as const) {
      if (outer !== undefined) {
        if (inner === undefined) throw new Error("Healthy range is outside usable range");
        const a = number(inner.value), b = number(outer.value);
        if ((lower && a < b) || (!lower && a > b) || (a === b && inner.inclusive && !outer.inclusive)) {
          throw new Error("Healthy range is outside usable range");
        }
      }
    }
  }

  /** Check value identity and freshness before applying the policy. */
  evaluate(node: HealthNode, rule: ThresholdRule, context: HealthContext): HealthNodeResult {
    const definition = this.definitions.get(rule.metric)!;
    const sample = this.observations.get(rule.metric);
    let reason: string | undefined;
    if (!sample) reason = "Missing metric sample";
    else if (sample.bootId !== context.bootId) reason = "Metric sample belongs to another boot";
    else if (definition.owner !== undefined && sample.owner !== definition.owner) reason = "Metric sample owner does not match";
    else if (definition.expectedTag !== undefined && sample.tag !== definition.expectedTag) reason = "Metric sample tag does not match";
    else if (BigInt(sample.observedAtMs) > BigInt(context.nowMs)) reason = "Metric sample is in the future";
    else if (BigInt(context.nowMs) - BigInt(sample.observedAtMs) >= BigInt(Math.min(
      sample.maxAgeMs, definition.maxAgeMs ?? sample.maxAgeMs, rule.maxAgeMs ?? sample.maxAgeMs,
    ))) reason = "Metric sample has expired";
    if (reason !== undefined) return {
      id: node.id, state: "unknown", inputs: [], message: reason,
    };
    const value = sample!.value;
    const state = contains(rule.healthy, value) ? "healthy"
      : contains(rule.usable, value) ? "degraded" : "unhealthy";
    return {
      id: node.id, state, inputs: [],
      ...(sample!.message === undefined ? {} : { message: sample!.message }),
    };
  }
}

/** Reject non-finite measurements and endpoints before comparing them. */
function number(value: builtins.F64): number {
  if (typeof value !== "number" || !Number.isFinite(value)) {
    throw new Error("Numeric value must be finite");
  }
  return value;
}

/** Compare both endpoints, including their declared equality behavior. */
function contains(range: MetricRange, value: builtins.F64): boolean {
  const measured = number(value);
  return (range.lower === undefined || measured > number(range.lower.value)
      || (measured === number(range.lower.value) && range.lower.inclusive))
    && (range.upper === undefined || measured < number(range.upper.value)
      || (measured === number(range.upper.value) && range.upper.inclusive));
}
