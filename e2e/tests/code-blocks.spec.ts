import { expect, type Page, test } from "@playwright/test";

// A post with several highlighted code blocks and a Mermaid diagram.
const POST = "/blog/asynchronous-message-support";

interface BlockColours {
  background: number;
  text: number;
}

/**
 * Relative luminance (0 = black, 1 = white) of each code block's background
 * and of its first highlighted token.
 */
function blockColours(page: Page): Promise<BlockColours[]> {
  return page.evaluate(() => {
    const canvas = document.createElement("canvas");
    canvas.width = 1;
    canvas.height = 1;
    const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
    function luminance(colour: string): number {
      ctx.clearRect(0, 0, 1, 1);
      ctx.fillStyle = colour;
      ctx.fillRect(0, 0, 1, 1);
      const [r, g, b] = [...ctx.getImageData(0, 0, 1, 1).data].map((v) => {
        const c = v / 255;
        return c <= 0.039_28 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
      });
      return 0.2126 * r + 0.7152 * g + 0.0722 * b;
    }
    return [...document.querySelectorAll("pre.astro-code")].map((pre) => {
      const token = pre.querySelector(".line span") ?? pre;
      return {
        background: luminance(getComputedStyle(pre).backgroundColor),
        text: luminance(getComputedStyle(token).color),
      };
    });
  });
}

test.describe("Code blocks in dark mode", () => {
  test.use({ colorScheme: "dark" });

  test("no code block is white", async ({ page }) => {
    await page.goto(POST);
    const blocks = await blockColours(page);
    expect(blocks.length).toBeGreaterThan(0);
    for (const { background, text } of blocks) {
      expect(background).toBeLessThan(0.1);
      expect(text).toBeGreaterThan(background);
    }
  });
});

test.describe("Code blocks in light mode", () => {
  test.use({ colorScheme: "light" });

  test("code blocks are light with dark text", async ({ page }) => {
    await page.goto(POST);
    const blocks = await blockColours(page);
    expect(blocks.length).toBeGreaterThan(0);
    for (const { background, text } of blocks) {
      expect(background).toBeGreaterThan(0.8);
      expect(text).toBeLessThan(background);
    }
  });
});

test.describe("Mermaid diagrams", () => {
  test("render as inline SVG without JavaScript", async ({ browser }) => {
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();
    await page.goto(POST);
    await expect(page.locator("figure.mermaid svg")).toBeVisible();
    await expect(page.locator("pre[data-language='mermaid']")).toHaveCount(0);
    await context.close();
  });
});
