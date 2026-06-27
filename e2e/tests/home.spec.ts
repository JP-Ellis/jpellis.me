import { expect, test } from "@playwright/test";

const DIGIT_RE = /\d/u;

test.describe("Home page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("page title is 'JP Ellis'", async ({ page }) => {
    await expect(page).toHaveTitle("JP Ellis");
  });

  test("hero h1 contains key phrases", async ({ page }) => {
    const h1 = page.locator("main h1");
    await expect(h1).toContainText("contracts");
    await expect(h1).toContainText("Rust");
    await expect(h1).toContainText("antimatter");
  });

  test("Year in Code band shows commit count", async ({ page }) => {
    const band = page.locator("section[data-testid='year-in-code']");
    await expect(band).toBeVisible();
    await expect(band).toContainText(DIGIT_RE);
  });

  test("commit grid renders a year of week columns", async ({ page }) => {
    const count = await page.locator("[data-testid='commit-col']").count();
    expect(count).toBeGreaterThanOrEqual(52);
    expect(count).toBeLessThanOrEqual(53);
  });

  test("commit feed shows at least 1 row", async ({ page }) => {
    await expect(
      page.locator("[data-testid='commit-row']").first(),
    ).toBeVisible();
  });

  test("Selected Projects shows 3 project rows", async ({ page }) => {
    await expect(page.locator("[data-testid='projects-row']")).toHaveCount(3);
  });

  test("Selected Projects lists the three projects", async ({ page }) => {
    const work = page
      .locator("section")
      .filter({ hasText: "Selected projects" });
    await expect(work).toContainText("pact-python");
    await expect(work).toContainText("tikz-feynman");
    await expect(work).toContainText("rust-skiplist");
  });

  test("Recent Writing shows at least 1 cross-post row", async ({ page }) => {
    await expect(
      page.locator("[data-testid='cross-post-row']").first(),
    ).toBeVisible();
  });
});
