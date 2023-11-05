import { skeleton } from "@skeletonlabs/tw-plugin";
import tailwindContainerQueries from "@tailwindcss/container-queries";
import tailwindForms from "@tailwindcss/forms";
import tailwindTypography from "@tailwindcss/typography";
import flowbite from "flowbite/plugin";
import { join } from "path";
import type { Config } from "tailwindcss";

import { jpellisTheme } from "./src/lib/styles/theme";

const config = {
  darkMode: "class",
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    join(
      require.resolve("@skeletonlabs/skeleton"),
      "../**/*.{html,js,svelte,ts}",
    ),
    join(require.resolve("flowbite"), "../**/*.{html,js,svelte,ts}"),
    // "./node_modules/@skeletonlabs/skeleton/**/*.{html,js,svelte,ts}",
    // "./node_modules/@flowbite/**/*.{html,js,svelte,ts}",
    "./node_modules/@flowbite-svelte/**/*.{html,js,svelte,ts}",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    tailwindForms,
    tailwindTypography,
    tailwindContainerQueries,
    flowbite,
    skeleton({
      themes: {
        custom: [jpellisTheme],
      },
    }),
  ],
} satisfies Config;

export default config;
