# System Runtime Specification

The System Runtime Specification is a common format for describing a system's components and its
health at runtime. It is developed by the [Rugix Project](https://rugix.io), but does not depend on
Rugix.

A device typically runs a system image, apps, and services that come from different teams and are
updated independently. An update can remove a runtime that an installed app still needs, and a
device that boots after an update is not necessarily healthy. With this specification, each part
publishes small records that describe only itself, and tools combine them into a picture of the
whole system:

- **Components:** Parts declare what they provide and require. A compatibility checker evaluates
  the declarations together, for example, to reject an update before installing it if an
  installed app needs something the update removes.
- **Health:** Services publish the results of their checks, and parts declare how the results
  combine into the health of larger functions. A health evaluator determines whether each function
  is usable and whether it is fully healthy, for example, so that an update controller commits an
  update only once the updated system is healthy.

For example, an app that needs Docker Compose 2.20 or later declares:

```json
{
  "specVersion": "1.0",
  "id": "app.telemetry",
  "version": "2.1.0",
  "requires": [{ "id": "docker.compose", "version": ">=2.20,<3" }]
}
```

On Linux, records are JSON files in standard directories, such as
`/usr/lib/system-runtime/components` for the components of the system image. No daemon has to run
for a part to publish its records.

## Documents

- **[Specification](SPEC.md):** the normative definition of records, versions, compatibility
  checking, health evaluation, discovery, and transport, together with the [schemas](schemas/).
- **[Rugix documentation](https://rugix.io/docs/ctrl/next/reference/system-runtime/):** an
  introduction with a worked example and an explanation of the design.

## Contents of This Repository

| Path | Contents |
| --- | --- |
| [`SPEC.md`](SPEC.md) | The specification. |
| [`schemas/`](schemas/) | The record types in [Sidex](https://github.com/silitics/sidex), part of the specification. |
| [`json-schema/`](json-schema/) | JSON Schemas generated from the Sidex schemas. |
| [`tests/fixtures/`](tests/fixtures/) | The conformance suite, shared by all implementations. |
| [`crates/libs/system-runtime/`](crates/libs/system-runtime/) | The Rust implementation. |
| [`python/`](python/) | The Python implementation. |
| [`typescript/`](typescript/) | The TypeScript implementation. |

## Implementations

All three implementations are published as `system-runtime-spec`. They check compatibility,
evaluate health, and create and read snapshots; the Rust and Python implementations also discover
records in the filesystem. Version 0.2 of the packages implements version 1.0 (draft) of the
specification.

| Language | Package | Documentation |
| --- | --- | --- |
| Rust | [crates.io](https://crates.io/crates/system-runtime-spec) | [README](crates/libs/system-runtime/README.md) |
| Python | [PyPI](https://pypi.org/project/system-runtime-spec/) | [README](python/README.md) |
| TypeScript | [npm](https://www.npmjs.com/package/system-runtime-spec) | [README](typescript/README.md) |

## Development

The repository uses [mise](https://mise.jdx.dev) for its tooling. To regenerate the models and
JSON Schemas from the Sidex schemas and to run all checks, including the conformance suite and a
check that `SPEC.md` quotes the schemas exactly:

```sh
mise install
mise run generate
mise run check
```

CI runs the same checks. [RELEASING.md](RELEASING.md) describes how the packages are published.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
