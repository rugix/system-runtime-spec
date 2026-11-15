# System Runtime Specification for Rust

`system-runtime-spec` implements the
[System Runtime Specification](https://github.com/rugix/system-runtime-spec/blob/main/SPEC.md)
in Rust: it discovers records on Linux, checks the compatibility of compositions and updates,
evaluates health, and sends records to other systems as snapshots.

## Installation

```toml
[dependencies]
system-runtime-spec = "0.2"
```

Import the crate as `system_runtime_spec`. Functions that take records accept the generated
models in `types` or [`sidex::types::JsonValue`](https://docs.rs/sidex) values. `Records` keeps
records as JSON values, so that snapshots carry them unchanged and records of newer minor versions
keep working. `Records::with_components` and similar methods add typed models.

## Read and Check a System

```rust
use system_runtime_spec::{
    compatibility::check_components,
    discovery::read_records,
    evaluation::{current_context, evaluate_health},
};

fn main() -> Result<(), system_runtime_spec::Error> {
    let records = read_records("/");
    let compatibility = check_components(&records.components, &records.diagnostics);
    let health = evaluate_health(&records, &current_context()?);
    println!("{}", compatibility.consistent);
    for node in &health.nodes {
        println!("{}: {:?}/{:?}", node.id, node.usable, node.healthy);
    }
    Ok(())
}
```

`read_records` reads the standard directories and the directories that directory configurations
add, and applies overrides. Files that cannot be read or do not contain a valid record are skipped
and reported as diagnostics. Checks and evaluation take the diagnostics into account: an error
makes a composition inconsistent, and a decision should not rely on a health report with errors
about health nodes or directory configurations.

## Check an Update

```rust
use sidex::types::JsonValue;
use system_runtime_spec::{
    compatibility::{candidate_composition, check_components},
    discovery::read_records,
};

fn main() {
    let records = read_records("/");
    let update: JsonValue = sidex::formats::json::from_str(
        r#"{"id": "os", "owner": "rugix.system", "version": "2"}"#,
    )
    .unwrap();
    let (components, diagnostics) = candidate_composition(
        &records.components,
        &records.diagnostics,
        &["rugix.system"],
        &[update],
    );
    println!("{}", check_components(&components, &diagnostics).consistent);
}
```

## Send Records as a Snapshot

`make_snapshot` creates a snapshot with the original records, and `read_snapshot` reads one on
the receiving system. Take the clock sample after reading the records:

```rust
use system_runtime_spec::{
    clock::current_clock_sample,
    discovery::read_records,
    evaluation::evaluate_health,
    snapshot::{Parts, make_snapshot, read_snapshot},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = read_records("/");
    let snapshot = make_snapshot(&records, &current_clock_sample()?, Parts::ALL);
    let received = read_snapshot(&snapshot)?;
    let report = evaluate_health(&received.records, &received.clock.context);
    println!("{:?}", report.nodes);
    Ok(())
}
```

`Parts` selects whether a snapshot includes component declarations, health records, or both.
`read_snapshot` fails with a `SnapshotError` for snapshots of an unsupported version or with an
invalid structure. Evaluating a snapshot gives health at the time of its clock sample. `clock::estimate_timestamp`
places evidence in wall-clock time for display.

`discovery`, `current_context`, and `current_clock_sample` are available on Linux only.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
