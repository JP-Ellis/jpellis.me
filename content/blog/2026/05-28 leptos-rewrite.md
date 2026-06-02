+++
date = 2026-05-28
description = "Notes on rewriting a personal portfolio from Svelte to Leptos."
tags = ["rust", "leptos", "web", "cloudflare"]
title = "Rewriting jpellis.me in Leptos"
+++

My personal site ran on SvelteKit for a few years and worked perfectly well, but it was starting to be dated, featuring that classic 'mid 2010s web' look. Anyone who knows me will be aware of my fondness for Rust, and the itch to rewrite my website in Rust was strong.

I opted to rewrite the site with [Leptos](https://leptos.dev), a fullstack Rust framework that supports reactive signals and server-side rendering with client-side hydration. The deployment target was Cloudflare Workers, which supports running `wasm32` binaries directly at the edge.

The post is in two parts. Part 1 is a narrative walkthrough of how the rewrite unfolded, phase by phase. Part 2 goes deep on the two things I found most interesting: running Leptos SSR inside a wasm32 Cloudflare Worker, and building a CSS design system that works well alongside a Rust project.

<!-- more -->

---

## Part 1: the Rewrite

## Why Rewrite a Working Site?

The [Svelte + SvelteKit](https://svelte.dev) site worked, but it didn't excite me anymore. It looked a bit dated, and dependency updates were starting to cause build breakages (like the Svelte [3 to 4 migration](https://svelte.dev/docs/svelte/v4-migration-guide) and then the [4 to 5 upgrade](https://svelte.dev/docs/svelte/v5-migration-guide)). While I would still highly recommend Svelte, I wanted to experiment with something new in a low-stakes way.

I am very fond of Rust, and have been following the rise of tools like [Leptos](https://leptos.dev), [Sycamore](https://sycamore.dev/), [Dioxus](https://dioxuslabs.com), and [Yew](https://yew.rs) for a while. Dioxus and Yew follow in the footsteps of React with a virtual DOM, while Leptos and Sycamore take a more Svelte-like approach with reactive signals and fine-grained reactivity.

I opted for Leptos, as I valued the server-side rendering (SSR) with hydration strategy it offers. As you have seen, my site is mostly static content with a few dynamic components. Leptos's first-class support for SSR means that I can write a component once, render it to HTML on the server, and then hydrate it in the browser without a full re-render.

I was also hoping that the SSR and minimal interactivity would result in a fast, performant site with good SEO. As of the time of writing, this is _mostly_ achieved, though I would like to see the `wasm` payload size decrease from the current 544 kB. I can only imagine that the Rust web ecosystem will continue to improve in this regard, and there may be some low-hanging optimizations I could implement.

The deployment target was Cloudflare Workers from the start. CloudFlare announced the ability to run WebAssembly directly [back in 2018](https://blog.cloudflare.com/webassembly-on-cloudflare-workers/), and with Rust compiling easily to `wasm32-wasi`, it seemed like a natural fit.

### Design System Before Components

The first code written wasn't a Leptos component, it was actually SCSS tokens. That's unusual enough to be worth explaining.

I used Anthropic's [Claude Design](https://www.anthropic.com/news/claude-design-anthropic-labs) tool to ideate through a few different design directions. Testing out a modern terminal-inspired appearance (not just green-on-black), or a sleek minimalist look, or even a more traditional paper-and-ink vibe. The terminal style was a bit too... on the nose, and as you can see, I ended up going with a blend of minimal and paper-inspired aesthetics.

With a design direction in place, it was then a matter of setting up the CSS foundations. No TailWind, no component library, just good old-fashioned CSS with all of the modern features it provides.

For example, this website's colour system is built on OkLCh. For those unfamiliar, OkLCh is a colour space that models human colour perception more closely than RGB or HSL. It consists of three numbers: a lightness percentage `L`, a chroma value `C` that represents colour intensity/saturation, and a hue angle `h`. Most critical though is that it is perceptually uniform, so all colours with the same `L` will appear equally light to the human eye, or all colours with the same `C` will appear equally saturated. Furthermore, since the colour space aligns with human perception, mixing colours in OkLCh produces more intuitive result than mixing the corresponding RGB or HSL values, as illustrated in the four colour gradients below:

<div class="gradient-comparison">
  <div class="gradient-row">
    <div class="gradient-swatch" style="background: linear-gradient(in srgb to right, #fffe00, #0000ff)"></div>
    <code>sRGB</code>
  </div>
  <div class="gradient-row">
    <div class="gradient-swatch" style="background: linear-gradient(in hsl to right, #fffe00, #0000ff)"></div>
    <code>HSL</code>
  </div>
  <div class="gradient-row">
    <div class="gradient-swatch" style="background: linear-gradient(in oklch to right, #fffe00, #0000ff)"></div>
    <code>OkLCh</code>
  </div>
  <div class="gradient-row">
    <div class="gradient-swatch" style="background: linear-gradient(in oklab to right, #fffe00, #0000ff)"></div>
    <code>OkLab</code>
  </div>
</div>
<style>
  .gradient-comparison {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin: 1.5rem 0 2rem;
  }
  .gradient-comparison .gradient-row {
    display: grid;
    grid-template-columns: 0.9fr 5rem;
    align-items: center;
    gap: 1rem;
  }
  .gradient-comparison .gradient-swatch {
    height: 2.5rem;
    border-radius: 0.25rem;
  }
  .gradient-comparison code {
    font-size: 0.875em;
    text-align: right;
  }
</style>

<details>
<summary>Notes</summary>

Both RGB and HSL gradients produce somewhat unexpected colour shifts. The RGB gradient goes through a grey-ish (and somewhat dark) colour in the middle, while the HSL gradient goes through a bright red and pink before reaching blue.

OkLCh and OkLab produce more smooth gradients. Whether the green is expected (as a mix between yellow and blue), or the more neutral colours in the middle of the OkLab gradient are expected, is a matter of personal intuition.
</details>

These gradients only show one dimension — the straight line between two colours. To explore the full three-dimensional gamut geometry of each colour space, try the [interactive colour space visualiser](/tools/colorspace-visualizer).

Derived tokens, like tinted variants, opacity values, are computed in CSS using
relative colour syntax:

```css
--color-paper-deep: oklch(from var(--color-paper) calc(l - 0.04) c h);
```

This means changing `--color-paper` automatically propagates to all its derivatives at paint time, with no SCSS colour functions involved, and the use of OkLCh ensures that these derived tokens maintain a consistent perceptual relationship to the base colour.

Cascade layers (`@layer reset, tokens, base, components, utilities`) replace specificity wrestling. The declaration order defines which layer wins conflicts; no `!important` appears anywhere in the codebase.

[Stylance](https://github.com/basro/stylance-rs) handles scoped CSS modules: each component has a `.module.scss` file, and Stylance hashes class names at build time so styles can't leak between components. Dark mode is progressive enhancement: `prefers-color-scheme` works without any JavaScript, and a `data-theme` attribute on `<html>` can override it for a manual toggle added later.

### Routing, Pages, and Components

Leptos's router uses declarative `<Route>` components, which will feel familiar if you've used React Router or SvelteKit's file-based routing:

```rust
<Routes fallback=|| view! { <pages::NotFoundPage /> }>
    <Route path=path!("") view=pages::HomePage />
    <Route path=path!("projects") view=pages::ProjectsPage />
    <Route path=path!("projects/:slug") view=pages::ProjectDetailPage />
    <Route path=path!("blog/:slug") view=pages::BlogPostPage />
    // ...
</Routes>
```

Each component lives in its own `mod.rs`[^mod-rs] alongside a `*.module.scss` file. Styling is co-located with the component it belongs to, not in a separate `styles/` tree. HTML is semantic throughout: `<header>`, `<nav>`, `<aside>`, `<footer>`, `<section>`, and avoiding the `<div>` soup, design tokens everywhere, no magic numbers.

[^mod-rs]: The `mod.rs` pattern was the original way to define modules in Rust, but

One pattern worth naming: in debug builds, a set of `__test/` routes provides isolated visual test pages for individual components. Just a simple page that renders the component in isolation so you can eyeball it. Simple, and surprisingly useful for quick visual checks, and useful too for targeted Playwright tests that don't have to navigate through the full app to get to the component under test.

### Server Functions and the GitHub Integration

The home page shows GitHub contribution stats fetched live from the GitHub API. The stats are fetched on the server with caching backed by CloudFlare's KV storage. A traditional stale-while-revalidate strategy is used: the server returns cached data immediately, then triggers a background refresh so the next request gets fresh data.

The GitHub fetch is implemented as a Leptos server function:

```rust
#[server]
pub async fn get_github_stats() -> Result<GitHubStats, ServerFnError> {
    let provider = use_context::<StatsProvider>()
        .ok_or_else(|| ServerFnError::new("StatsProvider not in Leptos context"))?;
    // ...
    Ok(stats)
}
```

The `#[server]` macro generates an RPC stub for the client automatically. The client calls `get_github_stats()` as if it were a regular async function; the macro arranges for that call to hit the server endpoint. The implementation body is never included in the browser bundle.

There's a subtlety here related to the deployment target — the full story is in [Deep-dive 1](#deep-dive-1-running-leptos-ssr-inside-cloudflare-workers), but the short version is: the "server" in this case is a `wasm32` binary running inside a Cloudflare Worker, and that has some interesting consequences.

### Blog Engine (Compile-Time)

Blog posts are Markdown files in `content/blog/` with TOML frontmatter:

```toml
+++
date = 2026-03-01
title = "Rewriting jpellis.me in Leptos"
tags = ["rust", "leptos", "web"]
+++
```

`build.rs` renders every post at compile time: it parses the frontmatter, renders the Markdown to HTML with `pulldown-cmark`, and writes a Rust source file to `OUT_DIR`. The crate then pulls it in with a single line:

```rust
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));
```

The generated file contains a `static POSTS: &[BlogPost]` array where every field is a `&'static str`. Zero runtime filesystem access; the content is baked into the binary.

The `POSTS` array has unit tests baked in: sort order, unique slugs, ISO date format. If a post breaks any of these, `cargo build` fails before the code even links. The blog engine can't ship broken invariants.

### Deployment and Post-Launch

The site runs as a single Cloudflare Worker. The Worker handles SSR for all `text/html` requests, and everything else (CSS, JS bundle, images) are served from a separate asset bucket via the `[assets]` binding in `wrangler.toml`.

Favicons are generated by a [`cargo` script](https://rust-lang.github.io/rfcs/3502-cargo-script.html) that produces the modern set from a single source, and it's wired into both the `mise` task graph and the `wrangler.toml` build command so it runs automatically before every deploy.

---

## Part 2: Deep-Dives

### Deep-Dive 1: Running Leptos SSR Inside Cloudflare Workers

Most Leptos tutorials and examples target a native server. That is, you compile to `x86` or `arm`, which you can run locally or deploy to a VPS. This project targets something different: a `wasm32` Cloudflare Worker. This does complicate things a bit:

| Target | Feature flag | Where it runs |
| ------------ | ------------ | --------------------------------- |
| native (x86) | `ssr` | Dev server (`cargo leptos watch`) |
| `wasm32` | `hydrate` | Browser bundle |
| `wasm32` | `ssr` | Cloudflare Worker (production) |

The Leptos SSR and hydration story is the same across all three: components render to HTML on the server, the browser receives that HTML, then the WASM bundle hydrates it in-place without a full re-render. What changes is the runtime underneath.

#### The `#[server]` Macro

Server functions are the bridge between SSR and client. You write one async function, annotate it with `#[server]`, and Leptos generates two things: a server-side implementation and a client-side RPC stub. The browser calls the function as if it's local; the macro routes that call to an HTTP endpoint.

```rust
#[server]
pub async fn get_github_stats() -> Result<GitHubStats, ServerFnError> {
    let provider = use_context::<StatsProvider>()
        .ok_or_else(|| ServerFnError::new("StatsProvider not in Leptos context"))?;
    let stats = provider.get().await;
    Ok(stats)
}
```

The implementation body is excluded from the browser bundle entirely, and is only compiled when the `ssr` feature is active.

#### The `SendWrapper` Problem

Here's where the wasm32 SSR target gets interesting. Cloudflare Workers expose
KV storage through a JavaScript API. When you call into that API from Rust,
you're wrapping a `JsFuture` — and `JsFuture` is `!Send`.

That's a problem because Leptos server functions are expected to return `Send`
futures (for compatibility with multi-threaded native runtimes). The compiler
will reject code that returns a `!Send` future from a `#[server]` function.

The fix is `SendWrapper`:

```rust
#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
let stats = send_wrapper::SendWrapper::new(provider.get()).await;

#[cfg(not(all(target_arch = "wasm32", feature = "ssr")))]
let stats = provider.get().await;
```

`SendWrapper` lies to the type system — it implements `Send` unconditionally.
This is safe here because the Workers runtime is single-threaded; the future
will never actually be sent across threads. But the compiler can't verify that,
so you have to assert it manually with the wrapper.

This is the kind of thing documentation doesn't surface. You only find it when
the compiler rejects your code with a confusing `!Send` error, and then you have
to reason through why `JsFuture` isn't `Send` and what the runtime guarantee
actually is.

#### The Provider Pattern

The `StatsProvider` in the example above is injected into Leptos context by the
application entry point — `main.rs` on native, `workers.rs` on CF Workers. The
server function itself has no platform-specific logic; it just pulls the
provider from context.

This separation matters: server functions can be tested against a mock provider
on native without spinning up a Worker, and the Worker-specific code is
contained to a single entry point file.

#### `cfg` Gates

The idiom for "this code runs on CF Workers specifically — not the browser, not
native" is:

```rust
#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
```

`target_arch = "wasm32"` alone isn't enough — it matches both the browser bundle
and the Worker. The `ssr` feature flag distinguishes them.

### Deep-Dive 2: a CSS Design System Inside a Rust Project

The Leptos ecosystem doesn't have a `styled-components` equivalent. When you
search for CSS-in-Rust solutions, you find either inline styles (loses all CSS
features) or crates that are experimental or abandoned. The right answer,
I think, is to not look for a Rust solution at all — CSS is already good at
this problem. Let CSS do CSS.

#### OKLCH and Relative Colour Syntax

The colour system is built on OKLCH. The practical reason: OKLCH is perceptually
uniform, which means adjusting the lightness channel doesn't shift perceived
saturation the way HSL does. You can change how light or dark a colour is
without accidentally making it look more or less vivid.

The design uses a pattern where hue and chroma are shared between light and
dark themes as SCSS variables, and lightness is tuned per theme:

```scss
// Shared across both themes — only lightness varies
$accent-ch: 0.1497 34.55;

@mixin theme-light {
    --color-accent: oklch(0.5497 #{$accent-ch}); // #b8492f
    @include derived-tokens;
}

@mixin theme-dark {
    --color-accent: oklch(0.6205 #{$accent-ch}); // brighter on dark bg
    @include derived-tokens;
}
```

Derived tokens — tinted variants, opacity-reduced versions — are computed using
CSS relative colour syntax:

```scss
@mixin derived-tokens {
    --color-paper-deep: oklch(from var(--color-paper) calc(l - 0.04) c h);
    --color-panel:      oklch(from var(--color-paper) calc(l + 0.025) c h);
    --color-muted:      oklch(from var(--color-ink) l c h / 55%);
}
```

`oklch(from var(--color-paper) calc(l - 0.04) c h)` reads as: take the value
of `--color-paper`, keep its chroma and hue, subtract 0.04 from its lightness.
This computation happens in CSS at paint time. Change `--color-paper` and
`--color-paper-deep` updates automatically — no SCSS colour functions, no
hardcoded hex for each shade, no pre-computation required.

#### Theme Mixin Pattern

Both `@mixin theme-light` and `@mixin theme-dark` set the same primary
tokens, then call `@mixin derived-tokens`. The derived-tokens mixin is the
single source of truth for everything that flows from the primary colours.
Adding a new derived token means adding it once; it appears in both themes
automatically.

#### Cascade Layers

```scss
@layer reset, tokens, base, components, utilities;
```

Layer ordering defines specificity. A utility class in the `utilities` layer
always beats a component style in the `components` layer, regardless of selector
weight. No `!important` anywhere in the codebase — the layers handle it.

#### Stylance and the `:global()` Quirk

[Stylance](https://github.com/basro/stylance-rs) scopes CSS modules by hashing
class names at build time: `.button` in `component.module.scss` becomes
`.button-a3f9c2` in the output. Class names passed as component props are
unscoped strings, so the CSS selector must also be unscoped.

The quirk: Stylance only recognises `:global()` when it wraps the _entire_
selector, not when it wraps just the class:

```scss
// Correct — entire selector is global
:global(nav.vertical-layout) {
    flex-direction: column;
}

// Wrong — Stylance doesn't recognise this form
nav:global(.vertical-layout) {
    flex-direction: column;
}
```

The wrong form silently does nothing: Stylance doesn't process it, the class
never gets scoped, but it also doesn't match the unscoped string passed as a
prop. It burns you once and you remember it forever.

#### Progressive Dark Mode

Dark mode works without JavaScript. The default is `prefers-color-scheme`:

```scss
:root { @include tokens.theme-light; }

@media (prefers-color-scheme: dark) {
    :root { @include tokens.theme-dark; }
}
```

A `data-theme` attribute on `<html>` can override it when present:

```scss
[data-theme="dark"]  { @include tokens.theme-dark; }
[data-theme="light"] { @include tokens.theme-light; }
```

The JavaScript for a manual toggle can be added later as enhancement. The
baseline experience never depends on it.

#### Why CSS-First in a Rust Project

The temptation in a Rust-heavy project is to solve everything in Rust. Resist
it for styling. Leptos is good at reactive logic. CSS is good at styling.
They're already designed to work together — the platform solved this problem
before Rust web frameworks existed. The right boundary is a clean one: Rust
generates semantic HTML and class names; CSS handles everything visual.

---

## Would I Do It Again?

Yes — for the right project. Leptos is genuinely production-ready for a site
at this scale, the SSR + hydration story works, and the Cloudflare Workers
deployment is solid. The rough edges are real: the `SendWrapper` dance for
wasm32 SSR is unpleasant, tooling is slower than the JS ecosystem, and the
community is smaller so you'll hit underdocumented corners. But the type safety,
the performance, and the satisfaction of shipping Rust to production make those
tradeoffs worthwhile — at least for a project where you own all the code.
