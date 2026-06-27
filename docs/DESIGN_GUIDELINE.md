# Implementation Guidelines

## 1. Core Principles

Three rules that everything else defers to:

1. **Progressive enhancement first** — every page must render fully useful HTML from the server. JS adds behaviour; it never gates content.
2. **Tokens before values** — no colour, font size, or spacing value appears in component CSS unless it is already defined as a CSS custom property in the token layer. Magic numbers are forbidden.
3. **Locality** — a component's visual rules live in a scoped `<style lang="scss">` block in its own `.astro` file. Global layers hold only what is genuinely shared: tokens, base resets, and reusable patterns.

## 2. File Layout

```text
app/styles/
  _layer-order.scss         ← @layer declaration; must compile first
  main.scss                 ← imports everything else
  tokens/
    _colors.scss            ← source tokens, derived tokens, dark mode
    _typography.scss        ← font-family tokens, type-ramp tokens
    _spacing.scss           ← --space-1 through --space-8
    _layout.scss            ← --width-*, --gutter-*, --z-*
    _motion.scss            ← --ease-*, --duration-*
    _borders.scss           ← --radius-*, --border-width-*
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
    _buttons.scss           ← .btn
    _prose.scss             ← long-form article styling
    _prism.scss             ← code-block syntax highlighting
  mixins/
    _focus.scss             ← @mixin focus-ring
  utilities/
    _sr-only.scss           ← .sr-only

app/components/<name>.astro
  <style lang="scss">        ← component-scoped styles (Astro scopes them)
```

UI lives in `.astro` components (with Svelte islands for interactivity). The
full-bleed `.band`, the masthead, and the footer are `.astro` components with
their own scoped styles, not global SCSS partials.

### Cascade Layers

Lowest → highest priority: `reset, tokens, base, components, utilities`

| Belongs in | When |
| ------------------- | --------------------------------------------------- |
| `tokens` | Defining a CSS custom property; nothing else |
| `base` | Styling a bare HTML element (no class needed) |
| `components` | A reusable named pattern used across multiple pages |
| `utilities` | A single-purpose helper class (`.sr-only`) |
| scoped `<style>` | Visual rules specific to one `.astro` component |

## 3. Using Tokens in Components

### Colour

Three source tokens drive everything: `--color-paper` (background), `--color-ink` (foreground), `--color-accent` (brand). Use the derived tokens for softer treatments:

| Token | Use for |
| --------------------- | ------------------------------- |
| `--color-paper` | page / card background |
| `--color-paper-deep` | slightly sunken surface |
| `--color-panel` | slightly raised surface |
| `--color-ink` | primary text, borders, icons |
| `--color-muted` | secondary text, placeholders |
| `--color-faint` | hover backgrounds, subtle fills |
| `--color-rule` | hairline borders |
| `--color-accent` | interactive elements, emphasis |
| `--color-accent-soft` | accent fills, tag borders |

Inside a `.band`, the three source tokens (`--color-paper`, `--color-ink`, `--color-accent`) are locally reassigned to invert the contrast. All derived tokens update automatically. **Components rendered inside a band require no changes** — they continue to use `--color-ink`, `--color-muted`, `--color-faint`, `--color-rule`, etc. and the correct inverted values cascade in.

```scss
// ✓ correct — works everywhere, including inside .band
.my-label   { color: var(--color-muted); }
.my-divider { border-top: 1px solid var(--color-faint); }

// ✗ wrong — use semantic tokens, not hard-coded values
.my-label { color: oklch(59.686% 0.156 29.234); }
```

Never use a hardcoded colour.

### Spacing

All `padding`, `margin`, and `gap` values must use a `--space-*` token (4 px base scale, `--space-1` through `--space-8`). When a gap feels wrong for any of the eight steps, reconsider the layout rather than introducing a new value.

### Typography

The type ramp maps font roles to HTML elements. Components do not re-specify font families or sizes for standard elements — base handles those. Only set what deviates:

| Token group | Maps to | Font |
| ------------------ | -------------------- | ------------------- |
| `--text-display-*` | h1 | Fraunces 300 |
| `--text-section-*` | h2 | Fraunces 400 |
| `--text-title-*` | h3 | Newsreader 500 |
| `--text-lead-*` | intro prose | Newsreader |
| `--text-body-*` | body | Newsreader |
| `--text-meta-*` | timestamps, captions | Fira Code |
| `--text-eyebrow-*` | labels, tags | Fira Code uppercase |

### Layout and Z-Index

Use `--width-prose` (880 px) and `--width-index` (1280 px) for `max-width`. Horizontal padding uses `--gutter-page-mobile` on mobile and `--gutter-page` at `$bp-lg`. The two-column editorial layout uses `--gutter-eyebrow` as the left column width.

Always use a `--z-*` token for `z-index` — never a raw integer.

## 4. Global Component Classes

These classes are available everywhere. Do not re-implement them in a module.

| Class | Purpose |
| ------------------- | ---------------------------------------------------------------------------------------------------- |
| `.container` | Centred wrapper, responsive gutters, max-width `--width-index` |
| `.container--prose` | Same but max-width `--width-prose` |
| `.eyebrow-grid` | Two-column label + content grid; stacks on mobile |
| `.eyebrow` | Fira Code uppercase label in `--color-accent` |
| `.eyebrow--muted` | Same but `--color-muted` (dates, locations) |
| `.rule-section` | 1 px `--color-ink` hairline between major sections |
| `.rule-list` | 1 px `--color-faint` hairline between list items |
| `.tag` | Base pill/chip; combine with `--pill`, `--hash`, `--accent` |
| `.band` | Full-bleed contrast section; locally inverts colour tokens so children need no band-specific changes |
| `.btn` | Monospace bordered button or link; inherits inverted tokens automatically inside `.band` |
| `.sr-only` | Visually hidden, accessible to screen readers |

