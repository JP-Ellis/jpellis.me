import adapter from "@sveltejs/adapter-cloudflare";
import { vitePreprocess } from "@sveltejs/kit/vite";

// const dev = process.argv.includes("dev");
// if (dev) {
//   process.env.BASE_PATH = "";
// }

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: [vitePreprocess()],

  kit: {
    // Cloudflare
    adapter: adapter({
      routes: {
        include: ["/*"],
        exclude: ["<all>"],
      },
    }),
    // Static
    // adapter: adapter({
    //   pages: "build",
    //   assets: "build",
    //   fallback: null,
    //   precompress: true,
    //   strict: true,
    // }),
    // paths: {
    //   base: process.env.BASE_PATH,
    // },
  },
};

export default config;
