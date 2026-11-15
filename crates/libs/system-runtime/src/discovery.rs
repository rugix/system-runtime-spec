//! Discover records in the standard directories and added directories.

use std::collections::BTreeMap;
use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use sidex::types::JsonValue;

use crate::records::{Records, check, diagnostic};
use crate::types::common::{Diagnostic, RecordKind, Severity};

/// Levels of directories; records of higher levels override those of lower levels.
const LIB: u8 = 0;
const RUN: u8 = 1;
const ETC: u8 = 2;

const PREFIXES: [(&str, u8); 3] = [
    ("/usr/lib/system-runtime", LIB),
    ("/etc/system-runtime", ETC),
    ("/run/system-runtime", RUN),
];

const KINDS: [(RecordKind, &str); 5] = [
    (RecordKind::ComponentDeclaration, "components"),
    (RecordKind::HealthNode, "healthNodes"),
    (RecordKind::HealthStatus, "healthStatuses"),
    (RecordKind::MetricDefinition, "metrics"),
    (RecordKind::MetricSample, "metricSamples"),
];

fn standard(kind: RecordKind) -> Vec<(String, u8)> {
    let below = |suffix: &str| {
        PREFIXES
            .iter()
            .map(|(prefix, level)| (format!("{prefix}/{suffix}"), *level))
            .collect()
    };
    match kind {
        RecordKind::ComponentDeclaration => below("components"),
        RecordKind::HealthNode => below("health/nodes"),
        RecordKind::HealthStatus => vec![("/run/system-runtime/health/status".into(), RUN)],
        RecordKind::MetricDefinition => below("metrics/definitions"),
        RecordKind::MetricSample => vec![("/run/system-runtime/metrics/samples".into(), RUN)],
        RecordKind::DirectoryConfig => below("directories"),
    }
}

/// Read the records of all kinds from the directories below `root`.
///
/// Files that cannot be read or do not contain a valid record are skipped with an error
/// diagnostic. Overrides are applied, so the records are those that readers use.
#[tracing::instrument(level = "debug", skip_all)]
pub fn read_records(root: impl AsRef<Path>) -> Records {
    let mut reader = Reader {
        root: root.as_ref().to_owned(),
        diagnostics: Vec::new(),
    };
    let configs = reader.read(
        RecordKind::DirectoryConfig,
        &standard(RecordKind::DirectoryConfig),
    );
    let mut lists = BTreeMap::new();
    for (kind, field) in KINDS {
        let mut directories = standard(kind);
        for config in &configs {
            for path in config
                .get(field)
                .and_then(JsonValue::as_seq)
                .into_iter()
                .flatten()
            {
                directories.push((path.as_str().unwrap_or_default().to_owned(), LIB));
            }
        }
        lists.insert(field, reader.read(kind, &directories));
    }
    let mut take = |field: &str| {
        lists
            .remove(field)
            .expect("records of every kind were read")
    };
    Records {
        components: take("components"),
        health_nodes: take("healthNodes"),
        health_statuses: take("healthStatuses"),
        metrics: take("metrics"),
        metric_samples: take("metricSamples"),
        diagnostics: reader.diagnostics,
    }
}

fn overridable(kind: RecordKind) -> bool {
    matches!(
        kind,
        RecordKind::ComponentDeclaration | RecordKind::HealthNode | RecordKind::MetricDefinition
    )
}

/// Entries of a directory that are no symlinks and whose names do not start with `.`.
struct Entry {
    path: PathBuf,
    directory: bool,
}

struct Reader {
    root: PathBuf,
    diagnostics: Vec<Diagnostic>,
}

impl Reader {
    fn report(&mut self, severity: Severity, kind: RecordKind, message: String, path: &Path) {
        self.diagnostics.push(diagnostic(
            severity,
            kind,
            message,
            Some(path.display().to_string()),
        ));
    }

    fn read(&mut self, kind: RecordKind, directories: &[(String, u8)]) -> Vec<JsonValue> {
        let mut files: Vec<(PathBuf, PathBuf, u8)> = Vec::new();
        for (directory, level) in directories {
            for source in self.expand(kind, directory) {
                for path in self.walk(kind, &source) {
                    let canonical = std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
                    match files
                        .iter_mut()
                        .find(|(_, existing, _)| *existing == canonical)
                    {
                        Some(entry) => entry.2 = entry.2.max(*level),
                        None => files.push((path, canonical, *level)),
                    }
                }
            }
        }
        let mut entries = Vec::new();
        for (path, _, level) in files {
            if let Some(record) = self.load(kind, &path) {
                entries.push((level, record));
            }
        }
        if overridable(kind) {
            let key = |record: &JsonValue| {
                (
                    record
                        .get("id")
                        .and_then(JsonValue::as_str)
                        .map(str::to_owned),
                    record
                        .get("owner")
                        .and_then(JsonValue::as_str)
                        .map(str::to_owned),
                )
            };
            let mut highest = BTreeMap::new();
            for (level, record) in &entries {
                let entry = highest.entry(key(record)).or_insert(*level);
                *entry = (*entry).max(*level);
            }
            entries.retain(|(level, record)| highest[&key(record)] == *level);
        }
        entries.into_iter().map(|(_, record)| record).collect()
    }

