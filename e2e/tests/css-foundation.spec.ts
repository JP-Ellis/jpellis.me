import { expect, test } from "@playwright/test";

// CSS foundation checks on the home page and blog listing.

// Design colours are authored in oklch; the sRGB values a browser computes
// from them vary by a few units per channel across contexts and rendering
// paths (the same text colour can land one unit apart per channel).
// Compare with a small tolerance rather than asserting bit-exact sRGB.
const RGB_TOLERANCE = 3;

function expectRgbClose(actual: number[], expected: number[]): void {
  for (let i = 0; i < 3; i += 1) {
    expect(Math.abs(actual[i] - expected[i])).toBeLessThanOrEqual(
      RGB_TOLERANCE,
    );
  }
}

// Surface and text for each theme. The dark theme is tuned independently,
// not a straight inversion of the light palette. Bands invert the page
// palette with a surface lifted toward the page lightness.
const SURFACE: [number, number, number] = [248, 246, 244];
const TEXT: [number, number, number] = [24, 22, 19];
const DARK_SURFACE: [number, number, number] = [20, 18, 16];
const DARK_TEXT: [number, number, number] = [236, 235, 233];
const BAND_ON_LIGHT: [number, number, number] = [36, 33, 31];
const BAND_ON_DARK: [number, number, number] = [233, 232, 230];

test.describe("CSS foundation — typography and color", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("body uses Newsreader font", async ({ page }) => {
    const ff = await page.evaluate(
      () => getComputedStyle(document.body).fontFamily,
    );
    expect(ff).toContain("Newsreader");
  });

  test("h1 uses Fraunces font", async ({ page }) => {
    const ff = await page.evaluate(() => {
      const h1 = document.querySelector("main h1") as HTMLElement;
      return getComputedStyle(h1).fontFamily;
    });
    expect(ff).toContain("Fraunces");
  });

  test("body background and text colour match the active colour scheme", async ({
    page,
  }) => {
    const isDark = await page.evaluate(
      () => globalThis.matchMedia("(prefers-color-scheme: dark)").matches,
    );
    const [bg, fg] = await page.evaluate((): [number[], number[]] => {
      function toRgb(color: string): number[] {
        const c = document.createElement("canvas");
        c.width = 1;
        c.height = 1;
        const ctx = c.getContext("2d")!;
        ctx.fillStyle = color;
        ctx.fillRect(0, 0, 1, 1);
        const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
        return [r, g, b];
      }
      return [
        toRgb(getComputedStyle(document.body).backgroundColor),
        toRgb(getComputedStyle(document.body).color),
      ];
    });
    expectRgbClose(bg, isDark ? DARK_SURFACE : SURFACE);
    expectRgbClose(fg, isDark ? DARK_TEXT : TEXT);
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

  test("h1 font-size is larger at wide viewport than narrow", async ({
    page,
  }) => {
    await page.setViewportSize({ width: 1280, height: 900 });
    const wideSize = await page.evaluate(() =>
      Number.parseFloat(
        getComputedStyle(document.querySelector("main h1")!).fontSize,
      ),
    );
    await page.setViewportSize({ width: 375, height: 812 });
    const narrowSize = await page.evaluate(() =>
      Number.parseFloat(
        getComputedStyle(document.querySelector("main h1")!).fontSize,
      ),
    );
    expect(wideSize).toBeGreaterThan(narrowSize);
  });

  test(".container max-width is 1280px", async ({ page }) => {
    const mw = await page.evaluate(() => {
      const el = document.querySelector(".container") as HTMLElement;
      return getComputedStyle(el).maxWidth;
    });
    expect(mw).toBe("1280px");
  });

  test("band background is inverse of the page colour scheme", async ({
    page,
  }) => {
    const [bandBg, isDark] = await page.evaluate((): [number[], boolean] => {
      const band = document.querySelector(
        "[data-testid='year-in-code']",
      ) as HTMLElement;
      const c = document.createElement("canvas");
      c.width = 1;
      c.height = 1;
      const ctx = c.getContext("2d")!;
      ctx.fillStyle = getComputedStyle(band).backgroundColor;
      ctx.fillRect(0, 0, 1, 1);
      const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
      return [
        [r, g, b],
        globalThis.matchMedia("(prefers-color-scheme: dark)").matches,
      ];
    });
    expectRgbClose(bandBg, isDark ? BAND_ON_DARK : BAND_ON_LIGHT);
  });
});

test.describe("CSS foundation — blog page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/blog");
  });

  test("tag--pill has border-radius 999px", async ({ page }) => {
    const r = await page.evaluate(() => {
      const el = document.querySelector(".tag--pill") as HTMLElement;
      return getComputedStyle(el).borderRadius;
    });
    expect(r).toBe("999px");
  });
});

test.describe("CSS foundation — projects page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/projects");
  });

  test("sr-only element is 1px × 1px with position absolute", async ({
    page,
  }) => {
    const [w, h, pos] = await page.evaluate(() => {
      const el = document.querySelector(".sr-only") as HTMLElement;
      const cs = getComputedStyle(el);
      return [cs.width, cs.height, cs.position];
    });
    expect(w).toBe("1px");
    expect(h).toBe("1px");
    expect(pos).toBe("absolute");
  });
});
