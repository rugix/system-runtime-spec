"""Generate every language package from the shared Sidex bundle."""

import argparse
import filecmp
import re
import shutil
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TARGETS = {
    "rust": ROOT / "crates/libs/system-runtime/src/generated",
    "py": ROOT / "python/system_runtime/models",
    "ts": ROOT / "typescript/src/generated",
    "json-schema": ROOT / "json-schema",
}


def equal_directories(left: Path, right: Path) -> bool:
    """Compare generated files, excluding Python bytecode."""
    left_files = {
        p.relative_to(left)
        for p in left.rglob("*")
        if p.is_file() and "__pycache__" not in p.parts
    }
    right_files = (
        {
            p.relative_to(right)
            for p in right.rglob("*")
            if p.is_file() and "__pycache__" not in p.parts
        }
        if right.exists()
        else set()
    )
    return left_files == right_files and all(
        filecmp.cmp(left / name, right / name, shallow=False) for name in left_files
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as temporary:
        for backend, destination in TARGETS.items():
            output = Path(temporary) / backend
            subprocess.run(
                ["sidex", "generate", backend, str(output)], cwd=ROOT, check=True
            )
            if backend == "rust":
                for path in output.rglob("*.rs"):
                    subprocess.run(
                        ["rustfmt", "--edition", "2024", str(path)], check=True
                    )
            if backend == "ts":
                for path in output.rglob("*.ts"):
                    source = re.sub(
                        r'(from ")(\./[^"]+)(")', r"\1\2.js\3", path.read_text()
                    )
                    path.write_text(source)
            if args.check:
                if not equal_directories(output, destination):
                    raise SystemExit(
                        f"Generated files differ: {destination.relative_to(ROOT)}"
                    )
            else:
                if destination.exists():
                    shutil.rmtree(destination)
                shutil.copytree(output, destination)


if __name__ == "__main__":
    main()
