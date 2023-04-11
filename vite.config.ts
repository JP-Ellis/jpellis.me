import svg from "@poppanator/sveltekit-svg";
import { sveltekit } from "@sveltejs/kit/vite";
import { imagetools } from "@zerodevx/svelte-img/vite";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [sveltekit(), imagetools(), svg()],
  test: {
    include: ["src/**/*.{test,spec}.{js,ts}"],
  },
});
