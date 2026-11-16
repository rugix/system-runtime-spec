# System Runtime Specification for TypeScript

`system-runtime-spec` implements the
[System Runtime Specification](https://github.com/rugix/system-runtime-spec/blob/main/SPEC.md)
in TypeScript: it checks the compatibility of compositions and updates, evaluates health, and
reads snapshots, for example, in a fleet-management service or a user interface. Discovering
records on a device is the job of the Rust and Python packages.

## Installation

The package uses ES modules and includes TypeScript declarations:

```sh
npm install system-runtime-spec
```

`Records` holds the records of a system and the diagnostics about them. The models, such as
`component.ComponentDeclaration`, are generated from the specification's schemas and come with
runtime checkers.

## Evaluate a Snapshot

```typescript
import { Snapshot } from "system-runtime-spec";

const snapshot = Snapshot.fromJSON({
  specVersion: "1.0",
  clock: { context: { bootId: "boot", nowMs: 45000 }, timestamp: "2026-10-14T12:00:00Z" },
  healthNodes: [{ id: "app" }],
  healthStatuses: [{ id: "app", usable: "Yes", healthy: "Yes", bootId: "boot", observedAtMs: 42000, maxAgeMs: 5000 }],
  metrics: [],
  metricSamples: [],
});
const report = snapshot.records.evaluateHealth(snapshot.clock.context);
console.log(report.nodes);
```

`Snapshot.fromJSON` throws a `SnapshotError` for snapshots of an unsupported version or with an
invalid structure, and skips invalid records with an error diagnostic. `JSON.stringify` encodes a
snapshot. Evaluating a snapshot gives health at the time of its clock sample, and
`estimateTimestamp` places evidence in wall-clock time for display.

## Check an Update

`withUpdate` builds the composition that an update would produce: it replaces all component
declarations of the given owners with those of the update. In this example, the new system image
no longer provides `runtime`, so the check reports the app's requirement as unsatisfied:

```typescript
import { Records } from "system-runtime-spec";

const records = new Records();
records.pushJson("ComponentDeclaration", {
  id: "os", owner: "rugix.system", version: "1", provides: [{ id: "runtime", version: "3.2" }],
});
records.pushJson("ComponentDeclaration", {
  id: "app", owner: "rugix.apps.example", requires: [{ id: "runtime", version: ">=3" }],
});
const update = new Records();
update.pushJson("ComponentDeclaration", { id: "os", owner: "rugix.system", version: "2" });
const report = records.withUpdate(["rugix.system"], update).checkCompatibility();
console.log(report.consistent, report.problems);
```

`pushJson` adds a record given as JSON, and `push` one given as a model. Both validate the record
and skip it with an error diagnostic if it is invalid.

Fields defined as integers, such as `observedAtMs` and `maxAgeMs`, hold values beyond
`Number.MAX_SAFE_INTEGER` as decimal strings, so that they keep their precision. The `U64` helpers
of `@sidex/types` convert them to `bigint`.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
