# System Runtime Specification for Rust

`system-runtime-spec` implements the
[System Runtime Specification](https://github.com/rugix/system-runtime-spec/blob/main/SPEC.md)
in Rust: it discovers records on Linux, checks the compatibility of compositions and updates,
evaluates health, and sends records to other systems as snapshots.

## Installation

```toml
[dependencies]
system-runtime-spec = "0.2"
sidex = "0.5"
```

Import the crate as `system_runtime_spec`. The `sidex` crate provides the JSON values and
functions that some examples use. `Records` holds the records of a system and the
diagnostics about them. The models in `types` are generated from the specification's schemas.

## Read and Check a System

```rust,no_run
use system_runtime_spec::Records;
use system_runtime_spec::types::health::HealthContext;

fn main() -> std::io::Result<()> {
    let records = Records::discover("/");
    let compatibility = records.check_compatibility();
    let health = records.evaluate_health(&HealthContext::current()?);
    println!("consistent: {}", compatibility.consistent);
    for node in &health.nodes {
        println!("{}: usable {:?}, healthy {:?}", node.id, node.usable, node.healthy);
    }
    Ok(())
}
```

`Records::discover` reads the standard directories and the directories that directory
configurations add, and applies overrides. Files that cannot be read or do not contain a valid
record are skipped and reported as diagnostics. Reports take the diagnostics of their part into
account: an error about component declarations makes a composition inconsistent, and a decision
should not rely on a health report with errors about health nodes or directory configurations.

## Add Records in Code

`push` adds a record given as a model, and `push_json` one given as JSON, such as a record
received from another system. Both validate the record and skip it with an error diagnostic if it
is invalid:

```rust
use sidex::types::JsonValue;
use system_runtime_spec::Records;
use system_runtime_spec::types::component::{CapabilitySelectorDeclaration, ComponentDeclaration};

let mut records = Records::new();
records.push(ComponentDeclaration::new("os").with_version(Some("2.1".into())));
records.push(
    ComponentDeclaration::new("app")
        .with_requires(Some(vec![
            CapabilitySelectorDeclaration::new("os").with_version(Some(">=2".into())),
        ])),
);
records.push_json::<ComponentDeclaration>(JsonValue::from_iter([("id", "tool")]));
assert!(records.check_compatibility().consistent);
```

## Check an Update

`with_update` builds the composition that an update would produce: it replaces all component
declarations of the given owners with those of the update.

```rust,no_run
use system_runtime_spec::Records;
use system_runtime_spec::types::component::ComponentDeclaration;

let records = Records::discover("/");
let mut update = Records::new();
update.push(
    ComponentDeclaration::new("os")
        .with_owner(Some("rugix.system".into()))
        .with_version(Some("2.0".into())),
);
let candidate = records.with_update(&["rugix.system"], &update);
println!("consistent: {}", candidate.check_compatibility().consistent);
```

## Send Records as a Snapshot

A `Snapshot` contains the original records, so the receiving system evaluates them as the sending
system read them. Take the clock sample after reading the records:

```rust,no_run
use system_runtime_spec::types::health::HealthClockSample;
use system_runtime_spec::{Parts, Records, Snapshot};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = Records::discover("/");
    let snapshot = Snapshot::new(records, HealthClockSample::now()?, Parts::ALL);
    let json = sidex::formats::json::to_string(&snapshot.to_json())?;

    // On the receiving system:
    let received = Snapshot::from_json(&sidex::formats::json::from_str(&json)?)?;
    let report = received.records().evaluate_health(&received.clock().context);
    for status in received.records().health_statuses() {
        println!("{}: {:?}", status.id, received.clock().estimate_timestamp(status));
    }
    println!("{:?}", report.nodes);
    Ok(())
}
```

`Parts` selects whether a snapshot includes component declarations, health records, or both.
`Snapshot::from_json` fails with a `SnapshotError` for snapshots of an unsupported version or with
an invalid structure. Evaluating a snapshot gives health at the time of its clock sample, and
`HealthClockSample::estimate_timestamp` places evidence in wall-clock time for display.

`Records::discover`, `HealthContext::current`, and `HealthClockSample::now` are available on Linux
only.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
