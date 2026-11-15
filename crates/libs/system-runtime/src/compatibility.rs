use std::collections::BTreeMap;

use anyver::{Version, VersionReq};

use crate::Records;
use crate::types::common::{Diagnostic, RecordKind, Severity};
use crate::types::component::*;

impl Records {
    /// Check the composition of the component declarations.
    ///
    /// The report contains the diagnostics about component declarations and directory
    /// configurations. The composition is consistent if there are no problems and none of
    /// these diagnostics has the severity `Error`.
    #[tracing::instrument(level = "debug", skip_all, fields(components = self.components.len()))]
    pub fn check_compatibility(&self) -> CompatibilityReport {
        let diagnostics = self.diagnostics_about(&[
            RecordKind::ComponentDeclaration,
            RecordKind::DirectoryConfig,
        ]);
        let declarations: Vec<&ComponentDeclaration> = self.components().collect();
        check(&declarations, diagnostics)
    }

    /// Build the candidate composition of an update.
    ///
    /// The update replaces all component declarations of `owners` with the component
    /// declarations of `update`. A new declaration without a replaced owner is skipped with an
    /// error diagnostic. The candidate has the current records and diagnostics, and the
    /// diagnostics of `update` about component declarations.
    ///
    /// ```
    /// use system_runtime_spec::Records;
    /// use system_runtime_spec::types::component::ComponentDeclaration;
    ///
    /// let os = |version: &str| {
    ///     ComponentDeclaration::new("os")
    ///         .with_owner(Some("rugix.system".into()))
    ///         .with_version(Some(version.into()))
    /// };
    /// let mut records = Records::new();
    /// records.push(os("1.0"));
    /// let mut update = Records::new();
    /// update.push(os("2.0"));
    /// let candidate = records.with_update(&["rugix.system"], &update);
    /// assert_eq!(candidate.components().next().unwrap().version.as_deref(), Some("2.0"));
    /// ```
    pub fn with_update(&self, owners: &[&str], update: &Records) -> Records {
        let replaced = |declaration: &ComponentDeclaration| {
            declaration
                .owner
                .as_deref()
                .is_some_and(|owner| owners.contains(&owner))
        };
        let mut candidate = self.clone();
        candidate
            .components
            .retain(|record| !replaced(&record.value));
        candidate
            .diagnostics
            .extend(update.diagnostics_about(&[RecordKind::ComponentDeclaration]));
        for record in &update.components {
            if replaced(&record.value) {
                candidate.components.push(record.clone());
            } else {
                candidate.diagnostics.push(Diagnostic::new(
                    Severity::Error,
                    RecordKind::ComponentDeclaration,
                    format!(
                        "new declaration {:?} does not have a replaced owner",
                        record.value.id
                    ),
                ));
            }
        }
        candidate
    }
}

fn check(
    declarations: &[&ComponentDeclaration],
    diagnostics: Vec<Diagnostic>,
) -> CompatibilityReport {
    let mut capabilities = Vec::new();
    let mut selectors = Vec::new();
    let mut ids = BTreeMap::<&[u8], (&str, usize)>::new();
    let mut claims = BTreeMap::<&[u8], (&str, Vec<String>)>::new();
    for component in declarations {
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
