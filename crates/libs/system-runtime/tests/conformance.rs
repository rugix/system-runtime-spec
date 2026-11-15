//! Run the conformance suite shared by all implementations.

use std::path::{Path, PathBuf};

use sidex::types::JsonValue;
use system_runtime_spec::types::common::{Diagnostic, Severity};
use system_runtime_spec::types::component::ComponentDeclaration;
use system_runtime_spec::types::health::{
    HealthClockSample, HealthContext, HealthNode, HealthStatus,
};
use system_runtime_spec::types::metric::{MetricDefinition, MetricSample};
use system_runtime_spec::{Parts, Records, Snapshot};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures")
}

fn cases(section: &str) -> Vec<JsonValue> {
    let text = std::fs::read_to_string(fixtures().join("conformance.json")).unwrap();
    let value: JsonValue = sidex::formats::json::from_str(&text).unwrap();
    value.get(section).unwrap().as_seq().unwrap().to_vec()
}

fn list(value: &JsonValue, field: &str) -> Vec<JsonValue> {
    value
        .get(field)
        .and_then(JsonValue::as_seq)
        .map(|items| items.to_vec())
        .unwrap_or_default()
}

fn decode<T: sidex::Decode>(value: &JsonValue) -> T {
    sidex::formats::json::from_value(value).unwrap()
}

fn to_json(value: &impl sidex::Encode) -> JsonValue {
    sidex::formats::json::to_value(value).unwrap()
}

fn records(value: &JsonValue) -> Records {
    let mut records = Records::new();
    for diagnostic in list(value, "diagnostics") {
        records.push_diagnostic(decode::<Diagnostic>(&diagnostic));
    }
    for record in list(value, "components") {
        records.push_json::<ComponentDeclaration>(record);
    }
    for record in list(value, "healthNodes") {
        records.push_json::<HealthNode>(record);
    }
    for record in list(value, "healthStatuses") {
        records.push_json::<HealthStatus>(record);
    }
    for record in list(value, "metrics") {
        records.push_json::<MetricDefinition>(record);
    }
    for record in list(value, "metricSamples") {
        records.push_json::<MetricSample>(record);
    }
    records
}

fn originals(records: &Records) -> JsonValue {
    let clock = HealthClockSample::new(
        HealthContext::new("boot", 0),
        "2026-01-01T00:00:00Z".parse().unwrap(),
    );
    Snapshot::new(records.clone(), clock, Parts::ALL).to_json()
}

/// Keep the severity and kind of diagnostics, whose messages are not specified.
fn brief(report: &impl sidex::Encode) -> JsonValue {
    let JsonValue::Map(mut report) = to_json(report) else {
        unreachable!("reports are objects")
    };
    let mut entries: Vec<(String, String)> = report
        .get("diagnostics")
        .and_then(JsonValue::as_seq)
        .into_iter()
        .flatten()
        .map(|item| {
            let field = |name: &str| item.get(name).unwrap().as_str().unwrap().to_owned();
            (field("kind"), field("severity"))
        })
        .collect();
    entries.sort();
    report.insert(
        "diagnostics".into(),
        entries
            .into_iter()
            .map(|(kind, severity)| JsonValue::from_iter([("severity", severity), ("kind", kind)]))
            .collect::<Vec<_>>()
            .into(),
    );
    JsonValue::Map(report)
}

fn name(case: &JsonValue) -> &str {
    case.get("name")
        .and_then(JsonValue::as_str)
        .unwrap_or_default()
}

#[test]
fn versions() {
    for case in cases("versions") {
        let version = case.get("version").unwrap().clone();
        let requirement = case.get("requirement").unwrap().clone();
        let mut records = Records::new();
        records.push_json::<ComponentDeclaration>(JsonValue::from_iter([
            ("id", JsonValue::from("provider")),
            ("version", version),
        ]));
        records.push_json::<ComponentDeclaration>(JsonValue::from_iter([
            ("id", JsonValue::from("consumer")),
            (
                "requires",
                vec![JsonValue::from_iter([
                    ("id", JsonValue::from("provider")),
                    ("version", requirement),
                ])]
                .into(),
            ),
        ]));
        let report = records.check_compatibility();
        let error = case.get("error").and_then(JsonValue::as_bool) == Some(true);
        let errors = report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == Severity::Error);
        assert_eq!(errors, error, "{case}");
        if !error {
            assert_eq!(
                Some(report.consistent),
                case.get("matches").and_then(JsonValue::as_bool),
                "{case}"
            );
        }
    }
}

#[test]
fn components() {
    for case in cases("components") {
        let records = records(&case);
        let report = records.check_compatibility();
        assert_eq!(
            &brief(&report),
            case.get("report").unwrap(),
            "{}",
            name(&case)
        );
    }
}

#[test]
fn typed_records() {
    let mut records = Records::new();
    records.push(
        ComponentDeclaration::new("os")
            .with_owner(Some("rugix.system".into()))
            .with_version(Some("2.1".into())),
    );
    records.push(ComponentDeclaration::new("not an ID"));
    let ids: Vec<&str> = records.components().map(|c| c.id.as_str()).collect();
    assert_eq!(ids, ["os"]);
    let severities: Vec<Severity> = records.diagnostics().iter().map(|d| d.severity).collect();
    assert_eq!(severities, [Severity::Error]);
    let os = JsonValue::from_iter([("id", "os"), ("owner", "rugix.system"), ("version", "2.1")]);
    assert_eq!(
        originals(&records).get("components"),
        Some(&JsonValue::from(vec![os]))
    );
}

