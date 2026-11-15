/** Correlate the boot-time clock with wall-clock time. */

import { builtins } from "@sidex/types";
import type { HealthClockSample } from "./generated/health.js";
import type { JsonObject } from "./records.js";

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
  evidence: JsonObject, clock: HealthClockSample,
): builtins.Timestamp | undefined {
  if (evidence.bootId !== clock.context.bootId) return undefined;
  const age = BigInt(clock.context.nowMs) - BigInt(evidence.observedAtMs as string | number);
  if (age < 0n) return undefined;
  const wallMillis = builtins.Timestamp.toDate(clock.timestamp).getTime();
  if (!Number.isFinite(wallMillis)) return undefined;
  const millis = BigInt(wallMillis) - age;
  if (millis < MIN_TIMESTAMP_MS || millis > MAX_TIMESTAMP_MS) return undefined;
  return builtins.Timestamp.fromDate(new Date(Number(millis)));
}
