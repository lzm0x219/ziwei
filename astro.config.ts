import { defineConfig } from "astro/config";
import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  site: "https://insight.ziweijs.com",
  integrations: [tailwind()],
  vite: {
    build: {
      rollupOptions: {
        output: {
          entryFileNames: "entry.[hash].js",
          chunkFileNames: "chunks/chunk.[hash].js",
          assetFileNames: "assets/asset.[hash][extname]",
        },
      },
    },
  },
});
