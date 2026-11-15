"""Evaluate the compatibility of a complete component composition."""

from collections import Counter
from collections.abc import Sequence

from ._version import Comparator, Version, matches_requirement, parse_requirement
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


def check_components(components: Sequence[ComponentDeclaration]) -> CompatibilityReport:
    """Validate declarations and report every violated composition constraint.

    Invalid version syntax or simultaneous version and value fields raise ValueError.
    """
    provided: list[tuple[str, CapabilityDeclaration, Version | None]] = []
    selectors: list[
        tuple[str, bool, CapabilitySelectorDeclaration, list[Comparator] | None]
    ] = []
    claims: dict[str, list[str]] = {}
    for component in components:
        implicit = CapabilityDeclaration(id=component.id, version=component.version)
        for capability in [implicit, *(component.provides or [])]:
            _validate(capability)
            version = (
                Version.parse(capability.version)
                if capability.version is not None
                else None
            )
            provided.append((component.id, capability, version))
        for claim in sorted({claim.id for claim in component.claims or []}):
            claims.setdefault(claim, []).append(component.id)
        for conflict, declarations in [
            (False, component.requires),
            (True, component.conflicts),
        ]:
            for selector in declarations or []:
                _validate(selector)
                requirement = (
                    parse_requirement(selector.version)
                    if selector.version is not None
                    else None
                )
                selectors.append((component.id, conflict, selector, requirement))
    problems: list[CompatibilityProblem] = [
        CompatibilityProblem_DuplicateComponent(id=id)
        for id, count in sorted(
            Counter(component.id for component in components).items()
        )
        if count > 1
    ]
    problems.extend(
        CompatibilityProblem_DuplicateClaim(id=id, components=owners)
        for id, owners in sorted(claims.items())
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
    return CompatibilityReport(consistent=not problems, problems=problems)


def _validate(
    declaration: CapabilityDeclaration | CapabilitySelectorDeclaration,
) -> None:
    """Reject ambiguous capability declarations before matching."""
    if declaration.version is not None and declaration.value is not None:
        raise ValueError(f"Capability {declaration.id!r} has both version and value")
