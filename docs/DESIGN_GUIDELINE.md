# Implementation Guidelines

## 1. Core Principles

Three rules that everything else defers to:

1. **Progressive enhancement first** — every page must render fully useful HTML from the server. JS adds behaviour; it never gates content.
2. **Tokens before values** — no colour, font size, or spacing value appears in component CSS unless it is already defined as a CSS custom property in the token layer. Magic numbers are forbidden.
3. **Locality** — a component's visual rules live in its own `.module.scss` file. Global layers hold only what is genuinely shared: tokens, base resets, and reusable patterns.

## 2. File Layout

```text
style/
  _layer-order.scss         ← @layer declaration; must compile first
  main.scss                 ← imports everything else
  tokens/
    _colors.scss            ← source tokens, derived tokens, dark mode
    _typography.scss        ← font-family tokens, type-ramp tokens
    _spacing.scss           ← --space-1 through --space-8
    _layout.scss            ← --width-*, --gutter-*, --z-*
    _motion.scss            ← --ease-*, --duration-*
    _breakpoints.scss       ← SCSS variables only (media queries can't use var())
  base/
    _reset.scss             ← modern CSS reset + prefers-reduced-motion
    _root.scss              ← body wired to tokens; global :focus-visible
    _typography.scss        ← h1–h6, p, a, code defaults
  components/
    _container.scss         ← .container and .container--prose
    _eyebrow-grid.scss      ← .eyebrow-grid two-column layout
    _hairlines.scss         ← .rule-section, .rule-list
    _eyebrow.scss           ← .eyebrow, .eyebrow--muted
    _tags.scss              ← .tag with --pill, --hash, --accent variants
    _band.scss              ← .band full-bleed contrast section
    _masthead.scss          ← site header
    _footer.scss            ← site footer
  mixins/
    _focus.scss             ← @mixin focus-ring
  utilities/
    _sr-only.scss           ← .sr-only

src/components/<name>/
  mod.rs                    ← Leptos component
  style.module.scss         ← component-scoped styles (via stylance)
```

### Cascade Layers

Lowest → highest priority: `reset, tokens, base, components, utilities`

| Belongs in          | When                                                |
| ------------------- | --------------------------------------------------- |
| `tokens`            | Defining a CSS custom property; nothing else        |
| `base`              | Styling a bare HTML element (no class needed)       |
| `components`        | A reusable named pattern used across multiple pages |
| `utilities`         | A single-purpose helper class (`.sr-only`)          |
| `style.module.scss` | Visual rules specific to one Leptos component       |

## 3. Using Tokens in Components

### Colour

Three source tokens drive everything: `--color-paper` (background), `--color-ink` (foreground), `--color-accent` (brand). Use the derived tokens for softer treatments:

| Token                 | Use for                         |
| --------------------- | ------------------------------- |
| `--color-paper`       | page / card background          |
| `--color-paper-deep`  | slightly sunken surface         |
| `--color-panel`       | slightly raised surface         |
| `--color-ink`         | primary text, borders, icons    |
| `--color-muted`       | secondary text, placeholders    |
| `--color-faint`       | hover backgrounds, subtle fills |
| `--color-rule`        | hairline borders                |
| `--color-accent`      | interactive elements, emphasis  |
| `--color-accent-soft` | accent fills, tag borders       |

Inside a `.band` element, swap to the `--color-band-*` equivalents — they are automatically inverted relative to the page (dark band in light mode, light band in dark mode):

```scss
.band .my-label { color: var(--color-band-muted); }
.band .my-divider { border-top: 1px solid var(--color-band-faint); }
```

Never use a hardcoded colour. Never reference `--color-ink` / `--color-paper` directly inside a `.band` — the band tokens already handle the inversion.

### Spacing

All `padding`, `margin`, and `gap` values must use a `--space-*` token (4 px base scale, `--space-1` through `--space-8`). When a gap feels wrong for any of the eight steps, reconsider the layout rather than introducing a new value.

### Typography

The type ramp maps font roles to HTML elements. Components do not re-specify font families or sizes for standard elements — base handles those. Only set what deviates:

| Token group        | Maps to              | Font                |
| ------------------ | -------------------- | ------------------- |
| `--text-display-*` | h1                   | Fraunces 300        |
| `--text-section-*` | h2                   | Fraunces 400        |
| `--text-title-*`   | h3                   | Newsreader 500      |
| `--text-lead-*`    | intro prose          | Newsreader          |
| `--text-body-*`    | body                 | Newsreader          |
| `--text-meta-*`    | timestamps, captions | Fira Code           |
| `--text-eyebrow-*` | labels, tags         | Fira Code uppercase |

### Layout and Z-Index

Use `--width-prose` (880 px) and `--width-index` (1280 px) for `max-width`. Horizontal padding uses `--gutter-page-mobile` on mobile and `--gutter-page` at `$bp-lg`. The two-column editorial layout uses `--gutter-eyebrow` as the left column width.

Always use a `--z-*` token for `z-index` — never a raw integer.

## 4. Global Component Classes

These classes are available everywhere. Do not re-implement them in a module.

| Class               | Purpose                                                        |
| ------------------- | -------------------------------------------------------------- |
| `.container`        | Centred wrapper, responsive gutters, max-width `--width-index` |
| `.container--prose` | Same but max-width `--width-prose`                             |
| `.eyebrow-grid`     | Two-column label + content grid; stacks on mobile              |
| `.eyebrow`          | Fira Code uppercase label in `--color-accent`                  |
| `.eyebrow--muted`   | Same but `--color-muted` (dates, locations)                    |
| `.rule-section`     | 1 px `--color-ink` hairline between major sections             |
| `.rule-list`        | 1 px `--color-faint` hairline between list items               |
| `.tag`              | Base pill/chip; combine with `--pill`, `--hash`, `--accent`    |
| `.band`             | Full-bleed contrast section; use `--color-band-*` inside       |
| `.sr-only`          | Visually hidden, accessible to screen readers                  |