    /// List a directory in byte order, skipping hidden entries and symlinks.
    fn list(&mut self, kind: RecordKind, directory: &Path) -> Vec<Entry> {
        let entries = match std::fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
            Err(error) => {
                self.report(
                    Severity::Error,
                    kind,
                    format!("cannot list directory: {error}"),
                    directory,
                );
                return Vec::new();
            }
        };
        let mut result: Vec<_> = entries
            .filter_map(Result::ok)
            .filter(|entry| !entry.file_name().as_encoded_bytes().starts_with(b"."))
            .filter_map(|entry| {
                let kind = entry.file_type().ok()?;
                (!kind.is_symlink()).then(|| Entry {
                    path: entry.path(),
                    directory: kind.is_dir(),
                })
            })
            .collect();
        result.sort_by(|a, b| {
            a.path
                .file_name()
                .map(|name| name.as_encoded_bytes())
                .cmp(&b.path.file_name().map(|name| name.as_encoded_bytes()))
        });
        result
    }

    /// Expand whole `*` segments over directories that are no symlinks.
    fn expand(&mut self, kind: RecordKind, directory: &str) -> Vec<PathBuf> {
        let mut matches = vec![self.root.clone()];
        for segment in directory.split('/').filter(|segment| !segment.is_empty()) {
            if segment != "*" {
                for path in &mut matches {
                    path.push(segment);
                }
                continue;
            }
            let mut expanded = Vec::new();
            for path in matches {
                if path.is_dir() {
                    expanded.extend(
                        self.list(kind, &path)
                            .into_iter()
                            .filter(|entry| entry.directory)
                            .map(|entry| entry.path),
                    );
                }
            }
            matches = expanded;
        }
        matches
    }

    fn walk(&mut self, kind: RecordKind, source: &Path) -> Vec<PathBuf> {
        match std::fs::symlink_metadata(source) {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
            Err(error) => {
                self.report(
                    Severity::Error,
                    kind,
                    format!("cannot inspect directory: {error}"),
                    source,
                );
                return Vec::new();
            }
        }
        let metadata = match std::fs::metadata(source) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                self.report(
                    Severity::Error,
                    kind,
                    "directory is a broken symlink".into(),
                    source,
                );
                return Vec::new();
            }
            Err(error) => {
                self.report(
                    Severity::Error,
                    kind,
                    format!("cannot inspect directory: {error}"),
                    source,
                );
                return Vec::new();
            }
        };
        if !metadata.is_dir() {
            self.report(
                Severity::Error,
                kind,
                "path is not a directory".into(),
                source,
            );
            return Vec::new();
        }
        let resolved = std::fs::canonicalize(source).unwrap_or_else(|_| source.to_owned());
        let mut files = Vec::new();
        self.traverse(kind, &resolved, &mut files);
        files
    }

    fn traverse(&mut self, kind: RecordKind, directory: &Path, files: &mut Vec<PathBuf>) {
        for entry in self.list(kind, directory) {
            if entry.directory {
                self.traverse(kind, &entry.path, files);
            } else if entry
                .path
                .extension()
                .is_some_and(|extension| extension == "json")
                && entry.path.is_file()
            {
                files.push(entry.path);
            }
        }
    }

    fn load(&mut self, kind: RecordKind, path: &Path) -> Option<JsonValue> {
        let mut text = Vec::new();
        let result = std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32)
            .open(path)
            .and_then(|mut file| file.read_to_end(&mut text));
        match result {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return None,
            Err(error) => {
                self.report(
                    Severity::Error,
                    kind,
                    format!("cannot read file: {error}"),
                    path,
                );
                return None;
            }
        }
        let value: JsonValue = match sidex::formats::json::from_slice(&text) {
            Ok(value) => value,
            Err(error) => {
                self.report(
                    Severity::Error,
                    kind,
                    format!("invalid JSON: {error}"),
                    path,
                );
                return None;
            }
        };
        match check(kind, &value) {
            Ok(newer) => {
                if newer {
                    self.report(
                        Severity::Warning,
                        kind,
                        "record of a newer minor version".into(),
                        path,
                    );
                }
                Some(value)
            }
            Err(message) => {
                self.report(Severity::Error, kind, message, path);
                None
            }
        }
    }
}
