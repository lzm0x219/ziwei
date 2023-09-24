import { defineConfig } from "astro/config";
import tailwind from "@astrojs/tailwind";
import partytown from "@astrojs/partytown";
import prefetch from "@astrojs/prefetch";
import mkcert from "vite-plugin-mkcert";

// https://astro.build/config
export default defineConfig({
  site: "https://insight.ziweijs.com",
  integrations: [tailwind(), partytown(), prefetch()],
  vite: {
    plugins: [mkcert()],
    build: {
      rollupOptions: {
        output: {
          entryFileNames: "entry.[hash].js",
          chunkFileNames: "chunks/chunk.[hash].js",
          assetFileNames: "assets/asset.[hash][extname]",
        },
      },
      cssMinify: "lightningcss",
    },
  },
});
