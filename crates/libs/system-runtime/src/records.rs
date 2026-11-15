//! Decode and validate records, applying the fallbacks for unknown values.

use std::collections::BTreeSet;

use anyver::{Version, VersionReq};
use sidex::types::{JsonValue, Map, Str};

use crate::types::common::{Diagnostic, RecordKind, Severity};
use crate::types::component::ComponentDeclaration;
use crate::types::filesystem::DirectoryConfig;
use crate::types::health::{HealthNode, HealthStatus};
use crate::types::metric::{MetricDefinition, MetricSample};

/// Version of the specification that this crate implements.
pub const SPEC_VERSION: &str = "1.0";
pub(crate) const SUPPORTED_MAJOR: u64 = 1;
const SUPPORTED_MINOR: u64 = 0;

const RULE_TYPES: [&str; 3] = ["Observed", "Threshold", "Aggregate"];
const STATES: [&str; 4] = ["Healthy", "Degraded", "Unhealthy", "Unknown"];
const MODES: [&str; 3] = ["Counted", "Mandatory", "Optional"];

/// Records of all kinds, as JSON objects, with the diagnostics about them.
///
/// Records keep their original JSON, so snapshots contain them unchanged.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Records {
    /// Component declarations.
    pub components: Vec<JsonValue>,
    /// Health nodes.
    pub health_nodes: Vec<JsonValue>,
    /// Status records.
    pub health_statuses: Vec<JsonValue>,
    /// Metric definitions.
    pub metrics: Vec<JsonValue>,
    /// Metric samples.
    pub metric_samples: Vec<JsonValue>,
    /// Diagnostics about the records and about directory configurations.
    pub diagnostics: Vec<Diagnostic>,
}

/// Convert a record, given as a typed model or as a JSON value, to its JSON value.
pub fn to_json<T: sidex::Encode + ?Sized>(record: &T) -> JsonValue {
    sidex::formats::json::to_value(record).expect("records are valid JSON")
}

impl Records {
    /// Add component declarations, given as typed models or as JSON values.
    pub fn with_components(mut self, records: &[impl sidex::Encode]) -> Self {
        self.components.extend(records.iter().map(to_json));
        self
    }

    /// Add health nodes, given as typed models or as JSON values.
    pub fn with_health_nodes(mut self, records: &[impl sidex::Encode]) -> Self {
        self.health_nodes.extend(records.iter().map(to_json));
        self
    }

    /// Add status records, given as typed models or as JSON values.
    pub fn with_health_statuses(mut self, records: &[impl sidex::Encode]) -> Self {
        self.health_statuses.extend(records.iter().map(to_json));
        self
    }

    /// Add metric definitions, given as typed models or as JSON values.
    pub fn with_metrics(mut self, records: &[impl sidex::Encode]) -> Self {
        self.metrics.extend(records.iter().map(to_json));
        self
    }

    /// Add metric samples, given as typed models or as JSON values.
    pub fn with_metric_samples(mut self, records: &[impl sidex::Encode]) -> Self {
        self.metric_samples.extend(records.iter().map(to_json));
        self
    }

    /// Add diagnostics.
    pub fn with_diagnostics(mut self, diagnostics: &[Diagnostic]) -> Self {
        self.diagnostics.extend_from_slice(diagnostics);
        self
    }
}

/// Create a diagnostic.
pub fn diagnostic(
    severity: Severity,
    kind: RecordKind,
    message: impl Into<String>,
    path: Option<String>,
) -> Diagnostic {
    Diagnostic {
        severity,
        kind,
        message: message.into(),
        path,
    }
}

/// Implement conversions for an ID wrapper, whose generated field is private to the crate.
///
/// Converting does not validate the ID; evaluation skips records with invalid IDs.
macro_rules! id_conversions {
    ($($id:ty),*) => {$(
        impl $id {
            /// The ID as a string.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $id {
            fn from(id: String) -> Self {
                Self(id)
            }
        }

        impl From<&str> for $id {
            fn from(id: &str) -> Self {
                Self(id.to_owned())
            }
        }

        impl AsRef<str> for $id {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }
    )*};
}

id_conversions!(
    crate::types::health::HealthNodeId,
    crate::types::metric::MetricId
);

