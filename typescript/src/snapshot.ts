/** Select contributions by their ownership owner before evaluating a candidate. */

import type { RuntimeSnapshot } from "./generated/filesystem.js";

/** Remove an exact owner from all collections, preserving other constraints. */
export function withoutOwner(snapshot: RuntimeSnapshot, owner: string): RuntimeSnapshot {
  return {
    ...(snapshot.metrics === undefined ? {} : { metrics: snapshot.metrics.filter((r) => r.owner !== owner) }),
    ...(snapshot.metricSamples === undefined ? {} : { metricSamples: snapshot.metricSamples.filter((r) => r.owner !== owner) }),
    components: snapshot.components.filter((record) => record.owner !== owner),
    healthNodes: snapshot.healthNodes.filter((record) => record.owner !== owner),
    healthStatuses: snapshot.healthStatuses.filter((record) => record.owner !== owner),
  };
}
