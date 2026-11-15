/** Rugix-compatible version ordering and comparator requirements. */

type Identifier = bigint | string;
type Version = { release: bigint[]; prerelease: Identifier[] };
export type Requirement = { operator: string; version: Version; wildcard: boolean }[];

const grammar = /^([0-9]+(?:\.[0-9]+)*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$/;
const max = (1n << 64n) - 1n;

/** Parse numeric releases and prereleases, ignoring build metadata in ordering. */
export function parseVersion(text: string): Version {
  const match = grammar.exec(text.trim());
  if (!match) throw new Error(`Invalid version: ${text}`);
  const release = match[1].split(".").map(BigInt);
  const prerelease = (match[2]?.split(".") ?? []).map(
    (part): Identifier => /^[0-9]+$/.test(part) ? BigInt(part) : part,
  );
  if ([...release, ...prerelease].some((part) => typeof part === "bigint" && part > max)) {
    throw new Error(`Version number exceeds u64: ${text}`);
  }
  return { release, prerelease };
}

/** Parse comma-separated comparators, or `*` alone for any version. */
export function parseRequirement(text: string): Requirement {
  if (text.trim() === "*") return [];
  return text.split(",").map((part) => {
    const match = /^(==|!=|>=|<=|=|>|<)?\s*(.+?)(\.\*)?$/.exec(part.trim());
    if (!match) throw new Error(`Invalid version requirement: ${text}`);
    const operator = match[1] ?? "=";
    const version = parseVersion(match[2]);
    const wildcard = match[3] !== undefined;
    if (wildcard && (!["=", "==", "!="].includes(operator) || version.prerelease.length > 0 || match[2].includes("+"))) {
      throw new Error(`Invalid wildcard comparator: ${text}`);
    }
    return { operator, version, wildcard };
  });
}

/** Match all comparators; a version without a prerelease names a line of versions. */
export function matchesRequirement(requirement: Requirement, version: Version): boolean {
  return requirement.every(({ operator, version: target, wildcard }) => {
    if (wildcard) {
      const inLine = compareLine(version, target) === 0;
      return operator === "!=" ? !inLine : inLine;
    }
    if (["=", "==", "!="].includes(operator) || target.prerelease.length > 0) {
      const comparison = compare(version, target);
      switch (operator) {
        case "=": case "==": return comparison === 0;
        case "!=": return comparison !== 0;
        case ">": return comparison > 0;
        case ">=": return comparison >= 0;
        case "<": return comparison < 0;
        case "<=": return comparison <= 0;
        default: throw new Error("Invalid comparator operator");
      }
    }
    const start = compareRelease(version.release, target.release);
    switch (operator) {
      case ">=": return start >= 0;
      case "<": return start < 0;
      case ">": return compareLine(version, target) > 0;
      case "<=": return compareLine(version, target) <= 0;
      default: throw new Error("Invalid comparator operator");
    }
  });
}

/** Compare release numbers, ignoring trailing zeros and prereleases. */
function compareRelease(left: bigint[], right: bigint[]): number {
  for (let i = 0; i < Math.max(left.length, right.length); i++) {
    const a = left[i] ?? 0n;
    const b = right[i] ?? 0n;
    if (a !== b) return a > b ? 1 : -1;
  }
  return 0;
}

/** Compare a version with the line of `target`, at the precision it spells. */
function compareLine(version: Version, target: Version): number {
  const prefix = target.release.map((_, i) => version.release[i] ?? 0n);
  return compareRelease(prefix, target.release);
}

/** Compare normalized release numbers, then SemVer-style prerelease identifiers. */
function compare(left: Version, right: Version): number {
  const release = compareRelease(left.release, right.release);
  if (release !== 0) return release;
  if (!left.prerelease.length || !right.prerelease.length) {
    return Number(!left.prerelease.length) - Number(!right.prerelease.length);
  }
  for (let i = 0; i < Math.min(left.prerelease.length, right.prerelease.length); i++) {
    const a = left.prerelease[i];
    const b = right.prerelease[i];
    if (a === b) continue;
    if (typeof a !== typeof b) return typeof a === "bigint" ? -1 : 1;
    return a > b ? 1 : -1;
  }
  return Math.sign(left.prerelease.length - right.prerelease.length);
}
