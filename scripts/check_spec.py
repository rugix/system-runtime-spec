"""Check that the schema definitions quoted in SPEC.md match the schemas."""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
QUOTE = re.compile(r"^```sidex\n(.*?)^```", re.MULTILINE | re.DOTALL)
# Attributes for the Rust code generator do not concern readers of the specification.
RUST_ATTRIBUTE = re.compile(r"^#\[rust\(.*\)\]\n", re.MULTILINE)


def definitions(text: str) -> list[str]:
    """Split Sidex source into its top-level definitions, without Rust attributes."""
    text = RUST_ATTRIBUTE.sub("", text)
    return [part.strip() for part in text.split("\n\n") if part.strip()]


def check_spec(spec: str, schemas: list[str]) -> list[str]:
    """Return the quoted definitions of `spec` that no schema contains verbatim."""
    known = {definition for schema in schemas for definition in definitions(schema)}
    return [
        quoted
        for match in QUOTE.finditer(spec)
        for quoted in definitions(match[1])
        if quoted not in known
    ]


def main() -> None:
    schemas = [path.read_text() for path in sorted((ROOT / "schemas").glob("*.sidex"))]
    mismatches = check_spec((ROOT / "SPEC.md").read_text(), schemas)
    for quoted in mismatches:
        print(f"SPEC.md quotes a definition that differs from the schemas:\n{quoted}\n")
    if mismatches:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
