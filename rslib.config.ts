import { defineConfig } from "@rslib/core";

export default defineConfig({
  lib: [
    {
      format: "esm",
      dts: {
        bundle: true,
      },
      output: {
        distPath: {
          root: "./dist/",
        },
      },
    },
    {
      format: "cjs",
      output: {
        distPath: {
          root: "./dist/",
        },
      },
    },
    {
      format: "umd",
      umdName: "ZiWei",
      output: {
        filename: {
          js: "ziweijs.min.js",
        },
        distPath: {
          root: "./dist",
        },
      },
    },
  ],
});
