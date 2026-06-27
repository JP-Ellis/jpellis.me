import { expect, test } from "@playwright/test";

const DIGIT_RE = /\d/u;

test.use({ javaScriptEnabled: false });

test.describe("No-JS guard — SSR content renders without JavaScript", () => {
  test("home /: year-in-code band is present and contains digit text", async ({
    page,
  }) => {
    await page.goto("/");
    const band = page.locator("section[data-testid='year-in-code']");
    await expect(band).toBeAttached();
    await expect(band.locator("em").first()).toBeAttached();
    const text = await band.textContent();
    expect(text).toMatch(DIGIT_RE);
  });

  test("home /: commit grid is in the DOM without JS", async ({ page }) => {
    await page.goto("/");
    await expect(
      page.locator("[data-testid='commit-col']").first(),
    ).toBeAttached();
    const count = await page.locator("[data-testid='commit-col']").count();
    expect(count).toBeGreaterThan(0);
  });

  test("/blog: post list is in the DOM without JS", async ({ page }) => {
    await page.goto("/blog");
    await expect(
      page.locator("[data-testid='post-row']").first(),
    ).toBeAttached();
    const count = await page.locator("[data-testid='post-row']").count();
    expect(count).toBeGreaterThan(0);
  });
});
