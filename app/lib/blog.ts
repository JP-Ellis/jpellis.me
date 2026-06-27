import rehypeStringify from "rehype-stringify";
import remarkGfm from "remark-gfm";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";

const monthFormatter = new Intl.DateTimeFormat("en-US", {
  month: "short",
  timeZone: "UTC",
});
const ISO_DATE_RE = /^\d{4}-\d{2}-\d{2}$/u;

const excerptProcessor = unified()
  .use(remarkParse)
  .use(remarkGfm)
  .use(remarkRehype)
  .use(rehypeStringify);

/**
 * Render a markdown fragment to HTML string.
 */
async function renderMarkdown(md: string): Promise<string> {
  const result = await excerptProcessor.process(md);
  return String(result);
}

/**
 * Format a YYYY-MM-DD date string as "D Mon YYYY" (no leading zero on day).
 * Returns the input unchanged on malformed input.
 */
export function formatDate(date: string): string {
  if (!ISO_DATE_RE.test(date)) {
    return date;
  }
  const parsed = new Date(`${date}T00:00:00Z`);
  if (Number.isNaN(parsed.getTime())) {
    return date;
  }
  const day = parsed.getUTCDate();
  const month = monthFormatter.format(parsed);
  const year = parsed.getUTCFullYear();
  return `${day} ${month} ${year}`;
}

/**
 * Extract an excerpt from a blog post's raw markdown body.
 *
 * 1. If the body contains `<!-- more -->`: renders the markdown before that
 *    marker to HTML and returns it.
 * 2. Else if `description` is provided: returns `<p>{description}</p>`.
 * 3. Else: returns empty string.
 */
export function splitExcerpt(
  rawBody: string,
  description?: string,
): Promise<string> {
  const MoreMarker = "<!-- more -->";
  const markerIndex = rawBody.indexOf(MoreMarker);
  if (markerIndex !== -1) {
    const before = rawBody.slice(0, markerIndex).trim();
    return renderMarkdown(before);
  }
  if (description) {
    return Promise.resolve(`<p>${description}</p>`);
  }
  return Promise.resolve("");
}

/**
 * Extract the host portion from a URL string.
 * Mirrors Rust `source_domain`: splits on "//" and takes the first segment.
 * Returns undefined if the string contains no "//".
 */
export function sourceDomain(url: string): string | undefined {
  const afterScheme = url.split("//");
  if (afterScheme.length < 2) {
    return;
  }
  return afterScheme[1].split("/")[0];
}
