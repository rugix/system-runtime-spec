/** Check the compatibility of compositions and candidate compositions. */

import type { Diagnostic } from "./generated/common.js";
import type {
  CapabilityDeclaration, CapabilitySelectorDeclaration, CompatibilityProblem, CompatibilityReport,
} from "./generated/component.js";
import { type JsonObject, decodeAll, diagnostic } from "./records.js";
import { type Requirement, type Version, matchesRequirement, parseRequirement, parseVersion } from "./version.js";

function bytes(text: string): string {
  return Array.from(new TextEncoder().encode(text), (byte) => byte.toString(16).padStart(2, "0")).join("");
}

function byteOrder(a: string, b: string): number {
  const [x, y] = [bytes(a), bytes(b)];
  return x < y ? -1 : x > y ? 1 : 0;
}

/** Keep the fields that the specification defines for capabilities and selectors. */
function declared(capability: JsonObject): CapabilityDeclaration {
  const result: JsonObject = { id: capability.id };
  for (const key of ["version", "value"]) {
    if (capability[key] !== undefined) result[key] = capability[key];
  }
  return result as CapabilityDeclaration;
}

/**
 * Check a composition, given as component declarations and diagnostics.
 *
 * Invalid declarations are skipped with an error diagnostic. The composition is consistent if
 * there are no problems and no diagnostics with the severity `Error`.
 */
export function checkComponents(
  components: readonly unknown[], diagnostics: readonly Diagnostic[] = [],
): CompatibilityReport {
  const allDiagnostics = [...diagnostics];
  const declarations = decodeAll("ComponentDeclaration", components, allDiagnostics).map((d) => d.record);
  const provided: [string, JsonObject, Version | undefined][] = [];
  const selectors: [string, boolean, JsonObject, Requirement | undefined][] = [];
  const ids = new Map<string, number>();
  const claims = new Map<string, string[]>();
  for (const component of declarations) {
    const id = component.id as string;
    ids.set(id, (ids.get(id) ?? 0) + 1);
    const implicit: JsonObject = component.version === undefined ? { id } : { id, version: component.version };
    for (const capability of [implicit, ...((component.provides as JsonObject[] | undefined) ?? [])]) {
      const version = typeof capability.version === "string" ? parseVersion(capability.version) : undefined;
      provided.push([id, capability, version]);
    }
    const ownClaims = new Set(((component.claims as JsonObject[] | undefined) ?? []).map((claim) => claim.id as string));
    for (const claim of ownClaims) claims.set(claim, [...(claims.get(claim) ?? []), id]);
    for (const [conflict, name] of [[false, "requires"], [true, "conflicts"]] as const) {
      for (const selector of (component[name] as JsonObject[] | undefined) ?? []) {
        const requirement = typeof selector.version === "string" ? parseRequirement(selector.version) : undefined;
        selectors.push([id, conflict, selector, requirement]);
      }
    }
  }
  const problems: CompatibilityProblem[] = [];
  for (const id of [...ids.keys()].sort(byteOrder)) {
    if ((ids.get(id) ?? 0) > 1) problems.push({ problem: "DuplicateComponent", id });
  }
  for (const id of [...claims.keys()].sort(byteOrder)) {
    const owners = claims.get(id) ?? [];
    if (owners.length > 1) problems.push({ problem: "DuplicateClaim", id, components: owners });
  }
  for (const [component, conflict, selector, requirement] of selectors) {
    const matches = provided.filter(([, capability, version]) =>
      capability.id === selector.id
      && (selector.value === undefined || selector.value === capability.value)
      && (requirement === undefined || (version !== undefined && matchesRequirement(requirement, version))));
    if (conflict) {
      for (const [provider, capability] of matches) {
        problems.push({
          problem: "Conflict",
          component,
          selector: declared(selector) as CapabilitySelectorDeclaration,
          provider,
          capability: declared(capability),
        });
      }
    } else if (matches.length === 0) {
      problems.push({
        problem: "UnsatisfiedRequirement",
        component,
        selector: declared(selector) as CapabilitySelectorDeclaration,
      });
    }
  }
  const consistent = problems.length === 0 && allDiagnostics.every((d) => d.severity !== "Error");
  return { consistent, problems, diagnostics: allDiagnostics };
}

/**
 * Build the composition that would result from an update.
 *
 * The update replaces all declarations of `owners` with `declarations`. A new declaration
 * without a replaced owner is skipped with an error diagnostic.
 */
export function candidateComposition(
  components: readonly unknown[],
  diagnostics: readonly Diagnostic[],
  owners: readonly string[],
  declarations: readonly unknown[],
): [unknown[], Diagnostic[]] {
  const replaced = (value: unknown) =>
    typeof value === "object" && value !== null && owners.includes((value as JsonObject).owner as string);
  const result = components.filter((component) => !replaced(component));
  const resultDiagnostics = [...diagnostics];
  for (const declaration of declarations) {
    if (replaced(declaration)) result.push(declaration);
    else {
      resultDiagnostics.push(
        diagnostic("Error", "ComponentDeclaration", "new declaration does not have a replaced owner"),
      );
    }
  }
  return [result, resultDiagnostics];
}
