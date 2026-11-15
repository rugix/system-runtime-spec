/** Send records to another system as snapshots. */

import { KINDS, type RecordModelKind, SPEC_VERSION, SUPPORTED_MAJOR, isObject, parseSpecVersion } from "./decoding.js";
import { Diagnostic } from "./generated/common.js";
import type { RuntimeSnapshot } from "./generated/filesystem.js";
import { HealthClockSample } from "./generated/health.js";
import { Records } from "./records.js";

const LISTS: readonly [keyof RuntimeSnapshot, RecordModelKind][] = [
  ["components", "ComponentDeclaration"],
  ["healthNodes", "HealthNode"],
  ["healthStatuses", "HealthStatus"],
  ["metrics", "MetricDefinition"],
  ["metricSamples", "MetricSample"],
];
const HEALTH = ["healthNodes", "healthStatuses", "metrics", "metricSamples"] as const;
const TIMESTAMP = /^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:([0-9]{2})(\.[0-9]+)?Z$/;

/** Parts of the specification whose records a snapshot includes. */
export type Parts = {
  /** Whether the snapshot includes component declarations. */
  components: boolean;
  /** Whether the snapshot includes health nodes, status records, and metrics. */
  health: boolean;
};

/** Error thrown by `Snapshot.fromJSON` for a snapshot that cannot be read. */
export class SnapshotError extends Error {
  override name = "SnapshotError";
}

/**
 * Records of a system with a clock sample, for sending them to another system.
 *
 * A snapshot contains the original records, so that the receiving system evaluates them as the
 * sending system read them, including fields of newer minor versions. `JSON.stringify` encodes it
 * as a `RuntimeSnapshot`.
 */
export class Snapshot {
  /** Records of the snapshot; the lists of parts it does not include are empty. */
  readonly records: Records;
  /** Correlation of the boot-time clock with wall-clock time. */
  readonly clock: HealthClockSample;
  /** Parts whose records the snapshot includes. */
  readonly parts: Readonly<Parts>;

  /**
   * Create a snapshot of the records of the given parts and their diagnostics.
   *
   * Take the clock sample after reading the records.
   */
  constructor(records: Records, clock: HealthClockSample, parts: Partial<Parts> = {}) {
    this.parts = { components: parts.components ?? true, health: parts.health ?? true };
    this.records = records.only(this.parts);
    this.clock = clock;
  }

  /** Encode the snapshot as a `RuntimeSnapshot`. */
  toJSON(): RuntimeSnapshot {
    const snapshot: { [key: string]: unknown } = { specVersion: SPEC_VERSION, clock: this.clock };
    for (const [name, kind] of LISTS) {
      if (name === "components" ? this.parts.components : this.parts.health) {
        snapshot[name] = this.records.originals(kind);
      }
    }
    if (this.records.diagnostics.length > 0) snapshot.diagnostics = [...this.records.diagnostics];
    return snapshot as RuntimeSnapshot;
  }

  /**
   * Decode a `RuntimeSnapshot`, applying the fallbacks to its diagnostics.
   *
   * Throw a `SnapshotError` for snapshots of an unsupported version or with an invalid structure.
   * Invalid records are skipped with an error diagnostic, as when they are read from files.
   */
  static fromJSON(value: unknown): Snapshot {
    if (!isObject(value)) throw new SnapshotError("snapshot is not an object");
    const version = parseSpecVersion(value.specVersion);
    if (version === undefined || version[0] !== SUPPORTED_MAJOR) {
      throw new SnapshotError(`unsupported snapshot version ${JSON.stringify(value.specVersion)}`);
    }
    const clock = value.clock;
    const timestamp = isObject(clock) && typeof clock.timestamp === "string" ? TIMESTAMP.exec(clock.timestamp) : null;
    if (timestamp === null || timestamp[1] === "60") {
      throw new SnapshotError(`invalid clock timestamp ${JSON.stringify(isObject(clock) ? clock.timestamp : clock)}`);
    }
    const check = HealthClockSample.check(clock);
    if (check.isErr()) throw new SnapshotError(`invalid clock sample: ${check}`);
    const present = HEALTH.map((field) => value[field] !== undefined);
    if (present.some(Boolean) && !present.every(Boolean)) {
      throw new SnapshotError("snapshot includes only some lists of the health part");
    }
    const list = (field: string): unknown[] => {
      if (value[field] === undefined) return [];
      if (!Array.isArray(value[field])) throw new SnapshotError(`${field} is not a list`);
      return value[field];
    };
    const records = new Records();
    for (const item of list("diagnostics")) records.pushDiagnostic(readDiagnostic(item));
    for (const [name, kind] of LISTS) {
      for (const item of list(name)) records.pushJson(kind, item);
    }
    const snapshot = new Snapshot(records, clock as HealthClockSample);
    // Keep all received diagnostics, also those about parts the snapshot omits.
    return Object.assign(snapshot, {
      records, parts: { components: value.components !== undefined, health: present.every(Boolean) },
    });
  }
}

/** Decode a diagnostic, treating unknown kinds as `DirectoryConfig` and unknown severities as `Error`. */
function readDiagnostic(item: unknown): Diagnostic {
  if (!isObject(item)) throw new SnapshotError("diagnostic is not an object");
  const { kind, severity } = item;
  const diagnostic = {
    ...item,
    kind: KINDS.includes(kind as Diagnostic["kind"]) ? kind : "DirectoryConfig",
    severity: severity === "Error" || severity === "Warning" ? severity : "Error",
  };
  const report = Diagnostic.check(diagnostic);
  if (report.isErr()) throw new SnapshotError(`invalid diagnostic: ${report}`);
  return diagnostic as Diagnostic;
}
