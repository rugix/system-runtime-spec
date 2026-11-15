/**
 * Compatibility checking and health evaluation for the System Runtime Specification.
 *
 * Records are plain JSON objects, so that snapshots carry them unchanged and records of newer
 * minor versions keep working. The generated models describe their structure.
 */

export * as common from "./generated/common.js";
export * as component from "./generated/component.js";
export * as filesystem from "./generated/filesystem.js";
export * as health from "./generated/health.js";
export * as metric from "./generated/metric.js";
export { estimateTimestamp } from "./clock.js";
export { candidateComposition, checkComponents } from "./components.js";
export { evaluateHealth } from "./health.js";
export { type JsonObject, type Records, SPEC_VERSION } from "./records.js";
export { type Snapshot, makeSnapshot, readSnapshot } from "./snapshot.js";
