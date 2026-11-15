"""Check the compatibility of compositions and candidate compositions."""

from collections import Counter
from collections.abc import Iterable, Sequence

from ._records import Json, JsonObject, decode_all, diagnostic, to_json
from ._version import Comparator, Version, matches_requirement, parse_requirement
from .models.common import Diagnostic
from .models.component import CompatibilityReport


def check_components(
    components: Iterable[Json], diagnostics: Sequence[Diagnostic] = ()
) -> CompatibilityReport:
    """Check a composition, given as component declarations and diagnostics.

    Declarations are JSON objects or models. Invalid declarations are skipped with an
    error diagnostic. The composition is consistent if there are no problems and no
    diagnostics with the severity `Error`.
    """
    all_diagnostics = list(diagnostics)
    declarations = [
        decoded.record
        for decoded in decode_all("ComponentDeclaration", components, all_diagnostics)
    ]
    provided: list[tuple[str, JsonObject, Version | None]] = []
    selectors: list[tuple[str, bool, JsonObject, list[Comparator] | None]] = []
    claims: dict[str, list[str]] = {}
    for component in declarations:
        implicit = {"id": component["id"]}
        if "version" in component:
            implicit["version"] = component["version"]
        for capability in [implicit, *(component.get("provides") or [])]:
            version = capability.get("version")
            provided.append(
                (
                    component["id"],
                    capability,
                    Version.parse(version) if version is not None else None,
                )
            )
        for claim in dict.fromkeys(
            claim["id"] for claim in component.get("claims") or []
        ):
            claims.setdefault(claim, []).append(component["id"])
        for conflict, name in ((False, "requires"), (True, "conflicts")):
            for selector in component.get(name) or []:
                requirement = selector.get("version")
                selectors.append(
                    (
                        component["id"],
                        conflict,
                        selector,
                        parse_requirement(requirement)
                        if requirement is not None
                        else None,
                    )
                )
    problems: list[JsonObject] = [
        {"problem": "DuplicateComponent", "id": id}
        for id, count in sorted(
            Counter(component["id"] for component in declarations).items(),
            key=lambda item: item[0].encode(),
        )
        if count > 1
    ]
    problems.extend(
        {"problem": "DuplicateClaim", "id": id, "components": owners}
        for id, owners in sorted(claims.items(), key=lambda item: item[0].encode())
        if len(owners) > 1
    )
    for component_id, conflict, selector, requirement in selectors:
        matches = [
            (provider, capability)
            for provider, capability, version in provided
            if capability["id"] == selector["id"]
            and (
                "value" not in selector or selector["value"] == capability.get("value")
            )
            and (
                requirement is None
                or (version is not None and matches_requirement(requirement, version))
            )
        ]
        if conflict:
            problems.extend(
                {
                    "problem": "Conflict",
                    "component": component_id,
                    "selector": _declared(selector),
                    "provider": provider,
                    "capability": _declared(capability),
                }
                for provider, capability in matches
            )
        elif not matches:
            problems.append(
                {
                    "problem": "UnsatisfiedRequirement",
                    "component": component_id,
                    "selector": _declared(selector),
                }
            )
    reported = [to_json(item) for item in all_diagnostics]
    return CompatibilityReport.model_validate(
        {
            "consistent": not problems
            and all(item["severity"] != "Error" for item in reported),
            "problems": problems,
            "diagnostics": reported,
        }
    )


def _declared(capability: JsonObject) -> JsonObject:
    """Keep the fields that the specification defines for capabilities and selectors."""
    return {
        key: capability[key] for key in ("id", "version", "value") if key in capability
    }


def candidate_composition(
    components: Iterable[Json],
    diagnostics: Sequence[Diagnostic],
    owners: Iterable[str],
    declarations: Iterable[Json],
) -> tuple[list[JsonObject], list[Diagnostic]]:
    """Build the composition that would result from an update.

    The update replaces all declarations of `owners` with `declarations`. A new
    declaration without a replaced owner is skipped with an error diagnostic.
    """
    replaced = set(owners)
    result = [
        component
        for component in map(to_json, components)
        if component.get("owner") not in replaced
    ]
    result_diagnostics = list(diagnostics)
    for declaration in declarations:
        value = to_json(declaration)
        if isinstance(value, dict) and value.get("owner") in replaced:
            result.append(value)
        else:
            result_diagnostics.append(
                diagnostic(
                    "Error",
                    "ComponentDeclaration",
                    "new declaration does not have a replaced owner",
                )
            )
    return result, result_diagnostics
