import { expect, test } from "@playwright/test";

test.describe("Home page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("page title is 'Joshua Ellis'", async ({ page }) => {
    await expect(page).toHaveTitle("Joshua Ellis");
  });

  test("hero h1 contains key phrases", async ({ page }) => {
    const h1 = page.locator("main h1");
    await expect(h1).toContainText("contracts");
    await expect(h1).toContainText("Rust");
    await expect(h1).toContainText("antimatter");
  });

  test("Year in Code band shows commit count", async ({ page }) => {
    const band = page.locator("section[data-testid='year-in-code']");
    await expect(band).toContainText("1,247");
  });

  test("commit grid renders 53 columns", async ({ page }) => {
    await expect(page.locator("[data-testid='commit-col']")).toHaveCount(53);
  });

  test("commit feed shows 5 rows", async ({ page }) => {
    await expect(page.locator("[data-testid='commit-row']")).toHaveCount(5);
  });

  test("Selected Work shows 3 project rows", async ({ page }) => {
    await expect(page.locator("[data-testid='work-row']")).toHaveCount(3);
  });

  test("Selected Work lists the three projects", async ({ page }) => {
    const work = page.locator("section").filter({ hasText: "Selected work" });
    await expect(work).toContainText("pact-python");
    await expect(work).toContainText("tikz-feynman");
    await expect(work).toContainText("boltzmann-solver");
  });

  test("Elsewhere shows 3 cross-post rows", async ({ page }) => {
    await expect(page.locator("[data-testid='cross-post-row']")).toHaveCount(3);
  });
});
