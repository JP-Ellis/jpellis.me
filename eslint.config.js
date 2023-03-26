import js from "@eslint/js";
import typescriptPlugin from "@typescript-eslint/eslint-plugin";
import typescriptParser from "@typescript-eslint/parser";
import prettierConfig from "eslint-config-prettier";
import prettierPlugin from "eslint-plugin-prettier";
import sveltePlugin from "eslint-plugin-svelte";
import svelteParser from "svelte-eslint-parser";

export default [
  // Glocal ignores
  {
    ignores: [".svelte-kit/**/*"],
  },

  // Load predefined config
  js.configs.recommended,
  js.configs.all,
  prettierConfig,

  // JavaScript
  {
    files: ["**/*.js", "**/*.cjs"],
    plugins: {
      prettier: prettierPlugin,
    },
  },

  // TypeScript
  // The following config is not used because it is not compatible with Svelte.
  // Issue: ota-meshi/eslint-plugin-svelte#422
  // {
  //   files: ["**/*.ts"],
  //   ignores: ["playwright.config.ts"],
  //   languageOptions: {
  //     parser: typescriptParser,
  //     parserOptions: {
  //       project: "./tsconfig.json",
  //     },
  //   },
  //   plugins: {
  //     prettier: prettierPlugin,
  //     "@typescript-eslint": typescriptPlugin,
  //   },
  //   rules: {
  //     ...typescriptPlugin.configs.recommended.rules,
  //     ...typescriptPlugin.configs["recommended-requiring-type-checking"].rules,
  //     ...typescriptPlugin.configs.strict.rules,
  //   },
  // },

  // Svelte
  {
    files: ["**/*.svelte"],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: typescriptParser,
        project: "./tsconfig.json",
        extraFileExtensions: [".svelte"],
      },
    },
    plugins: {
      svelte: sveltePlugin,
      prettier: prettierPlugin,
      "@typescript-eslint": typescriptPlugin,
    },
    rules: {
      ...typescriptPlugin.configs.recommended.rules,
      ...typescriptPlugin.configs["recommended-requiring-type-checking"].rules,
      ...typescriptPlugin.configs.strict.rules,
      ...sveltePlugin.configs.recommended.rules,
      ...sveltePlugin.configs.prettier.rules,
    },
  },

  // Shared rule ignores
  {
    rules: {
      // Prefer `const` over `let`
      "prefer-const": "error",
      "one-var": ["error", "never"],

      // Disable rules that are too strict
      "sort-imports": "off",
      "sort-keys": "off",
      "multiline-comment-style": "off",
      "capitalized-comments": "off",
    },
  },
];
