//! Run the conformance suite shared by all implementations.

use std::path::{Path, PathBuf};

use sidex::types::JsonValue;
use system_runtime_spec::clock::estimate_timestamp;
use system_runtime_spec::compatibility::{candidate_composition, check_components};
use system_runtime_spec::evaluation::evaluate_health;
use system_runtime_spec::records::{Records, to_json};
use system_runtime_spec::snapshot::{make_snapshot, read_snapshot};
use system_runtime_spec::types::common::{Diagnostic, Severity};
use system_runtime_spec::types::health::{HealthClockSample, HealthContext};

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

fn diagnostics(value: &JsonValue) -> Vec<Diagnostic> {
    list(value, "diagnostics").iter().map(decode).collect()
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
        let report = check_components(
            &[
                JsonValue::from_iter([("id", JsonValue::from("provider")), ("version", version)]),
                JsonValue::from_iter([
                    ("id", JsonValue::from("consumer")),
                    (
                        "requires",
                        vec![JsonValue::from_iter([
                            ("id", JsonValue::from("provider")),
                            ("version", requirement),
                        ])]
                        .into(),
                    ),
                ]),
            ],
            &[],
        );
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
        let report = check_components(&list(&case, "components"), &diagnostics(&case));
        assert_eq!(
            &brief(&report),
            case.get("report").unwrap(),
            "{}",
            name(&case)
        );
    }
}

#[test]
fn candidates() {
    for case in cases("candidates") {
        let owners: Vec<String> = list(&case, "replace")
            .iter()
            .map(|owner| owner.as_str().unwrap().to_owned())
            .collect();
        let owners: Vec<&str> = owners.iter().map(String::as_str).collect();
        let (components, candidate_diagnostics) = candidate_composition(
            &list(&case, "components"),
            &diagnostics(&case),
            &owners,
            &list(&case, "new"),
        );
        let report = check_components(&components, &candidate_diagnostics);
        assert_eq!(
            &brief(&report),
            case.get("report").unwrap(),
            "{}",
            name(&case)
        );
    }
}

#[test]
fn health() {
    for case in cases("health") {
        let records = case.get("records").unwrap();
        let input = Records {
            health_nodes: list(records, "healthNodes"),
            health_statuses: list(records, "healthStatuses"),
            metrics: list(records, "metrics"),
            metric_samples: list(records, "metricSamples"),
            diagnostics: diagnostics(records),
            ..Records::default()
        };
        let context: HealthContext = decode(case.get("context").unwrap());
        let report = evaluate_health(&input, &context);
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
        let result = read_snapshot(case.get("snapshot").unwrap());
        let valid = case.get("valid").and_then(JsonValue::as_bool) == Some(true);
        assert_eq!(result.is_ok(), valid, "{}", name(&case));
        if let Ok(snapshot) = result {
            let report = brief(&JsonValue::from_iter([(
                "diagnostics",
                JsonValue::from(
                    snapshot
                        .records
                        .diagnostics
                        .iter()
                        .map(to_json)
                        .collect::<Vec<_>>(),
                ),
            )]));
            assert_eq!(
                report.get("diagnostics"),
                case.get("diagnostics"),
                "{}",
                name(&case)
            );
        }
    }
}

#[test]
fn timestamps() {
    for case in cases("timestamps") {
        let clock: HealthClockSample = decode(case.get("clock").unwrap());
        let estimate = estimate_timestamp(case.get("evidence").unwrap(), &clock);
        let expected = case
            .get("expected")
            .and_then(JsonValue::as_str)
            .map(|text| text.parse::<sidex::types::Timestamp>().unwrap());
        assert_eq!(estimate, expected, "{}", name(&case));
    }
}

#[cfg(target_os = "linux")]
#[test]
fn discovery() {
    use system_runtime_spec::discovery::read_records;

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
    let records = read_records(&root);
    let expected = spec.get("expected").unwrap();
    assert_eq!(records.components, list(expected, "components"));
    assert_eq!(records.health_nodes, list(expected, "healthNodes"));
    assert_eq!(records.health_statuses, list(expected, "healthStatuses"));
    assert_eq!(records.metrics, list(expected, "metrics"));
    assert_eq!(records.metric_samples, list(expected, "metricSamples"));
    let real = std::fs::canonicalize(&root).unwrap();
    let mut found: Vec<(String, String, String)> = records
        .diagnostics
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

    let clock: HealthClockSample = sidex::formats::json::from_str(
        r#"{"context": {"bootId": "boot", "nowMs": 100}, "timestamp": "2026-10-14T12:00:00Z"}"#,
    )
    .unwrap();
    let snapshot = read_snapshot(&make_snapshot(&records, &clock, true, true)).unwrap();
    assert!(snapshot.components && snapshot.health);
    assert_eq!(snapshot.records, records);
}
