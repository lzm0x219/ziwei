import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    projects: ["packages/*"],
    coverage: {
      include: ["src/**/*.{ts,tsx}"],
      provider: "v8"
    }
  }
});
