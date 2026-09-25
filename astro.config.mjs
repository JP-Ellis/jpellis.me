import { readdirSync } from "node:fs";
import cloudflare from "@astrojs/cloudflare";
import { unified } from "@astrojs/markdown-remark";
import sitemap from "@astrojs/sitemap";
import { defineConfig } from "astro/config";
import favicons from "astro-favicons";

import { rehypePymdownx } from "./app/lib/rehype-pymdownx.ts";

const SITE = "https://jpellis.me";

// The sitemap integration lists only prerendered pages, so the on-demand
// routes (home, blog index, projects and project pages) are added by hand.
const projectPages = readdirSync("./app/content/projects")
  .filter((file) => file.endsWith(".md"))
  .map((file) => `${SITE}/projects/${file.slice(0, -".md".length)}`);
const onDemandPages = [
  `${SITE}/`,
  `${SITE}/blog`,
  `${SITE}/projects`,
  ...projectPages,
];

export default defineConfig({
  site: SITE,
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
    sitemap({
      customPages: onDemandPages,
      filter: (page) => !page.endsWith("/404"),
    }),
    favicons({
      input: "app/assets/favicon.png",
      name: "Joshua Ellis",
      // biome-ignore lint/style/useNamingConvention: favicons/web-manifest field
      short_name: "JP-Ellis",
      background: "#FFDDD2",
      // Light and dark --color-accent from app/styles/tokens/_colors.scss.
      themes: ["#b8492f", "#d05f44"],
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
      // Emit only CSS variables; app/styles/components/_code.scss picks the
      // light or dark set from prefers-color-scheme.
      defaultColor: false,
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
