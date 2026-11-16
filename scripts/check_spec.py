"""Check that SPEC.md matches the schemas and that all parts agree on its version."""

import re
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
QUOTE = re.compile(r"^```sidex\n(.*?)^```", re.MULTILINE | re.DOTALL)
# Attributes for the Rust code generator do not concern readers of the specification.
RUST_ATTRIBUTE = re.compile(r"^#\[rust\(.*\)\]\n", re.MULTILINE)
HEADER = re.compile(r"^Version ([0-9]+\.[0-9]+)( \(draft\))?$", re.MULTILINE)
# The constant through which each implementation states the spec version it implements.
SPEC_VERSION_CONSTANTS = {
    "crates/libs/system-runtime/src/records.rs": re.compile(
        r'^pub const SPEC_VERSION: &str = "(.*)";$', re.MULTILINE
    ),
    "python/system_runtime_spec/_decoding.py": re.compile(
        r'^SPEC_VERSION = "(.*)"$', re.MULTILINE
    ),
    "typescript/src/decoding.ts": re.compile(
        r'^export const SPEC_VERSION = "(.*)";$', re.MULTILINE
    ),
}


def check_spec(spec: str, schemas: list[str]) -> list[str]:
    """Return the quoted definitions of `spec` that no schema contains verbatim."""
    known = {definition for schema in schemas for definition in definitions(schema)}
    return [
        quoted
        for match in QUOTE.finditer(spec)
        for quoted in definitions(match[1])
        if quoted not in known
    ]


def check_spec_version(root: Path) -> str:
    """Return the spec version of SPEC.md, such as `1.0 (draft)`, if all parts agree."""
    header = HEADER.search((root / "SPEC.md").read_text())
    if header is None:
        raise ValueError("SPEC.md must state its version as `Version X.Y`")
    version = header[1]
    for path, constant in SPEC_VERSION_CONSTANTS.items():
        match = constant.search((root / path).read_text())
        if match is None or match[1] != version:
            raise ValueError(f"SPEC_VERSION in {path} differs from {version}")
    bundle = tomllib.loads((root / "sidex.toml").read_text())["bundle"]["version"]
    if bundle != f"{version}.0":
        raise ValueError(f"Sidex bundle version must be {version}.0, got {bundle}")
    return header[0].removeprefix("Version ")


def definitions(text: str) -> list[str]:
    """Split Sidex source into its top-level definitions, without Rust attributes."""
    text = RUST_ATTRIBUTE.sub("", text)
    return [part.strip() for part in text.split("\n\n") if part.strip()]


def main() -> None:
    schemas = [path.read_text() for path in sorted((ROOT / "schemas").glob("*.sidex"))]
    mismatches = check_spec((ROOT / "SPEC.md").read_text(), schemas)
    for quoted in mismatches:
        print(f"SPEC.md quotes a definition that differs from the schemas:\n{quoted}\n")
    try:
        check_spec_version(ROOT)
    except ValueError as error:
        print(error)
        raise SystemExit(1) from None
    if mismatches:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
