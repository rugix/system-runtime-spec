/** Decode and validate records, applying the fallbacks for unknown values. */

import type { Checker } from "@sidex/types";
import { Diagnostic, RecordKind, Severity } from "./generated/common.js";
import { ComponentDeclaration } from "./generated/component.js";
import { DirectoryConfig } from "./generated/filesystem.js";
import { HealthNode, HealthStatus } from "./generated/health.js";
import { MetricDefinition, MetricSample } from "./generated/metric.js";
import { parseRequirement, parseVersion } from "./version.js";

/** Version of the specification that this package implements. */
export const SPEC_VERSION = "1.0";
const SUPPORTED_MAJOR = 1n;
const SUPPORTED_MINOR = 0n;
const U64_MAX = (1n << 64n) - 1n;

const RULE_TYPES = ["Observed", "Threshold", "Aggregate"];
const STATES = ["Healthy", "Degraded", "Unhealthy", "Unknown"];
const MODES = ["Counted", "Mandatory", "Optional"];
export const KINDS: readonly RecordKind[] = [
  "ComponentDeclaration",
  "HealthNode",
  "HealthStatus",
  "MetricDefinition",
  "MetricSample",
  "DirectoryConfig",
];

/** A JSON object. */
export type JsonObject = { [key: string]: unknown };

/**
 * Records of all kinds, as JSON objects, with the diagnostics about them.
 *
 * Records keep their original JSON, so snapshots contain them unchanged.
 */
export type Records = {
  components?: readonly unknown[];
  healthNodes?: readonly unknown[];
  healthStatuses?: readonly unknown[];
  metrics?: readonly unknown[];
  metricSamples?: readonly unknown[];
  diagnostics?: readonly Diagnostic[];
};

/**
 * A valid record, normalized for evaluation, with the values that need a fallback.
 *
 * Unknown rule types are removed, and unknown states and modes are replaced; the flags record
 * where the fallbacks apply.
 */
export type Decoded = {
  record: JsonObject;
  newer: boolean;
  unsupported: boolean;
  unsupportedAttachments: Set<string>;
};

export function diagnostic(
  severity: Severity, kind: RecordKind, message: string, path?: string,
): Diagnostic {
  return path === undefined ? { severity, kind, message } : { severity, kind, message, path };
}

const CHECKERS: Record<RecordKind, Checker<unknown>> = {
  ComponentDeclaration,
  HealthNode,
  HealthStatus,
  MetricDefinition,
  MetricSample,
  DirectoryConfig,
};

class InvalidRecord extends Error {}

