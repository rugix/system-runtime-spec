# System Runtime Specification

> [!IMPORTANT]
> **Draft:** Version 1.0 of the specification is a draft and may still change incompatibly.

The System Runtime Specification is a common format for describing a system's components and its
health at runtime. It is developed by the [Rugix](https://rugix.org) project, but does not depend
on Rugix.

A device typically runs a system image, apps, and services that come from different teams and are
updated independently. An update can remove a runtime that an installed app still needs, and a
device that boots after an update is not necessarily healthy. With this specification, each part
publishes small records that describe only itself, and tools combine them into a picture of the
whole system:

- **Components:** Parts declare what they provide and require. A compatibility checker evaluates
  the declarations together, for example, to reject an update that removes something an installed
  app needs.
- **Health:** Services publish the results of their checks, and health nodes combine the results
  into the health of larger functions, such as an app. A health evaluator determines whether each
  node is usable and fully healthy, for example, so that an update controller commits an update
  only once the system is healthy.

For example, a system image that ships Docker Compose 2.27 declares:

```json
{
  "specVersion": "1.0",
  "id": "system.os",
  "version": "5.2.0",
  "provides": [{ "id": "docker.compose", "version": "2.27.0" }]
}
```

An app that needs Docker Compose 2.20 or a later 2.x version declares:

```json
{
  "specVersion": "1.0",
  "id": "app.telemetry",
  "version": "2.1.0",
  "requires": [{ "id": "docker.compose", "version": ">=2.20,<3" }]
}
```

The two components are compatible. If an update of the system image no longer provides Docker
Compose 2, a compatibility checker reports that the app's requirement is not met.

On Linux, records are JSON files in standard directories, such as
`/usr/lib/system-runtime/components` for the components of the system image. No daemon has to run
for a part to publish its records.

## Documents

- **[Specification](SPEC.md):** the normative definition of records, versions, compatibility
  checking, health evaluation, discovery, and transport, together with the [schemas](schemas/).
- **[Rugix documentation](https://rugix.org/docs/ctrl/next/reference/system-runtime/):** an
  introduction with a worked example and an explanation of the design.

## Contents of This Repository

| Path | Contents |
| --- | --- |
| [`SPEC.md`](SPEC.md) | The specification. |
| [`schemas/`](schemas/) | The record types in [Sidex](https://github.com/silitics/sidex), part of the specification. |
| [`json-schema/`](json-schema/) | JSON Schemas generated from the Sidex schemas. |
| [`tests/fixtures/`](tests/fixtures/) | The conformance suite, shared by all implementations. |
| [`crates/libs/system-runtime/`](crates/libs/system-runtime/) | The Rust implementation. |
| [`crates/apps/system-runtime/`](crates/apps/system-runtime/) | The command-line tool. |
| [`python/`](python/) | The Python implementation. |
| [`typescript/`](typescript/) | The TypeScript implementation. |

## Implementations

All three implementations are published as `system-runtime-spec`. They check compatibility,
evaluate health, and create and read snapshots, which carry the records of a system so that
another system can check or evaluate them. The Rust and Python implementations also discover
records in the filesystem. Each package has its own version, independent of the specification's,
and exports the version of the specification that it implements as `SPEC_VERSION`. Version 0.2
of each package implements version 1.0 (draft) of the specification.

| Language | Package | Documentation |
| --- | --- | --- |
| Rust | [crates.io](https://crates.io/crates/system-runtime-spec) | [README](crates/libs/system-runtime/README.md) |
| Python | [PyPI](https://pypi.org/project/system-runtime-spec/) | [README](python/README.md) |
| TypeScript | [npm](https://www.npmjs.com/package/system-runtime-spec) | [README](typescript/README.md) |

## Command-Line Tool

The `system-runtime` command-line tool lists the components of a Linux system, checks whether they
are compatible, before or after an update, and evaluates the system's health.

Scripts can use its exit codes, for example, to commit an update only once the telemetry app is
healthy. If the app declares a health node `app.telemetry`, an update hook runs:

```sh
system-runtime health check app.telemetry
```

The command exits with 0 if the node is healthy, with 1 if it is not, and with 2 if the evidence
does not yet determine the answer, for example, because the app has not reported its status since
boot. It exits with 3 if it fails, for example, because no such node exists.

Each [release](https://github.com/rugix/system-runtime-spec/releases) has static binaries for
Linux, versioned with the release. The tool's [README](crates/apps/system-runtime/README.md)
describes the commands.

## Development

Development requires [mise](https://mise.jdx.dev). `mise run generate` generates the models and
JSON Schemas from the Sidex schemas. `mise run check` runs all checks, such as the conformance
suite and a check that `SPEC.md` quotes the schemas exactly:

```sh
mise install
mise run generate
mise run check
```

CI runs the same checks. [RELEASING.md](RELEASING.md) describes how releases are published.

## Licensing

This project is licensed under either [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache 2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

Made with ❤️ for OSS by [Silitics](https://www.silitics.com)
