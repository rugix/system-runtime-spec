//! Read standard and additional directories without a central service.

use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use crate::Error;
use crate::types::filesystem::{DirectoryConfig, RuntimeSnapshot};

const PREFIXES: [&str; 3] = [
    "/usr/lib/system-runtime",
    "/etc/system-runtime",
    "/run/system-runtime",
];

/// Directory sources, resolved beneath root, with explicit extension support.
#[derive(Clone, Debug)]
pub struct RuntimeDirectories {
    /// Filesystem root, normally "/".
    pub root: PathBuf,
    /// Component declaration directories.
    pub components: Vec<PathBuf>,
    /// Health definition directories.
    pub health_nodes: Vec<PathBuf>,
    /// Runtime health status directories.
    pub health_statuses: Vec<PathBuf>,
    /// Metric definition directories.
    pub metrics: Vec<PathBuf>,
    /// Runtime metric sample directories.
    pub metric_samples: Vec<PathBuf>,
    /// Directories containing extension configuration files.
    pub configuration: Vec<PathBuf>,
}

impl RuntimeDirectories {
    /// Create an empty source configuration beneath a filesystem root.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            components: Vec::new(),
            health_nodes: Vec::new(),
            health_statuses: Vec::new(),
            metrics: Vec::new(),
            metric_samples: Vec::new(),
            configuration: Vec::new(),
        }
    }

    /// Use the standard Linux directories beneath a filesystem root.
    pub fn standard(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            components: PREFIXES
                .map(|prefix| PathBuf::from(format!("{prefix}/components")))
                .into(),
            health_nodes: PREFIXES
                .map(|prefix| PathBuf::from(format!("{prefix}/health/nodes")))
                .into(),
            health_statuses: vec!["/run/system-runtime/health/status".into()],
            metrics: PREFIXES
                .map(|prefix| PathBuf::from(format!("{prefix}/metrics/definitions")))
                .into(),
            metric_samples: vec!["/run/system-runtime/metrics/samples".into()],
            configuration: PREFIXES
                .map(|prefix| PathBuf::from(format!("{prefix}/directories")))
                .into(),
        }
    }

    /// Append directory sources without replacing existing contributions.
    pub fn extend(&mut self, config: &DirectoryConfig) {
        self.components
            .extend(config.components.iter().flatten().map(PathBuf::from));
        self.health_nodes
            .extend(config.health_nodes.iter().flatten().map(PathBuf::from));
        self.health_statuses
            .extend(config.health_statuses.iter().flatten().map(PathBuf::from));
        self.metrics
            .extend(config.metrics.iter().flatten().map(PathBuf::from));
        self.metric_samples
            .extend(config.metric_samples.iter().flatten().map(PathBuf::from));
    }

    /// Reload extension configurations and collect regular JSON files.
    ///
    /// Whole '*' segments select real directories. Missing roots are ignored.
    /// An explicit root may be a symlink; nested symlinks are skipped.
    /// Overlapping sources read each canonical file path once.
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn load(&self) -> Result<RuntimeSnapshot, Error> {
        let mut effective = self.clone();
        for path in self.files(&self.configuration)? {
            effective.extend(&read::<DirectoryConfig>(&path)?);
        }
        let metrics = effective
            .files(&effective.metrics)?
            .iter()
            .map(|p| read(p))
            .collect::<Result<Vec<_>, _>>()?;
        let observations = effective
            .files(&effective.metric_samples)?
            .iter()
            .map(|p| read(p))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(RuntimeSnapshot {
            metrics: (!metrics.is_empty()).then_some(metrics),
            metric_samples: (!observations.is_empty()).then_some(observations),
            components: effective
                .files(&effective.components)?
                .iter()
                .map(|p| read(p))
                .collect::<Result<_, _>>()?,
            health_nodes: effective
                .files(&effective.health_nodes)?
                .iter()
                .map(|p| read(p))
                .collect::<Result<_, _>>()?,
            health_statuses: effective
                .files(&effective.health_statuses)?
                .iter()
                .map(|p| read(p))
                .collect::<Result<_, _>>()?,
        })
    }

    /// List contributions in source order, deduplicating canonical paths.
    fn files(&self, directories: &[PathBuf]) -> Result<Vec<PathBuf>, Error> {
        let mut files = Vec::new();
        let mut seen = BTreeSet::new();
        for source in self.sources(directories)? {
            if !is_directory(&source)? {
                continue;
            }
            let source = source.canonicalize().map_err(|error| Error::Io {
                path: source,
                source: error,
            })?;
            let mut candidates = Vec::new();
            walk(&source, &mut candidates)?;
            for path in candidates {
                let canonical = path.canonicalize().map_err(|source| Error::Io {
                    path: path.clone(),
                    source,
                })?;
                if seen.insert(canonical.clone()) {
                    files.push(canonical);
                }
            }
        }
        Ok(files)
    }

    /// Expand whole-segment wildcards over real directories in lexical order.
    fn sources(&self, directories: &[PathBuf]) -> Result<Vec<PathBuf>, Error> {
        let mut sources = Vec::new();
        for directory in directories {
            if !directory.is_absolute()
                || directory.components().any(|part| match part {
                    Component::ParentDir => true,
                    Component::Normal(name) => {
                        name.as_encoded_bytes().contains(&b'*') && name != "*"
                    }
                    _ => false,
                })
            {
                return Err(Error::Invalid(format!(
                    "directory must be absolute without '..' and use only whole '*' segments: {}",
                    directory.display()
                )));
            }
            let mut matches = vec![self.root.clone()];
            for part in directory.components() {
                let Component::Normal(name) = part else {
                    continue;
                };
                if name == "*" {
                    let mut expanded = Vec::new();
                    for source in matches {
                        if !is_directory(&source)? {
                            continue;
                        }
                        for entry in entries(&source)? {
                            if entry
                                .file_type()
                                .map_err(|error| Error::Io {
                                    path: entry.path(),
                                    source: error,
                                })?
                                .is_dir()
                            {
                                expanded.push(entry.path());
                            }
                        }
                    }
                    matches = expanded;
                } else {
                    for source in &mut matches {
                        source.push(name);
                    }
                }
            }
            sources.extend(matches);
        }
        Ok(sources)
    }
}

