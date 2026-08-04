import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import { defineFromBirthContract } from "../../../tests/bindings/from-birth-contract.mjs";
import init, * as sdk from "../index.js";

let initializationError;
try {
  sdk.Ziwei.fromBirth({
    gender: "Yang",
    year: 1984,
    month: 2,
    day: 1,
    hour: 4,
  });
} catch (error) {
  initializationError = error;
}

test("fromBirth requires explicit initialization", () => {
  assert.match(initializationError?.message, /must be initialized/);
});

const wasm = await readFile(new URL("../generated/native_bg.wasm", import.meta.url));
await init(wasm);

defineFromBirthContract(sdk);
