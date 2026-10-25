# System Runtime for Rust

Use `system-runtime-spec` to load component and health files on Linux, check component
compatibility, and evaluate health graphs. Its models implement the
[System Runtime Specification](https://github.com/rugix/system-runtime-spec).

## Installation

Add the crate from crates.io to your project's `Cargo.toml`:

```toml
[dependencies]
system-runtime-spec = "0.1"
```

The package is named `system-runtime-spec`; import the crate as `system_runtime`.

## Read and Check a System

`RuntimeDirectories::standard("/")` loads the specification's standard
directories and extension configurations. The returned `RuntimeSnapshot`
contains components, health nodes and statuses, and optional value definitions
and observations.

```rust
use system_runtime::{
    RuntimeDirectories, check_components, current_context, evaluate_health_with_values,
};

fn main() -> Result<(), system_runtime::Error> {
    let snapshot = RuntimeDirectories::standard("/").load()?;
    let compatibility = check_components(&snapshot.components)?;
    let health = evaluate_health_with_values(
        &snapshot.health_nodes,
        &snapshot.health_statuses,
        snapshot.values.as_deref().unwrap_or_default(),
        snapshot.value_observations.as_deref().unwrap_or_default(),
        &current_context()?,
    )?;
    println!("{compatibility:?}\n{health:?}");
    Ok(())
}
```

`current_context()` reads the Linux boot ID and `CLOCK_BOOTTIME` for freshness
checks. This helper and `current_clock_sample()` are available only on Linux.
For records from another system, supply that system's `HealthContext`.
Invalid declarations and discovery failures return `Error`; a valid but
incompatible composition returns a report with `consistent: false`.

The `evaluate_health_with_values` function handles numeric threshold nodes as
well as observed and aggregate nodes. Use `evaluate_health` for graphs without
values. The [value and threshold reference](https://github.com/rugix/system-runtime-spec#runtime-values-and-thresholds)
describes units, ranges, and evidence freshness.

## Add Directories

Use extension files as described in the
[file discovery specification](https://github.com/rugix/system-runtime-spec#file-discovery),
or extend the sources in code:

```rust
use system_runtime::{RuntimeDirectories, filesystem::DirectoryConfig};

fn main() -> Result<(), system_runtime::Error> {
    let mut directories = RuntimeDirectories::standard("/");
    directories.extend(&DirectoryConfig {
        components: Some(vec!["/opt/example/components".into()]),
        health_nodes: None,
        health_statuses: None,
        values: None,
        value_observations: None,
    });
    let snapshot = directories.load()?;
    println!("{snapshot:?}");
    Ok(())
}
```

Use `RuntimeDirectories::new("/")` to start with empty source lists. A different
root, such as `RuntimeDirectories::standard("/mnt/system")`, relocates absolute
source paths for tests or alternate installations. Absolute symlink targets
still resolve against the host's root, rather than the supplied root.

## Check an Update Candidate

An origin groups records owned by a system image, app, or other contributor.
`without_origin` copies a snapshot without records owned by that exact origin.
Other records retain their requirements and references.

```rust
use system_runtime::{
    RuntimeDirectories, check_components, component::ComponentDeclaration, without_origin,
};

fn main() -> Result<(), system_runtime::Error> {
    let current = RuntimeDirectories::standard("/").load()?;
    let mut candidate = without_origin(&current, "rugix.system");
    candidate.components.push(ComponentDeclaration {
        id: "example.system".into(),
        origin: Some("rugix.system".into()),
        version: Some("2.0".into()),
        provides: None,
        claims: None,
        requires: None,
        conflicts: None,
    });
    let report = check_components(&candidate.components)?;
    println!("{}", report.consistent);
    Ok(())
}
```

The current records must already carry origins. Discovery does not infer
ownership from paths; attribute imported records before assembling the snapshot.
This operation changes only the in-memory candidate. After activation, evaluate
health using fresh records from the running system.

## Capture Health for Transport

Add `serde_json = "1"` under `[dependencies]` for this JSON round trip. Collect
records first, then sample the observed system's clock:

```rust
use system_runtime::{
    RuntimeDirectories, current_clock_sample, health::HealthSnapshot, status_timestamp,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = RuntimeDirectories::standard("/").load()?;
    let snapshot = HealthSnapshot {
        clock: current_clock_sample()?,
        nodes: records.health_nodes,
        statuses: records.health_statuses,
        values: records.values,
        value_observations: records.value_observations,
    };
    let payload = serde_json::to_string(&snapshot)?;
    let received: HealthSnapshot = serde_json::from_str(&payload)?;
    for status in &received.statuses {
        println!("{:?}: {:?}", status.id, status_timestamp(status, &received.clock));
    }
    Ok(())
}
```

Models support Serde and Sidex encoding. `current_clock_sample()` samples UTC
between two boot-time reads and pairs it with their midpoint.
`status_timestamp` returns an estimated UTC observation time for display, or
`None` for a wrong boot, future observation, or unsupported timestamp range.
Sampling uncertainty and wall-clock corrections limit the estimate.

Evaluating a received snapshot with `snapshot.clock.context` gives health at
capture time. Obtain current evidence before making an update decision.

## Models

Generated models live in `component`, `health`, `value`, and `filesystem`. Health node IDs
use a distinct `HealthNodeId` type: construct one with
`HealthNodeId::from("database")` and read its string with `as_str()`.

See the [specification](https://github.com/rugix/system-runtime-spec) for wire
formats, version matching, and health semantics.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
