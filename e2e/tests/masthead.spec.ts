import { expect, test } from "@playwright/test";

test.describe("Masthead", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("logo displays 'Joshua Ellis'", async ({ page }) => {
    const logo = page.locator("header a").first();
    await expect(logo).toContainText("Joshua");
    await expect(logo).toContainText("Ellis");
  });

  test("all five nav links are present", async ({ page }) => {
    const nav = page.locator("header nav[aria-label='Site']");
    for (const label of ["Index", "Projects", "Résumé", "Blog", "Contact"]) {
      await expect(nav.getByRole("link", { name: label })).toBeVisible();
    }
  });

  test("Index link has aria-current='page' on home route", async ({ page }) => {
    const index = page
      .locator("header nav")
      .getByRole("link", { name: "Index" });
    await expect(index).toHaveAttribute("aria-current", "page");
  });

  test("non-home nav links do not have aria-current", async ({ page }) => {
    const nav = page.locator("header nav[aria-label='Site']");
    for (const label of ["Projects", "Résumé", "Blog", "Contact"]) {
      await expect(nav.getByRole("link", { name: label })).not.toHaveAttribute(
        "aria-current",
        "page",
      );
    }
  });
});
