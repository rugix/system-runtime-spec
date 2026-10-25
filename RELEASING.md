# Releasing the Packages

The Rust crate, Python distribution, and npm package share the name
`system-runtime-spec` and one release version. Rust and Python users import
`system_runtime`; TypeScript users import `system-runtime-spec`.

## Prepare a Release

Update the version in these files together:

- `Cargo.toml`, under `workspace.package`.
- `python/pyproject.toml`, under `project`.
- `typescript/package.json`.
- `sidex.toml`, under `bundle`.

Refresh the lockfiles after changing package versions or dependencies:

```sh
mise install
mise exec -- cargo update --workspace
mise exec -- uv lock --project python
mise exec -- npm install --package-lock-only --ignore-scripts --prefix typescript
mise run check
```

The checks verify generated models, test all three implementations, and
build the Python and npm distributions. Check the Rust distribution separately:

```sh
mise exec -- cargo publish --dry-run --locked
```

Run that dry run from a clean commit containing the release changes. Cargo
verifies the packaged crate against registry dependencies, without uploading it.
Confirm that the resulting packages install and expose their documented imports
before publishing.

## Publish a Release

Merge the release changes and check out that commit with a clean working tree.
Run the checks and Rust dry run above, then tag the commit. For example, for the
first release:

```sh
git tag v0.1.0
git push origin v0.1.0
```

Authenticate with accounts authorized to publish `system-runtime-spec` on each
registry. For a first publication, those accounts create the package entries.
Publish all three distributions from the same commit:

```sh
mise exec -- cargo publish --locked
mise exec -- uv publish python/dist/system_runtime_spec-0.1.0.tar.gz python/dist/system_runtime_spec-0.1.0-py3-none-any.whl
mise exec -- npm publish ./typescript/system-runtime-spec-0.1.0.tgz --access public
```

Use the release version's filenames for subsequent releases. The repository's CI
validates releases; pushing a tag does not publish packages automatically.

Verify each registry entry and install the published version in a fresh consumer
project before announcing the release.
