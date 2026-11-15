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

Records are plain JSON objects, so that snapshots carry them unchanged and records of newer minor
versions keep working. The generated models, such as `component.ComponentDeclaration`, describe
their structure and provide runtime checkers.

## Check an Update

The new system image in this example no longer provides `runtime`, so the check reports the app's
requirement as unsatisfied:

```typescript
import { candidateComposition, checkComponents } from "system-runtime-spec";

const current = [
  { id: "os", owner: "rugix.system", version: "1", provides: [{ id: "runtime", version: "3.2" }] },
  { id: "app", owner: "rugix.apps.example", requires: [{ id: "runtime", version: ">=3" }] },
];
const [components, diagnostics] = candidateComposition(
  current, [], ["rugix.system"], [{ id: "os", owner: "rugix.system", version: "2" }],
);
const report = checkComponents(components, diagnostics);
console.log(report.consistent, report.problems);
```

## Evaluate a Snapshot

```typescript
import { evaluateHealth, readSnapshot } from "system-runtime-spec";

const snapshot = readSnapshot({
  specVersion: "1.0",
  clock: { context: { bootId: "boot", nowMs: 45000 }, timestamp: "2026-10-14T12:00:00Z" },
  healthNodes: [{ id: "app" }],
  healthStatuses: [{ id: "app", state: "Healthy", bootId: "boot", observedAtMs: 42000, maxAgeMs: 5000 }],
  metrics: [],
  metricSamples: [],
});
const report = evaluateHealth(snapshot.records, snapshot.clock.context);
console.log(report.nodes);
```

`readSnapshot` throws a `SnapshotError` for snapshots of an unsupported version or with an invalid
structure. Evaluating a snapshot gives health at the time of its clock sample. `estimateTimestamp`
places evidence in wall-clock time for display.

Fields defined as integers, such as `observedAtMs` and `maxAgeMs`, hold values beyond
`Number.MAX_SAFE_INTEGER` as decimal strings, so that they keep their precision; the functions
accept both forms.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
