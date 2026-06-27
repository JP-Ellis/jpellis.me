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

  test("band background is visually distinct when CSS is loaded", async ({
    page,
  }) => {
    const [bandBg, bodyBg] = await page.evaluate((): [string, string] => {
      const root = document.documentElement;
      const rootStyle = getComputedStyle(root);
      const paper = rootStyle.getPropertyValue("--color-paper").trim();
      if (!paper) {
        return ["skip", "skip"];
      }
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
      const band = document.querySelector(
        "[data-testid='year-in-code']",
      ) as HTMLElement;
      return [
        toRgb(getComputedStyle(band).backgroundColor),
        toRgb(getComputedStyle(document.body).backgroundColor),
      ];
    });
    if (bandBg !== "skip") {
      expect(bandBg).not.toBe(bodyBg);
    }
  });
});
