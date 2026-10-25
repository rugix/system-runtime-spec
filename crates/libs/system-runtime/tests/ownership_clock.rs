use serde::Deserialize;
use sidex::types::Timestamp;
use system_runtime::clock::status_timestamp;
use system_runtime::compatibility::check_components;
use system_runtime::evaluation::evaluate_health;
use system_runtime::snapshot::without_origin;
use system_runtime::types::component::ComponentDeclaration;
use system_runtime::types::filesystem::RuntimeSnapshot;
use system_runtime::types::health::{HealthClockSample, HealthSnapshot, HealthState, HealthStatus};

#[derive(Deserialize)]
struct Fixtures {
    origins: Vec<OriginCase>,
    timestamps: Vec<TimestampCase>,
}

#[derive(Deserialize)]
struct OriginCase {
    name: String,
    snapshot: RuntimeSnapshot,
    origin: String,
    expected: RuntimeSnapshot,
    replacement: Option<ComponentDeclaration>,
    consistent: bool,
}

#[derive(Deserialize)]
struct TimestampCase {
    name: String,
    status: HealthStatus,
    clock: HealthClockSample,
    expected: Option<Timestamp>,
}

/// Load language-independent update and clock correlation use cases.
fn fixtures() -> Fixtures {
    serde_json::from_str(include_str!(
        "../../../../tests/fixtures/ownership-clock.json"
    ))
    .unwrap()
}

/// Remove obsolete IDs by owner, preserving sibling owners and constraints.
#[test]
fn origin_updates() {
    for case in fixtures().origins {
        let original = case.snapshot.clone();
        let mut candidate = without_origin(&case.snapshot, &case.origin);
        assert_eq!(candidate, case.expected, "{}", case.name);
        assert_eq!(
            check_components(&candidate.components).unwrap().consistent,
            case.consistent,
            "{}",
            case.name
        );
        if let Some(replacement) = case.replacement {
            candidate.components.push(replacement);
            assert!(
                check_components(&candidate.components).unwrap().consistent,
                "{}",
                case.name
            );
        }
        assert_eq!(case.snapshot, original, "{}", case.name);
    }
}

/// Map boot milliseconds to UTC without rounding large integers or renewing evidence.
#[test]
fn timestamp_correlation() {
    for case in fixtures().timestamps {
        assert_eq!(
            status_timestamp(&case.status, &case.clock),
            case.expected,
            "{}",
            case.name
        );
    }
}

/// Round-trip an owned graph while preserving expiry and observation time.
#[test]
fn health_transport() {
    let mut cases = fixtures();
    let source = cases.origins.remove(0).snapshot;
    let clock = cases.timestamps.remove(0).clock;
    let snapshot = HealthSnapshot {
        clock,
        nodes: source.health_nodes,
        statuses: source.health_statuses,
        values: source.values,
        value_observations: source.value_observations,
    };
    let encoded = sidex::formats::json::to_string(&snapshot).unwrap();
    let mut received: HealthSnapshot = sidex::formats::json::from_str(&encoded).unwrap();
    assert_eq!(received, snapshot);
    let report =
        evaluate_health(&received.nodes, &received.statuses, &received.clock.context).unwrap();
    assert_eq!(
        report.nodes[1].origin.as_deref(),
        Some("rugix.apps.example")
    );
    assert_eq!(report.nodes[1].state, HealthState::Healthy);
    assert_eq!(
        status_timestamp(&received.statuses[1], &received.clock)
            .unwrap()
            .to_string(),
        "2026-10-14T11:59:58.623Z"
    );
    received.clock.context.now_ms = 5500;
    assert_eq!(
        evaluate_health(&received.nodes, &received.statuses, &received.clock.context)
            .unwrap()
            .nodes[1]
            .state,
        HealthState::Unknown
    );
    assert_eq!(received.statuses[1].observed_at_ms, 3500);
}

/// Capture a valid wall-clock and Linux boot-time correlation for transport.
#[cfg(target_os = "linux")]
#[test]
fn current_clock_sample() {
    let clock = system_runtime::clock::current_clock_sample().unwrap();
    assert!(!clock.context.boot_id.is_empty());
    let encoded = sidex::formats::json::to_string(&clock).unwrap();
    assert_eq!(
        sidex::formats::json::from_str::<HealthClockSample>(&encoded).unwrap(),
        clock
    );
}
