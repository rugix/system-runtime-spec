"""Exercise release validation against copies of the actual package manifests."""

import shutil
import tempfile
import unittest
from pathlib import Path

from check_release import ROOT, check_release


class ReleaseTests(unittest.TestCase):
    def test_matching_release(self) -> None:
        """Accept the repository's consistent release and its matching tag."""
        version = check_release(ROOT)
        self.assertEqual(check_release(ROOT, f"v{version}"), version)

    def test_wrong_tags(self) -> None:
        """Reject tags that could publish packages under an unrelated version."""
        version = check_release(ROOT)
        for tag in (f"v{version}.1", "main", f"v{version}-rc.1", f"v{version}\n"):
            with self.subTest(tag=tag), self.assertRaises(ValueError):
                check_release(ROOT, tag)

    def test_manifest_drift(self) -> None:
        """Reject each independently changed version or distribution identity."""
        version = check_release(ROOT)
        changes = (
            ("Cargo.toml", f'version = "{version}"', f'version = "{version}-rc.1"'),
            ("sidex.toml", f'version = "{version}"', f'version = "{version}.1"'),
            (
                "python/pyproject.toml",
                f'version = "{version}"',
                f'version = "{version}.1"',
            ),
            (
                "typescript/package.json",
                f'"version": "{version}"',
                f'"version": "{version}.1"',
            ),
            (
                "crates/libs/system-runtime/Cargo.toml",
                'name = "system-runtime-spec"',
                'name = "wrong"',
            ),
        )
        for path, before, after in changes:
            with self.subTest(path=path), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                for manifest in (
                    "Cargo.toml",
                    "sidex.toml",
                    "python/pyproject.toml",
                    "typescript/package.json",
                    "crates/libs/system-runtime/Cargo.toml",
                ):
                    target = root / manifest
                    target.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copyfile(ROOT / manifest, target)
                target = root / path
                target.write_text(target.read_text().replace(before, after, 1))
                with self.assertRaises(ValueError):
                    check_release(root)
