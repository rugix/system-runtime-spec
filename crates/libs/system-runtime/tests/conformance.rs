use serde::Deserialize;
use system_runtime::compatibility::check_components;
use system_runtime::evaluation::evaluate_health_with_values;
use system_runtime::types::component::{
    CapabilitySelectorDeclaration, CompatibilityReport, ComponentDeclaration,
};
use system_runtime::types::health::{HealthContext, HealthNode, HealthReport, HealthStatus};
use system_runtime::types::value::{ValueDefinition, ValueObservation};

#[derive(Deserialize)]
struct Fixtures {
    versions: Vec<VersionCase>,
    components: Vec<ComponentCase>,
    health: Vec<HealthCase>,
}

#[derive(Deserialize)]
struct VersionCase {
    version: String,
    requirement: String,
    matches: Option<bool>,
    #[serde(default)]
    error: bool,
}

#[derive(Deserialize)]
struct ComponentCase {
    name: String,
    components: Vec<ComponentDeclaration>,
    report: Option<CompatibilityReport>,
    #[serde(default)]
    error: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HealthCase {
    name: String,
    nodes: Vec<HealthNode>,
    statuses: Vec<HealthStatus>,
    context: HealthContext,
    #[serde(default)]
    values: Vec<ValueDefinition>,
    #[serde(default)]
    value_observations: Vec<ValueObservation>,
    report: Option<HealthReport>,
    #[serde(default)]
    error: bool,
}

/// Load language-independent specification use cases.
fn fixtures() -> Fixtures {
    serde_json::from_str(include_str!("../../../../tests/fixtures/conformance.json")).unwrap()
}

/// Construct a declaration for a version matching use case.
fn component(id: &str) -> ComponentDeclaration {
    ComponentDeclaration {
        id: id.into(),
        origin: None,
        version: None,
        provides: None,
        claims: None,
        requires: None,
        conflicts: None,
    }
}

/// Version requirements preserve numeric precision and Rugix ordering.
#[test]
fn version_conformance() {
    for case in fixtures().versions {
        let mut provider = component("provider");
        provider.version = Some(case.version.clone());
        let mut consumer = component("consumer");
        consumer.requires = Some(vec![CapabilitySelectorDeclaration {
            id: "provider".into(),
            version: Some(case.requirement.clone()),
            value: None,
        }]);
        let report = check_components(&[provider, consumer]);
        if case.error {
            assert!(
                report.is_err(),
                "{} against {}",
                case.version,
                case.requirement
            );
        } else {
            assert_eq!(
                report.unwrap().consistent,
                case.matches.unwrap(),
                "{} against {}",
                case.version,
                case.requirement
            );
        }
    }
}

/// Whole compositions produce the shared constraint diagnostics.
#[test]
fn component_conformance() {
    for case in fixtures().components {
        let report = check_components(&case.components);
        if case.error {
            assert!(report.is_err(), "{}", case.name);
        } else {
            assert_eq!(report.unwrap(), case.report.unwrap(), "{}", case.name);
        }
    }
}

/// Health evaluation preserves uncertainty, identity, and finite freshness.
#[test]
fn health_conformance() {
    for case in fixtures().health {
        let report = evaluate_health_with_values(
            &case.nodes,
            &case.statuses,
            &case.values,
            &case.value_observations,
            &case.context,
        );
        if case.error {
            assert!(report.is_err(), "{}", case.name);
        } else {
            assert_eq!(report.unwrap(), case.report.unwrap(), "{}", case.name);
        }
    }
}