/// A valid record, normalized for evaluation, with the values that need a fallback.
///
/// Unknown rule types are removed, and unknown states and modes are replaced; the flags
/// record where the fallbacks apply.
pub(crate) struct Decoded<T> {
    pub value: T,
    pub newer: bool,
    pub unsupported: bool,
    pub unsupported_attachments: BTreeSet<String>,
}

pub(crate) fn parse_spec_version(text: &str) -> Option<(u64, u64)> {
    let (major, minor) = text.split_once('.')?;
    Some((number(major)?, number(minor)?))
}

fn number(text: &str) -> Option<u64> {
    let valid = !text.is_empty()
        && text.bytes().all(|byte| byte.is_ascii_digit())
        && (text == "0" || !text.starts_with('0'));
    valid.then(|| text.parse().ok()).flatten()
}

/// Read integral JSON numbers that fit integer fields, such as `1.0`, as integers.
fn integral(value: &JsonValue) -> JsonValue {
    match value {
        JsonValue::Number(number) if number.is_f64() => {
            let float = number.as_f64();
            if float.fract() == 0.0 && (0.0..18_446_744_073_709_551_616.0).contains(&float) {
                JsonValue::from(float as u64)
            } else {
                value.clone()
            }
        }
        JsonValue::Seq(items) => JsonValue::from(items.iter().map(integral).collect::<Vec<_>>()),
        JsonValue::Map(map) => {
            JsonValue::Map(map.iter().map(|(k, v)| (k.clone(), integral(v))).collect())
        }
        _ => value.clone(),
    }
}

fn object(value: &JsonValue) -> Result<Map<Str, JsonValue>, String> {
    value
        .as_map()
        .cloned()
        .ok_or_else(|| "record is not a JSON object".to_owned())
}

/// Check a record's version; return whether it is a newer minor version.
fn check_version(kind: RecordKind, record: &Map<Str, JsonValue>) -> Result<bool, String> {
    let Some(version) = record.get("specVersion") else {
        return Ok(false);
    };
    let Some((major, minor)) = version.as_str().and_then(parse_spec_version) else {
        return Err(format!("invalid specVersion {version}"));
    };
    if major != SUPPORTED_MAJOR {
        return Err(format!("unsupported specVersion {version}"));
    }
    let newer = minor > SUPPORTED_MINOR;
    if newer && kind == RecordKind::ComponentDeclaration {
        return Err(format!(
            "component declaration of newer specVersion {version}"
        ));
    }
    Ok(newer)
}

fn string<'a>(value: &'a JsonValue, what: &str) -> Result<&'a str, String> {
    value
        .as_str()
        .ok_or_else(|| format!("{what} is not a string"))
}

/// Replace an unknown input mode with `Counted`; return whether it was unknown.
fn apply_mode_fallback(entry: &mut Map<Str, JsonValue>) -> Result<bool, String> {
    let Some(mode) = entry.get("mode") else {
        return Ok(false);
    };
    if MODES.contains(&string(mode, "mode")?) {
        return Ok(false);
    }
    entry.insert("mode".into(), "Counted".into());
    Ok(true)
}

