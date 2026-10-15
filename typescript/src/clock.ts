/** Correlate health observation times with a system's wall clock for display. */

import { builtins } from "@sidex/types";
import { HealthStatus, HealthClockSample } from "./generated/health.js";

// Use Python datetime's representable years for consistent SDK display.
const minTimestampMs = -62135596800000n;
const maxTimestampMs = 253402300799999n;

/** Estimate UTC at millisecond precision; freshness still uses the boot-time clock. */
export function statusTimestamp(
  status: HealthStatus, clock: HealthClockSample,
): builtins.Timestamp | undefined {
  const statusCheck = HealthStatus.check(status);
  if (statusCheck.isErr()) throw new Error(statusCheck.toString());
  const clockCheck = HealthClockSample.check(clock);
  if (clockCheck.isErr()) throw new Error(clockCheck.toString());
  if (status.bootId !== clock.context.bootId) return undefined;
  const age = BigInt(clock.context.nowMs) - BigInt(status.observedAtMs);
  if (age < 0n) return undefined;
  const wallMillis = builtins.Timestamp.toDate(clock.timestamp).getTime();
  if (!Number.isFinite(wallMillis)) return undefined;
  const millis = BigInt(wallMillis) - age;
  if (millis < minTimestampMs || millis > maxTimestampMs) return undefined;
  return builtins.Timestamp.fromDate(new Date(Number(millis)));
}
