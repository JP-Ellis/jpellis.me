# CLAUDE.md

Guidance for AI agents working in this repository. For setup and a tour of the
project, read [`README.md`](README.md) first; this file covers the conventions
and gotchas that are easy to get wrong.

## Toolchain

- **mise** manages the toolchain and tasks. `mise install` provisions
  everything; `mise tasks` lists the tasks; run them with `mise run <task>`.
- **aube** is the package manager. Use `aube` (install), `aubr` (`aube run`,
  for `package.json` scripts) and `aubx` (`aube exec`, like `npx`). Do **not**
  use `npm`/`pnpm`/`npx`. Dependencies auto-install on stale `node_modules`.
- The **`e2e/` directory is a separate workspace** with its own
  `package.json` and lockfile. CI installs it explicitly; locally,
  `mise run e2e:test` handles it.

## Commands

- `mise run dev` — dev server. `mise run build` — production build.
- `mise run preview` — build then serve the Worker via `wrangler dev`.
- `mise run test` — Vitest unit tests + Playwright e2e.
- `mise run check` / `check:fix` — format + lint (Biome, tombi, yamlfix).
- `mise run data:update`, `mise run oss-contribs:update` — refresh baked-in
  GitHub data from the live API (need a token; see the README).

## Architecture

- **Astro 7, static-first.** `output: "static"` in `astro.config.mjs`
  pre-renders everything by default. Pages that need live data opt into SSR
  per-route with `export const prerender = false`.
- **Cloudflare Workers.** Deployed via `@astrojs/cloudflare`. The build emits
  `dist/server/wrangler.json`. Bindings live in `wrangler.toml`: `ASSETS`, and
  the `GITHUB_STATS` / `PROJECTS_STATS` KV namespaces. The GitHub PAT is a
  Worker secret (`GITHUB_TOKEN`).
- **SWR caching.** SSR pages read GitHub stats from KV via
  `app/lib/stats-cache.ts` (`readWithSWR`), refreshing in the background with
  `Astro.locals.cfContext.waitUntil`. On any failure they fall back to the
  JSON snapshots in `app/data/`.
- **Content** is Markdown in Astro content collections (`app/content/blog`,
  `app/content/projects`); schemas are in `app/content.config.ts`.

## Conventions

- **No-JS first, progressive enhancement.** Every page must render complete,
  useful HTML from the server. JS only enhances; it never gates content. This
  is the project's primary constraint — do not introduce client-only content.
- **Component/layout filenames are kebab-case** (a Biome rule): `base.astro`,
  `masthead.astro`. Import bindings stay PascalCase.
- **Tokens before values.** No colour, font size, or spacing literal in
  component CSS — use the CSS custom properties from `app/styles/tokens`. See
  [`docs/DESIGN_GUIDELINE.md`](docs/DESIGN_GUIDELINE.md).
- **Section markers** use `# MARK: Name` (also `// MARK:` / `<!-- MARK -->`),
  not `=== Name ===`.
- **The live site is the source of truth** for visual/behavioural parity.
- Our own data/cache types use **camelCase**. Only raw GitHub API responses
  are read in their native `snake_case` (e.g. `stargazers_count`) at the fetch
  boundary; everything we define or persist is camelCase.

## Quality Gates

- **Biome is the real lint/format gate.** It runs via the `biome-check`
  pre-commit hook (pinned in `prek.toml`, currently Biome 2.5) — that is
  authoritative. The config (`biome.jsonc`) is `linter.rules.preset: "all"`
  (every stable rule on) with a few global disables and scoped overrides,
  each justified inline. Resolve a new finding by climbing the ladder: fix it,
  else line-ignore, else file-ignore, else (last resort) a config override —
  always with a reason. Note `mise run lint` uses a different, locally-installed
  Biome that may reject the config; validate with `prek run biome-check`.
- `aubr check` runs `astro check` (type-checking). `scripts/` is **outside**
  the `tsconfig` include, so it is not type-checked there — but Biome still
  lints it.
- Pre-commit runs via **prek** (`prek.toml`): Biome, tombi, yamlfix, rumdl
  (Markdown), typos, shellcheck, and **committed** (commit-message lint —
  subjects ≤ 50 chars, Conventional Commits; `ci` is not an allowed type, use
  `chore`).
- CI: `.github/workflows/test.yml` (unit/check/e2e/prek) and `deploy.yml`.
  Playwright in CI runs Chromium projects only.

## Gotchas

- **Astro strips whitespace-only text nodes** between text and inline
  elements. Use `{" "}` to keep deliberate spaces (e.g. the hero headline).
- **`<script>` modules do not re-run after client-side navigation**
  (`<ClientRouter />`). Drive island setup from
  `document.addEventListener("astro:page-load", …)`, not top-level.
- **Scripts run via `tsx`** (`aubx tsx scripts/…`). Node's native TS runner
  chokes on the attribute-less JSON imports the lib modules use; `tsx` handles
  them.
- **The generated lockfiles are machine-owned artifacts.** `aube-lock.yaml` is
  excluded from both `typos` (it would "correct" package names/hashes) and
  `yamlfix`; commit it exactly as the package manager wrote it.
