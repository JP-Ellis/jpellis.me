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
    ignores: ["svelte.config.js"],
    plugins: {
      prettier: prettierPlugin,
    },
  },

  // Node scripts
  {
    files: ["svelte.config.js"],
    languageOptions: {
      globals: {
        process: "readonly",
      },
    },
    plugins: {
      prettier: prettierPlugin,
    },
  },

  // TypeScript
  //
  // The parser uses a singleton, therefore the options specified here _must_ be
  // compatible with its usage in the Svelte config below.
  {
    files: ["**/*.ts"],
    ignores: ["playwright.config.ts"],
    languageOptions: {
      parser: typescriptParser,
      parserOptions: {
        project: "./tsconfig.json",
        extraFileExtensions: [".svelte"],
      },
      globals: {
        process: "readonly",
      },
    },
    plugins: {
      prettier: prettierPlugin,
      "@typescript-eslint": typescriptPlugin,
    },
    rules: {
      ...typescriptPlugin.configs.recommended.rules,
      ...typescriptPlugin.configs["recommended-requiring-type-checking"].rules,
      ...typescriptPlugin.configs.strict.rules,
      "func-style": ["error", "declaration", { allowArrowFunctions: true }],
    },
  },

  // Typescript Type Declarations
  {
    files: ["**/*.d.ts"],
    languageOptions: {},
    plugins: {
      prettier: prettierPlugin,
      "@typescript-eslint": typescriptPlugin,
    },
    rules: {
      ...typescriptPlugin.configs.recommended.rules,
      ...typescriptPlugin.configs["recommended-requiring-type-checking"].rules,
      ...typescriptPlugin.configs.strict.rules,
      "init-declarations": "off",
    },
  },

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
      "no-continue": "off",
      "prefer-destructuring": ["error", { object: true, array: false }],
      "no-magic-numbers": [
        "error",
        {
          ignore: [0, 1, -1],
          ignoreArrayIndexes: true,
          ignoreDefaultValues: true,
          ignoreClassFieldInitialValues: true,
        },
      ],
    },
  },
];
