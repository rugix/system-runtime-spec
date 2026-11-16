"""Exercise release planning in Git repositories with copies of the actual manifests."""

import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

from check_release import ROOT, check_versions, plan_release, release_notes

MANIFESTS = (
    "Cargo.toml",
    "crates/libs/system-runtime/Cargo.toml",
    "python/pyproject.toml",
    "typescript/package.json",
)
GIT_IDENTITY = {
    "GIT_AUTHOR_NAME": "Test",
    "GIT_AUTHOR_EMAIL": "test@example.com",
    "GIT_COMMITTER_NAME": "Test",
    "GIT_COMMITTER_EMAIL": "test@example.com",
}


class ReleaseTests(unittest.TestCase):
    def setUp(self) -> None:
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        for manifest in MANIFESTS:
            target = self.root / manifest
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / manifest, target)
        self.git("init", "--quiet")
        self.commit_and_tag("2026.10.25")
        self.versions = check_versions(self.read)

    def test_first_release(self) -> None:
        """Publish every package when no earlier release exists."""
        release = plan_release(self.root, "2026.10.25")
        self.assertIsNone(release.previous)
        self.assertEqual(release.published, {"crates-io", "pypi", "npm"})

    def test_changed_package(self) -> None:
        """Publish only the changed package, compared with the previous release."""
        self.commit_and_tag("2026.11.01")
        self.replace(
            "python/pyproject.toml",
            f'version = "{self.versions["pypi"]}"',
            'version = "9.0.0"',
        )
        self.commit_and_tag("2026.11.01.1")
        release = plan_release(self.root, "2026.11.01.1")
        self.assertEqual(release.previous, "2026.11.01")
        self.assertEqual(release.published, {"pypi"})
        notes = release_notes(release, "1.0 (draft)").splitlines()
        self.assertEqual(notes[0], "Specification version 1.0 (draft).")
        self.assertIn(
            "| Python | [9.0.0](https://pypi.org/project/system-runtime-spec/9.0.0/)"
            " | New |",
            notes,
        )
        self.assertTrue(notes[-1].startswith("| TypeScript |"))
        self.assertTrue(notes[-1].endswith("| Unchanged |"))

    def test_release_without_packages(self) -> None:
        """Release a change outside the packages without publishing any of them."""
        (self.root / "SPEC.md").write_text("Clarified.\n")
        self.commit_and_tag("2026.11.02")
        self.assertEqual(plan_release(self.root, "2026.11.02").published, set())

    def test_untagged_commit(self) -> None:
        """Check a commit before it is tagged, as a release preview does."""
        (self.root / "typescript/README.md").write_text("Changed.\n")
        self.git("add", "--all")
        self.git("commit", "--quiet", "--message", "Change")
        with self.assertRaisesRegex(ValueError, "TypeScript package changed"):
            plan_release(self.root, "2026.11.02", "HEAD")

    def test_missing_bump(self) -> None:
        """Reject a changed package that keeps its version."""
        (self.root / "typescript/README.md").write_text("Changed.\n")
        self.commit_and_tag("2026.11.02")
        with self.assertRaisesRegex(ValueError, "TypeScript package changed"):
            plan_release(self.root, "2026.11.02")

    def test_lower_version(self) -> None:
        """Reject a changed package whose version decreases."""
        self.replace(
            "Cargo.toml",
            f'version = "{self.versions["crates-io"]}"',
            'version = "0.0.1"',
        )
        self.commit_and_tag("2026.11.02")
        with self.assertRaisesRegex(ValueError, "Rust package changed"):
            plan_release(self.root, "2026.11.02")

    def test_invalid_tags(self) -> None:
        """Reject tags that are not release dates."""
        for tag in (
            "v0.2.0",
            "2026.11.1",
            "2026.02.30",
            "2026.11.16.0",
            "2026.11.16.01",
        ):
            with (
                self.subTest(tag=tag),
                self.assertRaisesRegex(ValueError, "must be a date"),
            ):
                plan_release(self.root, tag)

    def test_manifest_identity(self) -> None:
        """Reject unstable versions and packages under another name."""
        changes = (
            (
                "Cargo.toml",
                f'version = "{self.versions["crates-io"]}"',
                'version = "1.0.0-rc.1"',
            ),
            (
                "crates/libs/system-runtime/Cargo.toml",
                "version.workspace = true",
                'version = "1.0.0"',
            ),
            (
                "python/pyproject.toml",
                f'version = "{self.versions["pypi"]}"',
                'version = "1.0"',
            ),
            (
                "typescript/package.json",
                '"name": "system-runtime-spec"',
                '"name": "wrong"',
            ),
        )
        for path, before, after in changes:
            with self.subTest(path=path):
                self.replace(path, before, after)
                with self.assertRaises(ValueError):
                    check_versions(self.read)
                self.replace(path, after, before)

    def read(self, path: str) -> str:
        return (self.root / path).read_text()

    def replace(self, path: str, before: str, after: str) -> None:
        text = self.read(path)
        self.assertIn(before, text)
        (self.root / path).write_text(text.replace(before, after, 1))

    def commit_and_tag(self, tag: str) -> None:
        self.git("add", "--all")
        self.git("commit", "--quiet", "--allow-empty", "--message", tag)
        self.git("tag", tag)

    def git(self, *args: str) -> None:
        subprocess.run(
            ["git", *args],
            cwd=self.root,
            check=True,
            env={**os.environ, **GIT_IDENTITY},
        )


if __name__ == "__main__":
    unittest.main()
