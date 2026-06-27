/**
 * TDD tests for the rehype-pymdownx plugin.
 *
 * These tests drive the plugin through a real unified pipeline and assert that
 * the emitted HTML matches the structures expected by _prose.scss.
 */

import rehypeStringify from "rehype-stringify";
import remarkGfm from "remark-gfm";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";
import { expect, test } from "vitest";

import { rehypePymdownx } from "./rehype-pymdownx.ts";

/** Process markdown through the full pipeline and return the HTML string. */
async function process(md: string): Promise<string> {
  const result = await unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkRehype, { allowDangerousHtml: true })
    .use(rehypePymdownx)
    .use(rehypeStringify, { allowDangerousHtml: true })
    .process(md);
  return String(result);
}

// MARK: Tab group structure

test("two consecutive tab blocks produce one .tabs wrapper", async () => {
  const md = [
    "/// tab | Before",
    "",
    "Content A",
    "",
    "///",
    "",
    "/// tab | After",
    "",
    "Content B",
    "",
    "///",
  ].join("\n");

  const html = await process(md);

  // One wrapper
  const tabDivMatches = [...html.matchAll(/<div class="tabs">/gu)];
  expect(tabDivMatches).toHaveLength(1);

  // Exactly two radio inputs
  const radioMatches = [...html.matchAll(/class="tab-radio"/gu)];
  expect(radioMatches).toHaveLength(2);

  // First radio is checked, second is not
  expect(html).toContain('id="tab-0-0" checked');
  expect(html).toContain('id="tab-0-1"');
  expect(html).not.toContain('id="tab-0-1" checked');

  // Both share the same name
  const nameMatches = [...html.matchAll(/name="tabs-0"/gu)];
  expect(nameMatches).toHaveLength(2);

  // tab-bar with two labels
  expect(html).toContain('<div class="tab-bar">');
  expect(html).toContain('<label for="tab-0-0">Before</label>');
  expect(html).toContain('<label for="tab-0-1">After</label>');

  // Two tab panels
  const panelMatches = [...html.matchAll(/class="tab-panel"/gu)];
  expect(panelMatches).toHaveLength(2);
  expect(html).toContain("Content A");
  expect(html).toContain("Content B");
});

test("radios appear before tab-bar which appears before panels", async () => {
  const md = [
    "/// tab | X",
    "",
    "body",
    "",
    "///",
    "",
    "/// tab | Y",
    "",
    "body2",
    "",
    "///",
  ].join("\n");

  const html = await process(md);

  const radioPos = html.indexOf('class="tab-radio"');
  const barPos = html.indexOf('class="tab-bar"');
  const panelPos = html.indexOf('class="tab-panel"');

  expect(radioPos).toBeGreaterThan(-1);
  expect(barPos).toBeGreaterThan(radioPos);
  expect(panelPos).toBeGreaterThan(barPos);
});

test("non-consecutive tab groups get different counter IDs", async () => {
  const md = [
    "/// tab | A1",
    "",
    "body",
    "",
    "///",
    "",
    "Some prose in between.",
    "",
    "/// tab | B1",
    "",
    "body",
    "",
    "///",
  ].join("\n");

  const html = await process(md);

  // Two separate .tabs wrappers
  const divMatches = [...html.matchAll(/<div class="tabs">/gu)];
  expect(divMatches).toHaveLength(2);

  // First group uses tabs-0, second uses tabs-1
  expect(html).toContain('name="tabs-0"');
  expect(html).toContain('name="tabs-1"');
});

// MARK: Details block

test("details block renders as native details/summary element", async () => {
  const md = [
    "/// details | Example",
    "",
    "Inner content here.",
    "",
    "///",
  ].join("\n");

  const html = await process(md);

  expect(html).toContain("<details>");
  expect(html).toContain("<summary>Example</summary>");
  expect(html).toContain("Inner content here.");
  expect(html).toContain("</details>");
  expect(html).not.toContain("/// details");
});

test("details without explicit title defaults to 'Details'", async () => {
  const md = ["/// details", "", "Body.", "", "///"].join("\n");

  const html = await process(md);

  expect(html).toContain("<summary>Details</summary>");
});

// MARK: No raw marker leakage

test("no raw /// tab markers leak into output", async () => {
  const md = ["/// tab | Title", "", "Content", "", "///"].join("\n");

  const html = await process(md);

  expect(html).not.toContain("/// tab");
  expect(html).not.toContain("<p>///</p>");
  expect(html).not.toContain("///</p>");
});

test("no raw /// details markers leak into output", async () => {
  const md = ["/// details | Example", "", "Content", "", "///"].join("\n");

  const html = await process(md);

  expect(html).not.toContain("/// details");
  expect(html).not.toContain("<p>///</p>");
});

test("unclosed tab block emits literal text (no crash, no leakage of ///)", async () => {
  const md = ["/// tab | Orphan", "", "Content without closer"].join("\n");

  // Should not throw; the literal text should appear (as fallback)
  const html = await process(md);
  expect(html).toBeDefined();
  // Must not produce a broken .tabs wrapper with missing closer
  expect(html).not.toContain('<div class="tabs">');
});

// MARK: GFM features

test("GFM table renders as <table>", async () => {
  const md = ["| A | B |", "| - | - |", "| 1 | 2 |"].join("\n");

  const html = await process(md);

  expect(html).toContain("<table>");
  expect(html).toContain("<th>");
  expect(html).toContain("<td>");
});

test("GFM strikethrough renders as <del>", async () => {
  const md = "~~deleted~~";
  const html = await process(md);
  expect(html).toContain("<del>deleted</del>");
});

// MARK: Nested blocks (inside list items)

test("details block nested inside a list item renders correctly", async () => {
  const md = [
    "- List item prose.",
    "",
    "  /// details | Example",
    "",
    "  Inner content here.",
    "",
    "  ///",
  ].join("\n");

  const html = await process(md);

  expect(html).toContain("<details>");
  expect(html).toContain("<summary>Example</summary>");
  expect(html).toContain("Inner content here.");
  expect(html).toContain("</details>");
  expect(html).not.toContain("/// details");
  expect(html).not.toContain("<p>///</p>");
  expect(html).not.toContain("///</p>");
});

test("tab group nested inside a list item renders correctly", async () => {
  const md = [
    "- List item prose.",
    "",
    "  /// tab | Alpha",
    "",
    "  Content alpha.",
    "",
    "  ///",
    "",
    "  /// tab | Beta",
    "",
    "  Content beta.",
    "",
    "  ///",
  ].join("\n");

  const html = await process(md);

  expect(html).toContain('<div class="tabs">');
  expect(html).toContain("Content alpha.");
  expect(html).toContain("Content beta.");
  expect(html).not.toContain("/// tab");
  expect(html).not.toContain("<p>///</p>");
  expect(html).not.toContain("///</p>");
});