## 5. Writing a Component's Scoped Styles

### Scoped Style Block

Each `.astro` component carries its own `<style lang="scss">` block, which
Astro scopes automatically. Pull in tokens, mixins, or breakpoint variables
with `@use` (paths are relative to the component file):

```astro
<article class="card"><slot /></article>

<style lang="scss">
  @use "../styles/tokens/breakpoints" as *;

  .card {
    display: grid;
    gap: var(--space-5);
    padding: var(--space-6);
    background: var(--color-panel);
    border: 1px solid var(--color-rule);
    border-radius: var(--radius-sm);
  }
</style>
```

### What the Block Should Contain

Only rules that deviate from base. If `h1` already has the right font and size from `base/_typography.scss`, the block does not touch it.

```scss
// ✓ fine — deviates from base by adding layout
.card {
  display: grid;
  gap: var(--space-5);
  padding: var(--space-6);
}

// ✗ wrong — duplicates what base already sets
.card h2 {
  font-family: var(--font-display);
  font-size: var(--text-section-size);
}
```

Astro scopes class names automatically, so BEM is unnecessary inside a component:

```scss
// ✓ fine inside a component
.card { … }
.card-title { … }
.card-meta { … }
```

Global component files (`app/styles/components/`) use BEM-lite (block + modifier, no deep element nesting).

### What a Scoped Block Must Not Do

- Use a hardcoded colour, size, or spacing value
- Set `z-index` to a raw number — use `--z-*`
- Override bare HTML element styles — that belongs in `base`
- Reach into another component's markup — extract shared rules to `app/styles/components/`
- Repeat typography rules already set by `base/_typography.scss`

### Red Flags in a Scoped Block

Stop and check if a global component already exists, or if you should create one:

- Writing `cursor: pointer` + `border` + `font-family: var(--font-mono)` — use `.btn`
- Use the standard token (`--color-ink`, `--color-muted`, `--color-faint`, `--color-rule`) and let `.band`'s cascade handle the inversion
- Defining the same single-property pattern in more than one component — promote to `app/styles/components/`
- Any button, badge, or card pattern is almost certainly reusable — start it in the global layer

### Promoting to a Global Component

Start scoped. If a second component needs the same pattern, extract it to `app/styles/components/` and wrap it in `@layer components { … }`.

Patterns that are almost certainly reusable and should start global rather than being scoped first: buttons, badges/chips, cards with bordered containers, and any pattern already listed in the global components table in section 4.

## 6. Focus Styles

All interactive elements must use the shared focus mixin:

```scss
@use '../styles/mixins/focus' as *;

.my-button:focus-visible { @include focus-ring; }
.my-link:focus-visible   { @include focus-ring(2px); }  // tighter offset
```

`focus-ring` produces a 2 px `--color-accent` outline that only appears on keyboard navigation (`:focus-visible`). Never write a custom outline or replicate the values inline.

## 7. Responsive Strategy

Breakpoints are SCSS variables (`$bp-sm` 480 px · `$bp-md` 768 px · `$bp-lg` 1024 px · `$bp-xl` 1280 px). Write mobile-first; add complexity with `min-width`:

```scss
@use '../styles/tokens/breakpoints' as *;

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

Every page must be fully usable with JavaScript disabled. Astro renders
complete HTML at build time (or on the Worker for SSR routes); these rules
protect that during development.

**Test:** disable JS. Every page must show all content, have working `<a>`-based navigation, and render correctly styled.

| Behaviour        | Without JS                     | With JS                                  |
| ---------------- | ------------------------------ | ---------------------------------------- |
| Dark mode        | `prefers-color-scheme` applies | `[data-theme]` override                  |
| Page transitions | Normal browser navigation      | View Transitions via `<ClientRouter />`  |
| Live data        | Server-rendered snapshot       | Island enhancement (e.g. the live clock) |

### Astro Rules

- Server-rendered HTML must be complete on its own; islands only enhance it.
  An island that supplies content that isn't already in the SSR output is a
  progressive-enhancement bug.
- Hydration must not shift layout. A shift on hydration is a bug.
- Drive DOM setup that must survive client-side navigation from
  `document.addEventListener("astro:page-load", …)` — `<script>` modules do
  not re-run after a `<ClientRouter />` navigation.
- Any control that only works with JS must be hidden without it (it would be
  non-functional):

```html
<noscript><style>.js-only-control { display: none; }</style></noscript>
```

### CSS-Only Features

Dark mode, focus styles, transitions, and reduced-motion are all CSS-only. Never condition them on a JS class:

```scss
// ✓ always applies
@media (prefers-color-scheme: dark) { … }

// ✗ broken without JS
.js-loaded .band { background: var(--color-paper); }
```

## 9. Dark Mode

Dark mode is driven by two Sass mixins in `tokens/_colors.scss`, `theme-light`
and `theme-dark`, which assign the three source tokens (`--color-paper`,
`--color-ink`, `--color-accent`) directly. The derived tokens
(`--color-muted`, `--color-faint`, `--color-rule`, …) are defined `from` those
sources, so they all update automatically — no extra work needed in
components. The mixins are the single source of truth (there are no
`--_color-*` private tokens).

Two mechanisms operate independently:

1. **CSS baseline:** `@media (prefers-color-scheme: dark)` applies
   `theme-dark` at `:root` — no JS, works on first render.
2. **`[data-theme]` override:** `[data-theme="light"]` / `[data-theme="dark"]`
   on `<html>` overrides the OS preference for a persisted user choice.

The `.band` component reuses the same mixins to invert locally: it applies
`theme-dark` while the page is light and `theme-light` while the page is dark,
so children inside a band need no band-specific changes.

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
