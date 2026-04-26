import { expect, test } from "@playwright/test";

test("/__test route exists", async ({ page }) => {
  await page.goto("/__test");
  await expect(page.locator("body")).not.toContainText("Page not found.");
});
