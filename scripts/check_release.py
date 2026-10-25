"""Check that all distributions and the Sidex bundle share a stable release version."""

import argparse
import json
import re
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def check_release(root: Path, tag: str | None = None) -> str:
    """Reject inconsistent package identities, versions, or release tags."""
    cargo = tomllib.loads((root / "Cargo.toml").read_text())
    crate = tomllib.loads((root / "crates/libs/system-runtime/Cargo.toml").read_text())
    python = tomllib.loads((root / "python/pyproject.toml").read_text())
    npm = json.loads((root / "typescript/package.json").read_text())
    sidex = tomllib.loads((root / "sidex.toml").read_text())
    version = cargo["workspace"]["package"]["version"]
    if not re.fullmatch(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)", version):
        raise ValueError("Only stable X.Y.Z releases are supported")
    for name, package in (
        ("Rust", crate["package"]),
        ("Python", python["project"]),
        ("npm", npm),
    ):
        if package["name"] != "system-runtime-spec":
            raise ValueError(f"{name} package must be named system-runtime-spec")
        expected = {"workspace": True} if name == "Rust" else version
        if package["version"] != expected:
            raise ValueError(f"{name} package version differs from {version}")
    if sidex["bundle"]["version"] != version:
        raise ValueError(f"Sidex bundle version differs from {version}")
    if tag is not None and tag != f"v{version}":
        raise ValueError(f"Release tag must be v{version}, got {tag!r}")
    return version


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tag")
    args = parser.parse_args()
    try:
        print(check_release(ROOT, args.tag))
    except ValueError as error:
        parser.exit(1, f"{error}\n")


if __name__ == "__main__":
    main()