#[test]
fn candidates() {
    for case in cases("candidates") {
        let owners: Vec<String> = list(&case, "replace")
            .iter()
            .map(|owner| owner.as_str().unwrap().to_owned())
            .collect();
        let owners: Vec<&str> = owners.iter().map(String::as_str).collect();
        let update = records(&JsonValue::from_iter([(
            "components",
            JsonValue::from(list(&case, "new")),
        )]));
        let candidate = records(&case).with_update(&owners, &update);
        assert_eq!(
            originals(&candidate).get("components"),
            case.get("candidate"),
            "{}",
            name(&case)
        );
        assert_eq!(
            &brief(&candidate.check_compatibility()),
            case.get("report").unwrap(),
            "{}",
            name(&case)
        );
    }
}

#[test]
fn health() {
    for case in cases("health") {
        let records = records(case.get("records").unwrap());
        let context: HealthContext = decode(case.get("context").unwrap());
        let report = records.evaluate_health(&context);
        assert_eq!(
            &brief(&report),
            case.get("report").unwrap(),
            "{}",
            name(&case)
        );
    }
}

#[test]
fn snapshots() {
    for case in cases("snapshots") {
        let original = case.get("snapshot").unwrap();
        let result = Snapshot::from_json(original);
        let valid = case.get("valid").and_then(JsonValue::as_bool) == Some(true);
        assert_eq!(result.is_ok(), valid, "{}", name(&case));
        let Ok(snapshot) = result else {
            continue;
        };
        let json = snapshot.to_json();
        assert_eq!(
            Snapshot::from_json(&json).unwrap(),
            snapshot,
            "{}",
            name(&case)
        );
        for field in [
            "components",
            "healthNodes",
            "healthStatuses",
            "metrics",
            "metricSamples",
        ] {
            assert_eq!(json.get(field), original.get(field), "{}", name(&case));
        }
        let diagnostics = JsonValue::from_iter([(
            "diagnostics",
            JsonValue::from(
                snapshot
                    .records()
                    .diagnostics()
                    .iter()
                    .map(to_json)
                    .collect::<Vec<_>>(),
            ),
        )]);
        assert_eq!(
            brief(&diagnostics).get("diagnostics"),
            case.get("diagnostics"),
            "{}",
            name(&case)
        );
    }
}

#[test]
fn timestamps() {
    for case in cases("timestamps") {
        let clock: HealthClockSample = decode(case.get("clock").unwrap());
        let evidence: HealthStatus = decode(case.get("evidence").unwrap());
        let expected = case
            .get("expected")
            .and_then(JsonValue::as_str)
            .map(|text| text.parse::<sidex::types::Timestamp>().unwrap());
        assert_eq!(
            clock.estimate_timestamp(&evidence),
            expected,
            "{}",
            name(&case)
        );
    }
}

#[cfg(target_os = "linux")]
#[test]
fn discovery() {
    fn copy_tree(source: &Path, destination: &Path) {
        std::fs::create_dir_all(destination).unwrap();
        for entry in std::fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let target = destination.join(entry.file_name());
            if entry.file_type().unwrap().is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), target).unwrap();
            }
        }
    }

    let fixture = fixtures().join("discovery");
    let text = std::fs::read_to_string(fixture.join("discovery.json")).unwrap();
    let spec: JsonValue = sidex::formats::json::from_str(&text).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("root");
    copy_tree(&fixture.join("root"), &root);
    for link in list(&spec, "symlinks") {
        let path = root.join(link.get("path").unwrap().as_str().unwrap());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::os::unix::fs::symlink(link.get("target").unwrap().as_str().unwrap(), path).unwrap();
    }
    let records = Records::discover(&root);
    let expected = spec.get("expected").unwrap();
    let json = originals(&records);
    for field in [
        "components",
        "healthNodes",
        "healthStatuses",
        "metrics",
        "metricSamples",
    ] {
        assert_eq!(json.get(field), expected.get(field), "{field}");
    }
    let real = std::fs::canonicalize(&root).unwrap();
    let mut found: Vec<(String, String, String)> = records
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            let path = PathBuf::from(diagnostic.path.as_deref().unwrap());
            let relative = path
                .strip_prefix(&real)
                .or_else(|_| path.strip_prefix(&root))
                .unwrap()
                .display()
                .to_string();
            let item = to_json(diagnostic);
            let field = |name: &str| item.get(name).unwrap().as_str().unwrap().to_owned();
            (relative, field("kind"), field("severity"))
        })
        .collect();
    found.sort();
    let expected_diagnostics: Vec<(String, String, String)> = list(expected, "diagnostics")
        .iter()
        .map(|item| {
            let field = |name: &str| item.get(name).unwrap().as_str().unwrap().to_owned();
            (field("path"), field("kind"), field("severity"))
        })
        .collect();
    assert_eq!(found, expected_diagnostics);
}
