use std::collections::BTreeMap;
use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use sidex::types::JsonValue;

use crate::Records;
use crate::records::{Record, RecordModel, decode};
use crate::types::common::{Diagnostic, RecordKind, Severity};
use crate::types::component::ComponentDeclaration;
use crate::types::filesystem::DirectoryConfig;
use crate::types::health::{HealthNode, HealthStatus};
use crate::types::metric::{MetricDefinition, MetricSample};

/// Levels of directories; records of higher levels override those of lower levels.
const LIB: u8 = 0;
const RUN: u8 = 1;
const ETC: u8 = 2;

const PREFIXES: [(&str, u8); 3] = [
    ("/usr/lib/system-runtime", LIB),
    ("/etc/system-runtime", ETC),
    ("/run/system-runtime", RUN),
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

impl Records {
    /// Read the records of a Linux system whose root directory is `root`.
    ///
    /// Reads the standard directories and the directories that directory configurations add,
    /// and applies overrides. Files that cannot be read or do not contain a valid record are
    /// skipped with an error diagnostic.
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn discover(root: impl AsRef<Path>) -> Self {
        let mut reader = Reader {
            root: root.as_ref().to_owned(),
            records: Records::new(),
        };
        let mut configs = Vec::new();
        for file in reader.read(
            RecordKind::DirectoryConfig,
            &standard(RecordKind::DirectoryConfig),
        ) {
            if let Some(config) =
                reader.decode::<DirectoryConfig>(RecordKind::DirectoryConfig, &file)
            {
                configs.push(config.value);
            }
        }
        let added = |select: fn(&DirectoryConfig) -> &Option<Vec<String>>| {
            configs
                .iter()
                .flat_map(|config| select(config).iter().flatten().cloned())
                .collect::<Vec<_>>()
        };
        reader.collect::<ComponentDeclaration>(added(|c| &c.components));
        reader.collect::<HealthNode>(added(|c| &c.health_nodes));
        reader.collect::<HealthStatus>(added(|c| &c.health_statuses));
        reader.collect::<MetricDefinition>(added(|c| &c.metrics));
        reader.collect::<MetricSample>(added(|c| &c.metric_samples));
        reader.records
    }
}

struct File {
    path: PathBuf,
    level: u8,
    json: JsonValue,
}

/// Directory or regular file in a directory whose name does not start with `.`.
struct Entry {
    path: PathBuf,
    directory: bool,
}

fn not_found(error: &std::io::Error) -> bool {
    error.kind() == std::io::ErrorKind::NotFound
}

struct Reader {
    root: PathBuf,
    records: Records,
}

impl Reader {
    fn report(&mut self, severity: Severity, kind: RecordKind, message: String, path: &Path) {
        self.records.push_diagnostic(
            Diagnostic::new(severity, kind, message).with_path(Some(path.display().to_string())),
        );
    }

    /// Decode the record of a file, reporting an error if it is invalid and a warning if it
    /// is of a newer minor version.
    fn decode<T: sidex::Decode>(&mut self, kind: RecordKind, file: &File) -> Option<Record<T>> {
        match decode::<T>(kind, file.json.clone()) {
            Ok(record) => {
                if record.newer {
                    let message = "record of a newer minor version".into();
                    self.report(Severity::Warning, kind, message, &file.path);
                }
                Some(record)
            }
            Err(message) => {
                self.report(Severity::Error, kind, message, &file.path);
                None
            }
        }
    }

    fn collect<T: RecordModel>(&mut self, added: Vec<String>) {
        let mut directories = standard(T::KIND);
        directories.extend(added.into_iter().map(|path| (path, LIB)));
        let mut records = Vec::new();
        for file in self.read(T::KIND, &directories) {
            if let Some(record) = self.decode::<T>(T::KIND, &file) {
                records.push((file.level, record));
            }
        }
        let mut highest = BTreeMap::new();
        for (level, record) in &records {
            if let Some(key) = record.value.override_key() {
                let entry = highest.entry(key).or_insert(*level);
                *entry = (*entry).max(*level);
            }
        }
        let keep: Vec<bool> = records
            .iter()
            .map(|(level, record)| {
                record
                    .value
                    .override_key()
                    .is_none_or(|key| highest[&key] == *level)
            })
            .collect();
        let entries = T::entries_mut(&mut self.records);
        for ((_, record), keep) in records.into_iter().zip(keep) {
            if keep {
                entries.push(record);
            }
        }
    }

    fn read(&mut self, kind: RecordKind, directories: &[(String, u8)]) -> Vec<File> {
        let mut files: Vec<(PathBuf, PathBuf, u8)> = Vec::new();
        for (directory, level) in directories {
            for source in self.expand(kind, directory) {
                for path in self.walk(kind, &source) {
                    let Some(canonical) = self.resolve(kind, &path) else {
                        continue;
                    };
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
        files
            .into_iter()
            .filter_map(|(path, _, level)| {
                let json = self.load(kind, &path)?;
                Some(File { path, level, json })
            })
            .collect()
    }

    /// Resolve a path; paths that disappeared resolve to None.
    fn resolve(&mut self, kind: RecordKind, path: &Path) -> Option<PathBuf> {
        match std::fs::canonicalize(path) {
            Ok(resolved) => Some(resolved),
            Err(error) if not_found(&error) => None,
            Err(error) => {
                self.report(
                    Severity::Error,
                    kind,
                    format!("cannot resolve path: {error}"),
                    path,
                );
                None
            }
        }
    }

    /// Check that a path is a directory; diagnose everything but its absence.
    fn check_directory(&mut self, kind: RecordKind, path: &Path) -> bool {
        let message = match std::fs::metadata(path) {
            Ok(metadata) if metadata.is_dir() => return true,
            Ok(_) => "path is not a directory".to_owned(),
            Err(error) if not_found(&error) => match std::fs::symlink_metadata(path) {
                Ok(metadata) if metadata.is_dir() => return true,
                Ok(metadata) if metadata.is_symlink() => "directory is a broken symlink".to_owned(),
                Ok(_) => "path is not a directory".to_owned(),
                Err(error) if not_found(&error) => return false,
                Err(error) => format!("cannot inspect path: {error}"),
            },
            Err(error) => format!("cannot inspect path: {error}"),
        };
        self.report(Severity::Error, kind, message, path);
        false
    }

    /// List a directory in byte order, skipping hidden entries and symlinks.
    fn list(&mut self, kind: RecordKind, directory: &Path) -> Vec<Entry> {
        let entries = match std::fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(error) if not_found(&error) => return Vec::new(),
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
        let mut result = Vec::new();
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) if not_found(&error) => continue,
                Err(error) => {
                    self.report(
                        Severity::Error,
                        kind,
                        format!("cannot list directory: {error}"),
                        directory,
                    );
                    continue;
                }
            };
            if entry.file_name().as_encoded_bytes().starts_with(b".") {
                continue;
            }
            match entry.file_type() {
                Ok(file_type) if file_type.is_dir() || file_type.is_file() => result.push(Entry {
                    path: entry.path(),
                    directory: file_type.is_dir(),
                }),
                Ok(_) => {}
                Err(error) if not_found(&error) => {}
                Err(error) => self.report(
                    Severity::Error,
                    kind,
                    format!("cannot inspect entry: {error}"),
                    &entry.path(),
                ),
            }
        }
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
                if self.check_directory(kind, &path) {
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
        if !self.check_directory(kind, source) {
            return Vec::new();
        }
        let Some(resolved) = self.resolve(kind, source) else {
            return Vec::new();
        };
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
            Err(error) if not_found(&error) => return None,
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
        match sidex::formats::json::from_slice(&text) {
            Ok(value) => Some(value),
            Err(error) => {
                self.report(
                    Severity::Error,
                    kind,
                    format!("invalid JSON: {error}"),
                    path,
                );
                None
            }
        }
    }
}
