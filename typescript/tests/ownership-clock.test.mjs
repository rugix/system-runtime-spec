/** Owner-based update candidates and transferable health clock correlation. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

import {
  statusTimestamp, checkComponents, evaluateHealth, filesystem, health, withoutOwner,
} from "../dist/index.js";

const cases = JSON.parse(readFileSync(new URL("../../tests/fixtures/ownership-clock.json", import.meta.url)));

for (const example of cases.owners) {
  test(`owner update: ${example.name}`, () => {
    assert(filesystem.RuntimeSnapshot.is(example.snapshot));
    const original = JSON.stringify(example.snapshot);
    const candidate = withoutOwner(example.snapshot, example.owner);
    assert.deepEqual(candidate, example.expected);
    assert.equal(checkComponents(candidate.components).consistent, example.consistent);
    if (example.replacement) {
      candidate.components.push(example.replacement);
      assert.equal(checkComponents(candidate.components).consistent, true);
    }
    assert.equal(JSON.stringify(example.snapshot), original);
  });
}

for (const example of cases.timestamps) {
  test(`timestamp: ${example.name}`, () => {
    assert.equal(
      statusTimestamp(example.status, example.clock),
      example.expected ?? undefined,
    );
  });
}

test("owned health graph survives transport without renewing evidence", () => {
  const source = cases.owners[0].snapshot;
  const snapshot = JSON.parse(JSON.stringify({
    clock: cases.timestamps[0].clock,
    nodes: source.healthNodes,
    statuses: source.healthStatuses,
    values: source.values,
    valueObservations: source.valueObservations,
  }));
  assert(health.HealthSnapshot.is(snapshot));
  assert.deepEqual(snapshot.values, source.values);
  assert.deepEqual(snapshot.valueObservations, source.valueObservations);
  const report = evaluateHealth(snapshot.nodes, snapshot.statuses, snapshot.clock.context);
  assert.equal(report.nodes[1].owner, "rugix.apps.example");
  assert.equal(report.nodes[1].state, "healthy");
  assert.equal(statusTimestamp(snapshot.statuses[1], snapshot.clock), "2026-10-14T11:59:58.623Z");
  snapshot.clock.context.nowMs = 5500;
  assert.equal(evaluateHealth(snapshot.nodes, snapshot.statuses, snapshot.clock.context).nodes[1].state, "unknown");
  assert.equal(snapshot.statuses[1].observedAtMs, 3500);
});
