import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["./src/index.ts"],
  format: ["esm", "cjs", "umd"],
  dts: true,
  exports: {
    devExports: true
  },
  globalName: "ZiweiCalendar",
  deps: {
    skipNodeModulesBundle: true
  },
  outputOptions: {
    globals: {
      tyme4ts: "tyme4ts"
    }
  },
  publint: true,
  minify: "dce-only"
});
