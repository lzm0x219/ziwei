import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["./src/index.ts"],
  format: ["esm", "cjs", "umd"],
  dts: true,
  exports: {
    devExports: true
  },
  globalName: "ZiweiAnalysis",
  deps: {
    skipNodeModulesBundle: true
  },
  publint: true,
  minify: "dce-only"
});
