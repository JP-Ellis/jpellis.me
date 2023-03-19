module.exports = {
  root: true,

  parser: "@typescript-eslint/parser",
  overrides: [{ files: ["*.svelte"], processor: "svelte3/svelte3" }],

  plugins: ["svelte3", "@typescript-eslint"],
  extends: [
    "eslint:all",
    "plugin:@typescript-eslint/recommended",
    "plugin:@typescript-eslint/recommended-requiring-type-checking",
    "plugin:@typescript-eslint/strict",
    "prettier",
  ],
  rules: {
    "capitalized-comments": "off",
    "sort-imports": "off",
    "sort-keys": "off",
    "multiline-comment-style": "off",
  },

  // ignorePatterns: ["*.cjs"],
  settings: {
    "svelte3/typescript": () => require("typescript"),
  },

  parserOptions: {
    project: ["./tsconfig.json"],
    sourceType: "module",
    ecmaVersion: 2020,
  },

  env: {
    browser: true,
    es2017: true,
    node: true,
  },
};