/// Decode a record with fallbacks and validate it.
pub(crate) fn decode<T: sidex::Decode>(
    kind: RecordKind,
    value: &JsonValue,
) -> Result<Decoded<T>, String> {
    let mut record = object(&integral(value))?;
    let newer = check_version(kind, &record)?;
    let mut unsupported = false;
    let mut unsupported_attachments = BTreeSet::new();
    match kind {
        RecordKind::HealthNode => {
            if let Some(JsonValue::Map(rule)) = record.get("rule").cloned() {
                let mut rule = rule;
                let rule_type = string(rule.get("type").unwrap_or(&JsonValue::Null), "rule.type")?;
                if !RULE_TYPES.contains(&rule_type) {
                    unsupported = true;
                    record.remove("rule");
                } else if rule_type == "Aggregate" {
                    if let Some(JsonValue::Seq(inputs)) = rule.get("inputs").cloned() {
                        let mut entries = Vec::new();
                        for entry in inputs.iter() {
                            if let JsonValue::Map(entry) = entry {
                                let mut entry = entry.clone();
                                unsupported |= apply_mode_fallback(&mut entry)?;
                                entries.push(JsonValue::Map(entry));
                            } else {
                                entries.push(entry.clone());
                            }
                        }
                        rule.insert("inputs".into(), JsonValue::from(entries));
                    }
                    record.insert("rule".into(), JsonValue::Map(rule));
                }
            }
            if let Some(JsonValue::Seq(attachments)) = record.get("partOf").cloned() {
                let mut entries = Vec::new();
                for entry in attachments.iter() {
                    if let JsonValue::Map(entry) = entry {
                        let mut entry = entry.clone();
                        if apply_mode_fallback(&mut entry)?
                            && let Some(node) = entry.get("node").and_then(JsonValue::as_str)
                        {
                            unsupported_attachments.insert(node.to_owned());
                        }
                        entries.push(JsonValue::Map(entry));
                    } else {
                        entries.push(entry.clone());
                    }
                }
                record.insert("partOf".into(), JsonValue::from(entries));
            }
        }
        RecordKind::HealthStatus => {
            let state = string(record.get("state").unwrap_or(&JsonValue::Null), "state")?;
            if !STATES.contains(&state) {
                unsupported = true;
                record.insert("state".into(), "Unknown".into());
            }
        }
        _ => {}
    }
    let record = JsonValue::Map(record);
    let value: T = sidex::formats::json::from_value(&record).map_err(|error| error.to_string())?;
    validate(kind, &record)?;
    Ok(Decoded {
        value,
        newer,
        unsupported,
        unsupported_attachments,
    })
}

/// Decode records, skipping invalid ones with an error diagnostic.
pub(crate) fn decode_all<T: sidex::Decode>(
    kind: RecordKind,
    values: &[JsonValue],
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<Decoded<T>> {
    values
        .iter()
        .filter_map(|value| match decode(kind, value) {
            Ok(decoded) => Some(decoded),
            Err(message) => {
                diagnostics.push(diagnostic(Severity::Error, kind, message, None));
                None
            }
        })
        .collect()
}

pub(crate) fn check(kind: RecordKind, value: &JsonValue) -> Result<bool, String> {
    let decoded = match kind {
        RecordKind::ComponentDeclaration => decode::<ComponentDeclaration>(kind, value)?.newer,
        RecordKind::HealthNode => decode::<HealthNode>(kind, value)?.newer,
        RecordKind::HealthStatus => decode::<HealthStatus>(kind, value)?.newer,
        RecordKind::MetricDefinition => decode::<MetricDefinition>(kind, value)?.newer,
        RecordKind::MetricSample => decode::<MetricSample>(kind, value)?.newer,
        RecordKind::DirectoryConfig => decode::<DirectoryConfig>(kind, value)?.newer,
    };
    Ok(decoded)
}

fn get<'a>(value: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
    value.get(key)
}

fn items<'a>(value: &'a JsonValue, key: &str) -> &'a [JsonValue] {
    get(value, key)
        .and_then(JsonValue::as_seq)
        .map_or(&[], |items| items.as_slice())
}

fn check_id(value: Option<&JsonValue>, what: &str) -> Result<(), String> {
    let Some(value) = value else {
        return Ok(());
    };
    let id = value.as_str().unwrap_or_default();
    if id.is_empty() || !id.bytes().all(|byte| (b'!'..=b'~').contains(&byte)) {
        return Err(format!("{what} is not a valid ID: {value}"));
    }
    Ok(())
}

fn check_positive(value: &JsonValue, key: &str) -> Result<(), String> {
    if get(value, key).and_then(JsonValue::as_u64) == Some(0) {
        return Err(format!("{key} is zero"));
    }
    Ok(())
}

fn finite(value: Option<&JsonValue>, what: &str) -> Result<f64, String> {
    value
        .and_then(JsonValue::as_f64)
        .filter(|value| value.is_finite())
        .ok_or_else(|| format!("{what} is not finite"))
}

