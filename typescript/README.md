# System Runtime for TypeScript

Use `system-runtime-spec` to check component compatibility and evaluate health
graphs in update tools, services, or user interfaces. Its models and runtime
validators implement the
[System Runtime Specification](https://github.com/rugix/system-runtime-spec).

Callers supply records and a clock context from the observed system. File
discovery and local clock sampling are provided by the Rust and Python packages.

## Installation

The package uses ES modules and includes TypeScript declarations. Install it
from npm:

```sh
npm install system-runtime-spec
```

## Check Component Compatibility

Validate incoming declarations with the generated model checkers, then evaluate
their requirements, conflicts, and claims:

```typescript
import { component, checkComponents } from "system-runtime-spec";

const app = {
  id: "example.app",
  requires: [{ id: "example.runtime", version: ">=3" }],
};
if (!component.ComponentDeclaration.is(app)) {
  throw new Error("Invalid declaration");
}
const report = checkComponents([
  app,
  { id: "example.runtime", version: "3.2" },
]);
console.log(report.consistent);
```

Invalid declarations throw errors. A valid but incompatible composition returns
a report with `consistent: false`.

## Check an Update Candidate

An owner groups records owned by a system image, app, or other contributor.
`withoutOwner` returns a snapshot with new arrays that exclude records owned by
that exact owner. Other records keep their requirements and references. Retained
record objects are shared with the original snapshot; copy a record before
modifying its fields:

```typescript
import { checkComponents, filesystem, withoutOwner } from "system-runtime-spec";

const current: filesystem.RuntimeSnapshot = {
  components: [
    { id: "example.runtime", owner: "rugix.system", version: "3.2" },
    {
      id: "example.app",
      owner: "rugix.apps.example",
      requires: [{ id: "example.runtime", version: ">=3" }],
    },
  ],
  healthNodes: [],
  healthStatuses: [],
};
const candidate = withoutOwner(current, "rugix.system");
candidate.components.push({
  id: "example.runtime", owner: "rugix.system", version: "3.3",
});
console.log(checkComponents(candidate.components).consistent);
```

The current records must already carry owners. This operation changes only the
in-memory candidate. After activation, evaluate health using fresh records from
the running system.

## Evaluate a Health Snapshot

A `HealthSnapshot` carries node definitions, original status records, and a clock
sample from the observed system. Validate a received snapshot before evaluation:

```typescript
import { evaluateHealth, health, statusTimestamp } from "system-runtime-spec";

const received: unknown = {
  clock: {
    context: { bootId: "example-boot", nowMs: 45000 },
    timestamp: "2026-10-14T12:00:00Z",
  },
  nodes: [{ id: "application", rule: { type: "observed" } }],
  statuses: [{
    id: "application",
    state: "healthy",
    bootId: "example-boot",
    observedAtMs: 42000,
    maxAgeMs: 5000,
  }],
};
if (!health.HealthSnapshot.is(received)) {
  throw new Error("Invalid health snapshot");
}
const report = evaluateHealth(
  received.nodes, received.statuses, received.clock.context,
  received.values, received.valueObservations,
);
console.log(report);
for (const status of received.statuses) {
  console.log(status.id, statusTimestamp(status, received.clock));
}
```

This evaluates health at capture time. Obtain current evidence from the observed
system before making an update decision; a receiver's local clock cannot supply
its boot-time context.

`statusTimestamp` returns an estimated UTC timestamp string for display, or
`undefined` for a wrong boot, future observation, or unsupported timestamp range.
Sampling uncertainty and wall-clock corrections limit the estimate.

Pass optional value definitions and observations as the fourth and fifth
arguments to evaluate numeric threshold nodes. The
[value and threshold reference](https://github.com/rugix/system-runtime-spec#runtime-values-and-thresholds)
describes units, ranges, and evidence freshness.

## Models

Sidex generates models and runtime checkers in the `component`, `health`, `value`, and
`filesystem` namespaces. Fields use the serialized names, such as `bootId`.
A model's `is(value)` method narrows its type; `check(value)` returns validation
details. Health node IDs are nominal strings, validated by
`health.HealthNodeId.is(value)`.

Unsigned 64-bit fields accept numbers within JavaScript's safe integer range and
decimal strings for larger values. Evaluation handles both representations.

See the [specification](https://github.com/rugix/system-runtime-spec) for wire
formats, version matching, and health semantics.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
