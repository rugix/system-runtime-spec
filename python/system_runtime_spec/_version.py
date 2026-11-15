"""Rugix-compatible version ordering and comparator requirements."""

import re
from dataclasses import dataclass
from itertools import zip_longest

_VERSION = re.compile(
    r"([0-9]+(?:\.[0-9]+)*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?"
)
_COMPARATOR = re.compile(r"(==|!=|>=|<=|=|>|<)?\s*(.+)")
_MAX = 2**64 - 1


@dataclass(frozen=True)
class Version:
    """Normalized numeric release and ordered prerelease identifiers."""

    release: tuple[int, ...]
    prerelease: tuple[int | str, ...]

    @classmethod
    def parse(cls, text: str) -> "Version":
        match = _VERSION.fullmatch(text.strip())
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

    def same_release(self, other: "Version") -> bool:
        """Compare release components only, ignoring trailing zeros."""
        return all(
            left == right
            for left, right in zip_longest(self.release, other.release, fillvalue=0)
        )

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


def parse_requirement(text: str) -> list[tuple[str, Version]]:
    """Parse comma-separated comparators, with a single wildcard allowed."""
    if text.strip() == "*":
        return []
    comparators = []
    for part in text.split(","):
        match = _COMPARATOR.fullmatch(part.strip())
        if match is None:
            raise ValueError(f"Invalid version requirement: {text!r}")
        comparators.append((match[1] or "=", Version.parse(match[2])))
    return comparators


def matches_requirement(
    requirement: list[tuple[str, Version]], version: Version
) -> bool:
    """Match all comparators; `<V` also excludes prereleases of a final release V."""
    for operator, target in requirement:
        comparison = version.compare(target)
        prerelease_of_bound = (
            not target.prerelease
            and bool(version.prerelease)
            and version.same_release(target)
        )
        if not {
            "=": comparison == 0,
            "==": comparison == 0,
            "!=": comparison != 0,
            ">": comparison > 0,
            ">=": comparison >= 0,
            "<": comparison < 0 and not prerelease_of_bound,
            "<=": comparison <= 0,
        }[operator]:
            return False
    return True
