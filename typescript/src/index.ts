/**
 * Compatibility checking and health evaluation for the System Runtime Specification.
 *
 * `Records` holds the records of a system and the diagnostics about them. The models, such as
 * `component.ComponentDeclaration`, are generated from the specification's schemas.
 */

export * as common from "./generated/common.js";
export * as component from "./generated/component.js";
export * as filesystem from "./generated/filesystem.js";
export * as health from "./generated/health.js";
export * as metric from "./generated/metric.js";
export { estimateTimestamp } from "./clock.js";
export { type RecordModelKind, type RecordModels, SPEC_VERSION } from "./decoding.js";
export { Records } from "./records.js";
export { type Parts, Snapshot, SnapshotError } from "./snapshot.js";
