//! Send records to another system as snapshots.

use sidex::types::{JsonValue, Map, Str};

use crate::records::{Records, SPEC_VERSION, SUPPORTED_MAJOR, parse_spec_version, to_json};
use crate::types::common::{Diagnostic, RecordKind};
use crate::types::health::HealthClockSample;

const HEALTH: [&str; 4] = ["healthNodes", "healthStatuses", "metrics", "metricSamples"];
const KINDS: [&str; 6] = [
    "ComponentDeclaration",
    "HealthNode",
    "HealthStatus",
    "MetricDefinition",
    "MetricSample",
    "DirectoryConfig",
];

/// Parts of the specification whose records a snapshot includes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parts {
    /// Whether the snapshot includes component declarations.
    pub components: bool,
    /// Whether the snapshot includes health nodes, status records, and metrics.
    pub health: bool,
}

impl Parts {
    /// Both parts.
    pub const ALL: Self = Self {
        components: true,
        health: true,
    };
}

impl Default for Parts {
    fn default() -> Self {
        Self::ALL
    }
}

/// A received snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    /// Records of the snapshot; omitted lists are empty.
    pub records: Records,
    /// Correlation of the boot-time clock with wall-clock time.
    pub clock: HealthClockSample,
    /// Parts whose records the snapshot includes.
    pub parts: Parts,
}

/// A snapshot that cannot be read.
#[derive(Debug, thiserror::Error)]
pub enum SnapshotError {
    /// The snapshot follows a version of the specification that this crate does not support.
    #[error("unsupported snapshot version {0:?}")]
    UnsupportedVersion(Option<String>),
    /// The clock sample is missing or invalid.
    #[error("invalid clock sample: {0}")]
    InvalidClock(String),
    /// A field that holds a list holds something else.
    #[error("{0} is not a list")]
    NotAList(&'static str),
    /// The snapshot includes only some lists of the health part.
    #[error("snapshot includes only some lists of the health part")]
    PartialHealth,
    /// A diagnostic is invalid.
    #[error("invalid diagnostic: {0}")]
    InvalidDiagnostic(String),
}

/// Create a snapshot with the original records of the given parts and their diagnostics.
///
/// Take the clock sample after reading the records.
pub fn make_snapshot(records: &Records, clock: &HealthClockSample, parts: Parts) -> JsonValue {
    let Parts { components, health } = parts;
    let mut snapshot: Map<Str, JsonValue> = Map::new();
    snapshot.insert("specVersion".into(), SPEC_VERSION.into());
    snapshot.insert("clock".into(), to_json(clock));
    if components {
        snapshot.insert("components".into(), records.components.clone().into());
    }
    if health {
        for (field, list) in HEALTH.into_iter().zip([
            &records.health_nodes,
            &records.health_statuses,
            &records.metrics,
            &records.metric_samples,
        ]) {
            snapshot.insert(field.into(), list.clone().into());
        }
    }
    let included = |kind: RecordKind| match kind {
        RecordKind::DirectoryConfig => true,
        RecordKind::ComponentDeclaration => components,
        _ => health,
    };
    let diagnostics: Vec<JsonValue> = records
        .diagnostics
        .iter()
        .filter(|diagnostic| included(diagnostic.kind))
        .map(to_json)
        .collect();
    if !diagnostics.is_empty() {
        snapshot.insert("diagnostics".into(), diagnostics.into());
    }
    JsonValue::Map(snapshot)
}

fn valid_timestamp(text: &str) -> bool {
    let bytes = text.as_bytes();
    let digits = |range: std::ops::Range<usize>| {
        bytes
            .get(range)
            .is_some_and(|part| part.iter().all(u8::is_ascii_digit))
    };
    let fraction = bytes
        .get(19..bytes.len().saturating_sub(1))
        .unwrap_or_default();
    bytes.len() >= 20
        && digits(0..4)
        && bytes[4] == b'-'
        && digits(5..7)
        && bytes[7] == b'-'
        && digits(8..10)
        && bytes[10] == b'T'
        && digits(11..13)
        && bytes[13] == b':'
        && digits(14..16)
        && bytes[16] == b':'
        && digits(17..19)
        && &bytes[17..19] != b"60"
        && bytes.ends_with(b"Z")
        && (fraction.is_empty()
            || (fraction.len() > 1
                && fraction[0] == b'.'
                && fraction[1..].iter().all(u8::is_ascii_digit)))
}

fn list(value: &JsonValue, field: &'static str) -> Result<Vec<JsonValue>, SnapshotError> {
    match value.get(field) {
        None => Ok(Vec::new()),
        Some(JsonValue::Seq(items)) => Ok(items.to_vec()),
        Some(_) => Err(SnapshotError::NotAList(field)),
    }
}

/// Read a snapshot, applying the fallbacks to its diagnostics.
///
/// Fail for snapshots of an unsupported version or with an invalid structure. Records are
/// validated when they are checked or evaluated.
pub fn read_snapshot(value: &JsonValue) -> Result<Snapshot, SnapshotError> {
    let version = value.get("specVersion").and_then(JsonValue::as_str);
    if version.and_then(parse_spec_version).map(|(major, _)| major) != Some(SUPPORTED_MAJOR) {
        return Err(SnapshotError::UnsupportedVersion(
            version.map(str::to_owned),
        ));
    }
    let clock = value.get("clock").cloned().unwrap_or_default();
    let timestamp = clock.get("timestamp").and_then(JsonValue::as_str);
    if !timestamp.is_some_and(valid_timestamp) {
        return Err(SnapshotError::InvalidClock(format!(
            "invalid timestamp {timestamp:?}"
        )));
    }
    let clock: HealthClockSample = sidex::formats::json::from_value(&clock)
        .map_err(|error| SnapshotError::InvalidClock(error.to_string()))?;
    let present = HEALTH.map(|field| value.get(field).is_some());
    if present.iter().any(|p| *p) && !present.iter().all(|p| *p) {
        return Err(SnapshotError::PartialHealth);
    }
    let mut diagnostics = Vec::new();
    for item in list(value, "diagnostics")? {
        let Some(mut item) = item.as_map().cloned() else {
            return Err(SnapshotError::InvalidDiagnostic("not an object".into()));
        };
        let known = |item: &Map<Str, JsonValue>, field: &str, values: &[&str]| {
            item.get(field)
                .and_then(JsonValue::as_str)
                .is_some_and(|value| values.contains(&value))
        };
        if !known(&item, "kind", &KINDS) {
            item.insert("kind".into(), "DirectoryConfig".into());
        }
        if !known(&item, "severity", &["Error", "Warning"]) {
            item.insert("severity".into(), "Error".into());
        }
        let diagnostic: Diagnostic = sidex::formats::json::from_value(&JsonValue::Map(item))
            .map_err(|error| SnapshotError::InvalidDiagnostic(error.to_string()))?;
        diagnostics.push(diagnostic);
    }
    Ok(Snapshot {
        records: Records {
            components: list(value, "components")?,
            health_nodes: list(value, "healthNodes")?,
            health_statuses: list(value, "healthStatuses")?,
            metrics: list(value, "metrics")?,
            metric_samples: list(value, "metricSamples")?,
            diagnostics,
        },
        clock,
        parts: Parts {
            components: value.get("components").is_some(),
            health: present.iter().all(|p| *p),
        },
    })
}
