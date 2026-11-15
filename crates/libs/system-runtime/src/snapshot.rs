//! Send records to another system as snapshots.

use sidex::types::{JsonValue, Map, Str};

use crate::Records;
use crate::records::{RecordModel, SPEC_VERSION, SUPPORTED_MAJOR, parse_spec_version, to_json};
use crate::types::common::{Diagnostic, RecordKind};
use crate::types::component::ComponentDeclaration;
use crate::types::health::{HealthClockSample, HealthNode, HealthStatus};
use crate::types::metric::{MetricDefinition, MetricSample};

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

    fn includes(self, kind: RecordKind) -> bool {
        match kind {
            RecordKind::DirectoryConfig => true,
            RecordKind::ComponentDeclaration => self.components,
            _ => self.health,
        }
    }
}

impl Default for Parts {
    fn default() -> Self {
        Self::ALL
    }
}

/// Records of a system with a clock sample, for sending them to another system.
///
/// A snapshot contains the original records, so that the receiving system evaluates them as
/// the sending system read them, including fields of newer minor versions.
///
/// ```
/// use system_runtime_spec::{Parts, Records, Snapshot};
/// use system_runtime_spec::types::health::{HealthClockSample, HealthContext, HealthNode};
///
/// let mut records = Records::new();
/// records.push(HealthNode::new("app".into()));
/// let context = HealthContext::new("boot", 45_000);
/// let clock = HealthClockSample::new(context, "2026-10-14T12:00:00Z".parse().unwrap());
/// let json = Snapshot::new(records, clock, Parts::ALL).to_json();
///
/// let received = Snapshot::from_json(&json).unwrap();
/// let report = received.records().evaluate_health(&received.clock().context);
/// assert_eq!(report.nodes.len(), 1);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    records: Records,
    clock: HealthClockSample,
    parts: Parts,
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

impl Snapshot {
    /// Create a snapshot of the records of the given parts and their diagnostics.
    ///
    /// Take the clock sample after reading the records.
    pub fn new(mut records: Records, clock: HealthClockSample, parts: Parts) -> Self {
        if !parts.components {
            records.components.clear();
        }
        if !parts.health {
            records.health_nodes.clear();
            records.health_statuses.clear();
            records.metrics.clear();
            records.metric_samples.clear();
        }
        records
            .diagnostics
            .retain(|diagnostic| parts.includes(diagnostic.kind));
        Self {
            records,
            clock,
            parts,
        }
    }

    /// Records of the snapshot; the lists of parts it does not include are empty.
    pub fn records(&self) -> &Records {
        &self.records
    }

    /// Take the records of the snapshot.
    pub fn into_records(self) -> Records {
        self.records
    }

    /// Correlation of the boot-time clock with wall-clock time.
    pub fn clock(&self) -> &HealthClockSample {
        &self.clock
    }

    /// Parts whose records the snapshot includes.
    pub fn parts(&self) -> Parts {
        self.parts
    }

    /// Encode the snapshot as a `RuntimeSnapshot`.
    pub fn to_json(&self) -> JsonValue {
        let mut snapshot: Map<Str, JsonValue> = Map::new();
        snapshot.insert("specVersion".into(), SPEC_VERSION.into());
        snapshot.insert("clock".into(), to_json(&self.clock));
        let mut insert = |field: &str, list: Vec<JsonValue>| {
            snapshot.insert(field.into(), list.into());
        };
        if self.parts.components {
            insert(
                "components",
                originals::<ComponentDeclaration>(&self.records),
            );
        }
        if self.parts.health {
            insert("healthNodes", originals::<HealthNode>(&self.records));
            insert("healthStatuses", originals::<HealthStatus>(&self.records));
            insert("metrics", originals::<MetricDefinition>(&self.records));
            insert("metricSamples", originals::<MetricSample>(&self.records));
        }
        if !self.records.diagnostics.is_empty() {
            let diagnostics: Vec<JsonValue> =
                self.records.diagnostics.iter().map(to_json).collect();
            snapshot.insert("diagnostics".into(), diagnostics.into());
        }
        JsonValue::Map(snapshot)
    }

    /// Decode a `RuntimeSnapshot`, applying the fallbacks to its diagnostics.
    ///
    /// Fail for snapshots of an unsupported version or with an invalid structure. Invalid
    /// records are skipped with an error diagnostic, as when they are read from files.
    pub fn from_json(value: &JsonValue) -> Result<Self, SnapshotError> {
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
        let health = present.iter().all(|p| *p);
        if present.iter().any(|p| *p) && !health {
            return Err(SnapshotError::PartialHealth);
        }
        let mut records = Records::new();
        for item in list(value, "diagnostics")? {
            records.push_diagnostic(read_diagnostic(item)?);
        }
        push_all::<ComponentDeclaration>(&mut records, list(value, "components")?);
        push_all::<HealthNode>(&mut records, list(value, "healthNodes")?);
        push_all::<HealthStatus>(&mut records, list(value, "healthStatuses")?);
        push_all::<MetricDefinition>(&mut records, list(value, "metrics")?);
        push_all::<MetricSample>(&mut records, list(value, "metricSamples")?);
        let parts = Parts {
            components: value.get("components").is_some(),
            health,
        };
        Ok(Self {
            records,
            clock,
            parts,
        })
    }
}

fn originals<T: RecordModel>(records: &Records) -> Vec<JsonValue> {
    T::entries(records)
        .iter()
        .map(|record| record.json.clone())
        .collect()
}

fn push_all<T: RecordModel>(records: &mut Records, items: Vec<JsonValue>) {
    for item in items {
        records.push_json::<T>(item);
    }
}

/// Decode a diagnostic, treating unknown kinds as `DirectoryConfig` and unknown severities as
/// `Error`.
fn read_diagnostic(item: JsonValue) -> Result<Diagnostic, SnapshotError> {
    let JsonValue::Map(mut item) = item else {
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
    if !known(&item, "severity", &["Warning"]) {
        item.insert("severity".into(), "Error".into());
    }
    sidex::formats::json::from_value(&JsonValue::Map(item))
        .map_err(|error| SnapshotError::InvalidDiagnostic(error.to_string()))
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
