import { expect, test } from "@playwright/test";

test.describe("Footer", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("copyright text is present", async ({ page }) => {
    await expect(page.locator("footer")).toContainText("Joshua P. Ellis");
  });

  test("social links have correct hrefs", async ({ page }) => {
    const footer = page.locator("footer");
    await expect(footer.getByRole("link", { name: "github" })).toHaveAttribute(
      "href",
      "https://github.com/JP-Ellis",
    );
    await expect(
      footer.getByRole("link", { name: "linkedin" }),
    ).toHaveAttribute("href", "https://linkedin.com/in/joshuapellis");
    await expect(footer.getByRole("link", { name: "email" })).toHaveAttribute(
      "href",
      "mailto:josh@jpellis.me",
    );
  });

  test("licence text is present", async ({ page }) => {
    await expect(page.locator("footer")).toContainText("cc by 4.0");
  });
});
