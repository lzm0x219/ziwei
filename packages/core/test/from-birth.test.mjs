import assert from "node:assert/strict";
import { createRequire } from "node:module";
import test from "node:test";

import { defineFromBirthContract } from "../../../tests/bindings/from-birth-contract.mjs";
import * as sdk from "../index.mjs";

const require = createRequire(import.meta.url);
const commonJsSdk = require("../index.cjs");

test("ESM and CommonJS expose the same classes", () => {
  assert.equal(sdk.Ziwei, commonJsSdk.Ziwei);
  assert.equal(sdk.ZiweiInputError, commonJsSdk.ZiweiInputError);
});

defineFromBirthContract(sdk);
