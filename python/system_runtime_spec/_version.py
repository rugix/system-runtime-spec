"""Versions, precedence, and version requirements."""

import re
from dataclasses import dataclass
from itertools import zip_longest

_VERSION = re.compile(
    r"([0-9]+(?:\.[0-9]+)*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?"
)
_OPERATORS = (">=", "<=", "==", "!=", ">", "<", "=")
_MAX = 2**64 - 1

# Characters with the Unicode property White_Space.
_WHITESPACE = "\t\n\v\f\r \x85\xa0                　"


def _trim(text: str) -> str:
    return text.strip(_WHITESPACE)


@dataclass(frozen=True)
class Version:
    """Normalized numeric release and ordered prerelease identifiers."""

    release: tuple[int, ...]
    prerelease: tuple[int | str, ...]

    @classmethod
    def parse(cls, text: str) -> "Version":
        match = _VERSION.fullmatch(_trim(text))
        if match is None:
            raise ValueError(f"Invalid version: {text!r}")
        release = tuple(int(part.lstrip("0") or "0") for part in match[1].split("."))
        prerelease = tuple(
            int(part.lstrip("0") or "0") if part.isdigit() else part
            for part in (match[2].split(".") if match[2] else [])
        )
        if any(
            part > _MAX for part in (*release, *prerelease) if isinstance(part, int)
        ):
            raise ValueError(f"Version number exceeds u64: {text!r}")
        return cls(release, prerelease)

    def compare(self, other: "Version") -> int:
        for left, right in zip_longest(self.release, other.release, fillvalue=0):
            if left != right:
                return 1 if left > right else -1
        if not self.prerelease or not other.prerelease:
            return (not self.prerelease) - (not other.prerelease)
        for left_id, right_id in zip(self.prerelease, other.prerelease, strict=False):
            if left_id == right_id:
                continue
            if isinstance(left_id, int) != isinstance(right_id, int):
                return -1 if isinstance(left_id, int) else 1
            if isinstance(left_id, int) and isinstance(right_id, int):
                return 1 if left_id > right_id else -1
            if isinstance(left_id, str) and isinstance(right_id, str):
                return 1 if left_id > right_id else -1
        return (len(self.prerelease) > len(other.prerelease)) - (
            len(self.prerelease) < len(other.prerelease)
        )


@dataclass(frozen=True)
class Comparator:
    """One comparator; a version without a prerelease names a line of versions."""

    operator: str
    version: Version
    wildcard: bool = False


def parse_requirement(text: str) -> list[Comparator]:
    """Parse comma-separated comparators, or `*` alone for any version."""
    if _trim(text) == "*":
        return []
    comparators = []
    for part in text.split(","):
        part = _trim(part)
        if not part:
            raise ValueError(f"Invalid version requirement: {text!r}")
        operator = next((op for op in _OPERATORS if part.startswith(op)), "")
        part = _trim(part[len(operator) :])
        wildcard = part.endswith(".*")
        if wildcard:
            part = part[:-2]
        if not part:
            raise ValueError(f"Invalid version requirement: {text!r}")
        version = Version.parse(part)
        operator = operator or "="
        if wildcard and (
            operator not in ("=", "==", "!=") or version.prerelease or "+" in part
        ):
            raise ValueError(f"Invalid wildcard comparator: {text!r}")
        comparators.append(Comparator(operator, version, wildcard))
    return comparators


def matches_requirement(requirement: list[Comparator], version: Version) -> bool:
    """Match all comparators of a requirement."""
    return all(_matches(comparator, version) for comparator in requirement)


def _matches(comparator: Comparator, version: Version) -> bool:
    """Match one comparator, using lines for versions without a prerelease."""
    operator, target = comparator.operator, comparator.version
    if comparator.wildcard:
        in_line = _compare_line(version, target) == 0
        return not in_line if operator == "!=" else in_line
    if operator in ("=", "==", "!=") or target.prerelease:
        comparison = version.compare(target)
        return {
            "=": comparison == 0,
            "==": comparison == 0,
            "!=": comparison != 0,
            ">": comparison > 0,
            ">=": comparison >= 0,
            "<": comparison < 0,
            "<=": comparison <= 0,
        }[operator]
    start = _compare_release(version.release, target.release)
    line = _compare_line(version, target)
    return {
        ">=": start >= 0,
        "<": start < 0,
        ">": line > 0,
        "<=": line <= 0,
    }[operator]


def _compare_release(left: tuple[int, ...], right: tuple[int, ...]) -> int:
    """Compare release components, ignoring trailing zeros and prereleases."""
    for a, b in zip_longest(left, right, fillvalue=0):
        if a != b:
            return 1 if a > b else -1
    return 0


def _compare_line(version: Version, target: Version) -> int:
    """Compare a version with the line of `target`, at the precision it spells."""
    precision = len(target.release)
    prefix = (*version.release, *(0,) * precision)[:precision]
    return _compare_release(prefix, target.release)
