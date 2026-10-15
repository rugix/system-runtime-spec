/** Select contributions by their ownership origin before evaluating a candidate. */

import type { RuntimeSnapshot } from "./generated/filesystem.js";

/** Remove an exact origin from all collections, preserving other constraints. */
export function withoutOrigin(snapshot: RuntimeSnapshot, origin: string): RuntimeSnapshot {
  return {
    components: snapshot.components.filter((record) => record.origin !== origin),
    healthNodes: snapshot.healthNodes.filter((record) => record.origin !== origin),
    healthStatuses: snapshot.healthStatuses.filter((record) => record.origin !== origin),
  };
}
