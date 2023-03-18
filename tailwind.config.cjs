/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  theme: {
    extend: {},
  },

  // Specify the paths to all of the template files in your project
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "./node_modules/@skeletonlabs/skeleton/**/*.{html,js,svelte,ts}",
    "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
  ],

  plugins: [
    require("flowbite/plugin"),
    require("@tailwindcss/forms"),
    require("@tailwindcss/typography"),
    require("@tailwindcss/line-clamp"),
    require("@tailwindcss/container-queries"),
    ...require("@skeletonlabs/skeleton/tailwind/skeleton.cjs")(),
  ],
};
