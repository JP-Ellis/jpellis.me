import { expect, type Page, test } from "@playwright/test";

const EMAIL = "website@jpellis.me";
const CONTACT_URL_RE = /\/contact$/u;
const HOME_URL_RE = /\/$/u;

async function copyAndCheck(page: Page): Promise<void> {
  const button = page.getByRole("button", { name: "copy address" });
  await button.click();
  await expect(page.getByRole("button", { name: "✓ copied" })).toBeVisible();
  expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(EMAIL);
}

test.describe("Contact page", () => {
  test("shows the contact address and its mail link", async ({ page }) => {
    await page.goto("/contact");
    await expect(page.locator(".email_display")).toHaveText(EMAIL);
    for (const link of await page.locator("a[href^='mailto:']").all()) {
      await expect(link).toHaveAttribute("href", `mailto:${EMAIL}`);
    }
  });

  test("the copy button survives client-side navigation", async ({
    page,
    context,
    browserName,
  }) => {
    test.skip(
      browserName !== "chromium",
      "Clipboard permissions can only be granted in Chromium",
    );
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    const nav = page.locator("nav[aria-label='Site']");

    await page.goto("/");
    await nav.getByRole("link", { name: "Contact" }).click();
    await expect(page).toHaveURL(CONTACT_URL_RE);
    await copyAndCheck(page);

    await nav.getByRole("link", { name: "Index" }).click();
    await expect(page).toHaveURL(HOME_URL_RE);
    await nav.getByRole("link", { name: "Contact" }).click();
    await expect(page).toHaveURL(CONTACT_URL_RE);
    await copyAndCheck(page);
  });

  test("the copy button is hidden without JavaScript", async ({ browser }) => {
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();
    await page.goto("/contact");
    await expect(page.locator(".copy-btn")).toBeHidden();
    await context.close();
  });
});
