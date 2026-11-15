"""Discover records in the standard directories and added directories."""

import json
import os
import stat
from dataclasses import dataclass
from pathlib import Path

from ._records import (
    InvalidRecord,
    JsonObject,
    Records,
    decode,
    diagnostic,
)
from .models.common import Diagnostic

# Levels of the directories, from lowest to highest.
_LIB, _RUN, _ETC = 0, 1, 2
_PREFIXES = (
    ("/usr/lib/system-runtime", _LIB),
    ("/etc/system-runtime", _ETC),
    ("/run/system-runtime", _RUN),
)
_STANDARD = {
    "ComponentDeclaration": [(f"{p}/components", level) for p, level in _PREFIXES],
    "HealthNode": [(f"{p}/health/nodes", level) for p, level in _PREFIXES],
    "HealthStatus": [("/run/system-runtime/health/status", _RUN)],
    "MetricDefinition": [(f"{p}/metrics/definitions", level) for p, level in _PREFIXES],
    "MetricSample": [("/run/system-runtime/metrics/samples", _RUN)],
    "DirectoryConfig": [(f"{p}/directories", level) for p, level in _PREFIXES],
}
_CONFIG_FIELDS = {
    "ComponentDeclaration": "components",
    "HealthNode": "healthNodes",
    "HealthStatus": "healthStatuses",
    "MetricDefinition": "metrics",
    "MetricSample": "metricSamples",
}
_OVERRIDABLE = ("ComponentDeclaration", "HealthNode", "MetricDefinition")


@dataclass
class _Entry:
    path: str
    level: int
    record: JsonObject


def read_records(root: str | Path = "/") -> Records:
    """Read the records of all kinds from the directories below `root`.

    Files that cannot be read or do not contain a valid record are skipped with an error
    diagnostic. Overrides are applied, so the records are those that readers use.
    """
    reader = _Reader(Path(root))
    configs = reader.read("DirectoryConfig", _STANDARD["DirectoryConfig"])
    added: dict[str, list[tuple[str, int]]] = {kind: [] for kind in _CONFIG_FIELDS}
    for config in configs:
        for kind, name in _CONFIG_FIELDS.items():
            added[kind].extend((path, _LIB) for path in config.get(name) or [])
    lists = {
        kind: reader.read(kind, _STANDARD[kind] + added[kind])
        for kind in _CONFIG_FIELDS
    }
    return Records(
        components=lists["ComponentDeclaration"],
        health_nodes=lists["HealthNode"],
        health_statuses=lists["HealthStatus"],
        metrics=lists["MetricDefinition"],
        metric_samples=lists["MetricSample"],
        diagnostics=reader.diagnostics,
    )


def _byte_order(name: str) -> bytes:
    return os.fsencode(name)


class _Reader:
    def __init__(self, root: Path) -> None:
        self.root = root
        self.diagnostics: list[Diagnostic] = []

    def _report(self, severity: str, kind: str, message: str, path: Path | str) -> None:
        self.diagnostics.append(diagnostic(severity, kind, message, str(path)))

    def read(self, kind: str, directories: list[tuple[str, int]]) -> list[JsonObject]:
        files: dict[str, tuple[Path, int]] = {}
        for directory, level in directories:
            for source in self._expand(kind, directory):
                for path in self._walk(kind, source):
                    canonical = os.path.realpath(path)
                    if canonical in files:
                        files[canonical] = (
                            files[canonical][0],
                            max(level, files[canonical][1]),
                        )
                    else:
                        files[canonical] = (path, level)
        entries = []
        for path, level in files.values():
            record = self._load(kind, path)
            if record is not None:
                entries.append(_Entry(str(path), level, record))
        if kind in _OVERRIDABLE:
            highest: dict[tuple[str, str | None], int] = {}
            for entry in entries:
                key = (entry.record["id"], entry.record.get("owner"))
                highest[key] = max(highest.get(key, entry.level), entry.level)
            entries = [
                entry
                for entry in entries
                if entry.level
                == highest[(entry.record["id"], entry.record.get("owner"))]
            ]
        return [entry.record for entry in entries]

    def _expand(self, kind: str, directory: str) -> list[Path]:
        """Expand whole `*` segments over directories that are no symlinks."""
        matches = [self.root]
        for segment in directory.strip("/").split("/"):
            if not segment:
                continue
            if segment != "*":
                matches = [match / segment for match in matches]
                continue
            expanded = []
            for match in matches:
                try:
                    with os.scandir(match) as entries:
                        children = sorted(entries, key=lambda e: _byte_order(e.name))
                except FileNotFoundError:
                    continue
                except NotADirectoryError:
                    continue
                except OSError as error:
                    self._report(
                        "Error", kind, f"cannot list directory: {error}", match
                    )
                    continue
                for child in children:
                    try:
                        if not child.name.startswith(".") and child.is_dir(
                            follow_symlinks=False
                        ):
                            expanded.append(Path(child.path))
                    except FileNotFoundError:
                        continue
            matches = expanded
        return matches

    def _walk(self, kind: str, source: Path) -> list[Path]:
        try:
            os.lstat(source)
        except FileNotFoundError:
            return []
        except OSError as error:
            self._report("Error", kind, f"cannot inspect directory: {error}", source)
            return []
        try:
            status = os.stat(source)
        except FileNotFoundError:
            self._report("Error", kind, "directory is a broken symlink", source)
            return []
        except OSError as error:
            self._report("Error", kind, f"cannot inspect directory: {error}", source)
            return []
        if not stat.S_ISDIR(status.st_mode):
            self._report("Error", kind, "path is not a directory", source)
            return []
        files: list[Path] = []
        self._traverse(kind, Path(os.path.realpath(source)), files)
        return files

    def _traverse(self, kind: str, directory: Path, files: list[Path]) -> None:
        try:
            with os.scandir(directory) as entries:
                children = sorted(entries, key=lambda e: _byte_order(e.name))
        except FileNotFoundError:
            return
        except OSError as error:
            self._report("Error", kind, f"cannot list directory: {error}", directory)
            return
        for child in children:
            if child.name.startswith("."):
                continue
            try:
                if child.is_symlink():
                    continue
                if child.is_dir(follow_symlinks=False):
                    self._traverse(kind, Path(child.path), files)
                elif child.is_file(follow_symlinks=False) and child.name.endswith(
                    ".json"
                ):
                    files.append(Path(child.path))
            except FileNotFoundError:
                continue

    def _load(self, kind: str, path: Path) -> JsonObject | None:
        try:
            descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
            with os.fdopen(descriptor, "rb") as file:
                data = file.read()
        except FileNotFoundError:
            return None
        except OSError as error:
            self._report("Error", kind, f"cannot read file: {error}", path)
            return None
        try:
            value = json.loads(data)
        except ValueError as error:
            self._report("Error", kind, f"invalid JSON: {error}", path)
            return None
        try:
            decoded = decode(kind, value)
        except InvalidRecord as error:
            self._report("Error", kind, str(error), path)
            return None
        if decoded.newer:
            self._report("Warning", kind, "record of a newer minor version", path)
        assert isinstance(value, dict)
        return value
