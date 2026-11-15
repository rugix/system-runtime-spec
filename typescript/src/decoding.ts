import { type Checker, builtins } from "@sidex/types";
import type { Diagnostic, RecordKind, Severity } from "./generated/common.js";
import { ComponentDeclaration } from "./generated/component.js";
import { DirectoryConfig } from "./generated/filesystem.js";
import { HealthNode, HealthStatus } from "./generated/health.js";
import { MetricDefinition, type MetricRange, MetricSample } from "./generated/metric.js";
import { parseRequirement, parseVersion } from "./version.js";

/** Version of the specification that this package implements. */
export const SPEC_VERSION = "1.0";
export const SUPPORTED_MAJOR = 1n;
const SUPPORTED_MINOR = 0n;
const U64_MAX = (1n << 64n) - 1n;

const RULE_TYPES = ["Observed", "Threshold", "Aggregate"];
const STATES = ["Healthy", "Degraded", "Unhealthy", "Unknown"];
const MODES = ["Counted", "Mandatory", "Optional"];

/** Models of the kinds of records that `Records` holds, by kind. */
export interface RecordModels {
  ComponentDeclaration: ComponentDeclaration;
  HealthNode: HealthNode;
  HealthStatus: HealthStatus;
  MetricDefinition: MetricDefinition;
  MetricSample: MetricSample;
}

/** Kind of record that `Records` holds. */
export type RecordModelKind = keyof RecordModels;

const CHECKERS: { [K in RecordKind]: Checker<unknown> } = {
  ComponentDeclaration,
  HealthNode,
  HealthStatus,
  MetricDefinition,
  MetricSample,
  DirectoryConfig,
};

export const KINDS = Object.keys(CHECKERS) as RecordKind[];

/**
 * A valid record with its original JSON and its model, normalized for evaluation.
 *
 * Unknown rule types are removed, and unknown states and modes are replaced; the flags record
 * where the fallbacks apply.
 */
export type Record<T> = {
  readonly json: unknown;
  readonly value: T;
  readonly newer: boolean;
  readonly unsupported: boolean;
  readonly unsupportedAttachments: ReadonlySet<string>;
};

/** A record that is not valid and is skipped with an error diagnostic. */
export class InvalidRecord extends Error {}

type JsonObject = { [key: string]: unknown };

