//! Evaluate a complete component composition using Rugix's capability model.

use std::collections::{BTreeMap, BTreeSet};

use anyver::{Version, VersionReq};

use crate::Error;
use crate::component::*;

/// Validate declarations and report every violated composition constraint.
///
/// Invalid versions or simultaneous version and value fields return an error.
#[tracing::instrument(level = "debug", skip_all, fields(components = components.len()))]
pub fn check_components(components: &[ComponentDeclaration]) -> Result<CompatibilityReport, Error> {
    let mut capabilities = Vec::new();
    let mut selectors = Vec::new();
    let mut ids = BTreeMap::<&str, usize>::new();
    let mut claims = BTreeMap::<&str, Vec<String>>::new();
    for component in components {
        *ids.entry(&component.id).or_default() += 1;
        let implicit = CapabilityDeclaration {
            id: component.id.clone(),
            version: component.version.clone(),
            value: None,
        };
        for capability in std::iter::once(&implicit).chain(component.provides.iter().flatten()) {
            validate(&capability.id, &capability.version, &capability.value)?;
            let version = capability
                .version
                .as_deref()
                .map(Version::parse)
                .transpose()
                .map_err(|error| Error::Invalid(error.to_string()))?;
            capabilities.push((component.id.clone(), capability.clone(), version));
        }
        let own_claims: BTreeSet<_> = component
            .claims
            .iter()
            .flatten()
            .map(|claim| claim.id.as_str())
            .collect();
        for id in own_claims {
            claims.entry(id).or_default().push(component.id.clone());
        }
        for (conflict, declarations) in [(false, &component.requires), (true, &component.conflicts)]
        {
            for selector in declarations.iter().flatten() {
                validate(&selector.id, &selector.version, &selector.value)?;
                let requirement = selector
                    .version
                    .as_deref()
                    .map(VersionReq::parse)
                    .transpose()
                    .map_err(|error| Error::Invalid(error.to_string()))?;
                selectors.push((&component.id, conflict, selector, requirement));
            }
        }
    }
    let mut problems = Vec::new();
    for (id, count) in ids {
        if count > 1 {
            problems.push(CompatibilityProblem::DuplicateComponent(
                ComponentIdProblem { id: id.into() },
            ));
        }
    }
    for (id, owners) in claims {
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
    Ok(CompatibilityReport {
        consistent: problems.is_empty(),
        problems,
    })
}

/// Reject ambiguous declarations before parsing version predicates.
fn validate(id: &str, version: &Option<String>, value: &Option<String>) -> Result<(), Error> {
    if version.is_some() && value.is_some() {
        return Err(Error::Invalid(format!(
            "capability {id:?} has both version and value"
        )));
    }
    Ok(())
}