## 5. Writing a Component Module

### Stylance Wiring

```rust
use stylance::import_style;
import_style!(style, "style.module.scss");

#[component]
pub fn Card() -> impl IntoView {
    view! { <article class=style::card> … </article> }
}
```

### What the Module Should Contain

Only rules that deviate from base. If `h1` already has the right font and size from `base/_typography.scss`, the module does not touch it.

```scss
// ✓ fine — deviates from base by adding layout
.card {
  display: grid;
  gap: var(--space-5);
  padding: var(--space-6);
  background: var(--color-panel);
  border: 1px solid var(--color-rule);
  border-radius: 4px;
}

// ✗ wrong — duplicates what base already sets
.card h2 {
  font-family: var(--font-display);
  font-size: var(--text-section-size);
}
```

Stylance scopes class names automatically, so BEM is unnecessary inside a module:

```scss
// ✓ fine inside a module
.card { … }
.card-title { … }
.card-meta { … }
```

Global component files (`style/components/`) use BEM-lite (block + modifier, no deep element nesting).

### What a Module Must Not Do

- Use a hardcoded colour, size, or spacing value
- Set `z-index` to a raw number — use `--z-*`
- Override bare HTML element styles — that belongs in `base`
- Import from another component's module — extract to `style/components/`
- Repeat typography rules already set by `base/_typography.scss`

### Promoting to a Global Component

Start in a module. If a second component needs the same pattern, extract it to `style/components/` and wrap it in `@layer components { … }`.

## 6. Focus Styles

All interactive elements must use the shared focus mixin:

```scss
@use '../mixins/focus' as *;

.my-button:focus-visible { @include focus-ring; }
.my-link:focus-visible   { @include focus-ring(2px); }  // tighter offset
```

`focus-ring` produces a 2 px `--color-accent` outline that only appears on keyboard navigation (`:focus-visible`). Never write a custom outline or replicate the values inline.

## 7. Responsive Strategy

Breakpoints are SCSS variables (`$bp-sm` 480 px · `$bp-md` 768 px · `$bp-lg` 1024 px · `$bp-xl` 1280 px). Write mobile-first; add complexity with `min-width`:

```scss
@use '../../tokens/breakpoints' as *;

.my-grid {
  display: flex;
  flex-direction: column;
  gap: var(--space-5);

  @media (min-width: $bp-md) {
    flex-direction: row;
  }
}
```

`max-width` queries are permitted only for complex desktop grids that collapse to a single column. They must be commented explaining why.

Touch targets on mobile must be at least 44 × 44 px, achieved with padding rather than fixed dimensions.

## 8. Progressive Enhancement

Every page must be fully usable with JavaScript disabled. Leptos SSR guarantees complete server-rendered HTML; these rules protect that during development.

**Test:** disable JS. Every page must show all content, have working `<a>`-based navigation, and render correctly styled.

| Behaviour            | Without JS                     | With JS                                |
| -------------------- | ------------------------------ | -------------------------------------- |
| Dark mode            | `prefers-color-scheme` applies | Manual toggle sets `data-theme` cookie |
| Tag filtering        | All posts shown                | Client-side reactive filtering         |
| Page transitions     | Normal browser navigation      | View Transitions API crossfade         |
| Masthead italic word | Server picks a random word     | — (server-side only)                   |

### Leptos Rules

- Never use an empty `Suspense` fallback — SSR output must include a skeleton that matches the loaded state in dimensions.
- Hydration must not shift layout. A shift on hydration is a bug.
- The dark mode toggle must not be visible without JS (it would be non-functional):

```html
<noscript><style>.theme-toggle { display: none; }</style></noscript>
```

### CSS-Only Features

Dark mode, focus styles, transitions, and reduced-motion are all CSS-only. Never condition them on a JS class:

```scss
// ✓ always applies
@media (prefers-color-scheme: dark) { … }

// ✗ broken without JS
.js-loaded .band { background: var(--color-band-bg); }
```

## 9. Dark Mode

Dark mode reassigns only the three source tokens (`--color-paper`, `--color-ink`, `--color-accent`). All derived tokens update automatically — no extra work needed in components.

Two mechanisms operate independently:

1. **CSS baseline:** `@media (prefers-color-scheme: dark)` — no JS, works on first render.
2. **JS enhancement:** `[data-theme]` on `<html>` — overrides OS preference. The server reads a cookie and sets the attribute before hydration to prevent flash.

Components always use semantic tokens, never raw values:

```scss
// ✓ works in both modes
.card { background: var(--color-paper); border: 1px solid var(--color-rule); }

// ✗ hardcoded — breaks dark mode
.card { background: #f5f1ea; border: 1px solid rgba(26,22,18,0.18); }
```

View Transitions CSS is unconditional (not JS-gated):

```scss
@layer base {
  ::view-transition-old(root),
  ::view-transition-new(root) {
    animation-duration: var(--duration-fast);
  }
}
```

## 10. Accessibility Baseline

These apply to all components:

- Every interactive element is reachable by keyboard (`Tab`, `Enter`, `Space`)
- Focus styles use `@mixin focus-ring` — never a custom outline
- Meaningful images have descriptive `alt`; decorative images use `alt=""`
- `<nav>` landmarks have `aria-label` when more than one appears on a page
- Color is never the sole means of conveying information — use both color and a symbol (`●`, `○`, `◐`)
