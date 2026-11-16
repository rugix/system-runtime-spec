from __future__ import annotations

import json
import os
import stat
from dataclasses import dataclass
from pathlib import Path
from typing import TYPE_CHECKING

import pydantic

from ._decoding import (
    KINDS,
    InvalidRecord,
    Json,
    Record,
    RecordModel,
    decode,
    diagnostic,
)
from .models.common import DiagnosticKind, Severity
from .models.component import ComponentDeclaration
from .models.filesystem import DirectoryConfig
from .models.health import HealthNode, HealthStatus
from .models.metric import MetricDefinition, MetricSample

if TYPE_CHECKING:
    from ._records import Records

# Levels of the directories, from lowest to highest.
_LIB, _RUN, _ETC = 0, 1, 2
_PREFIXES = (
    ("/usr/lib/system-runtime", _LIB),
    ("/etc/system-runtime", _ETC),
    ("/run/system-runtime", _RUN),
)
_STANDARD: dict[DiagnosticKind, list[tuple[str, int]]] = {
    "ComponentDeclaration": [(f"{p}/components", level) for p, level in _PREFIXES],
    "HealthNode": [(f"{p}/health/nodes", level) for p, level in _PREFIXES],
    "HealthStatus": [("/run/system-runtime/health/status", _RUN)],
    "MetricDefinition": [(f"{p}/metrics/definitions", level) for p, level in _PREFIXES],
    "MetricSample": [("/run/system-runtime/metrics/samples", _RUN)],
    "DirectoryConfig": [(f"{p}/directories", level) for p, level in _PREFIXES],
}
_OVERRIDABLE = (ComponentDeclaration, HealthNode, MetricDefinition)
_NEWER = "record of a newer minor version"


@dataclass
class _File:
    path: Path
    level: int
    json: Json


def discover(root: str | os.PathLike[str], records: Records) -> None:
    """Add the records below `root` to `records`, applying overrides."""
    reader = _Reader(Path(root), records)
    configs: list[DirectoryConfig] = []
    for file in reader.read("DirectoryConfig", _STANDARD["DirectoryConfig"]):
        config = reader.decode(DirectoryConfig, file)
        if config is not None:
            configs.append(config.value)
    for model, added in (
        (ComponentDeclaration, [c.components for c in configs]),
        (HealthNode, [c.health_nodes for c in configs]),
        (HealthStatus, [c.health_statuses for c in configs]),
        (MetricDefinition, [c.metrics for c in configs]),
        (MetricSample, [c.metric_samples for c in configs]),
    ):
        paths = [(path, _LIB) for paths in added for path in paths or []]
        reader.collect(model, paths)


def _byte_order(name: str) -> bytes:
    return os.fsencode(name)


