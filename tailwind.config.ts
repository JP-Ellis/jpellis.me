import skeleton from "@skeletonlabs/skeleton/tailwind/skeleton.cjs";
import tailwindContainerQueries from "@tailwindcss/container-queries";
import tailwindForms from "@tailwindcss/forms";
import tailwindTypography from "@tailwindcss/typography";
import flowbite from "flowbite/plugin";
import type { Config } from "tailwindcss";

export default {
  darkMode: "class",
  theme: {
    extend: {},
  },

  // Specify the paths to all of the template files in your project
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "./node_modules/@skeletonlabs/skeleton/**/*.{html,js,svelte,ts}",
    "./node_modules/flowbite/**/*.js",
    "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
  ],

  plugins: [
    tailwindForms,
    tailwindTypography,
    tailwindContainerQueries,
    flowbite,
    ...skeleton(),
  ],
} satisfies Config;
