"""Check the compatibility of compositions."""

from collections import Counter
from collections.abc import Sequence

from ._version import Comparator, Version, matches_requirement, parse_requirement
from .models.common import Diagnostic
from .models.component import (
    CapabilityDeclaration,
    CapabilitySelectorDeclaration,
    CompatibilityProblem,
    CompatibilityProblem_Conflict,
    CompatibilityProblem_DuplicateClaim,
    CompatibilityProblem_DuplicateComponent,
    CompatibilityProblem_UnsatisfiedRequirement,
    CompatibilityReport,
    ComponentDeclaration,
)


def check(
    declarations: Sequence[ComponentDeclaration], diagnostics: Sequence[Diagnostic]
) -> CompatibilityReport:
    """Check a composition of valid declarations, with the diagnostics about it."""
    provided: list[tuple[str, CapabilityDeclaration, Version | None]] = []
    selectors: list[
        tuple[str, bool, CapabilitySelectorDeclaration, list[Comparator] | None]
    ] = []
    claims: dict[str, list[str]] = {}
    for component in declarations:
        implicit = CapabilityDeclaration(id=component.id, version=component.version)
        for capability in [implicit, *(component.provides or [])]:
            version = capability.version
            provided.append(
                (
                    component.id,
                    capability,
                    Version.parse(version) if version is not None else None,
                )
            )
        for claim in dict.fromkeys(claim.id for claim in component.claims or []):
            claims.setdefault(claim, []).append(component.id)
        for conflict, entries in (
            (False, component.requires),
            (True, component.conflicts),
        ):
            for selector in entries or []:
                text = selector.version
                selectors.append(
                    (
                        component.id,
                        conflict,
                        selector,
                        parse_requirement(text) if text is not None else None,
                    )
                )
    problems: list[CompatibilityProblem] = [
        CompatibilityProblem_DuplicateComponent(id=id)
        for id, count in sorted(
            Counter(component.id for component in declarations).items(),
            key=lambda item: item[0].encode(),
        )
        if count > 1
    ]
    problems.extend(
        CompatibilityProblem_DuplicateClaim(id=id, components=owners)
        for id, owners in sorted(claims.items(), key=lambda item: item[0].encode())
        if len(owners) > 1
    )
    for component_id, conflict, selector, requirement in selectors:
        matches = [
            (provider, capability)
            for provider, capability, version in provided
            if capability.id == selector.id
            and (selector.value is None or selector.value == capability.value)
            and (
                requirement is None
                or (version is not None and matches_requirement(requirement, version))
            )
        ]
        if conflict:
            problems.extend(
                CompatibilityProblem_Conflict(
                    component=component_id,
                    selector=selector,
                    provider=provider,
                    capability=capability,
                )
                for provider, capability in matches
            )
        elif not matches:
            problems.append(
                CompatibilityProblem_UnsatisfiedRequirement(
                    component=component_id, selector=selector
                )
            )
    return CompatibilityReport(
        consistent=not problems
        and all(item.severity != "Error" for item in diagnostics),
        problems=problems,
        diagnostics=list(diagnostics),
    )