/// Accept directories and missing optional sources, reporting other failures.
fn is_directory(path: &Path) -> Result<bool, Error> {
    match std::fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => Ok(true),
        Ok(_) => Err(Error::Invalid(format!(
            "not a directory: {}",
            path.display()
        ))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !path.is_symlink() => {
            Ok(false)
        }
        Err(source) => Err(Error::Io {
            path: path.into(),
            source,
        }),
    }
}

/// List entries in lexical order, preserving filesystem errors.
fn entries(directory: &Path) -> Result<Vec<std::fs::DirEntry>, Error> {
    let mut entries = std::fs::read_dir(directory)
        .map_err(|source| Error::Io {
            path: directory.into(),
            source,
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| Error::Io {
            path: directory.into(),
            source,
        })?;
    entries.sort_by_key(|entry| entry.file_name());
    Ok(entries)
}

/// Traverse regular files in lexical order without following nested symlinks.
fn walk(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in entries(directory)? {
        let path = entry.path();
        let kind = entry.file_type().map_err(|source| Error::Io {
            path: path.clone(),
            source,
        })?;
        if kind.is_dir() {
            walk(&path, files)?;
        } else if kind.is_file()
            && path
                .extension()
                .is_some_and(|extension| extension == "json")
        {
            files.push(path);
        }
    }
    Ok(())
}

/// Parse one typed contribution with its path attached to errors.
fn read<T: sidex::Decode>(path: &Path) -> Result<T, Error> {
    let text = std::fs::read_to_string(path).map_err(|source| Error::Io {
        path: path.into(),
        source,
    })?;
    sidex::formats::json::from_str(&text).map_err(|source| Error::Decode {
        path: path.into(),
        source,
    })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::types::component::ComponentDeclaration;

    /// Switching current after discovery preserves the collected generation's files.
    #[test]
    fn discovery_pins_generation_before_reading() {
        let root = tempfile::tempdir().unwrap();
        for generation in 1..=2 {
            let dir = root.path().join(generation.to_string());
            std::fs::create_dir(&dir).unwrap();
            std::fs::write(
                dir.join("app.json"),
                format!(r#"{{"id":"app.{generation}"}}"#),
            )
            .unwrap();
        }
        let current = root.path().join("current");
        std::os::unix::fs::symlink("1", &current).unwrap();
        let mut directories = RuntimeDirectories::new(root.path());
        directories.components = vec!["/current".into()];
        let files = directories.files(&directories.components).unwrap();
        std::fs::remove_file(&current).unwrap();
        std::os::unix::fs::symlink("2", &current).unwrap();
        assert_eq!(read::<ComponentDeclaration>(&files[0]).unwrap().id, "app.1");
        assert_eq!(directories.load().unwrap().components[0].id, "app.2");
    }
}
