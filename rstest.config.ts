import { defineConfig } from "@rstest/core";

export default defineConfig({
  testEnvironment: "node",
  coverage: {
    enabled: true,
    include: ["src/**/*.{js,jsx,ts,tsx}"],
  },
});
