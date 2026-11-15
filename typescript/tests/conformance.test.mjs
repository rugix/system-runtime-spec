/** Run the conformance suite shared by all implementations. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

import { Records, Snapshot, SnapshotError, estimateTimestamp } from "../dist/index.js";

const cases = JSON.parse(readFileSync(new URL("../../tests/fixtures/conformance.json", import.meta.url)));
const LISTS = {
  components: "ComponentDeclaration",
  healthNodes: "HealthNode",
  healthStatuses: "HealthStatus",
  metrics: "MetricDefinition",
  metricSamples: "MetricSample",
};
const CLOCK = { context: { bootId: "boot", nowMs: 0 }, timestamp: "2026-01-01T00:00:00Z" };

/** Read the records and diagnostics of the lists of a fixture object. */
function records(value) {
  const result = new Records();
  for (const diagnostic of value.diagnostics ?? []) result.pushDiagnostic(diagnostic);
  for (const [name, kind] of Object.entries(LISTS)) {
    for (const record of value[name] ?? []) result.pushJson(kind, record);
  }
  return result;
}

/** The original JSON of records, as a snapshot contains them. */
function originals(value) {
  return new Snapshot(value, CLOCK).toJSON();
}

/** Keep the severity and kind of diagnostics, whose messages are not specified. */
function brief(report) {
  const diagnostics = report.diagnostics
    .map(({ severity, kind }) => ({ severity, kind }))
    .sort((a, b) => (a.kind + a.severity < b.kind + b.severity ? -1 : a.kind + a.severity > b.kind + b.severity ? 1 : 0));
  return JSON.parse(JSON.stringify({ ...report, diagnostics }));
}

for (const example of cases.versions) {
  test(`version ${JSON.stringify(example.version)} against ${JSON.stringify(example.requirement)}`, () => {
    const report = records({
      components: [
        { id: "provider", version: example.version },
        { id: "consumer", requires: [{ id: "provider", version: example.requirement }] },
      ],
    }).checkCompatibility();
    const errors = report.diagnostics.some((d) => d.severity === "Error");
    assert.equal(errors, Boolean(example.error));
    if (!example.error) assert.equal(report.consistent, example.matches);
  });
}

for (const example of cases.components) {
  test(`components: ${example.name}`, () => {
    assert.deepEqual(brief(records(example).checkCompatibility()), example.report);
  });
}

for (const example of cases.candidates) {
  test(`candidates: ${example.name}`, () => {
    const candidate = records(example).withUpdate(example.replace, records({ components: example.new }));
    assert.deepEqual(originals(candidate).components, example.candidate);
    assert.deepEqual(brief(candidate.checkCompatibility()), example.report);
  });
}

for (const example of cases.health) {
  test(`health: ${example.name}`, () => {
    assert.deepEqual(brief(records(example.records).evaluateHealth(example.context)), example.report);
  });
}

for (const example of cases.snapshots) {
  test(`snapshots: ${example.name}`, () => {
    if (!example.valid) {
      assert.throws(() => Snapshot.fromJSON(example.snapshot), SnapshotError);
      return;
    }
    const snapshot = Snapshot.fromJSON(example.snapshot);
    const json = JSON.parse(JSON.stringify(snapshot));
    const again = Snapshot.fromJSON(json);
    assert.deepEqual([again.toJSON(), again.parts], [snapshot.toJSON(), snapshot.parts]);
    for (const name of Object.keys(LISTS)) assert.deepEqual(json[name], example.snapshot[name]);
    assert.deepEqual(brief({ diagnostics: snapshot.records.diagnostics }).diagnostics, example.diagnostics);
  });
}

for (const example of cases.timestamps) {
  test(`timestamps: ${example.name}`, () => {
    const estimate = estimateTimestamp(example.clock, example.evidence);
    const expected = example.expected === null ? undefined : new Date(example.expected).getTime();
    assert.equal(estimate === undefined ? undefined : new Date(estimate).getTime(), expected);
  });
}
