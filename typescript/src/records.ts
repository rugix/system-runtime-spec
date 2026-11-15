import { check } from "./compatibility.js";
import {
  InvalidRecord, type Record, type RecordModelKind, type RecordModels, decode, diagnostic,
} from "./decoding.js";
import { evaluate } from "./evaluation.js";
import type { Diagnostic, RecordKind } from "./generated/common.js";
import type { CompatibilityReport, ComponentDeclaration } from "./generated/component.js";
import type { HealthContext, HealthNode, HealthReport, HealthStatus } from "./generated/health.js";
import type { MetricDefinition, MetricSample } from "./generated/metric.js";

const HEALTH_KINDS: readonly RecordKind[] = ["HealthNode", "HealthStatus", "MetricDefinition", "MetricSample"];

type Lists = { [K in RecordModelKind]: Record<RecordModels[K]>[] };

/**
 * The records of a system and the diagnostics about them.
 *
 * Records are added as models or as JSON values. Adding a record decodes and validates it, and an
 * invalid record is skipped with an error diagnostic. Records keep their original JSON, so that
 * snapshots contain them unchanged, including fields of newer minor versions.
 */
export class Records {
  #lists: Lists = {
    ComponentDeclaration: [],
    HealthNode: [],
    HealthStatus: [],
    MetricDefinition: [],
    MetricSample: [],
  };
  #diagnostics: Diagnostic[] = [];

  /** Add a record given as a model. */
  push<K extends RecordModelKind>(kind: K, record: RecordModels[K]): void {
    this.pushJson(kind, record);
  }

  /** Add a record given as JSON, such as a record received from another system. */
  pushJson(kind: RecordModelKind, value: unknown): void {
    try {
      (this.#lists[kind] as Record<unknown>[]).push(decode(kind, value));
    } catch (error) {
      if (!(error instanceof InvalidRecord)) throw error;
      this.#diagnostics.push(diagnostic("Error", kind, error.message));
    }
  }

  /** Add a diagnostic, such as one that another reader reported. */
  pushDiagnostic(diagnostic: Diagnostic): void {
    this.#diagnostics.push(diagnostic);
  }

  /** Component declarations, in the order in which they were added. */
  get components(): readonly ComponentDeclaration[] {
    return this.#values("ComponentDeclaration");
  }

  /**
   * Health nodes, in the order in which they were added.
   *
   * A node with an unknown rule type has no rule, and unknown input modes are `Counted`;
   * evaluation reports such nodes as unsupported.
   */
  get healthNodes(): readonly HealthNode[] {
    return this.#values("HealthNode");
  }

  /**
   * Status records, in the order in which they were added.
   *
   * Unknown states are `Unknown`; evaluation reports such records as unsupported.
   */
  get healthStatuses(): readonly HealthStatus[] {
    return this.#values("HealthStatus");
  }

  /** Metric definitions, in the order in which they were added. */
  get metrics(): readonly MetricDefinition[] {
    return this.#values("MetricDefinition");
  }

  /** Metric samples, in the order in which they were added. */
  get metricSamples(): readonly MetricSample[] {
    return this.#values("MetricSample");
  }

  /** Diagnostics about the records and about directory configurations. */
  get diagnostics(): readonly Diagnostic[] {
    return this.#diagnostics;
  }

  /**
   * Check the composition of the component declarations.
   *
   * The report contains the diagnostics about component declarations and directory
   * configurations. The composition is consistent if there are no problems and none of these
   * diagnostics has the severity `Error`.
   */
  checkCompatibility(): CompatibilityReport {
    return check(this.components, this.#diagnosticsAbout(["ComponentDeclaration", "DirectoryConfig"]));
  }

  /**
   * Evaluate the health records in a health context.
   *
   * The report contains one result per node ID, in the order of the node definitions, and the
   * diagnostics about health records and directory configurations, including those that
   * evaluation adds.
   */
  evaluateHealth(context: HealthContext): HealthReport {
    const lists = this.#lists;
    return evaluate(
      lists.HealthNode, lists.HealthStatus, lists.MetricDefinition, lists.MetricSample,
      this.#diagnosticsAbout([...HEALTH_KINDS, "DirectoryConfig"]), context,
    );
  }

  /**
   * Build the candidate composition of an update.
   *
   * The update replaces all component declarations of `owners` with the component declarations
   * of `update`. A new declaration without a replaced owner is skipped with an error diagnostic.
   * The candidate has the current records and diagnostics, and the diagnostics of `update` about
   * component declarations.
   */
  withUpdate(owners: Iterable<string>, update: Records): Records {
    const replaced = new Set(owners);
    const isReplaced = (declaration: ComponentDeclaration) =>
      declaration.owner !== undefined && replaced.has(declaration.owner);
    const candidate = this.#copy();
    const components = candidate.#lists.ComponentDeclaration.filter((record) => !isReplaced(record.value));
    candidate.#diagnostics.push(...update.#diagnosticsAbout(["ComponentDeclaration"]));
    for (const record of update.#lists.ComponentDeclaration) {
      if (isReplaced(record.value)) components.push(record);
      else {
        candidate.#diagnostics.push(diagnostic(
          "Error", "ComponentDeclaration",
          `new declaration ${JSON.stringify(record.value.id)} does not have a replaced owner`,
        ));
      }
    }
    candidate.#lists.ComponentDeclaration = components;
    return candidate;
  }

  /** @internal Copy the records and diagnostics of the given parts. */
  only(parts: { components: boolean; health: boolean }): Records {
    const result = this.#copy();
    if (!parts.components) result.#lists.ComponentDeclaration = [];
    if (!parts.health) {
      for (const kind of HEALTH_KINDS as RecordModelKind[]) result.#lists[kind] = [];
    }
    result.#diagnostics = result.#diagnostics.filter((d) =>
      d.kind === "DirectoryConfig" || (d.kind === "ComponentDeclaration" ? parts.components : parts.health));
    return result;
  }

  /** @internal The original JSON of the records of a kind. */
  originals(kind: RecordModelKind): unknown[] {
    return this.#lists[kind].map((record) => structuredClone(record.json));
  }

  #values<K extends RecordModelKind>(kind: K): RecordModels[K][] {
    return (this.#lists[kind] as Record<RecordModels[K]>[]).map((record) => record.value);
  }

  #diagnosticsAbout(kinds: readonly RecordKind[]): Diagnostic[] {
    return this.#diagnostics.filter((d) => kinds.includes(d.kind));
  }

  #copy(): Records {
    const result = new Records();
    for (const kind of Object.keys(this.#lists) as RecordModelKind[]) {
      (result.#lists[kind] as Record<unknown>[]) = [...this.#lists[kind]];
    }
    result.#diagnostics = [...this.#diagnostics];
    return result;
  }
}
