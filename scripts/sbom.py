"""Set the version of the command-line tool in its CycloneDX SBOM.

The crate of the command-line tool has the placeholder version 0.0.0, because a release sets the
tool's version when it builds it. cargo-cyclonedx takes the version from the crate, so this script
replaces it with the version of the build.

Usage: python3 scripts/sbom.py SBOM VERSION
"""

import json
import sys
from pathlib import Path
from typing import Any


def set_version(bom: dict[str, Any], version: str) -> None:
    """Set the version of the SBOM's main component and of its Cargo targets."""
    main = bom["metadata"]["component"]
    for component in [main, *main.get("components", [])]:
        previous = component["version"]
        component["version"] = version
        if "purl" in component:
            component["purl"] = component["purl"].replace(f"@{previous}", f"@{version}", 1)


def main() -> None:
    path, version = Path(sys.argv[1]), sys.argv[2]
    bom = json.loads(path.read_text())
    set_version(bom, version)
    path.write_text(json.dumps(bom, indent=2) + "\n")


if __name__ == "__main__":
    main()