export function isObject(value: unknown): value is JsonObject {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

export function diagnostic(
  severity: Severity, kind: RecordKind, message: string, path?: string,
): Diagnostic {
  return path === undefined ? { severity, kind, message } : { severity, kind, message, path };
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
  if (minor <= SUPPORTED_MINOR) return false;
  if (kind === "ComponentDeclaration") {
    throw new InvalidRecord(`component declaration of newer specVersion ${record.specVersion}`);
  }
  return true;
}

/** Replace an unknown input mode with `Counted`; return whether it was unknown. */
function applyModeFallback(entry: JsonObject): boolean {
  if (entry.mode === undefined) return false;
  if (typeof entry.mode !== "string") throw new InvalidRecord("mode is not a string");
  if (MODES.includes(entry.mode)) return false;
  entry.mode = "Counted";
  return true;
}

/** Decode and validate a record; throw `InvalidRecord` if it is not valid. */
export function decode<K extends RecordKind>(
  kind: K, json: unknown,
): Record<K extends RecordModelKind ? RecordModels[K] : DirectoryConfig> {
  if (!isObject(json)) throw new InvalidRecord("record is not a JSON object");
  const original = structuredClone(json);
  const data = structuredClone(json);
  const newer = checkVersion(kind, data);
  let unsupported = false;
  const unsupportedAttachments = new Set<string>();
  if (kind === "HealthNode") {
    const rule = data.rule;
    if (isObject(rule)) {
      if (typeof rule.type !== "string") throw new InvalidRecord("rule.type is not a string");
      if (!RULE_TYPES.includes(rule.type)) {
        unsupported = true;
        delete data.rule;
      } else if (rule.type === "Aggregate" && Array.isArray(rule.inputs)) {
        for (const entry of rule.inputs) {
          if (isObject(entry) && applyModeFallback(entry)) unsupported = true;
        }
      }
    }
    if (Array.isArray(data.partOf)) {
      for (const entry of data.partOf) {
        if (isObject(entry) && applyModeFallback(entry) && typeof entry.node === "string") {
          unsupportedAttachments.add(entry.node);
        }
      }
    }
  } else if (kind === "HealthStatus") {
    if (typeof data.state !== "string") throw new InvalidRecord("state is not a string");
    if (!STATES.includes(data.state)) {
      unsupported = true;
      data.state = "Unknown";
    }
  }
  const report = CHECKERS[kind].check(data);
  if (report.isErr()) {
    const [error] = report.errors;
    throw new InvalidRecord(`${error.path}: ${error.message}`);
  }
  if (kind !== "DirectoryConfig") {
    const { id, owner } = data as RecordModels[RecordModelKind];
    checkId(id, "id");
    checkId(owner, "owner");
  }
  VALIDATORS[kind](data as never);
  return { json: original, value: data as never, newer, unsupported, unsupportedAttachments };
}

function checkId(value: string | undefined, what: string): void {
  if (value !== undefined && !/^[!-~]+$/.test(value)) {
    throw new InvalidRecord(`${what} is not a valid ID: ${JSON.stringify(value)}`);
  }
}

function checkPositive(value: builtins.U64 | undefined, what: string): void {
  if (value !== undefined && builtins.U64.toBigInt(value) === 0n) throw new InvalidRecord(`${what} is zero`);
}

function finite(value: builtins.F64, what: string): void {
  if (!Number.isFinite(builtins.F64.toNumber(value))) throw new InvalidRecord(`${what} is not finite`);
}

function parse(parser: (text: string) => unknown, text: string): void {
  try {
    parser(text);
  } catch (error) {
    throw new InvalidRecord((error as Error).message);
  }
}

function checkCapability(id: string, version: string | undefined, value: string | undefined): void {
  checkId(id, "capability ID");
  if (version !== undefined && value !== undefined) {
    throw new InvalidRecord(`capability ${JSON.stringify(id)} has both version and value`);
  }
}

function checkRanges(healthy: MetricRange, usable: MetricRange): void {
  const value = (bound: { value: builtins.F64 }) => builtins.F64.toNumber(bound.value);
  for (const range of [healthy, usable]) {
    for (const bound of [range.lower, range.upper]) {
      if (bound !== undefined) finite(bound.value, "range bound");
    }
    const { lower, upper } = range;
    if (lower !== undefined && upper !== undefined) {
      if (value(lower) > value(upper) || (value(lower) === value(upper) && !(lower.inclusive && upper.inclusive))) {
        throw new InvalidRecord("range is empty");
      }
    }
  }
  for (const side of ["lower", "upper"] as const) {
    const outer = usable[side];
    if (outer === undefined) continue;
    const inner = healthy[side];
    if (inner === undefined) throw new InvalidRecord("healthy range is not a subset of the usable range");
    const beyond = side === "lower" ? value(inner) < value(outer) : value(inner) > value(outer);
    if (beyond || (value(inner) === value(outer) && inner.inclusive && !outer.inclusive)) {
      throw new InvalidRecord("healthy range is not a subset of the usable range");
    }
  }
}

function checkDirectoryPath(path: string): void {
  if (!path.startsWith("/")) throw new InvalidRecord(`path is not absolute: ${path}`);
  if (path.includes("\0")) throw new InvalidRecord(`path contains a NUL character: ${JSON.stringify(path)}`);
  for (const segment of path.split("/")) {
    if (segment === "..") throw new InvalidRecord(`path contains '..': ${path}`);
    if (segment.includes("*") && segment !== "*") throw new InvalidRecord(`path contains a partial wildcard: ${path}`);
  }
}

const VALIDATORS: { [K in RecordKind]: (record: K extends RecordModelKind ? RecordModels[K] : DirectoryConfig) => void } = {
  ComponentDeclaration(record) {
    if (record.version !== undefined) parse(parseVersion, record.version);
    for (const claim of record.claims ?? []) checkId(claim.id, "claim ID");
    for (const capability of record.provides ?? []) {
      checkCapability(capability.id, capability.version, capability.value);
      if (capability.version !== undefined) parse(parseVersion, capability.version);
    }
    for (const selector of [...(record.requires ?? []), ...(record.conflicts ?? [])]) {
      checkCapability(selector.id, selector.version, selector.value);
      if (selector.version !== undefined) parse(parseRequirement, selector.version);
    }
  },
  HealthNode(record) {
    for (const component of record.components ?? []) checkId(component, "component ID");
    for (const attachment of record.partOf ?? []) checkId(attachment.node, "attachment node");
    const rule = record.rule;
    if (rule === undefined) return;
    if (rule.type !== "Aggregate") checkPositive(rule.maxAgeMs, "maxAgeMs");
    if (rule.type === "Threshold") {
      checkId(rule.metric, "metric");
      checkRanges(rule.healthy, rule.usable);
    } else if (rule.type === "Aggregate") {
      for (const input of rule.inputs ?? []) checkId(input.node, "input node");
    }
  },
  HealthStatus(record) {
    checkPositive(record.maxAgeMs, "maxAgeMs");
  },
  MetricDefinition(record) {
    for (const component of record.components ?? []) checkId(component, "component ID");
    checkPositive(record.maxAgeMs, "maxAgeMs");
  },
  MetricSample(record) {
    finite(record.value, "value");
    checkPositive(record.maxAgeMs, "maxAgeMs");
  },
  DirectoryConfig(record) {
    for (const paths of [record.components, record.healthNodes, record.healthStatuses, record.metrics, record.metricSamples]) {
      for (const path of paths ?? []) checkDirectoryPath(path);
    }
  },
};
