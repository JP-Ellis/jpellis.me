/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  theme: {
    extend: {},
  },

  // Specify the paths to all of the template files in your project
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    require("path").join(
      require.resolve("@skeletonlabs/skeleton"),
      "../**/*.{html,js,svelte,ts}"
    ),
  ],

  plugins: [
    require("@tailwindcss/forms"),
    require("@tailwindcss/typography"),
    require("@tailwindcss/line-clamp"),
    require("@tailwindcss/container-queries"),
    ...require("@skeletonlabs/skeleton/tailwind/skeleton.cjs")(),
  ],
};
