import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["./src/index.ts"],
  format: ["esm", "cjs", "umd"],
  dts: true,
  exports: {
    devExports: true
  },
  globalName: "ZiweiCore",
  deps: {
    skipNodeModulesBundle: true
  },
  publint: true,
  minify: "dce-only"
});
