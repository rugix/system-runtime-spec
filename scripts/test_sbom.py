"""Exercise setting the version of the command-line tool in its SBOM."""

import unittest

from sbom import set_version


class SbomTests(unittest.TestCase):
    def test_set_version(self) -> None:
        """Set the version of the main component and its targets, not of dependencies."""
        bom = {
            "metadata": {
                "component": {
                    "bom-ref": "path+file:///src/system-runtime#0.0.0",
                    "name": "system-runtime",
                    "version": "0.0.0",
                    "purl": "pkg:cargo/system-runtime@0.0.0?download_url=file://.",
                    "components": [
                        {
                            "name": "system-runtime",
                            "version": "0.0.0",
                            "purl": "pkg:cargo/system-runtime@0.0.0?download_url=file://.#src/main.rs",
                        }
                    ],
                }
            },
            "components": [
                {"name": "clap", "version": "4.6.7", "purl": "pkg:cargo/clap@4.6.7"}
            ],
        }
        set_version(bom, "2026.11.16")
        main = bom["metadata"]["component"]
        self.assertEqual(main["version"], "2026.11.16")
        self.assertEqual(
            main["purl"], "pkg:cargo/system-runtime@2026.11.16?download_url=file://."
        )
        self.assertEqual(
            main["components"][0]["purl"],
            "pkg:cargo/system-runtime@2026.11.16?download_url=file://.#src/main.rs",
        )
        # References between components keep their identifiers.
        self.assertEqual(main["bom-ref"], "path+file:///src/system-runtime#0.0.0")
        self.assertEqual(bom["components"][0]["version"], "4.6.7")


if __name__ == "__main__":
    unittest.main()
