/** Send records to another system as snapshots. */

import { Diagnostic } from "./generated/common.js";
import { HealthClockSample } from "./generated/health.js";
import {
  KINDS, type JsonObject, type Records, SPEC_VERSION, SUPPORTED_MAJOR, isObject, parseSpecVersion,
} from "./records.js";

const HEALTH = ["healthNodes", "healthStatuses", "metrics", "metricSamples"] as const;
const TIMESTAMP = /^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:([0-9]{2})(\.[0-9]+)?Z$/;

/** A received snapshot. Omitted lists are empty in `records`. */
export type Snapshot = {
  records: Required<Records>;
  clock: HealthClockSample;
  /** Whether the snapshot includes the records of the components part. */
  components: boolean;
  /** Whether the snapshot includes the records of the health part. */
  health: boolean;
};

/** Create a snapshot with the original records and their diagnostics. Take the clock sample after reading the records. */
export function makeSnapshot(
  records: Records, clock: HealthClockSample, options: { components?: boolean; health?: boolean } = {},
): JsonObject {
  const { components = true, health = true } = options;
  const snapshot: JsonObject = { specVersion: SPEC_VERSION, clock };
  if (components) snapshot.components = [...(records.components ?? [])];
  if (health) for (const field of HEALTH) snapshot[field] = [...(records[field] ?? [])];
  const diagnostics = (records.diagnostics ?? []).filter((d) =>
    d.kind === "DirectoryConfig" || (d.kind === "ComponentDeclaration" ? components : health));
  if (diagnostics.length > 0) snapshot.diagnostics = diagnostics;
  return snapshot;
}

/** Read a snapshot, applying the fallbacks to its diagnostics. Throw for snapshots of an unsupported version or with an invalid structure. */
export function readSnapshot(value: JsonObject): Snapshot {
  const version = parseSpecVersion(value.specVersion);
  if (version === undefined || version[0] !== SUPPORTED_MAJOR) {
    throw new Error(`unsupported snapshot version ${JSON.stringify(value.specVersion)}`);
  }
  const clock = value.clock as JsonObject | undefined;
  const timestamp = TIMESTAMP.exec(typeof clock?.timestamp === "string" ? clock.timestamp : "");
  if (timestamp === null || timestamp[1] === "60") {
    throw new Error(`invalid clock timestamp ${JSON.stringify(clock?.timestamp)}`);
  }
  const check = HealthClockSample.check(clock);
  if (check.isErr()) throw new Error(check.toString());
  const present = HEALTH.map((field) => value[field] !== undefined);
  if (present.some(Boolean) && !present.every(Boolean)) {
    throw new Error("snapshot includes only some lists of the health part");
  }
  const list = (field: string): unknown[] => {
    if (value[field] === undefined) return [];
    if (!Array.isArray(value[field])) throw new Error(`${field} is not a list`);
    return [...value[field]];
  };
  const diagnostics = list("diagnostics").map((item): Diagnostic => {
    if (!isObject(item)) throw new Error("diagnostic is not an object");
    const { kind, severity } = item;
    const diagnostic = {
      ...item,
      kind: KINDS.includes(kind as Diagnostic["kind"]) ? kind : "DirectoryConfig",
      severity: severity === "Error" || severity === "Warning" ? severity : "Error",
    };
    const report = Diagnostic.check(diagnostic);
    if (report.isErr()) throw new Error(report.toString());
    return diagnostic as Diagnostic;
  });
  return {
    records: {
      components: list("components"),
      healthNodes: list("healthNodes"),
      healthStatuses: list("healthStatuses"),
      metrics: list("metrics"),
      metricSamples: list("metricSamples"),
      diagnostics,
    },
    clock: clock as HealthClockSample,
    components: value.components !== undefined,
    health: present.every(Boolean),
  };
}
