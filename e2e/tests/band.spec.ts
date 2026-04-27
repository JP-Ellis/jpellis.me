import { expect, test } from "@playwright/test";

test.describe("Band", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("Year in Code band is present", async ({ page }) => {
    await expect(
      page.locator("section[data-testid='year-in-code']"),
    ).toBeVisible();
  });

  test("band background is visually distinct from page background", async ({
    page,
  }) => {
    const band = page.locator("section[data-testid='year-in-code']");
    const bandBg = await band.evaluate(
      (el) => getComputedStyle(el).backgroundColor,
    );
    const bodyBg = await page
      .locator("body")
      .evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(bandBg).not.toBe(bodyBg);
  });
});
