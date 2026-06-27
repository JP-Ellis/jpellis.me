import cloudflare from "@astrojs/cloudflare";
import { unified } from "@astrojs/markdown-remark";
import svelte from "@astrojs/svelte";
import { defineConfig } from "astro/config";
import favicons from "astro-favicons";

import { rehypePymdownx } from "./app/lib/rehype-pymdownx.ts";

export default defineConfig({
  site: "https://jpellis.me",
  srcDir: "./app",
  publicDir: "./public",
  output: "static",
  trailingSlash: "never",
  build: { format: "file" },
  adapter: cloudflare({
    platformProxy: { enabled: true },
    imageService: "passthrough",
    prerenderEnvironment: "node",
  }),
  integrations: [
    svelte(),
    favicons({
      input: "app/assets/favicon.png",
      name: "Joshua Ellis",
      // biome-ignore lint/style/useNamingConvention: favicons/web-manifest field
      short_name: "JP Ellis",
      background: "#FFDDD2",
      themes: ["#D43400"],
      icons: {
        favicons: true,
        android: true,
        appleIcon: [
          {
            name: "apple-touch-icon.png",
            sizes: [{ width: 180, height: 180 }],
            transparent: false,
            rotate: false,
          },
        ],
        appleStartup: false,
        windows: false,
        yandex: false,
      },
    }),
  ],
  markdown: {
    syntaxHighlight: "shiki",
    shikiConfig: {
      themes: {
        light: "github-light",
        dark: "github-dark",
      },
    },
    processor: unified({
      gfm: true,
      smartypants: true,
      rehypePlugins: [rehypePymdownx],
    }),
  },
});
