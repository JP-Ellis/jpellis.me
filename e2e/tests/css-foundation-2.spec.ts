import { expect, test } from "@playwright/test";

// CSS foundation section 2 — masthead, footer, and focus checks on the home page.

const JOSHUA_RE = /Joshua/u;

test.describe("CSS foundation — masthead and footer on home page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("masthead contains site nav", async ({ page }) => {
    await expect(page.locator("header")).toBeVisible();
    await expect(page.locator("nav[aria-label='Site']")).toBeVisible();
    await expect(
      page.locator("header a").filter({ hasText: JOSHUA_RE }).first(),
    ).toBeVisible();
  });

  test("footer has three items", async ({ page }) => {
    await expect(page.locator("footer > span")).toHaveCount(3);
  });

  test("page has no horizontal overflow at narrow viewport", async ({
    page,
  }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    const overflows = await page.evaluate(
      () => document.documentElement.scrollWidth > window.innerWidth,
    );
    expect(overflows).toBe(false);
  });

  test("eyebrow inside band uses the chromatic accent (not ink or paper)", async ({
    page,
  }) => {
    // The band eyebrow renders in the solid accent colour. Assert it is a
    // saturated hue rather than the near-grey ink/paper tones, which is what
    // distinguishes it visually (the live site renders it fully opaque).
    const [r, g, b] = await page.evaluate((): number[] => {
      const el = document.querySelector(
        "[data-testid='year-in-code'] .eyebrow",
      ) as HTMLElement;
      const c = document.createElement("canvas");
      c.width = 1;
      c.height = 1;
      const ctx = c.getContext("2d")!;
      ctx.fillStyle = getComputedStyle(el).color;
      ctx.fillRect(0, 0, 1, 1);
      const { data } = ctx.getImageData(0, 0, 1, 1);
      return [data[0], data[1], data[2]];
    });
    const chroma = Math.max(r, g, b) - Math.min(r, g, b);
    expect(chroma).toBeGreaterThan(50);
  });

  test("focused nav link has solid outline", async ({ page, browserName }) => {
    // WebKit does not move keyboard focus to links via Tab by default
    // (it mirrors Safari's "Full Keyboard Access off" behaviour), so the
    // Tab-to-link premise below cannot hold there.
    test.skip(
      browserName === "webkit",
      "WebKit does not Tab-focus links by default",
    );
    const link = page.locator("header nav a").first();
    // The outline is applied via :focus-visible, which only engages for
    // keyboard-driven focus. Tab through the document until the first nav
    // link is the active element so the visible outline applies reliably.
    for (let i = 0; i < 20; i += 1) {
      await page.keyboard.press("Tab");
      const focused = await link.evaluate(
        (el) => el === document.activeElement,
      );
      if (focused) {
        break;
      }
    }
    await expect(link).toBeFocused();
    const outlineStyle = await link.evaluate(
      (el) => getComputedStyle(el).outlineStyle,
    );
    expect(outlineStyle).toBe("solid");
  });
});