fn validate(kind: RecordKind, record: &JsonValue) -> Result<(), String> {
    check_id(get(record, "id"), "id")?;
    check_id(get(record, "owner"), "owner")?;
    for component in items(record, "components") {
        if kind != RecordKind::DirectoryConfig {
            check_id(Some(component), "component ID")?;
        }
    }
    match kind {
        RecordKind::ComponentDeclaration => {
            for capability in items(record, "provides") {
                check_capability(capability, true)?;
            }
            for claim in items(record, "claims") {
                check_id(get(claim, "id"), "claim ID")?;
            }
            for selector in items(record, "requires")
                .iter()
                .chain(items(record, "conflicts"))
            {
                check_capability(selector, false)?;
            }
            if let Some(version) = get(record, "version").and_then(JsonValue::as_str) {
                Version::parse(version).map_err(|error| error.to_string())?;
            }
        }
        RecordKind::HealthNode => {
            for attachment in items(record, "partOf") {
                check_id(get(attachment, "node"), "attachment node")?;
            }
            if let Some(rule) = get(record, "rule") {
                check_positive(rule, "maxAgeMs")?;
                match get(rule, "type").and_then(JsonValue::as_str) {
                    Some("Threshold") => {
                        check_id(get(rule, "metric"), "metric")?;
                        check_ranges(
                            get(rule, "healthy").expect("decoded"),
                            get(rule, "usable").expect("decoded"),
                        )?;
                    }
                    Some("Aggregate") => {
                        for input in items(rule, "inputs") {
                            check_id(get(input, "node"), "input node")?;
                        }
                    }
                    _ => {}
                }
            }
        }
        RecordKind::HealthStatus | RecordKind::MetricDefinition => {
            check_positive(record, "maxAgeMs")?;
        }
        RecordKind::MetricSample => {
            finite(get(record, "value"), "value")?;
            check_positive(record, "maxAgeMs")?;
        }
        RecordKind::DirectoryConfig => {
            for key in [
                "components",
                "healthNodes",
                "healthStatuses",
                "metrics",
                "metricSamples",
            ] {
                for path in items(record, key) {
                    check_directory_path(path.as_str().unwrap_or_default())?;
                }
            }
        }
    }
    Ok(())
}

fn check_capability(value: &JsonValue, provided: bool) -> Result<(), String> {
    check_id(get(value, "id"), "capability ID")?;
    if get(value, "version").is_some() && get(value, "value").is_some() {
        return Err(format!(
            "capability {} has both version and value",
            get(value, "id").expect("decoded")
        ));
    }
    if let Some(text) = get(value, "version").and_then(JsonValue::as_str) {
        if provided {
            Version::parse(text).map_err(|error| error.to_string())?;
        } else {
            VersionReq::parse(text).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn check_ranges(healthy: &JsonValue, usable: &JsonValue) -> Result<(), String> {
    for range in [healthy, usable] {
        let lower = get(range, "lower");
        let upper = get(range, "upper");
        for bound in [lower, upper].into_iter().flatten() {
            finite(get(bound, "value"), "range bound")?;
        }
        if let (Some(lower), Some(upper)) = (lower, upper) {
            let (low, high) = (bound_value(lower), bound_value(upper));
            if low > high || (low == high && !(inclusive(lower) && inclusive(upper))) {
                return Err("range is empty".into());
            }
        }
    }
    for (is_lower, key) in [(true, "lower"), (false, "upper")] {
        let Some(outer) = get(usable, key) else {
            continue;
        };
        let Some(inner) = get(healthy, key) else {
            return Err("healthy range is not a subset of the usable range".into());
        };
        let (inner_value, outer_value) = (bound_value(inner), bound_value(outer));
        let beyond = if is_lower {
            inner_value < outer_value
        } else {
            inner_value > outer_value
        };
        if beyond || (inner_value == outer_value && inclusive(inner) && !inclusive(outer)) {
            return Err("healthy range is not a subset of the usable range".into());
        }
    }
    Ok(())
}

fn bound_value(bound: &JsonValue) -> f64 {
    get(bound, "value")
        .and_then(JsonValue::as_f64)
        .unwrap_or(f64::NAN)
}

fn inclusive(bound: &JsonValue) -> bool {
    get(bound, "inclusive").and_then(JsonValue::as_bool) == Some(true)
}

fn check_directory_path(path: &str) -> Result<(), String> {
    if !path.starts_with('/') {
        return Err(format!("path is not absolute: {path:?}"));
    }
    if path.contains('\0') {
        return Err(format!("path contains a NUL character: {path:?}"));
    }
    for segment in path.split('/') {
        if segment == ".." {
            return Err(format!("path contains '..': {path:?}"));
        }
        if segment.contains('*') && segment != "*" {
            return Err(format!("path contains a partial wildcard: {path:?}"));
        }
    }
    Ok(())
}
