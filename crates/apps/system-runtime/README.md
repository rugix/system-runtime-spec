# system-runtime

`system-runtime` is a command-line tool for the
[System Runtime Specification](https://github.com/rugix/system-runtime-spec). It reads the
records of a Linux system, checks whether its components are compatible, and evaluates its
health. Scripts, such as update hooks, can use its exit codes to decide whether an update
can be installed or committed.

## Installation

Each [release](https://github.com/rugix/system-runtime-spec/releases) has static binaries for
`x86_64`, `aarch64`, `armv7`, and `arm` (ARMv6 with hardware floating point) Linux. To install
the `aarch64` binary of the latest release:

```sh
curl -LO https://github.com/rugix/system-runtime-spec/releases/latest/download/system-runtime-aarch64-unknown-linux-musl.tar.gz
tar -xzf system-runtime-aarch64-unknown-linux-musl.tar.gz
install system-runtime-aarch64-unknown-linux-musl/system-runtime /usr/local/bin/
```

To install a specific release, such as one in an image build, replace `latest/download` with
`download/<release>`.

Each archive also contains `system-runtime.cdx.json`, a software bill of materials (SBOM) in
[CycloneDX](https://cyclonedx.org) format. It lists the Rust crates that the tool is built from,
for example, for vulnerability scanners.

`SHA256SUMS` contains the checksums of the archives. To check a downloaded archive:

```sh
curl -LO https://github.com/rugix/system-runtime-spec/releases/latest/download/SHA256SUMS
sha256sum --check --ignore-missing SHA256SUMS
```

Checksums detect corrupted downloads. To verify that the release workflow of this repository
built an archive, check its GitHub build provenance attestation with the
[GitHub CLI](https://cli.github.com):

```sh
gh attestation verify system-runtime-aarch64-unknown-linux-musl.tar.gz \
  --repo rugix/system-runtime-spec
```

Release 2026.11.16 predates both conventions: its asset names include the version, and its
archives have no attestations.

## Components

`system-runtime components list` lists the component declarations:

```console
$ system-runtime components list
ID             VERSION  OWNER                 PROVIDES             REQUIRES
app.telemetry  2.1.0    rugix.apps.telemetry  -                    docker.compose >=2.20,<3
os             2.1      rugix.system          docker.compose 2.24  -
```

`system-runtime components check` checks whether the components are compatible. With
`--update`, it checks the composition after an update: the update's component declarations,
read from the update's root directory, replace those of each `--owner`.

```console
$ system-runtime components check --update /mnt/update --owner rugix.system
The components are inconsistent.
- app.telemetry requires docker.compose >=2.20,<3, which no component provides
```

## Health

`system-runtime health show` shows the health of all nodes, or of one node and its inputs:

```console
$ system-runtime health show
USABLE   HEALTHY  NODE
unknown  no       rugix.apps
yes      no       ├── app.telemetry: queue backlog
unknown  unknown  └── app.db (missing)
```

`system-runtime health check <node>` prints whether a node is healthy, or with `--usable`
whether it is usable, and reports the answer through its exit code.

## Common Options

| Option | Description |
| --- | --- |
| `--root <dir>` | Read the records below another root directory, such as a mounted image, instead of `/`. |
| `--json` | Print the report that the specification's schemas define, such as `CompatibilityReport` or `HealthReport`, as JSON. |

Problems with records, such as invalid files, are printed to standard error, unless `--json`
prints a report that includes them.

Set `SYSTEM_RUNTIME_LOG` to a [filter](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html)
such as `debug` for log messages on standard error.

## Exit Codes

| Code | `components check` | `health check` | Other commands |
| --- | --- | --- | --- |
| 0 | Consistent | Yes | Success |
| 1 | Inconsistent | No | |
| 2 | | Unknown | |
| 3 | Failure | Failure | Failure |

A failure is an invalid argument, a missing root directory, an unknown node, or a system state
that cannot be read.
`health check` reports _unknown_ when the evidence does not determine the answer, for
example, because a service has not yet reported its status after boot.
