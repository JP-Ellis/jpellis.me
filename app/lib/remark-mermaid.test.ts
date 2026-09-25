import rehypeStringify from "rehype-stringify";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";
import { describe, expect, it } from "vitest";
import { remarkMermaid, renderDiagram } from "./remark-mermaid.ts";

const SEQUENCE = `sequenceDiagram
    participant A as Alice
    participant B as Bob
    A->>B: Hello`;

describe("renderDiagram", () => {
  const html = renderDiagram(SEQUENCE);

  it("wraps an inline SVG in a .mermaid figure", () => {
    expect(html.startsWith('<figure class="mermaid"><svg')).toBe(true);
    expect(html.endsWith("</figure>")).toBe(true);
    expect(html).toContain("Alice");
  });

  it("colours the diagram with design tokens", () => {
    expect(html).toContain("--fg:var(--color-text)");
    expect(html).toContain("--accent:var(--color-accent)");
  });

  it("scopes the SVG stylesheet and drops the font import", () => {
    expect(html).not.toContain("@import");
    expect(html).toContain(".mermaid svg {");
    expect(html).toContain(".mermaid text {");
  });

  it("gives element ids a per-diagram suffix", () => {
    const other = renderDiagram(`${SEQUENCE}\n    B->>A: Hi`);
    const ids = (s: string) =>
      [...s.matchAll(/\sid="([^"]+)"/gu)].map((m) => m[1]);
    expect(ids(html).length).toBeGreaterThan(0);
    expect(ids(html).some((id) => ids(other).includes(id))).toBe(false);
    for (const id of ids(html)) {
      expect(html).not.toMatch(
        new RegExp(`url\\(#${id.replace(/-[^-]+$/u, "")}\\)`, "u"),
      );
    }
  });
});

describe("remarkMermaid", () => {
  it("replaces mermaid code blocks and leaves other code alone", async () => {
    const md = `\`\`\`mermaid\n${SEQUENCE}\n\`\`\`\n\n\`\`\`python\nprint(1)\n\`\`\`\n`;
    const out = String(
      await unified()
        .use(remarkParse)
        .use(remarkMermaid)
        .use(remarkRehype, { allowDangerousHtml: true })
        .use(rehypeStringify, { allowDangerousHtml: true })
        .process(md),
    );
    expect(out).toContain('<figure class="mermaid">');
    expect(out).not.toContain("language-mermaid");
    expect(out).toContain('class="language-python"');
  });
});
