"""Preview the release of the current commit, or tag the release and push the tag.

Without a tag, both commands use the current UTC date, followed by the lowest free
number if a release of that date exists. Both fetch the tags of `origin` first.
"""

import argparse
import datetime
import subprocess

from check_release import PACKAGES, ROOT, Release, git, plan_release, release_notes
from check_spec import check_spec_version

REPOSITORY = "rugix/system-runtime-spec"


def preview(tag: str) -> None:
    """Print the packages that a release of `HEAD` publishes and its release notes."""
    release = plan_release(ROOT, tag, "HEAD")
    print(f"Release {tag} of {git(ROOT, 'rev-parse', '--short', 'HEAD').strip()}")
    print(f"Previous release: {release.previous or 'none'}")
    print(f"Publishes: {published_packages(release)}")
    print()
    print(release_notes(release, check_spec_version(ROOT)))
    print(generated_notes(tag, release.previous))


def push_tag(tag: str) -> None:
    """Tag the latest commit of `main` as release `tag` and push the tag."""
    if git(ROOT, "status", "--porcelain"):
        raise ValueError("Commit or stash all changes before tagging a release")
    git(ROOT, "fetch", "--quiet", "origin", "main")
    if git(ROOT, "rev-parse", "HEAD") != git(ROOT, "rev-parse", "origin/main"):
        raise ValueError("Check out the latest commit of origin/main to release it")
    release = plan_release(ROOT, tag, "HEAD")
    git(ROOT, "tag", "--annotate", tag, "--message", f"Release {tag}")
    git(ROOT, "push", "origin", f"refs/tags/{tag}")
    print(f"Pushed {tag}, which publishes: {published_packages(release)}")
    print(f"Follow the Release workflow: https://github.com/{REPOSITORY}/actions")


def default_tag() -> str:
    """Return the tag of a release today, numbering further releases of the same day."""
    date = datetime.datetime.now(datetime.UTC).strftime("%Y.%m.%d")
    existing = set(git(ROOT, "tag", "--list", f"{date}*").split())
    tag, number = date, 0
    while tag in existing:
        number += 1
        tag = f"{date}.{number}"
    return tag


def published_packages(release: Release) -> str:
    names = [
        f"{package.language} {release.versions[package.registry]}"
        for package in PACKAGES
        if package.registry in release.published
    ]
    return ", ".join(names) or "no packages"


def generated_notes(tag: str, previous: str | None) -> str:
    """Generate GitHub's list of merged pull requests without creating a release."""
    head = git(ROOT, "rev-parse", "HEAD").strip()
    fields = [f"tag_name={tag}", f"target_commitish={head}"]
    if previous is not None:
        fields.append(f"previous_tag_name={previous}")
    unavailable = (
        "GitHub's list of merged pull requests is unavailable. It requires the GitHub "
        "CLI with access to the repository and HEAD pushed to GitHub."
    )
    try:
        result = subprocess.run(
            [
                "gh",
                "api",
                f"repos/{REPOSITORY}/releases/generate-notes",
                "--jq",
                ".body",
            ]
            + [argument for field in fields for argument in ("-f", field)],
            capture_output=True,
            text=True,
        )
    except FileNotFoundError:
        return unavailable
    if result.returncode != 0:
        return f"{unavailable}\n{result.stderr.strip()}"
    return result.stdout


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["preview", "tag"])
    parser.add_argument("tag", nargs="?", help="release tag, such as 2026.11.16")
    args = parser.parse_args()
    try:
        git(ROOT, "fetch", "--quiet", "--tags", "origin")
        tag = args.tag or default_tag()
        if args.command == "preview":
            preview(tag)
        else:
            push_tag(tag)
    except ValueError as error:
        parser.exit(1, f"{error}\n")


if __name__ == "__main__":
    main()
