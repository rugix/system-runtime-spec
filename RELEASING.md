# Releasing the Packages

The Rust crate, Python distribution, and npm package share the name
`system-runtime-spec` and one release version. Rust and Python users import
`system_runtime`; TypeScript users import `system-runtime-spec`.

Pushing a stable version tag, such as `v0.1.0`, runs the Release workflow. It checks
that all package versions and the tag agree, runs the full CI checks, verifies the
Cargo package, and builds the Python and npm distributions. All checks must pass
before any registry receives a package. Python and npm publish the built artifacts;
Cargo packages and publishes the same tagged source.

## Configure Publishing

Create three GitHub environments in `rugix/system-runtime-spec`: `crates-io`,
`pypi`, and `npm`. Restrict their deployment refs to release tags matching `v*`.
Trusted publishing lets a registry verify a GitHub Actions job's identity and
issue short-lived credentials. Configure each registry with these exact values:

| Registry | GitHub Owner | Repository | Workflow Filename | Environment |
| --- | --- | --- | --- | --- |
| crates.io | `rugix` | `system-runtime-spec` | `release.yml` | `crates-io` |
| PyPI | `rugix` | `system-runtime-spec` | `release.yml` | `pypi` |
| npm | `rugix` | `system-runtime-spec` | `release.yml` | `npm` |

For [PyPI](https://docs.pypi.org/trusted-publishers/creating-a-project-through-oidc/),
add a pending trusted publisher for `system-runtime-spec` under your account's
Publishing settings. This creates the project on its first successful publication.

For a new crate, add a short-lived crates.io API token as the `CARGO_REGISTRY_TOKEN`
secret in the `crates-io` GitHub environment. It must permit creating
`system-runtime-spec`. After the first publication, configure
[trusted publishing](https://crates.io/docs/trusted-publishing) in the crate's
settings, then delete the secret and revoke the token. Subsequent releases use
trusted publishing automatically.

For a new npm package, add a short-lived granular token as the `NPM_TOKEN` secret
in the `npm` GitHub environment. In [npm's token settings](https://docs.npmjs.com/creating-and-viewing-access-tokens/),
select **Read and write (publish and stage)** and **Bypass two-factor authentication**
for unattended publication. The token's package access must allow creating the new
package. After the first publication, configure a
[trusted publisher](https://docs.npmjs.com/trusted-publishers/) in the
package's settings. Allow direct `npm publish` for this workflow; a publisher that
only permits staged publication cannot run it. Then delete the secret and revoke
the token. Complete the trusted publisher's first successful publication within
npm's two-day configuration window, or recreate the configuration before releasing.

The first releases require authorized registry accounts and available package
names. A PyPI pending publisher does not reserve its project name. The workflow
does not create registry accounts or configure trusted publishers.

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
