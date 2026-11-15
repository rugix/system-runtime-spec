//! Check the compatibility of compositions and candidate compositions.

use std::collections::BTreeMap;

use anyver::{Version, VersionReq};
use sidex::types::JsonValue;

use crate::records::{decode_all, diagnostic};
use crate::types::common::{Diagnostic, RecordKind, Severity};
use crate::types::component::*;

/// Check a composition, given as component declarations and diagnostics.
///
/// Invalid declarations are skipped with an error diagnostic. The composition is consistent
/// if there are no problems and no diagnostics with the severity `Error`.
#[tracing::instrument(level = "debug", skip_all, fields(components = components.len()))]
pub fn check_components(
    components: &[JsonValue],
    diagnostics: &[Diagnostic],
) -> CompatibilityReport {
    let mut diagnostics = diagnostics.to_vec();
    let declarations: Vec<ComponentDeclaration> = decode_all(
        RecordKind::ComponentDeclaration,
        components,
        &mut diagnostics,
    )
    .into_iter()
    .map(|decoded| decoded.value)
    .collect();
    let mut capabilities = Vec::new();
    let mut selectors = Vec::new();
    let mut ids = BTreeMap::<&[u8], (&str, usize)>::new();
    let mut claims = BTreeMap::<&[u8], (&str, Vec<String>)>::new();
    for component in &declarations {
        ids.entry(component.id.as_bytes())
            .or_insert((&component.id, 0))
            .1 += 1;
        let implicit = CapabilityDeclaration {
            id: component.id.clone(),
            version: component.version.clone(),
            value: None,
        };
        for capability in std::iter::once(&implicit).chain(component.provides.iter().flatten()) {
            let version = capability
                .version
                .as_deref()
                .map(|version| Version::parse(version).expect("validated"));
            capabilities.push((component.id.clone(), capability.clone(), version));
        }
        let mut own_claims = Vec::new();
        for claim in component.claims.iter().flatten() {
            if !own_claims.contains(&claim.id.as_str()) {
                own_claims.push(claim.id.as_str());
            }
        }
        for id in own_claims {
            claims
                .entry(id.as_bytes())
                .or_insert((id, Vec::new()))
                .1
                .push(component.id.clone());
        }
        for (conflict, declarations) in [(false, &component.requires), (true, &component.conflicts)]
        {
            for selector in declarations.iter().flatten() {
                let requirement = selector
                    .version
                    .as_deref()
                    .map(|version| VersionReq::parse(version).expect("validated"));
                selectors.push((&component.id, conflict, selector, requirement));
            }
        }
    }
    let mut problems = Vec::new();
    for (id, count) in ids.into_values() {
        if count > 1 {
            problems.push(CompatibilityProblem::DuplicateComponent(
                ComponentIdProblem { id: id.into() },
            ));
        }
    }
    for (id, owners) in claims.into_values() {
        if owners.len() > 1 {
            problems.push(CompatibilityProblem::DuplicateClaim(ClaimProblem {
                id: id.into(),
                components: owners,
            }));
        }
    }
    for (component, conflict, selector, requirement) in selectors {
        let matches: Vec<_> = capabilities
            .iter()
            .filter(|(_, capability, version)| {
                capability.id == selector.id
                    && selector
                        .value
                        .as_ref()
                        .is_none_or(|value| capability.value.as_ref() == Some(value))
                    && requirement.as_ref().is_none_or(|requirement| {
                        version
                            .as_ref()
                            .is_some_and(|version| requirement.matches(version))
                    })
            })
            .collect();
        if conflict {
            for (provider, capability, _) in matches {
                problems.push(CompatibilityProblem::Conflict(ConflictProblem {
                    component: component.clone(),
                    selector: selector.clone(),
                    provider: provider.clone(),
                    capability: capability.clone(),
                }));
            }
        } else if matches.is_empty() {
            problems.push(CompatibilityProblem::UnsatisfiedRequirement(
                SelectorProblem {
                    component: component.clone(),
                    selector: selector.clone(),
                },
            ));
        }
    }
    let consistent = problems.is_empty()
        && diagnostics
            .iter()
            .all(|diagnostic| diagnostic.severity != Severity::Error);
    CompatibilityReport {
        consistent,
        problems,
        diagnostics,
    }
}

/// Build the composition that would result from an update.
///
/// The update replaces all declarations of `owners` with `declarations`. A new declaration
/// without a replaced owner is skipped with an error diagnostic.
pub fn candidate_composition(
    components: &[JsonValue],
    diagnostics: &[Diagnostic],
    owners: &[&str],
    declarations: &[JsonValue],
) -> (Vec<JsonValue>, Vec<Diagnostic>) {
    let replaced = |value: &JsonValue| {
        value
            .get("owner")
            .and_then(JsonValue::as_str)
            .is_some_and(|owner| owners.contains(&owner))
    };
    let mut result: Vec<_> = components
        .iter()
        .filter(|component| !replaced(component))
        .cloned()
        .collect();
    let mut result_diagnostics = diagnostics.to_vec();
    for declaration in declarations {
        if replaced(declaration) {
            result.push(declaration.clone());
        } else {
            result_diagnostics.push(diagnostic(
                Severity::Error,
                RecordKind::ComponentDeclaration,
                "new declaration does not have a replaced owner",
                None,
            ));
        }
    }
    (result, result_diagnostics)
}
