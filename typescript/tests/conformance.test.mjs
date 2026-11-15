/** Run the conformance suite shared by all implementations. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

import {
  candidateComposition, checkComponents, estimateTimestamp, evaluateHealth, readSnapshot,
} from "../dist/index.js";

const cases = JSON.parse(readFileSync(new URL("../../tests/fixtures/conformance.json", import.meta.url)));

/** Keep the severity and kind of diagnostics, whose messages are not specified. */
function brief(report) {
  const diagnostics = report.diagnostics
    .map(({ severity, kind }) => ({ severity, kind }))
    .sort((a, b) => (a.kind + a.severity < b.kind + b.severity ? -1 : a.kind + a.severity > b.kind + b.severity ? 1 : 0));
  return JSON.parse(JSON.stringify({ ...report, diagnostics }));
}

for (const example of cases.versions) {
  test(`version ${JSON.stringify(example.version)} against ${JSON.stringify(example.requirement)}`, () => {
    const report = checkComponents([
      { id: "provider", version: example.version },
      { id: "consumer", requires: [{ id: "provider", version: example.requirement }] },
    ]);
    const errors = report.diagnostics.some((d) => d.severity === "Error");
    assert.equal(errors, Boolean(example.error));
    if (!example.error) assert.equal(report.consistent, example.matches);
  });
}

for (const example of cases.components) {
  test(`components: ${example.name}`, () => {
    assert.deepEqual(brief(checkComponents(example.components, example.diagnostics)), example.report);
  });
}

for (const example of cases.candidates) {
  test(`candidates: ${example.name}`, () => {
    const [components, diagnostics] = candidateComposition(
      example.components, example.diagnostics ?? [], example.replace, example.new,
    );
    assert.deepEqual(brief(checkComponents(components, diagnostics)), example.report);
  });
}

for (const example of cases.health) {
  test(`health: ${example.name}`, () => {
    assert.deepEqual(brief(evaluateHealth(example.records, example.context)), example.report);
  });
}

for (const example of cases.snapshots) {
  test(`snapshots: ${example.name}`, () => {
    if (!example.valid) {
      assert.throws(() => readSnapshot(example.snapshot));
      return;
    }
    const snapshot = readSnapshot(example.snapshot);
    assert.deepEqual(brief({ diagnostics: snapshot.records.diagnostics }).diagnostics, example.diagnostics);
  });
}

for (const example of cases.timestamps) {
  test(`timestamps: ${example.name}`, () => {
    const estimate = estimateTimestamp(example.evidence, example.clock);
    const expected = example.expected === null ? undefined : new Date(example.expected).getTime();
    assert.equal(estimate === undefined ? undefined : new Date(estimate).getTime(), expected);
  });
}
