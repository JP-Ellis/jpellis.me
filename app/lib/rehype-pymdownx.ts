/**
 * Rehype plugin that post-processes rendered HTML for PyMdown-compatible
 * tab groups (`/// tab | Title` … `///`) and details blocks
 * (`/// details | Title` … `///`).
 *
 * The plugin mirrors the logic in `build/markdown.rs` (`postprocess_pymdownx`):
 * it scans hast paragraph nodes for `/// <type> | <title>` opener text and
 * `///` closer text, extracts the nodes between them, and replaces them with
 * the appropriate HTML structure.
 *
 * Output structures must match `app/styles/components/_prose.scss` exactly:
 * - Tab group: radios first, then `.tab-bar` labels, then `.tab-panel` sections
 * - Details: native `<details>`/`<summary>` element
 */

import type { Element, Node, Properties, Root, RootContent } from "hast";
import type { Plugin, Transformer } from "unified";

/** Length of the "/// " prefix stripped when parsing opener text. */
const OPENER_PREFIX_LEN = 4;

/** Length of the " | " separator stripped when splitting type from title. */
const BAR_SEP_LEN = 3;

// MARK: Type helpers

/** Returns true if a hast node is an element with the given tag name. */
function isElement(node: Node, tag: string): node is Element {
  return node.type === "element" && (node as Element).tagName === tag;
}

/**
 * Returns the plain-text content of an element by concatenating all
 * descendant text nodes.
 */
function textContent(node: Element): string {
  let out = "";
  for (const child of node.children) {
    if (child.type === "text") {
      out += child.value;
    } else if (child.type === "element") {
      out += textContent(child);
    }
  }
  return out;
}

/**
 * Returns true if the element is a paragraph whose only text content
 * (trimmed) matches `text`.
 */
function isParagraphWithText(node: RootContent, text: string): boolean {
  if (!isElement(node, "p")) {
    return false;
  }
  return textContent(node).trim() === text;
}

/**
 * Returns the opener type and title if the paragraph is a PyMdown opener
 * (`/// <type>` or `/// <type> | <title>`), otherwise undefined.
 */
function parseOpener(
  node: RootContent,
): { type: string; title: string } | undefined {
  if (!isElement(node, "p")) {
    return;
  }
  const raw = textContent(node).trim();
  if (!raw.startsWith("/// ")) {
    return;
  }
  const payload = raw.slice(OPENER_PREFIX_LEN);
  const barIdx = payload.indexOf(" | ");
  if (barIdx !== -1) {
    return {
      type: payload.slice(0, barIdx).trim(),
      title: payload.slice(barIdx + BAR_SEP_LEN).trim(),
    };
  }
  return { type: payload.trim(), title: "" };
}

/** Returns true if the node is a whitespace-only text node. */
function isWhitespaceText(node: RootContent): boolean {
  return (
    node.type === "text" && (node as { value: string }).value.trim() === ""
  );
}

/**
 * Find the index of the first non-whitespace node at or after `start`.
 * Returns `start` if the node at `start` is already non-whitespace.
 */
function skipWhitespace(children: RootContent[], start: number): number {
  let idx = start;
  while (idx < children.length && isWhitespaceText(children[idx])) {
    idx += 1;
  }
  return idx;
}

/** Filter whitespace text nodes from a node list. */
function stripWhitespaceNodes(nodes: RootContent[]): RootContent[] {
  return nodes.filter((n) => !isWhitespaceText(n));
}

// MARK: HTML builders

/** Build a single `<input class="tab-radio">` element. */
function buildRadio(g: number, i: number): Element {
  const props: Properties = {
    className: ["tab-radio"],
    type: "radio",
    name: `tabs-${g}`,
    id: `tab-${g}-${i}`,
  };
  if (i === 0) {
    props.checked = true;
  }
  return {
    type: "element",
    tagName: "input",
    properties: props,
    children: [],
  };
}

/** Build the tab-group element tree for a set of (title, nodes[]) pairs. */
function buildTabGroup(
  tabs: Array<{ title: string; nodes: RootContent[] }>,
  counter: number,
): Element {
  const g = counter;
  const tabChildren: Element[] = [];

  for (let i = 0; i < tabs.length; i += 1) {
    tabChildren.push(buildRadio(g, i));
  }

  const labels: Element[] = tabs.map((tab, i) => ({
    type: "element",
    tagName: "label",
    properties: { htmlFor: [`tab-${g}-${i}`] },
    children: [{ type: "text", value: tab.title }],
  }));

  tabChildren.push({
    type: "element",
    tagName: "div",
    properties: { className: ["tab-bar"] },
    children: labels,
  });

  for (const tab of tabs) {
    tabChildren.push({
      type: "element",
      tagName: "section",
      properties: { className: ["tab-panel"] },
      children: stripWhitespaceNodes(tab.nodes) as Element["children"],
    });
  }

  return {
    type: "element",
    tagName: "div",
    properties: { className: ["tabs"] },
    children: tabChildren,
  };
}

