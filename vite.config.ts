import svg from "@poppanator/sveltekit-svg";
import { sveltekit } from "@sveltejs/kit/vite";
import { imagetools } from "@zerodevx/svelte-img/vite";
import { purgeCss } from "vite-plugin-tailwind-purgecss";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [
    sveltekit(),
    imagetools({
      profiles: {
        run: new URLSearchParams("w=480;1024;1920&format=avif;heif;webp;jpg"),
      },
    }),
    svg(),
    purgeCss({
      safelist: {
        greedy: [/^hljs-/u],
      },
    }),
  ],
  test: {
    include: ["src/**/*.{test,spec}.{js,ts}"],
  },
});
