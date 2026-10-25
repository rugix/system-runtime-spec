# Releasing the Packages

The Rust crate, Python distribution, and npm package share the name
`system-runtime-spec` and one release version. Rust and Python users import
`system_runtime`; TypeScript users import `system-runtime-spec`.

Pushing a stable version tag, such as `v0.1.0`, runs the Release workflow. It checks
that all package versions and the tag agree, runs the full CI checks, verifies the
Cargo package, and builds the Python and npm distributions. All checks must pass
before any registry receives a package. Python and npm publish the built artifacts;
Cargo packages and publishes the same tagged source.

## Prepare a Release

Only stable `X.Y.Z` versions are supported. Update these files together:

- `Cargo.toml`, under `workspace.package`.
- `python/pyproject.toml`, under `project`.
- `typescript/package.json`.
- `sidex.toml`, under `bundle`.

Refresh the lockfiles and check the release:

```sh
mise install
mise exec -- cargo update --workspace
mise exec -- uv lock --project python
mise exec -- npm install --package-lock-only --ignore-scripts --prefix typescript
mise exec -- python scripts/check_release.py
mise run check
```

Merge the release changes. From GitHub Actions, run the Release workflow on that
commit's branch with `publish` left false. This dry run checks and builds all
packages without accessing publishing environments or uploading to registries.
Download its Python and npm artifacts and verify that they install and expose the
documented imports. To verify the Rust package locally from a clean release commit:

```sh
mise exec -- cargo publish --dry-run --locked
```

## Publish a Release

Check out the merged release commit with a clean working tree, then push its
matching tag. For example, to release version 0.1.0:

```sh
git tag v0.1.0
git push origin v0.1.0
```

Watch the Release workflow until all three publishing jobs succeed. Verify each
registry entry and install the published version in a fresh consumer project
before announcing the release.

Registry uploads are independent and published versions cannot be overwritten.
If only some registries succeed, keep the tag unchanged and rerun only failed jobs.
To target a remaining registry explicitly, run the Release workflow manually,
select the same release tag, set `publish` to true, and choose that registry.
PyPI retries skip files already uploaded, so an interrupted wheel or source archive
upload can finish. For npm and crates.io, check whether the version reached the
registry even if the job failed; an existing version cannot be uploaded again.
Do not retry a successful registry. Fix released package defects in a new version.
