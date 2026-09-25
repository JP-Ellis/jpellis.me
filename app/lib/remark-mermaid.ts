/**
 * Remark plugin that renders ```mermaid code blocks to inline SVG at build
 * time with beautiful-mermaid, so diagrams need no client-side JS.
 *
 * Colours are CSS custom properties from the design tokens, so a diagram
 * follows the page's colour scheme. The SVG's own stylesheet is scoped to
 * `.mermaid` and its element ids are made unique per diagram.
 */

import { renderMermaidSVG } from "beautiful-mermaid";
import type { Plugin, Transformer } from "unified";

/** Minimal mdast shapes; the plugin only reads `code` nodes and writes `html`. */
interface MdNode {
  type: string;
  lang?: string | null;
  value?: string;
  children?: MdNode[];
}

const HASH_SEED = 5381;
const HASH_SHIFT = 5;
const HASH_RADIX = 36;

/** Short djb2 hash of the diagram source, for per-diagram element ids. */
function hashSource(source: string): string {
  let hash = HASH_SEED;
  for (const char of source) {
    // biome-ignore lint/suspicious/noBitwiseOperators: djb2 hashing is bitwise by definition
    hash = ((hash << HASH_SHIFT) + hash + (char.codePointAt(0) ?? 0)) | 0;
  }
  // biome-ignore lint/suspicious/noBitwiseOperators: reinterpret as unsigned
  return (hash >>> 0).toString(HASH_RADIX);
}

/** Renders one diagram and adapts the SVG for inline use on the site. */
export function renderDiagram(source: string): string {
  const svg = renderMermaidSVG(source, {
    bg: "var(--color-surface)",
    fg: "var(--color-text)",
    accent: "var(--color-accent)",
    muted: "var(--color-muted)",
    line: "var(--color-muted)",
    border: "var(--color-rule)",
    surface: "var(--color-surface-raised)",
    font: "Newsreader",
    transparent: true,
  });

  const suffix = hashSource(source);
  const ids = [...svg.matchAll(/\sid="([^"]+)"/gu)].map((m) => m[1]);

  let out = svg
    // The site already loads Newsreader; skip the SVG's own font request.
    .replace(/^\s*@import url\([^)]*\);\n/mu, "")
    .replace(/^(\s*)text \{/mu, "$1.mermaid text {")
    .replace(/^(\s*)svg \{/mu, "$1.mermaid svg {");
  for (const id of ids) {
    out = out
      .replaceAll(`id="${id}"`, `id="${id}-${suffix}"`)
      .replaceAll(`url(#${id})`, `url(#${id}-${suffix})`);
  }

  if (out.includes("@import") || !out.includes(".mermaid svg {")) {
    throw new Error("remark-mermaid: unexpected beautiful-mermaid SVG layout");
  }
  return `<figure class="mermaid">${out}</figure>`;
}

function transform(node: MdNode): void {
  for (const child of node.children ?? []) {
    if (child.type === "code" && child.lang === "mermaid") {
      child.type = "html";
      child.value = renderDiagram(child.value ?? "");
      child.lang = undefined;
    } else {
      transform(child);
    }
  }
}

export const remarkMermaid: Plugin<[], MdNode, MdNode> =
  (): Transformer<MdNode, MdNode> =>
  (tree: MdNode): void => {
    transform(tree);
  };
