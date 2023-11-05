import svg from "@poppanator/sveltekit-svg";
import { sveltekit } from "@sveltejs/kit/vite";
import { imagetools } from "@zerodevx/svelte-img/vite";
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
  ],
  test: {
    include: ["src/**/*.{test,spec}.{js,ts}"],
  },
});