class _Reader:
    def __init__(self, root: Path, records: Records) -> None:
        self.root = root
        self.records = records

    def report(
        self, severity: Severity, kind: DiagnosticKind, message: str, path: Path | str
    ) -> None:
        self.records.push_diagnostic(diagnostic(severity, kind, message, str(path)))

    def decode[M: pydantic.BaseModel](
        self, model: type[M], file: _File
    ) -> Record[M] | None:
        """Decode the record of a file, reporting an error if it is invalid and a
        warning if it is of a newer minor version."""
        kind = KINDS[model]
        try:
            record = decode(model, file.json)
        except InvalidRecord as error:
            self.report("Error", kind, str(error), file.path)
            return None
        if record.newer:
            self.report("Warning", kind, _NEWER, file.path)
        return record

    def collect(self, model: type[RecordModel], added: list[tuple[str, int]]) -> None:
        kind = KINDS[model]
        accepted = []
        for file in self.read(kind, _STANDARD[kind] + added):
            record = self.decode(model, file)
            if record is not None:
                accepted.append((file.level, record))
        if model in _OVERRIDABLE:
            highest: dict[tuple[str, str | None], int] = {}
            for level, record in accepted:
                key = (record.value.id, record.value.owner)
                highest[key] = max(highest.get(key, level), level)
            accepted = [
                (level, record)
                for level, record in accepted
                if level == highest[(record.value.id, record.value.owner)]
            ]
        for _, record in accepted:
            self.records._add(record)

    def read(
        self, kind: DiagnosticKind, directories: list[tuple[str, int]]
    ) -> list[_File]:
        files: dict[str, tuple[Path, int]] = {}
        for directory, level in directories:
            for source in self._expand(kind, directory):
                for path in self._walk(kind, source):
                    canonical = self._resolve(kind, path)
                    if canonical is None:
                        continue
                    if canonical in files:
                        files[canonical] = (
                            files[canonical][0],
                            max(level, files[canonical][1]),
                        )
                    else:
                        files[canonical] = (path, level)
        result = []
        for path, level in files.values():
            loaded, value = self._load(kind, path)
            if loaded:
                result.append(_File(path, level, value))
        return result

    def _resolve(self, kind: DiagnosticKind, path: Path) -> str | None:
        """Resolve a path; paths that disappeared resolve to None."""
        try:
            return os.path.realpath(path, strict=True)
        except FileNotFoundError:
            return None
        except OSError as error:
            self.report("Error", kind, f"cannot resolve path: {error}", path)
            return None

    def _check_directory(self, kind: DiagnosticKind, path: Path) -> bool:
        """Check that a path is a directory; diagnose everything but its absence."""
        try:
            status = os.stat(path)
        except FileNotFoundError:
            try:
                status = os.lstat(path)
            except FileNotFoundError:
                return False
            except OSError as error:
                self.report("Error", kind, f"cannot inspect path: {error}", path)
                return False
            if stat.S_ISLNK(status.st_mode):
                self.report("Error", kind, "directory is a broken symlink", path)
                return False
        except OSError as error:
            self.report("Error", kind, f"cannot inspect path: {error}", path)
            return False
        if not stat.S_ISDIR(status.st_mode):
            self.report("Error", kind, "path is not a directory", path)
            return False
        return True

    def _list(self, kind: DiagnosticKind, directory: Path) -> list[tuple[Path, bool]]:
        """List a directory in byte order, skipping hidden entries and symlinks.

        Return the paths of the entries and whether they are directories; regular files
        are the entries that are no directories.
        """
        try:
            with os.scandir(directory) as entries:
                children = sorted(entries, key=lambda e: _byte_order(e.name))
        except FileNotFoundError:
            return []
        except OSError as error:
            self.report("Error", kind, f"cannot list directory: {error}", directory)
            return []
        result = []
        for child in children:
            if child.name.startswith("."):
                continue
            try:
                if child.is_dir(follow_symlinks=False):
                    result.append((Path(child.path), True))
                elif child.is_file(follow_symlinks=False):
                    result.append((Path(child.path), False))
            except FileNotFoundError:
                continue
            except OSError as error:
                self.report("Error", kind, f"cannot inspect entry: {error}", child.path)
        return result

    def _expand(self, kind: DiagnosticKind, directory: str) -> list[Path]:
        """Expand whole `*` segments over directories that are no symlinks."""
        matches = [self.root]
        for segment in directory.strip("/").split("/"):
            if not segment:
                continue
            if segment != "*":
                matches = [match / segment for match in matches]
                continue
            matches = [
                path
                for match in matches
                if self._check_directory(kind, match)
                for path, is_directory in self._list(kind, match)
                if is_directory
            ]
        return matches

    def _walk(self, kind: DiagnosticKind, source: Path) -> list[Path]:
        if not self._check_directory(kind, source):
            return []
        resolved = self._resolve(kind, source)
        if resolved is None:
            return []
        files: list[Path] = []
        self._traverse(kind, Path(resolved), files)
        return files

    def _traverse(
        self, kind: DiagnosticKind, directory: Path, files: list[Path]
    ) -> None:
        for path, is_directory in self._list(kind, directory):
            if is_directory:
                self._traverse(kind, path, files)
            elif path.name.endswith(".json"):
                files.append(path)

    def _load(self, kind: DiagnosticKind, path: Path) -> tuple[bool, Json]:
        """Read a file that contains JSON; return whether it could be read."""
        try:
            descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
            with os.fdopen(descriptor, "rb") as file:
                data = file.read()
        except FileNotFoundError:
            return False, None
        except OSError as error:
            self.report("Error", kind, f"cannot read file: {error}", path)
            return False, None
        try:
            return True, json.loads(data)
        except ValueError as error:
            self.report("Error", kind, f"invalid JSON: {error}", path)
            return False, None
