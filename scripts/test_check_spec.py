"""Exercise the check of schema quotes in SPEC.md."""

import unittest

from check_spec import ROOT, check_spec

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


if __name__ == "__main__":
    unittest.main()
