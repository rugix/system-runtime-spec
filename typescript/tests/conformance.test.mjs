/** Shared cross-language compatibility and health use cases. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

import { checkComponents, evaluateHealth, component, health } from "../dist/index.js";

const cases = JSON.parse(readFileSync(new URL("../../tests/fixtures/conformance.json", import.meta.url)));

for (const example of cases.versions) {
  test(`version ${example.version} against ${example.requirement}`, () => {
    const components = [
      { id: "provider", version: example.version },
      { id: "consumer", requires: [{ id: "provider", version: example.requirement }] },
    ];
    if (example.error) assert.throws(() => checkComponents(components));
    else assert.equal(checkComponents(components).consistent, example.matches);
  });
}

for (const example of cases.components) {
  test(`composition: ${example.name}`, () => {
    for (const value of example.components) assert(component.ComponentDeclaration.is(value));
    if (example.error) assert.throws(() => checkComponents(example.components));
    else {
      const report = checkComponents(example.components);
      assert(component.CompatibilityReport.is(report));
      assert.deepEqual(report, example.report);
    }
  });
}

for (const example of cases.health) {
  test(`health: ${example.name}`, () => {
    for (const node of example.nodes) assert(health.HealthNode.is(node));
    for (const status of example.statuses) assert(health.HealthStatus.is(status));
    const evaluate = () => evaluateHealth(example.nodes, example.statuses, example.context, example.metrics, example.metricSamples);
    if (example.error) assert.throws(evaluate);
    else {
      const report = evaluate();
      assert(health.HealthReport.is(report));
      assert.deepEqual(report, example.report);
    }
  });
}
