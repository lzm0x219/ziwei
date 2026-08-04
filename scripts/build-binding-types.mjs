import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const common = await readFile(resolve(root, "bindings/ziweijs-core.d.ts"), "utf8");
const wasmInit = await readFile(resolve(root, "bindings/ziweijs-core-wasm-init.d.ts"), "utf8");

const outputs = [
  [resolve(root, "packages/core/index.d.ts"), common],
  [resolve(root, "packages/core-wasm/index.d.ts"), `${wasmInit}\n${common}`],
];

await Promise.all(
  outputs.map(async ([path, contents]) => {
    await mkdir(dirname(path), { recursive: true });
    await writeFile(path, contents);
  }),
);
