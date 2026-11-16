"""Exercise the checks of SPEC.md against the schemas and the spec version."""

import shutil
import tempfile
import unittest
from pathlib import Path

from check_spec import ROOT, SPEC_VERSION_CONSTANTS, check_spec, check_spec_version

SCHEMA = '''/// A record.
#[rust(derive(Eq))]
record Example {
    /// A field.
    field: string,
}
'''


class SpecTests(unittest.TestCase):
    def test_repository(self) -> None:
        """Accept the quotes of the repository's specification."""
        schemas = [path.read_text() for path in (ROOT / "schemas").glob("*.sidex")]
        self.assertEqual(check_spec((ROOT / "SPEC.md").read_text(), schemas), [])

    def test_matching_quote(self) -> None:
        """Accept a quote without the Rust attributes of the schema."""
        quote = "```sidex\n/// A record.\nrecord Example {\n    /// A field.\n    field: string,\n}\n```\n"
        self.assertEqual(check_spec(quote, [SCHEMA]), [])

    def test_changed_quote(self) -> None:
        """Reject a quote whose definition differs from the schema."""
        quote = "```sidex\n/// A record.\nrecord Example {\n    /// A field.\n    other: string,\n}\n```\n"
        self.assertEqual(len(check_spec(quote, [SCHEMA])), 1)

    def test_spec_version(self) -> None:
        """Accept the repository's spec version and reject each part that differs."""
        self.assertEqual(check_spec_version(ROOT), "1.0 (draft)")
        changes = {
            "SPEC.md": ("Version 1.0 (draft)", "Version 1.1 (draft)"),
            "sidex.toml": ('version = "1.0.0"', 'version = "1.0.1"'),
            **{path: ('"1.0"', '"1.1"') for path in SPEC_VERSION_CONSTANTS},
        }
        for path, (before, after) in changes.items():
            with self.subTest(path=path), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                for copied in changes:
                    (root / copied).parent.mkdir(parents=True, exist_ok=True)
                    shutil.copyfile(ROOT / copied, root / copied)
                text = (root / path).read_text()
                self.assertIn(before, text)
                (root / path).write_text(text.replace(before, after, 1))
                with self.assertRaises(ValueError):
                    check_spec_version(root)


if __name__ == "__main__":
    unittest.main()
