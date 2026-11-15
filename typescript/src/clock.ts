import { builtins } from "@sidex/types";
import type { HealthClockSample, HealthStatus } from "./generated/health.js";
import type { MetricSample } from "./generated/metric.js";

// Unix milliseconds of the first and the last instant of the years 0001 to 9999.
const MIN_TIMESTAMP_MS = -62135596800000n;
const MAX_TIMESTAMP_MS = 253402300799999n;

/**
 * Estimate the wall-clock time at which evidence was observed, with millisecond precision.
 *
 * Return undefined for evidence from another boot, evidence observed after the clock sample, and
 * estimates outside the years 0001 to 9999.
 */
export function estimateTimestamp(
  clock: HealthClockSample, evidence: HealthStatus | MetricSample,
): builtins.Timestamp | undefined {
  if (evidence.bootId !== clock.context.bootId) return undefined;
  const age = builtins.U64.toBigInt(clock.context.nowMs) - builtins.U64.toBigInt(evidence.observedAtMs);
  if (age < 0n) return undefined;
  const wallMillis = builtins.Timestamp.toDate(clock.timestamp).getTime();
  if (!Number.isFinite(wallMillis)) return undefined;
  const millis = BigInt(wallMillis) - age;
  if (millis < MIN_TIMESTAMP_MS || millis > MAX_TIMESTAMP_MS) return undefined;
  return builtins.Timestamp.fromDate(new Date(Number(millis)));
}
