import { expect, test } from "@playwright/test";

const JOSHUA_RE = /Joshua/u;

test.describe("/__test/css-foundation sections 7-12", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/__test/css-foundation");
  });

  test("section-eyebrow-grid renders the grid component", async ({ page }) => {
    const s = page.locator('[data-testid="section-eyebrow-grid"]');
    await expect(s).toBeVisible();
    await expect(s.locator(".eyebrow").first()).toBeVisible();
  });

  // Band is adaptive: dark in light mode, light in dark mode.
  test("band background is inverse of the page colour scheme", async ({
    page,
  }) => {
    const [bandBg, isDark] = await page.evaluate((): [string, boolean] => {
      const band = document.querySelector(
        '[data-testid="section-band"]',
      ) as HTMLElement;
      const c = document.createElement("canvas");
      c.width = 1;
      c.height = 1;
      const ctx = c.getContext("2d")!;
      ctx.fillStyle = getComputedStyle(band).backgroundColor;
      ctx.fillRect(0, 0, 1, 1);
      const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
      return [
        `rgb(${r}, ${g}, ${b})`,
        globalThis.matchMedia("(prefers-color-scheme: dark)").matches,
      ];
    });
    const expected = isDark ? "rgb(245, 241, 234)" : "rgb(26, 22, 18)";
    expect(bandBg).toBe(expected);
  });

  test("eyebrow inside band is semi-transparent (not solid ink or paper)", async ({
    page,
  }) => {
    const alpha = await page.evaluate((): number => {
      const el = document.querySelector(
        '[data-testid="section-band"] .eyebrow',
      ) as HTMLElement;
      // Normalise to sRGB via canvas to read actual alpha channel
      const c = document.createElement("canvas");
      c.width = 1;
      c.height = 1;
      const ctx = c.getContext("2d")!;
      ctx.fillStyle = getComputedStyle(el).color;
      ctx.fillRect(0, 0, 1, 1);
      return ctx.getImageData(0, 0, 1, 1).data[3]; // 0–255
    });
    expect(alpha).toBeGreaterThan(0);
    expect(alpha).toBeLessThan(255);
  });

  test(".container max-width is 1280px", async ({ page }) => {
    const mw = await page.evaluate(() => {
      const el = document.querySelector(".container") as HTMLElement;
      return getComputedStyle(el).maxWidth;
    });
    expect(mw).toBe("1280px");
  });

  test(".container--prose max-width is 880px", async ({ page }) => {
    const mw = await page.evaluate(() => {
      const el = document.querySelector(".container--prose") as HTMLElement;
      return getComputedStyle(el).maxWidth;
    });
    expect(mw).toBe("880px");
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

  test("section-masthead contains masthead markup", async ({ page }) => {
    const s = page.locator('[data-testid="section-masthead"]');
    await expect(s).toBeVisible();
    await expect(s.locator("header")).toBeVisible();
    await expect(s.locator("nav[aria-label='Site']")).toBeVisible();
    await expect(
      s.locator("a").filter({ hasText: JOSHUA_RE }).first(),
    ).toBeVisible();
  });

  test("section-footer has three footer items", async ({ page }) => {
    const s = page.locator('[data-testid="section-footer"]');
    await expect(s).toBeVisible();
    await expect(s.locator("footer > span")).toHaveCount(3);
  });

  test("focused button in section-focus has solid outline", async ({
    page,
  }) => {
    // Click then Tab so :focus-visible fires (keyboard modality required)
    await page.locator('[data-testid="section-focus"] button').click();
    await page.keyboard.press("Tab");
    await page.keyboard.press("Shift+Tab");
    const outlineStyle = await page.evaluate(() => {
      const btn = document.querySelector(
        '[data-testid="section-focus"] button',
      ) as HTMLElement;
      return getComputedStyle(btn).outlineStyle;
    });
    expect(outlineStyle).toBe("solid");
  });

  test("sr-only element is 1px × 1px with position absolute", async ({
    page,
  }) => {
    const [w, h, pos] = await page.evaluate(() => {
      const el = document.querySelector(
        '[data-testid="section-sr-only"] .sr-only',
      ) as HTMLElement;
      const cs = getComputedStyle(el);
      return [cs.width, cs.height, cs.position];
    });
    expect(w).toBe("1px");
    expect(h).toBe("1px");
    expect(pos).toBe("absolute");
  });
});
