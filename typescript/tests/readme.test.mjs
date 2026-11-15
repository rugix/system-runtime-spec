/** Run the examples of the README. */
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { pathToFileURL } from "node:url";

const readme = readFileSync(new URL("../README.md", import.meta.url), "utf8");
const entry = new URL("../dist/index.js", import.meta.url).href;
const directory = mkdtempSync(join(tmpdir(), "readme-"));

for (const [index, [, example]] of [...readme.matchAll(/```typescript\n(.*?)```/gs)].entries()) {
  test(`README example ${index + 1}`, async () => {
    const path = join(directory, `example-${index}.mjs`);
    writeFileSync(path, example.replaceAll('"system-runtime-spec"', JSON.stringify(entry)));
    await import(pathToFileURL(path).href);
  });
}
