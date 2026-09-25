import { expect, type Page, test } from "@playwright/test";

// A tag on some posts but not all, so filtering hides rows.
const TAG = "rust";
const TAG_URL_RE = /\/blog\?tag=rust$/u;
const BLOG_URL_RE = /\/blog$/u;

function rows(page: Page) {
  return page.locator("[data-testid='post-row']");
}

function tagPill(page: Page) {
  return page.locator(`[data-filter-tag='${TAG}']`);
}

/** Every visible row carries the tag, and at least one row is visible. */
async function expectOnlyTagged(page: Page): Promise<void> {
  const visible = rows(page).locator("visible=true");
  await expect(visible.first()).toBeVisible();
  for (const tags of await visible.evaluateAll((els) =>
    els.map((el) => (el as HTMLElement).dataset.tags ?? ""),
  )) {
    expect(tags.split(" ")).toContain(TAG);
  }
}

async function expectAllVisible(page: Page, total: number): Promise<void> {
  await expect(rows(page).locator("visible=true")).toHaveCount(total);
}

test.describe("Blog tag filter", () => {
  test("apply, clear and go back", async ({ page }) => {
    await page.goto("/blog");
    const total = await rows(page).count();
    expect(total).toBeGreaterThan(1);

    await tagPill(page).click();
    await expect(page).toHaveURL(TAG_URL_RE);
    await expect(tagPill(page)).toHaveAttribute("aria-pressed", "true");
    await expectOnlyTagged(page);
    expect(await rows(page).locator("visible=true").count()).toBeLessThan(
      total,
    );

    await tagPill(page).click();
    await expect(page).toHaveURL(BLOG_URL_RE);
    await expect(tagPill(page)).toHaveAttribute("aria-pressed", "false");
    await expectAllVisible(page, total);

    await page.goBack();
    await expect(page).toHaveURL(TAG_URL_RE);
    await expect(tagPill(page)).toHaveAttribute("aria-pressed", "true");
    await expectOnlyTagged(page);

    await page.goBack();
    await expect(page).toHaveURL(BLOG_URL_RE);
    await expectAllVisible(page, total);

    await page.goForward();
    await expect(page).toHaveURL(TAG_URL_RE);
    await expectOnlyTagged(page);
  });

  test("a direct load of a filtered URL can be cleared", async ({ page }) => {
    await page.goto("/blog");
    const total = await rows(page).count();

    await page.goto(`/blog?tag=${TAG}`);
    await expect(tagPill(page)).toHaveAttribute("aria-pressed", "true");
    await expectOnlyTagged(page);

    await tagPill(page).click();
    await expect(page).toHaveURL(BLOG_URL_RE);
    await expectAllVisible(page, total);
  });
});

test.describe("Blog tag filter without JavaScript", () => {
  test.use({ javaScriptEnabled: false });

  test("filter links reload the page filtered and unfiltered", async ({
    page,
  }) => {
    await page.goto("/blog");
    const total = await rows(page).locator("visible=true").count();

    await tagPill(page).click();
    await expect(page).toHaveURL(TAG_URL_RE);
    await expectOnlyTagged(page);

    await tagPill(page).click();
    await expect(page).toHaveURL(BLOG_URL_RE);
    await expectAllVisible(page, total);
  });
});
