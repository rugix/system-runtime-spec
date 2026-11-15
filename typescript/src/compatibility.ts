/** Check the compatibility of compositions. */

import type { Diagnostic } from "./generated/common.js";
import type {
  CapabilityDeclaration, CapabilitySelectorDeclaration, CompatibilityProblem, CompatibilityReport,
  ComponentDeclaration,
} from "./generated/component.js";
import { type Requirement, type Version, matchesRequirement, parseRequirement, parseVersion } from "./version.js";

const encoder = new TextEncoder();

function byteOrder(a: string, b: string): number {
  const [x, y] = [encoder.encode(a), encoder.encode(b)];
  for (let i = 0; i < Math.min(x.length, y.length); i++) {
    if (x[i] !== y[i]) return x[i] - y[i];
  }
  return x.length - y.length;
}

/** Keep the fields that the specification defines for capabilities and selectors. */
function declared<T extends CapabilityDeclaration | CapabilitySelectorDeclaration>({ id, version, value }: T): T {
  return { id, ...(version === undefined ? {} : { version }), ...(value === undefined ? {} : { value }) } as T;
}

/** Check a composition of valid declarations, with the diagnostics about it. */
export function check(
  declarations: readonly ComponentDeclaration[], diagnostics: readonly Diagnostic[],
): CompatibilityReport {
  const provided: [string, CapabilityDeclaration, Version | undefined][] = [];
  const selectors: [string, boolean, CapabilitySelectorDeclaration, Requirement | undefined][] = [];
  const ids = new Map<string, number>();
  const claims = new Map<string, string[]>();
  for (const component of declarations) {
    const { id } = component;
    ids.set(id, (ids.get(id) ?? 0) + 1);
    const implicit: CapabilityDeclaration = component.version === undefined ? { id } : { id, version: component.version };
    for (const capability of [implicit, ...(component.provides ?? [])]) {
      const version = capability.version === undefined ? undefined : parseVersion(capability.version);
      provided.push([id, capability, version]);
    }
    for (const claim of new Set((component.claims ?? []).map((claim) => claim.id))) {
      claims.set(claim, [...(claims.get(claim) ?? []), id]);
    }
    for (const [conflict, entries] of [[false, component.requires], [true, component.conflicts]] as const) {
      for (const selector of entries ?? []) {
        const requirement = selector.version === undefined ? undefined : parseRequirement(selector.version);
        selectors.push([id, conflict, selector, requirement]);
      }
    }
  }
  const problems: CompatibilityProblem[] = [];
  for (const [id, count] of [...ids].sort(([a], [b]) => byteOrder(a, b))) {
    if (count > 1) problems.push({ problem: "DuplicateComponent", id });
  }
  for (const [id, owners] of [...claims].sort(([a], [b]) => byteOrder(a, b))) {
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
          problem: "Conflict", component, selector: declared(selector), provider, capability: declared(capability),
        });
      }
    } else if (matches.length === 0) {
      problems.push({ problem: "UnsatisfiedRequirement", component, selector: declared(selector) });
    }
  }
  const consistent = problems.length === 0 && diagnostics.every((d) => d.severity !== "Error");
  return { consistent, problems, diagnostics: [...diagnostics] };
}
