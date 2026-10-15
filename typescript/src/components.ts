/** Evaluate the compatibility of a complete component composition. */

import { ComponentDeclaration } from "./generated/component.js";
import type {
  CapabilityDeclaration, CapabilitySelectorDeclaration, CompatibilityProblem, CompatibilityReport,
} from "./generated/component.js";
import { matchesRequirement, parseRequirement, parseVersion } from "./version.js";

/** Validate declarations and report every violated composition constraint. */
export function checkComponents(components: readonly ComponentDeclaration[]): CompatibilityReport {
  const provided: { provider: string; capability: CapabilityDeclaration; version?: ReturnType<typeof parseVersion> }[] = [];
  const selectors: { component: string; conflict: boolean; selector: CapabilitySelectorDeclaration; requirement?: ReturnType<typeof parseRequirement> }[] = [];
  const ids = new Map<string, number>();
  const claims = new Map<string, string[]>();
  for (const component of components) {
    const validation = ComponentDeclaration.check(component);
    if (validation.isErr()) throw new Error(validation.toString());
    ids.set(component.id, (ids.get(component.id) ?? 0) + 1);
    for (const capability of [{ id: component.id, ...(component.version === undefined ? {} : { version: component.version }) }, ...(component.provides ?? [])]) {
      validate(capability);
      provided.push({
        provider: component.id, capability,
        version: capability.version === undefined ? undefined : parseVersion(capability.version),
      });
    }
    for (const claim of new Set((component.claims ?? []).map((claim) => claim.id))) {
      const owners = claims.get(claim) ?? [];
      owners.push(component.id);
      claims.set(claim, owners);
    }
    for (const conflict of [false, true]) {
      for (const selector of (conflict ? component.conflicts : component.requires) ?? []) {
        validate(selector);
        selectors.push({
          component: component.id, conflict, selector,
          requirement: selector.version === undefined ? undefined : parseRequirement(selector.version),
        });
      }
    }
  }
  const problems: CompatibilityProblem[] = [];
  for (const id of [...ids.keys()].sort(compareIds)) {
    if (ids.get(id)! > 1) problems.push({ problem: "duplicateComponent", id });
  }
  for (const id of [...claims.keys()].sort(compareIds)) {
    const owners = claims.get(id)!;
    if (owners.length > 1) problems.push({ problem: "duplicateClaim", id, components: owners });
  }
  for (const { component, conflict, selector, requirement } of selectors) {
    const matches = provided.filter(({ capability, version }) =>
      capability.id === selector.id &&
      (selector.value === undefined || selector.value === capability.value) &&
      (requirement === undefined || (version !== undefined && matchesRequirement(requirement, version))),
    );
    if (conflict) {
      for (const { provider, capability } of matches) {
        problems.push({ problem: "conflict", component, selector, provider, capability });
      }
    } else if (!matches.length) {
      problems.push({ problem: "unsatisfiedRequirement", component, selector });
    }
  }
  return { consistent: problems.length === 0, problems };
}

/** Sort opaque IDs by Unicode scalar value, matching Rust and Python. */
function compareIds(left: string, right: string): number {
  const a = Array.from(left, (character) => character.codePointAt(0)!);
  const b = Array.from(right, (character) => character.codePointAt(0)!);
  for (let index = 0; index < Math.min(a.length, b.length); index++) {
    if (a[index] !== b[index]) return a[index] - b[index];
  }
  return a.length - b.length;
}

/** Reject ambiguous capability declarations before matching. */
function validate(declaration: CapabilityDeclaration | CapabilitySelectorDeclaration): void {
  if (declaration.version !== undefined && declaration.value !== undefined) {
    throw new Error(`Capability ${declaration.id} has both version and value`);
  }
}
