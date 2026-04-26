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

  test("h1 uses Fraunces font", async ({ page }) => {
    const ff = await page.evaluate(() => {
      const h1 = document.querySelector(
        '[data-testid="section-typography"] h1',
      )!;
      return getComputedStyle(h1).fontFamily;
    });
    expect(ff).toContain("Fraunces");
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

test.describe("/__test/css-foundation sections 7-12", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/__test/css-foundation");
  });

  test("section-eyebrow-grid renders the grid component", async ({ page }) => {
    const s = page.locator('[data-testid="section-eyebrow-grid"]');
    await expect(s).toBeVisible();
    await expect(s.locator(".eyebrow").first()).toBeVisible();
  });

  test("band background is dark in the current colour scheme", async ({
    page,
  }) => {
    const bandBg = await page.evaluate((): string => {
      const band = document.querySelector(
        '[data-testid="section-band"].band',
      ) as HTMLElement;
      // Normalise oklch/lab/rgb to sRGB via canvas
      const c = document.createElement("canvas");
      c.width = 1;
      c.height = 1;
      const ctx = c.getContext("2d")!;
      ctx.fillStyle = getComputedStyle(band).backgroundColor;
      ctx.fillRect(0, 0, 1, 1);
      const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
      return `rgb(${r}, ${g}, ${b})`;
    });
    expect(bandBg).toBe("rgb(26, 22, 18)");
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

  test("section-masthead contains masthead markup", async ({ page }) => {
    const s = page.locator('[data-testid="section-masthead"]');
    await expect(s).toBeVisible();
    await expect(s.locator(".masthead")).toBeVisible();
    await expect(s.locator(".masthead__logo")).toBeVisible();
    await expect(s.locator(".masthead__nav")).toBeVisible();
    await expect(s.locator(".masthead__nav-link--active")).toBeVisible();
  });

  test("section-footer has three footer__item elements", async ({ page }) => {
    const s = page.locator('[data-testid="section-footer"]');
    await expect(s).toBeVisible();
    await expect(s.locator(".footer__item")).toHaveCount(3);
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
