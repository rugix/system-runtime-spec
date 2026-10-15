"""Load file contributions from standard and additional Linux directories."""

import json
import stat
from dataclasses import dataclass, field
from pathlib import Path
from typing import TypeVar

from pydantic import BaseModel

from .models.component import ComponentDeclaration
from .models.filesystem import DirectoryConfig, RuntimeSnapshot
from .models.health import HealthNode, HealthStatus

_PREFIXES = ("/usr/lib/system-runtime", "/etc/system-runtime", "/run/system-runtime")
Model = TypeVar("Model", bound=BaseModel)


@dataclass
class RuntimeDirectories:
    """Directory sources, resolved beneath root, with explicit extension support."""

    root: Path = field(default_factory=lambda: Path("/"))
    components: list[str] = field(default_factory=list)
    health_nodes: list[str] = field(default_factory=list)
    health_statuses: list[str] = field(default_factory=list)
    configuration: list[str] = field(default_factory=list)

    @classmethod
    def standard(cls, root: str | Path = "/") -> "RuntimeDirectories":
        return cls(
            root=Path(root),
            components=[f"{prefix}/components" for prefix in _PREFIXES],
            health_nodes=[f"{prefix}/health/nodes" for prefix in _PREFIXES],
            health_statuses=["/run/system-runtime/health/status"],
            configuration=[f"{prefix}/directories" for prefix in _PREFIXES],
        )

    def extend(self, config: DirectoryConfig) -> None:
        """Append directory sources without replacing existing contributions."""
        self.components.extend(config.components or [])
        self.health_nodes.extend(config.health_nodes or [])
        self.health_statuses.extend(config.health_statuses or [])

    def load(self) -> RuntimeSnapshot:
        """Reload extension configurations and collect regular JSON files.

        Whole '*' segments select real directories. Missing roots are ignored.
        An explicit root may be a symlink; nested symlinks are skipped.
        Overlapping sources read each physical file once.
        """
        effective = RuntimeDirectories(
            root=self.root,
            components=self.components.copy(),
            health_nodes=self.health_nodes.copy(),
            health_statuses=self.health_statuses.copy(),
        )
        for path in self._files(self.configuration):
            effective.extend(_read(path, DirectoryConfig))
        return RuntimeSnapshot(
            components=[
                _read(path, ComponentDeclaration)
                for path in effective._files(effective.components)
            ],
            health_nodes=[
                _read(path, HealthNode)
                for path in effective._files(effective.health_nodes)
            ],
            health_statuses=[
                _read(path, HealthStatus)
                for path in effective._files(effective.health_statuses)
            ],
        )

    def _files(self, directories: list[str]) -> list[Path]:
        """List contributions in source order, deduplicating canonical paths."""
        files: list[Path] = []
        seen: set[Path] = set()
        for source in self._sources(directories):
            if not _is_directory(source):
                continue
            for candidate in _walk(source.resolve(strict=True)):
                canonical = candidate.resolve(strict=True)
                if canonical not in seen:
                    seen.add(canonical)
                    files.append(canonical)
        return files

    def _sources(self, directories: list[str]) -> list[Path]:
        """Expand whole-segment wildcards over real directories in lexical order."""
        sources = []
        for directory in directories:
            path = Path(directory)
            if (
                not path.is_absolute()
                or ".." in path.parts
                or any("*" in part and part != "*" for part in path.parts)
            ):
                raise ValueError(
                    "Directory must be absolute without '..' "
                    f"and use only whole '*' segments: {directory!r}"
                )
            matches = [self.root]
            for part in path.parts[1:]:
                if part == "*":
                    matches = [
                        entry
                        for source in matches
                        if _is_directory(source)
                        for entry in sorted(source.iterdir())
                        if not entry.is_symlink() and entry.is_dir()
                    ]
                else:
                    matches = [source / part for source in matches]
            sources.extend(matches)
        return sources


def _is_directory(path: Path) -> bool:
    """Accept directories and missing optional sources, reporting other failures."""
    try:
        metadata = path.stat()
    except FileNotFoundError:
        if path.is_symlink():
            raise OSError(f"Broken directory symlink: {path}") from None
        return False
    if not stat.S_ISDIR(metadata.st_mode):
        raise NotADirectoryError(path)
    return True


def _walk(directory: Path) -> list[Path]:
    """Recurse in lexical order without following contributed symlinks."""
    result = []
    for path in sorted(directory.iterdir()):
        if path.is_symlink():
            continue
        if path.is_dir():
            result.extend(_walk(path))
        elif path.is_file() and path.suffix == ".json":
            result.append(path)
    return result


def _read(path: Path, model: type[Model]) -> Model:
    """Parse one typed contribution with its file path attached to errors."""
    try:
        return model.model_validate(json.loads(path.read_bytes()))
    except (OSError, ValueError) as error:
        raise ValueError(f"Failed to read {path}: {error}") from error