function isObject(value: unknown): value is JsonObject {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function clone(value: unknown): unknown {
  return value === undefined ? undefined : JSON.parse(JSON.stringify(value));
}

export function parseSpecVersion(text: unknown): [bigint, bigint] | undefined {
  if (typeof text !== "string") return undefined;
  const match = /^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$/.exec(text);
  if (!match) return undefined;
  const [major, minor] = [BigInt(match[1]), BigInt(match[2])];
  return major > U64_MAX || minor > U64_MAX ? undefined : [major, minor];
}

/** Check a record's version; return whether it is a newer minor version. */
function checkVersion(kind: RecordKind, record: JsonObject): boolean {
  if (!("specVersion" in record)) return false;
  const version = parseSpecVersion(record.specVersion);
  if (version === undefined) throw new InvalidRecord(`invalid specVersion ${JSON.stringify(record.specVersion)}`);
  const [major, minor] = version;
  if (major !== SUPPORTED_MAJOR) throw new InvalidRecord(`unsupported specVersion ${record.specVersion}`);
  const newer = minor > SUPPORTED_MINOR;
  if (newer && kind === "ComponentDeclaration") {
    throw new InvalidRecord(`component declaration of newer specVersion ${record.specVersion}`);
  }
  return newer;
}

/** Replace an unknown input mode with `Counted`; return whether it was unknown. */
function unknownMode(entry: JsonObject): boolean {
  if (entry.mode === undefined) return false;
  if (typeof entry.mode !== "string") throw new InvalidRecord("mode is not a string");
  if (MODES.includes(entry.mode)) return false;
  entry.mode = "Counted";
  return true;
}

/** Validate a record of a kind and determine its fallbacks. */
export function decode(kind: RecordKind, value: unknown): Decoded {
  if (!isObject(value)) throw new InvalidRecord("record is not a JSON object");
  const record = clone(value) as JsonObject;
  const result: Decoded = {
    record,
    newer: checkVersion(kind, record),
    unsupported: false,
    unsupportedAttachments: new Set(),
  };
  if (kind === "HealthNode") {
    const rule = record.rule;
    if (isObject(rule)) {
      if (typeof rule.type !== "string") throw new InvalidRecord("rule.type is not a string");
      if (!RULE_TYPES.includes(rule.type)) {
        result.unsupported = true;
        delete record.rule;
      } else if (rule.type === "Aggregate" && Array.isArray(rule.inputs)) {
        for (const entry of rule.inputs) {
          if (isObject(entry) && unknownMode(entry)) result.unsupported = true;
        }
      }
    }
    if (Array.isArray(record.partOf)) {
      for (const entry of record.partOf) {
        if (isObject(entry) && unknownMode(entry) && typeof entry.node === "string") {
          result.unsupportedAttachments.add(entry.node);
        }
      }
    }
  } else if (kind === "HealthStatus") {
    if (typeof record.state !== "string") throw new InvalidRecord("state is not a string");
    if (!STATES.includes(record.state)) {
      result.unsupported = true;
      record.state = "Unknown";
    }
  }
  const report = CHECKERS[kind].check(record);
  if (report.isErr()) {
    const [error] = report.errors;
    throw new InvalidRecord(`${error.path}: ${error.message}`);
  }
  validate(kind, record);
  return result;
}

/** Decode records, skipping invalid ones with an error diagnostic. */
export function decodeAll(
  kind: RecordKind, values: readonly unknown[] | undefined, diagnostics: Diagnostic[],
): Decoded[] {
  const result: Decoded[] = [];
  for (const value of values ?? []) {
    try {
      result.push(decode(kind, value));
    } catch (error) {
      if (!(error instanceof InvalidRecord)) throw error;
      diagnostics.push(diagnostic("Error", kind, error.message));
    }
  }
  return result;
}

function items(record: JsonObject, key: string): JsonObject[] {
  const value = record[key];
  return Array.isArray(value) ? (value as JsonObject[]) : [];
}

function checkId(value: unknown, what: string): void {
  if (value === undefined) return;
  if (typeof value !== "string" || !/^[!-~]+$/.test(value)) {
    throw new InvalidRecord(`${what} is not a valid ID: ${JSON.stringify(value)}`);
  }
}

function checkPositive(record: JsonObject, key: string): void {
  if (record[key] !== undefined && BigInt(record[key] as string | number) === 0n) {
    throw new InvalidRecord(`${key} is zero`);
  }
}

export function finite(value: unknown, what: string): number {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new InvalidRecord(`${what} is not finite`);
  return value;
}

function parse(parser: (text: string) => unknown, text: string): void {
  try {
    parser(text);
  } catch (error) {
    throw new InvalidRecord((error as Error).message);
  }
}

function checkCapability(value: JsonObject, version: boolean): void {
  checkId(value.id, "capability ID");
  if (value.version !== undefined && value.value !== undefined) {
    throw new InvalidRecord(`capability ${JSON.stringify(value.id)} has both version and value`);
  }
  if (typeof value.version === "string") parse(version ? parseVersion : parseRequirement, value.version);
}

type Bound = { value: number; inclusive: boolean };
type Range = { lower?: Bound; upper?: Bound };

function checkRanges(healthy: Range, usable: Range): void {
  for (const range of [healthy, usable]) {
    for (const bound of [range.lower, range.upper]) {
      if (bound !== undefined) finite(bound.value, "range bound");
    }
    const { lower, upper } = range;
    if (lower !== undefined && upper !== undefined) {
      if (lower.value > upper.value || (lower.value === upper.value && !(lower.inclusive && upper.inclusive))) {
        throw new InvalidRecord("range is empty");
      }
    }
  }
  for (const side of ["lower", "upper"] as const) {
    const outer = usable[side];
    if (outer === undefined) continue;
    const inner = healthy[side];
    if (inner === undefined) throw new InvalidRecord("healthy range is not a subset of the usable range");
    const beyond = side === "lower" ? inner.value < outer.value : inner.value > outer.value;
    if (beyond || (inner.value === outer.value && inner.inclusive && !outer.inclusive)) {
      throw new InvalidRecord("healthy range is not a subset of the usable range");
    }
  }
}

function checkDirectoryPath(path: string): void {
  if (!path.startsWith("/")) throw new InvalidRecord(`path is not absolute: ${path}`);
  for (const segment of path.split("/")) {
    if (segment === "..") throw new InvalidRecord(`path contains '..': ${path}`);
    if (segment.includes("*") && segment !== "*") throw new InvalidRecord(`path contains a partial wildcard: ${path}`);
  }
}

function validate(kind: RecordKind, record: JsonObject): void {
  checkId(record.id, "id");
  checkId(record.owner, "owner");
  if (kind !== "DirectoryConfig") {
    for (const component of (record.components as unknown[] | undefined) ?? []) checkId(component, "component ID");
  }
  switch (kind) {
    case "ComponentDeclaration":
      for (const capability of items(record, "provides")) checkCapability(capability, true);
      for (const claim of items(record, "claims")) checkId(claim.id, "claim ID");
      for (const selector of [...items(record, "requires"), ...items(record, "conflicts")]) {
        checkCapability(selector, false);
      }
      if (typeof record.version === "string") parse(parseVersion, record.version);
      break;
    case "HealthNode": {
      for (const attachment of items(record, "partOf")) checkId(attachment.node, "attachment node");
      const rule = record.rule as JsonObject | undefined;
      if (rule === undefined) break;
      checkPositive(rule, "maxAgeMs");
      if (rule.type === "Threshold") {
        checkId(rule.metric, "metric");
        checkRanges(rule.healthy as Range, rule.usable as Range);
      } else if (rule.type === "Aggregate") {
        for (const input of items(rule, "inputs")) checkId(input.node, "input node");
      }
      break;
    }
    case "HealthStatus":
    case "MetricDefinition":
      checkPositive(record, "maxAgeMs");
      break;
    case "MetricSample":
      finite(record.value, "value");
      checkPositive(record, "maxAgeMs");
      break;
    case "DirectoryConfig":
      for (const key of ["components", "healthNodes", "healthStatuses", "metrics", "metricSamples"]) {
        for (const path of (record[key] as string[] | undefined) ?? []) checkDirectoryPath(path);
      }
      break;
  }
}