/** Build a `<details><summary>title</summary>…</details>` element. */
function buildDetails(title: string, nodes: RootContent[]): Element {
  const summary = title || "Details";
  return {
    type: "element",
    tagName: "details",
    properties: {},
    children: [
      {
        type: "element",
        tagName: "summary",
        properties: {},
        children: [{ type: "text", value: summary }],
      },
      ...(stripWhitespaceNodes(nodes) as Element["children"]),
    ],
  };
}

// MARK: Tab group collector

/**
 * Find the index of the next `<p>///</p>` closer at or after `start`.
 * Returns -1 if not found.
 */
function findCloser(children: RootContent[], start: number): number {
  for (let j = start; j < children.length; j += 1) {
    if (isParagraphWithText(children[j], "///")) {
      return j;
    }
  }
  return -1;
}

/**
 * Starting at `start` (first index after the first tab's closer), scan forward
 * collecting all consecutive tab blocks into one group. Returns the tabs array
 * and the index of the first child after the last consumed closer.
 */
function collectTabGroup(
  children: RootContent[],
  start: number,
  first: { title: string; nodes: RootContent[] },
): { tabs: Array<{ title: string; nodes: RootContent[] }>; nextIdx: number } {
  const tabs: Array<{ title: string; nodes: RootContent[] }> = [first];
  let nextIdx = skipWhitespace(children, start);

  while (nextIdx < children.length) {
    const peek = parseOpener(children[nextIdx]);
    if (peek?.type !== "tab") {
      break;
    }

    const closerIdx = findCloser(children, nextIdx + 1);
    if (closerIdx === -1) {
      break;
    }

    tabs.push({
      title: peek.title,
      nodes: children.slice(nextIdx + 1, closerIdx),
    });
    nextIdx = skipWhitespace(children, closerIdx + 1);
  }

  return { tabs, nextIdx };
}

// MARK: Core transform

/** State threaded through the child-transform loop. */
interface TransformState {
  children: RootContent[];
  counter: { value: number };
  out: RootContent[];
}

/**
 * Process one PyMdown opener at index `i`. Pushes the replacement element
 * into `state.out` and returns the next index to process.
 */
function processOpener(
  state: TransformState,
  i: number,
  opener: { type: string; title: string },
): number {
  const { children, counter, out } = state;
  const closerIdx = findCloser(children, i + 1);

  if (closerIdx === -1) {
    out.push(children[i]);
    return i + 1;
  }

  const innerNodes = children.slice(i + 1, closerIdx);

  if (opener.type === "tab") {
    const afterFirst = skipWhitespace(children, closerIdx + 1);
    const { tabs, nextIdx } = collectTabGroup(children, afterFirst, {
      title: opener.title,
      nodes: innerNodes,
    });
    out.push(buildTabGroup(tabs, counter.value));
    counter.value += 1;
    return nextIdx;
  }

  if (opener.type === "details") {
    out.push(buildDetails(opener.title, innerNodes));
    return closerIdx + 1;
  }

  out.push(children[i]);
  return i + 1;
}

/**
 * Scan `children` for PyMdown opener/closer pairs and replace them with
 * tab-group or details elements. Recurses into container elements (e.g. `<li>`,
 * `<blockquote>`) so nested blocks are transformed depth-first using the same
 * shared `counter` for globally-unique tab IDs.
 *
 * @param children - Hast child nodes of the root or a container element.
 * @param counter - Mutable box `{ value: number }` for tab-group IDs.
 */
function transformChildren(
  children: RootContent[],
  counter: { value: number },
): RootContent[] {
  const out: RootContent[] = [];
  const state: TransformState = { children, counter, out };
  let i = 0;

  while (i < children.length) {
    const node = children[i];
    const opener = parseOpener(node);

    if (opener) {
      i = processOpener(state, i, opener);
    } else {
      if (node.type === "element") {
        const el = node as Element;
        el.children = transformChildren(
          el.children as RootContent[],
          counter,
        ) as Element["children"];
      }
      out.push(node);
      i += 1;
    }
  }

  return out;
}

// MARK: Plugin export

/**
 * Rehype plugin: replaces PyMdown-style `/// tab | …` / `/// details | …`
 * marker paragraphs with proper tab-group and details HTML, matching the
 * output of `build/markdown.rs`'s `postprocess_pymdownx`.
 */
export const rehypePymdownx: Plugin<[], Root, Root> =
  (): Transformer<Root, Root> =>
  (tree: Root): void => {
    const counter = { value: 0 };
    tree.children = transformChildren(
      tree.children as RootContent[],
      counter,
    ) as Root["children"];
  };
