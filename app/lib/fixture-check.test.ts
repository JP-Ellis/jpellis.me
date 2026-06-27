import { readFileSync } from "node:fs";
import rehypeStringify from "rehype-stringify";
import remarkGfm from "remark-gfm";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";
import { expect, test } from "vitest";
import { rehypePymdownx } from "./rehype-pymdownx.ts";

const DETAILS_RE = /\/\/\/ details/u;
const TAB_RE = /\/\/\/ tab/u;

test("functional-arguments.md fixture has no PyMdown marker leaks", async () => {
  const md = readFileSync(
    new URL("../content/blog/functional-arguments.md", import.meta.url),
    "utf-8",
  );

  const result = await unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkRehype, { allowDangerousHtml: true })
    .use(rehypePymdownx)
    .use(rehypeStringify, { allowDangerousHtml: true })
    .process(md);

  const html = String(result);

  expect(html).not.toMatch(DETAILS_RE);
  expect(html).not.toMatch(TAB_RE);
  expect(html).not.toContain("<p>///</p>");
  expect(html).not.toContain("///</p>");
});
