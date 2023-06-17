import fs from "fs";
import katex from "katex";
import { escapeSvelte } from "mdsvex";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import { visit } from "unist-util-visit";

/**
 * Outputs the remark tree to remark-tree.json. This has no other functionality
 * other than debugging.
 */
/** @type {import('unified').Plugin<[Options?], Root>} */
// eslint-disable-next-line no-unused-vars
function remarkDebug() {
  return function transformer(tree) {
    // eslint-disable-next-line no-magic-numbers
    fs.writeFileSync("remark-tree.json", JSON.stringify(tree, null, 2));
  };
}

/**
 * Outputs the rehype tree to rehype-tree.json. This has no other functionality
 * other than debugging.
 */
/** @type {import('unified').Plugin<[Options?], Root>} */
// eslint-disable-next-line no-unused-vars
function rehypeDebug() {
  return function transformer(tree) {
    // eslint-disable-next-line no-magic-numbers
    fs.writeFileSync("rehype-tree.json", JSON.stringify(tree, null, 2));
  };
}

/**
 * Parse remark code blocks and convert them to Skeleton's CodeBlock component.
 */
/** @type {import('unified').Plugin<[Options?], Root>} */
function remarkSkeletonCode() {
  return function transformer(tree) {
    visit(tree, "code", (node) => {
      node.type = "raw";
      node.value = `<CodeBlock
        language="${node.lang}"
        lineNumbers="true"
        code={\`${escapeSvelte(node.value)}\`}
      />`;
    });
  };
}

/**
 * Parse code blocks with the `math` language and convert them to KaTeX. This must be run
 * _before_ `remarkSkeletonCode` so that the code blocks are not converted to Skeleton's
 * CodeBlock component.
 */
/** @type {import('unified').Plugin<[Options?], Root>} */
function remarkMathBlock() {
  return function transformer(tree) {
    visit(tree, "code", (node) => {
      if (node.lang === "math") {
        const rendered = katex.renderToString(node.value, {
          displayMode: true,
        });

        node.type = "raw";
        node.value = `{@html \`${rendered}\`}`;
      }
    });
  };
}

/**
 * Parse Skeleton base elements and add the appropriate typography classes.
 */
/** @type {import('unified').Plugin<[Options?], Root>} */
function rehypeSkeletonElements() {
  return function transformer(tree) {
    // eslint-disable-next-line max-statements
    visit(tree, "element", (node) => {
      // Add header classes
      if (node.tagName === "h1") {
        node.properties.class = "h1";
      }
      if (node.tagName === "h2") {
        node.properties.class = "h2";
      }
      if (node.tagName === "h3") {
        node.properties.class = "h3";
      }
      if (node.tagName === "h4") {
        node.properties.class = "h4";
      }
      if (node.tagName === "h5") {
        node.properties.class = "h5";
      }
      if (node.tagName === "h6") {
        node.properties.class = "h6";
      }

      // Add anchor
      if (node.tagName === "a") {
        node.properties.class = "anchor";
      }

      // code and kbd tags
      if (node.tagName === "code") {
        node.properties.class = "code text-md";
      }
      if (node.tagName === "kbd") {
        node.properties.class = "kbd";
      }
    });
  };
}

/**
 * Parse inline math and convert it to KaTeX. This must be run _after_
 * `remarkMath`.
 */
/** @type {import('unified').Plugin<[Options?], Root>} */
function remarkKatexInline() {
  return function transformer(tree) {
    visit(tree, "inlineMath", (node) => {
      const rendered = katex.renderToString(node.value, {
        displayMode: false,
      });

      node.type = "raw";
      node.value = `{@html \`${rendered}\`}`;
      delete node.data;
    });
  };
}

/**
 * Parse display math and convert it to KaTeX. This must be run _after_
 * `remarkMath`.
 */
/** @type {import('unified').Plugin<[Options?], Root>} */
function remarkKatexDisplay() {
  return function transformer(tree) {
    visit(tree, "math", (node) => {
      const rendered = katex.renderToString(node.value, {
        displayMode: true,
      });

      node.type = "raw";
      node.value = `{@html \`${rendered}\`}`;
      delete node.data;
    });
  };
}

/** @type {import('mdsvex').MdsvexOptions} */
export const config = {
  extensions: [".md", ".svx"],

  remarkPlugins: [
    remarkGfm,
    remarkMathBlock,
    remarkSkeletonCode,
    remarkMath,
    remarkKatexInline,
    remarkKatexDisplay,
    // remarkDebug,
  ],
  rehypePlugins: [
    rehypeSkeletonElements,
    rehypeSlug,
    rehypeAutolinkHeadings,
    // rehypeDebug,
  ],
};
