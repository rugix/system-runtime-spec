"""Check the package versions and determine which packages a release publishes.

Releases are tagged with their date, such as `2026.11.16`, or `2026.11.16.1` for a
second release on the same day. A release publishes each package whose files changed
since the previous release and requires a higher version for it.
"""

import argparse
import datetime
import json
import re
import subprocess
import tomllib
from collections.abc import Callable
from dataclasses import dataclass
from pathlib import Path

from check_spec import check_spec_version

ROOT = Path(__file__).resolve().parents[1]
NAME = "system-runtime-spec"
TAG = re.compile(r"([0-9]{4})\.([0-9]{2})\.([0-9]{2})(?:\.([1-9][0-9]*))?")
VERSION = re.compile(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)")

Reader = Callable[[str], str]
"""Read a file by its path relative to the repository root."""


@dataclass(frozen=True)
class Package:
    """One package, identified by the registry that it is published to."""

    registry: str
    language: str
    paths: tuple[str, ...]
    """Files whose changes require a new release of the package."""
    url: str
    """Registry page of one version, with `{}` for the version."""
    identity: Callable[[Reader], tuple[str, str]]
    """Read the name and version of the package from its manifests."""


@dataclass(frozen=True)
class Release:
    """Package versions of a release and the packages it publishes, by registry."""

    tag: str
    previous: str | None
    versions: dict[str, str]
    published: set[str]


def check_versions(read: Reader) -> dict[str, str]:
    """Return the stable version of each package, keyed by registry."""
    versions = {}
    for package in PACKAGES:
        name, version = package.identity(read)
        if name != NAME:
            raise ValueError(f"{package.language} package must be named {NAME}")
        if not VERSION.fullmatch(version):
            raise ValueError(
                f"{package.language} version must be X.Y.Z, got {version!r}"
            )
        versions[package.registry] = version
    return versions


def plan_release(root: Path, tag: str, commit: str | None = None) -> Release:
    """Compare release `tag` with the previous release and check the version bumps.

    The release consists of `commit`, which defaults to the commit that `tag` names.
    """
    commit = commit or tag
    key = tag_key(tag)
    if key is None:
        raise ValueError(f"Release tag must be a date such as 2026.11.16, got {tag!r}")
    earlier = [
        other
        for other in git(root, "tag", "--list").split()
        if (other_key := tag_key(other)) is not None and other_key < key
    ]
    previous = max(earlier, key=tag_key, default=None)
    versions = check_versions(lambda path: git(root, "show", f"{commit}:{path}"))
    if previous is None:
        return Release(tag, None, versions, set(versions))
    if run_git(root, "merge-base", "--is-ancestor", previous, commit).returncode != 0:
        raise ValueError(f"Release {tag} must contain the previous release {previous}")
    before = check_versions(lambda path: git(root, "show", f"{previous}:{path}"))
    # A package's version is part of its files, so unchanged packages keep it.
    published = set()
    for package in PACKAGES:
        if not git(root, "diff", "--name-only", previous, commit, "--", *package.paths):
            continue
        version, old = versions[package.registry], before[package.registry]
        if version_key(version) <= version_key(old):
            raise ValueError(
                f"{package.language} package changed since {previous}; "
                f"its version must be higher than {old}"
            )
        published.add(package.registry)
    return Release(tag, previous, versions, published)


def release_notes(release: Release, spec_version: str) -> str:
    """Describe the spec and package versions of a release for its notes."""
    lines = [
        f"Specification version {spec_version}.",
        "",
        "| Package | Version | Status |",
        "| --- | --- | --- |",
    ]
    for package in PACKAGES:
        version = release.versions[package.registry]
        status = "New" if package.registry in release.published else "Unchanged"
        link = f"[{version}]({package.url.format(version)})"
        lines.append(f"| {package.language} | {link} | {status} |")
    lines += [
        "",
        f"The assets contain the `system-runtime` command-line tool {release.tag}"
        " as static Linux binaries, with their checksums in `SHA256SUMS`.",
    ]
    return "\n".join(lines) + "\n"


def tag_key(tag: str) -> tuple[int, ...] | None:
    """Order release tags by date and number, or return `None` for other tags."""
    match = TAG.fullmatch(tag)
    if match is None:
        return None
    year, month, day = (int(part) for part in match.groups()[:3])
    try:
        datetime.date(year, month, day)
    except ValueError:
        return None
    return (year, month, day, int(match[4] or 0))


def version_key(version: str) -> tuple[int, ...]:
    return tuple(int(part) for part in version.split("."))


def git(root: Path, *args: str) -> str:
    result = run_git(root, *args)
    if result.returncode != 0:
        raise ValueError(f"git {' '.join(args)} failed: {result.stderr.strip()}")
    return result.stdout


def run_git(root: Path, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(["git", *args], cwd=root, capture_output=True, text=True)


def rust_identity(read: Reader) -> tuple[str, str]:
    crate = tomllib.loads(read("crates/libs/system-runtime/Cargo.toml"))["package"]
    if crate["version"] != {"workspace": True}:
        raise ValueError("Rust package must inherit the workspace version")
    workspace = tomllib.loads(read("Cargo.toml"))["workspace"]["package"]
    return crate["name"], workspace["version"]


def python_identity(read: Reader) -> tuple[str, str]:
    project = tomllib.loads(read("python/pyproject.toml"))["project"]
    return project["name"], project["version"]


def npm_identity(read: Reader) -> tuple[str, str]:
    package = json.loads(read("typescript/package.json"))
    return package["name"], package["version"]


PACKAGES = (
    Package(
        "crates-io",
        "Rust",
        ("Cargo.toml", "crates/libs/system-runtime"),
        "https://crates.io/crates/system-runtime-spec/{}",
        rust_identity,
    ),
    Package(
        "pypi",
        "Python",
        ("python",),
        "https://pypi.org/project/system-runtime-spec/{}/",
        python_identity,
    ),
    Package(
        "npm",
        "TypeScript",
        ("typescript",),
        "https://www.npmjs.com/package/system-runtime-spec/v/{}",
        npm_identity,
    ),
)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tag", help="check the release with this tag")
    parser.add_argument(
        "--notes", type=Path, help="write the release notes to this file"
    )
    args = parser.parse_args()
    if args.notes is not None and args.tag is None:
        parser.error("--notes requires --tag")
    try:
        if args.tag is None:
            versions = check_versions(lambda path: (ROOT / path).read_text())
            for registry, version in versions.items():
                print(f"{registry}={version}")
            return
        release = plan_release(ROOT, args.tag)
        if args.notes is not None:
            args.notes.write_text(release_notes(release, check_spec_version(ROOT)))
    except ValueError as error:
        parser.exit(1, f"{error}\n")
    print(f"previous={release.previous or ''}")
    for package in PACKAGES:
        print(
            f"{package.registry}={str(package.registry in release.published).lower()}"
        )


if __name__ == "__main__":
    main()
