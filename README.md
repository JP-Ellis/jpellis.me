<!-- rumdl-disable-next-line MD063 -->
# jpellis.me

The source for [jpellis.me](https://jpellis.me) — a personal portfolio and
blog. Static-first, server-rendered where it needs to be, and built to work
without JavaScript.

|             |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **CI/CD**   | [![Tests](https://img.shields.io/github/actions/workflow/status/JP-Ellis/jpellis.me/test.yml?branch=main&label=tests)](https://github.com/JP-Ellis/jpellis.me/actions/workflows/test.yml) [![Deploy](https://img.shields.io/github/actions/workflow/status/JP-Ellis/jpellis.me/deploy.yml?branch=main&label=deploy)](https://github.com/JP-Ellis/jpellis.me/actions/workflows/deploy.yml)                                                                                                                                                                                                            |
| **Stack**   | [![Astro](https://img.shields.io/badge/Astro-BC52EE?logo=astro&logoColor=white)](https://astro.build) [![Svelte](https://img.shields.io/badge/Svelte-FF3E00?logo=svelte&logoColor=white)](https://svelte.dev) [![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org) [![Sass](https://img.shields.io/badge/Sass-CC6699?logo=sass&logoColor=white)](https://sass-lang.com) [![Cloudflare Workers](https://img.shields.io/badge/Cloudflare%20Workers-F38020?logo=cloudflare&logoColor=white)](https://workers.cloudflare.com) |
| **Tooling** | [![Biome](https://img.shields.io/badge/Biome-60A5FA?logo=biome&logoColor=white)](https://biomejs.dev) [![Vitest](https://img.shields.io/badge/Vitest-6E9F18?logo=vitest&logoColor=white)](https://vitest.dev) [![Playwright](https://img.shields.io/badge/Playwright-2EAD33?logo=playwright&logoColor=white)](https://playwright.dev) [![mise](https://img.shields.io/badge/mise-managed-EF4444)](https://mise.jdx.dev) [![prek](https://img.shields.io/badge/pre--commit-prek-FAB040?logo=precommit&logoColor=white)](https://github.com/j178/prek)                                                 |
| **Meta**    | [![Code license](https://img.shields.io/badge/code-MIT-lightgrey)](LICENSE) [![Content license](https://img.shields.io/badge/content-CC%20BY%204.0-lightgrey)](https://creativecommons.org/licenses/by/4.0/) [![Site](https://img.shields.io/badge/site-jpellis.me-1A1612)](https://jpellis.me)                                                                                                                                                                                                                                                                                                      |

## Overview

The site is an [Astro](https://astro.build) project deployed to
[Cloudflare Workers](https://workers.cloudflare.com). Most pages are
pre-rendered to static HTML; a few that surface live GitHub data are
server-rendered on demand. Every page is fully functional without
JavaScript — client-side scripts only enhance an already-working page.

## Architecture

- **Static-first, selective SSR.** `output: "static"` pre-renders the whole
  site at build time. The pages that show live GitHub stats opt out per-route
  with `export const prerender = false`, so only those run on the Worker.
- **No-JS first, progressive enhancement.** The server emits complete HTML.
  Islands (e.g. the theme toggle, the live clock) layer behaviour on top via
  Astro's `<ClientRouter />` and `astro:page-load`; nothing gates content
  behind JS.
- **Stale-while-revalidate caching.** SSR pages read GitHub stats from a
  Workers KV namespace and refresh them in the background (`waitUntil`), so a
  request never blocks on the GitHub API. When KV and the API are both
  unavailable, the page falls back to baked-in JSON snapshots.
- **Token-driven design system.** All colours, spacing, and typography come
  from CSS custom properties in a layered Sass system. See
  [`docs/DESIGN_GUIDELINE.md`](docs/DESIGN_GUIDELINE.md).

## Tech Stack

| Area        | Choice                                                        |
| ----------- | ------------------------------------------------------------- |
| Framework   | Astro 7 (`output: static` + selective SSR)                    |
| Islands     | Svelte 5                                                      |
| Hosting     | Cloudflare Workers (`@astrojs/cloudflare`), Workers KV        |
| Language    | TypeScript                                                    |
| Styling     | Sass with cascade layers and design tokens                    |
| Content     | Markdown via Astro content collections (blog, projects)       |
| Lint/format | Biome (JS/TS), tombi (TOML), yamlfix (YAML), rumdl (Markdown) |
| Unit tests  | Vitest                                                        |
| E2E tests   | Playwright                                                    |
| Toolchain   | mise, aube (package manager), prek (pre-commit)               |

## Getting Started

### Prerequisites

The toolchain is managed by [mise](https://mise.jdx.dev) — it pins Node,
Rust-based hook tools, `aube`, and `wrangler`. Install mise, then let it
provision everything:

```sh
mise install
```

`aube` is the project's package manager (an `npm`/`pnpm` equivalent). The
shortcuts `aubr` (`aube run`) and `aubx` (`aube exec`, like `npx`) appear
throughout the tasks. Dependencies install automatically on first run.

### Common Commands

Run these as mise tasks (`mise run <task>`) or `mise tasks` to list them all:

| Task                  | What it does                                          |
| --------------------- | ----------------------------------------------------- |
| `dev`                 | Astro dev server with hot reload                      |
| `build`               | Production build for Cloudflare Workers               |
| `preview`             | Build, then serve the Worker locally via wrangler dev |
| `test`                | Unit tests (Vitest) and E2E tests (Playwright)        |
| `check` / `check:fix` | Format + lint (read-only / autofix)                   |
| `deploy`              | Build and deploy to Cloudflare Workers                |
| `data:update`         | Refresh the GitHub stats/projects fallback JSON       |
| `oss-contribs:update` | Regenerate the OSS contributions list                 |

The underlying `package.json` scripts (`dev`, `build`, `preview`, `deploy`,
`test`, `check`) are also available via `aubr <script>`.

## Project Structure

```text
app/                  Astro source (srcDir)
  components/         UI components (.astro, .svelte islands)
  content/            Markdown content collections (blog, projects)
  data/               Static data + GitHub fallback snapshots
  layouts/            Page shells
  lib/                Server logic: GitHub fetchers, SWR cache, helpers
  pages/              Routes (incl. SSR stats pages and api/, rss.xml)
  styles/            Sass design system (tokens, base, components, utilities)
docs/                 Design guideline and design exploration
e2e/                  Playwright workspace (own package.json + lockfile)
scripts/              Data-refresh scripts (run via tsx)
tests/                Vitest unit tests
public/               Static assets served as-is
```

## Testing

- **Unit** — Vitest covers the pure logic in `app/lib` (parsers, the SWR
  cache, formatting). Run with `mise run test` or `aubr test`.
- **End-to-end** — Playwright drives the built Worker (`e2e/`). It has its own
  `package.json`/lockfile; `mise run e2e:test` installs deps + Chromium and
  runs the suite. CI runs the Chromium projects only; the full multi-browser
  matrix runs locally.

## Deployment

Deployment targets Cloudflare Workers via `@astrojs/cloudflare`. The build
emits `dist/server/wrangler.json`, which `wrangler` uses to deploy.

- Bindings (see `wrangler.toml`): static `ASSETS`, and the `GITHUB_STATS` /
  `PROJECTS_STATS` KV namespaces.
- Secret: set the GitHub token with `wrangler secret put GITHUB_TOKEN`.
- CI deploys from `main` (`.github/workflows/deploy.yml`) using the
  `CLOUDFLARE_ACCOUNT_ID` and `CLOUDFLARE_API_TOKEN` repository secrets.

Deploy locally with `mise run deploy` (requires `wrangler` auth).

## Refreshing GitHub Data

Two scripts regenerate the data baked into the repo from the live GitHub API.
Both read a token from `GITHUB_TOKEN`/`GH_TOKEN`, falling back to
`gh auth token`:

- `mise run data:update` — refreshes the stats and projects fallback JSON
  used when KV and the API are unavailable. It calls the same
  `fetchGithubStats` / `fetchProjectStats` the site uses, so each run also
  exercises them end-to-end.
- `mise run oss-contribs:update` — regenerates the OSS contributions list
  (`app/data/oss-contribs.ts`), preserving any custom display names.

## License

- **Code** — [MIT](LICENSE).
- **Content** — the prose, blog posts, and images are released under
  [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).
