import { expect, test } from "@playwright/test";

test("/__test route exists", async ({ page }) => {
  await page.goto("/__test");
  await expect(page.locator("body")).not.toContainText("Page not found.");
});

test("TestLayout shows debug warning banner", async ({ page }) => {
  await page.goto("/__test");
  await expect(page.locator("body")).toContainText("⚠ debug build — /__test");
});

test("TestLayout nav has link to css-foundation", async ({ page }) => {
  await page.goto("/__test");
  await expect(
    page.locator('nav a[href="/__test/css-foundation"]'),
  ).toBeVisible();
});

test("TestIndex lists the css-foundation page", async ({ page }) => {
  await page.goto("/__test");
  await expect(
    page.locator('a[href="/__test/css-foundation"]').first(),
  ).toBeVisible();
});

test.describe("/__test/css-foundation sections 1-6", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/__test/css-foundation");
  });

  test("instruction text appears at top of page", async ({ page }) => {
    await expect(page.locator("p.eyebrow--muted").first()).toContainText(
      "To test dark mode",
    );
  });

  test("section-typography contains h1 through h6 and code", async ({
    page,
  }) => {
    const s = page.locator('[data-testid="section-typography"]');
    await expect(s).toBeVisible();
    for (const tag of ["h1", "h2", "h3", "h4", "h5", "h6"]) {
      await expect(s.locator(tag)).toBeVisible();
    }
    await expect(s.locator("code").first()).toBeVisible();
  });

  test("body uses Newsreader font", async ({ page }) => {
    const ff = await page.evaluate(
      () => getComputedStyle(document.body).fontFamily,
    );
    expect(ff).toContain("Newsreader");
  });

  test("h1 and h2 use Fraunces font", async ({ page }) => {
    const [h1ff, h2ff] = await page.evaluate(() => {
      const s = document.querySelector('[data-testid="section-typography"]')!;
      return ["h1", "h2"].map(
        (tag) => getComputedStyle(s.querySelector(tag)!).fontFamily,
      );
    });
    expect(h1ff).toContain("Fraunces");
    expect(h2ff).toContain("Fraunces");
  });

  test("h3 and h4 use Newsreader font", async ({ page }) => {
    const [h3ff, h4ff] = await page.evaluate(() => {
      const s = document.querySelector('[data-testid="section-typography"]')!;
      return ["h3", "h4"].map(
        (tag) => getComputedStyle(s.querySelector(tag)!).fontFamily,
      );
    });
    expect(h3ff).toContain("Newsreader");
    expect(h4ff).toContain("Newsreader");
  });

  test("h5 and h6 use Fira Code font", async ({ page }) => {
    const [h5ff, h6ff] = await page.evaluate(() => {
      const s = document.querySelector('[data-testid="section-typography"]')!;
      return ["h5", "h6"].map(
        (tag) => getComputedStyle(s.querySelector(tag)!).fontFamily,
      );
    });
    expect(h5ff).toContain("Fira Code");
    expect(h6ff).toContain("Fira Code");
  });

  test("code uses Fira Code font", async ({ page }) => {
    const ff = await page.evaluate(() => {
      const code = document.querySelector(
        '[data-testid="section-typography"] code',
      )!;
      return getComputedStyle(code).fontFamily;
    });
    expect(ff).toContain("Fira Code");
  });

  test("heading rendered sizes decrease from h1 to h4", async ({ page }) => {
    const sizes = await page.evaluate(() => {
      const s = document.querySelector('[data-testid="section-typography"]')!;
      return ["h1", "h2", "h3", "h4"].map((tag) =>
        Number.parseFloat(getComputedStyle(s.querySelector(tag)!).fontSize),
      );
    });
    for (let i = 0; i < sizes.length - 1; i += 1) {
      expect(sizes[i]).toBeGreaterThan(sizes[i + 1]);
    }
  });

  test("h1 font-size is larger at wide viewport than narrow", async ({
    page,
  }) => {
    await page.setViewportSize({ width: 1280, height: 900 });
    const wideSize = await page.evaluate(() =>
      Number.parseFloat(
        getComputedStyle(
          document.querySelector('[data-testid="section-typography"] h1')!,
        ).fontSize,
      ),
    );
    await page.setViewportSize({ width: 375, height: 812 });
    const narrowSize = await page.evaluate(() =>
      Number.parseFloat(
        getComputedStyle(
          document.querySelector('[data-testid="section-typography"] h1')!,
        ).fontSize,
      ),
    );
    expect(wideSize).toBeGreaterThan(narrowSize);
  });

  test("link has accent-coloured underline distinct from its text colour", async ({
    page,
  }) => {
    const [textColor, underlineColor] = await page.evaluate(
      (): [string, string] => {
        function toRgb(color: string): string {
          const c = document.createElement("canvas");
          c.width = 1;
          c.height = 1;
          const ctx = c.getContext("2d")!;
          ctx.fillStyle = color;
          ctx.fillRect(0, 0, 1, 1);
          const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
          return `rgb(${r}, ${g}, ${b})`;
        }
        const a = document.querySelector(
          '[data-testid="section-typography"] a',
        ) as HTMLElement;
        const cs = getComputedStyle(a);
        return [toRgb(cs.color), toRgb(cs.textDecorationColor)];
      },
    );
    expect(underlineColor).not.toBe(textColor);
  });

  test("body background and text colour match the active colour scheme", async ({
    page,
  }) => {
    const isDark = await page.evaluate(
      () => globalThis.matchMedia("(prefers-color-scheme: dark)").matches,
    );
    // Normalise CSS color (oklch, lab, rgb…) to sRGB via canvas paint
    const [bg, fg] = await page.evaluate((): [string, string] => {
      function toRgb(color: string): string {
        const c = document.createElement("canvas");
        c.width = 1;
        c.height = 1;
        const ctx = c.getContext("2d")!;
        ctx.fillStyle = color;
        ctx.fillRect(0, 0, 1, 1);
        const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
        return `rgb(${r}, ${g}, ${b})`;
      }
      return [
        toRgb(getComputedStyle(document.body).backgroundColor),
        toRgb(getComputedStyle(document.body).color),
      ];
    });
    const [expectedBg, expectedFg] = isDark
      ? ["rgb(26, 22, 18)", "rgb(245, 241, 234)"]
      : ["rgb(245, 241, 234)", "rgb(26, 22, 18)"];
    expect(bg).toBe(expectedBg);
    expect(fg).toBe(expectedFg);
  });

  test("section-colors shows paper and band-bg swatches", async ({ page }) => {
    const s = page.locator('[data-testid="section-colors"]');
    await expect(s).toBeVisible();
    await expect(s.locator('[data-testid="swatch-color-paper"]')).toBeVisible();
    await expect(
      s.locator('[data-testid="swatch-color-band-bg"]'),
    ).toBeVisible();
  });

  test("section-spacing has 8 token bars", async ({ page }) => {
    const s = page.locator('[data-testid="section-spacing"]');
    await expect(s).toBeVisible();
    await expect(s.locator("[data-spacing-bar]")).toHaveCount(8);
  });

  test("section-eyebrow shows .eyebrow and .eyebrow--muted variants", async ({
    page,
  }) => {
    const s = page.locator('[data-testid="section-eyebrow"]');
    await expect(s).toBeVisible();
    await expect(s.locator(".eyebrow").first()).toBeVisible();
    await expect(s.locator(".eyebrow--muted").first()).toBeVisible();
  });

  test("section-hairlines contains rule-section and rule-list", async ({
    page,
  }) => {
    const s = page.locator('[data-testid="section-hairlines"]');
    await expect(s).toBeVisible();
    await expect(s.locator("hr.rule-section")).toBeVisible();
    await expect(s.locator("hr.rule-list").first()).toBeVisible();
  });

  test("tag--pill has border-radius 999px", async ({ page }) => {
    const r = await page.evaluate(() => {
      const el = document.querySelector(
        '[data-testid="section-tags"] .tag--pill',
      )!;
      return getComputedStyle(el).borderRadius;
    });
    expect(r).toBe("999px");
  });

  test("tag--hash has border-radius 3px", async ({ page }) => {
    const r = await page.evaluate(() => {
      const el = document.querySelector(
        '[data-testid="section-tags"] .tag--hash',
      )!;
      return getComputedStyle(el).borderRadius;
    });
    expect(r).toBe("3px");
  });

  test("tag--accent colour differs from plain tag", async ({ page }) => {
    const [plain, accent] = await page.evaluate(() => {
      const p = document.querySelector(
        '[data-testid="section-tags"] .tag:not(.tag--accent)',
      )!;
      const a = document.querySelector(
        '[data-testid="section-tags"] .tag--accent',
      )!;
      return [getComputedStyle(p).color, getComputedStyle(a).color];
    });
    expect(accent).not.toBe(plain);
  });
});
