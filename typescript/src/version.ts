/** Rugix-compatible version ordering and comparator requirements. */

type Identifier = bigint | string;
type Version = { release: bigint[]; prerelease: Identifier[] };
export type Requirement = { operator: string; version: Version }[];

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

/** Parse comma-separated comparators, with a single wildcard allowed. */
export function parseRequirement(text: string): Requirement {
  if (text.trim() === "*") return [];
  return text.split(",").map((part) => {
    const match = /^(==|!=|>=|<=|=|>|<)?\s*(.+)$/.exec(part.trim());
    if (!match) throw new Error(`Invalid version requirement: ${text}`);
    return { operator: match[1] ?? "=", version: parseVersion(match[2]) };
  });
}

/** Match all comparators, including prereleases without implicit exclusions. */
export function matchesRequirement(requirement: Requirement, version: Version): boolean {
  return requirement.every(({ operator, version: target }) => {
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
  });
}

/** Compare normalized release numbers, then SemVer-style prerelease identifiers. */
function compare(left: Version, right: Version): number {
  for (let i = 0; i < Math.max(left.release.length, right.release.length); i++) {
    const a = left.release[i] ?? 0n;
    const b = right.release[i] ?? 0n;
    if (a !== b) return a > b ? 1 : -1;
  }
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
