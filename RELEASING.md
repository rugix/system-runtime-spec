# Releasing

The repository has three version lines:

- **The specification** has a `MAJOR.MINOR` version, such as `1.0`, which changes only when the
  format changes. `SPEC.md` states it, `sidex.toml` uses it as the bundle version `MAJOR.MINOR.0`,
  and each package exports it as `SPEC_VERSION`.
- **The packages** have independent SemVer versions. The Rust crate, Python distribution, and npm
  package share the name `system-runtime-spec`. Rust and Python users import
  `system_runtime_spec`; TypeScript users import `system-runtime-spec`.
- **Releases** are tagged with their date, such as `2026.11.16`, or `2026.11.16.1` for a second
  release on the same day. Each release has one GitHub release. The `system-runtime`
  command-line tool has the version of the release that built it.

A release publishes each package whose files changed since the previous release. These are the
files below `crates/libs/system-runtime/` and the workspace `Cargo.toml` for Rust, below `python/`
for Python, and below `typescript/` for TypeScript. A changed package needs a higher version than
in the previous release. Packages that did not change are not published again. Every release
builds the command-line tool.

Pushing a release tag runs the Release workflow. It checks the versions against the previous
release, runs the full CI checks, verifies the Cargo package, and builds the Python and npm
distributions and the command-line tool. All checks must pass before any registry receives a
package. Python and npm publish the built artifacts; Cargo packages and publishes the same tagged
source. Once every publishing job has succeeded, the workflow creates the GitHub release. Its notes
list the specification version and the version of each package, followed by the pull requests
merged since the previous release. Its assets are an archive of the command-line tool for each
Linux target, built with `mise run build:cli <target>`, and the archives' checksums in
`SHA256SUMS`. Each archive also contains a CycloneDX SBOM of the tool, generated with
[cargo-cyclonedx](https://github.com/CycloneDX/cyclonedx-rust-cargo).

## Prepare a Release

Bump the version of each package that changed since the previous release:

- Rust: `Cargo.toml`, under `workspace.package`.
- Python: `python/pyproject.toml`, under `project`.
- TypeScript: `typescript/package.json`.

A new specification version changes `SPEC.md`, the `bundle` version in `sidex.toml`, and
`SPEC_VERSION` in every package. `mise run check` verifies that they agree.

Refresh the lockfiles and run the checks:

```sh
mise install
mise run lock
mise run check
```

Merge the release changes. From GitHub Actions, run the Release workflow on that commit's branch
with `publish` left false. This dry run checks and builds all packages without accessing publishing
environments or uploading to registries. Download its Python and npm artifacts and verify that they
install and expose the documented imports, and run one of its command-line tool builds. To verify
the Rust package locally from a clean release commit:

```sh
mise exec -- cargo publish --dry-run --locked -p system-runtime-spec
```

To preview the release of the checked-out commit, run:

```sh
mise run release:preview
```

The preview checks the version bumps against the previous release, lists the packages that the
release publishes, and prints the release notes. GitHub's list of merged pull requests requires
the [GitHub CLI](https://cli.github.com) with access to the repository and the commit pushed to
GitHub. The tag defaults to the current UTC date; pass a tag to override it, such as
`mise run release:preview 2026.11.16.1`.

## Publish a Release

Check out the latest commit of `main` with a clean working tree, then tag and push the release:

```sh
mise run release:tag
```

The task refuses to tag anything other than the latest commit of `origin/main` and runs the same
version check as the preview before it pushes the tag.

Watch the Release workflow until the publishing jobs and the GitHub release job succeed. Verify
each registry entry and install the published versions in a fresh consumer project before
announcing the release.

Registry uploads are independent and published versions cannot be overwritten. If only some
registries succeed, keep the tag unchanged and rerun only failed jobs. To target a remaining
registry explicitly, run the Release workflow manually, select the same release tag, set `publish`
to true, and choose that registry. The GitHub release follows once no publishing job has failed;
rerunning its job uploads missing assets.
PyPI retries skip files already uploaded, so an interrupted wheel or source archive upload can
finish. For npm and crates.io, check whether the version reached the registry even if the job
failed; an existing version cannot be uploaded again. Do not retry a successful registry. Fix
released package defects in a new release.
