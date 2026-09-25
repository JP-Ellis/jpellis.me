import { expect, test } from "@playwright/test";

const SITE = "https://jpellis.me";
// A post that is not a cross-post, so its canonical URL is on this site.
const POST = "/blog/astro-rewrite";
const CROSS_POST = "/blog/functional-arguments";
const PAGE_TITLE_RE = /^\S.* · Joshua Ellis$/u;

const PAGES = [
  { path: "/", title: /^Joshua Ellis$/u },
  { path: "/projects", title: /^Projects · Joshua Ellis$/u },
  { path: "/projects/pact-python", title: PAGE_TITLE_RE },
  { path: "/resume", title: /^Résumé · Joshua Ellis$/u },
  { path: "/blog", title: /^Blog · Joshua Ellis$/u },
  { path: POST, title: PAGE_TITLE_RE },
  { path: CROSS_POST, title: PAGE_TITLE_RE },
  { path: "/contact", title: /^Contact · Joshua Ellis$/u },
];

// On-demand pages and their Cache-Control values (see app/lib/cache-control.ts).
const CACHED = [
  { path: "/", maxAge: 900 },
  { path: "/blog", maxAge: 300 },
  { path: "/projects", maxAge: 3600 },
  { path: "/projects/pact-python", maxAge: 3600 },
];

test.describe("Route smoke tests", () => {
  for (const { path, title } of PAGES) {
    test(`${path} renders`, async ({ page }) => {
      const response = await page.goto(path);
      expect(response?.status()).toBe(200);
      await expect(page).toHaveTitle(title);
      await expect(page.locator("main h1")).toHaveCount(1);
      await expect(page.locator("main h1")).toBeVisible();
    });

    test(`${path} has head metadata`, async ({ page }) => {
      await page.goto(path);
      const head = page.locator("head");
      await expect(head.locator("meta[name='description']")).toHaveAttribute(
        "content",
        /\S/u,
      );
      if (path !== CROSS_POST) {
        await expect(head.locator("link[rel='canonical']")).toHaveAttribute(
          "href",
          `${SITE}${path}`,
        );
      }
      await expect(head.locator("meta[property='og:title']")).toHaveCount(1);
      await expect(head.locator("meta[name='twitter:card']")).toHaveCount(1);
      await expect(head.locator("meta[name='color-scheme']")).toHaveAttribute(
        "content",
        "light dark",
      );
    });

    test(`${path} has no horizontal overflow at 320px`, async ({ page }) => {
      await page.setViewportSize({ width: 320, height: 800 });
      await page.goto(path);
      const overflows = await page.evaluate(
        () => document.documentElement.scrollWidth > window.innerWidth,
      );
      expect(overflows).toBe(false);
    });
  }

  test("canonical URLs drop the query string", async ({ page }) => {
    await page.goto("/blog?tag=pact");
    await expect(page.locator("link[rel='canonical']")).toHaveAttribute(
      "href",
      `${SITE}/blog`,
    );
  });

  test("cross-posts point their canonical URL at the original", async ({
    page,
  }) => {
    await page.goto(CROSS_POST);
    await expect(page.locator("link[rel='canonical']")).toHaveAttribute(
      "href",
      /^https:\/\/pact-foundation\.github\.io\//u,
    );
  });

  test("structured data describes the person and the posts", async ({
    page,
  }) => {
    const jsonLd = () =>
      page
        .locator("script[type='application/ld+json']")
        .textContent()
        .then((text) => JSON.parse(text ?? "{}"));

    await page.goto("/");
    expect(await jsonLd()).toMatchObject({
      "@type": "Person",
      name: "Joshua Ellis",
    });

    await page.goto(POST);
    expect(await jsonLd()).toMatchObject({
      "@type": "BlogPosting",
      headline: await page.locator("main h1").textContent(),
      url: `${SITE}${POST}`,
      author: { name: "Joshua Ellis" },
    });
  });
});

test.describe("Not found", () => {
  for (const path of ["/projects/no-such-project", "/no-such-page"]) {
    test(`${path} returns a 404 page`, async ({ page }) => {
      const response = await page.goto(path);
      expect(response?.status()).toBe(404);
      expect(response?.url()).toMatch(new RegExp(`${path}$`, "u"));
      await expect(page.locator("main h1")).toHaveText("Page not found.");
      await expect(page.locator("meta[name='robots']")).toHaveAttribute(
        "content",
        "noindex",
      );
      await expect(page.locator("link[rel='canonical']")).toHaveCount(0);
    });
  }
});

test.describe("Feeds", () => {
  test("RSS feed lists posts", async ({ request }) => {
    const response = await request.get("/rss.xml");
    expect(response.status()).toBe(200);
    expect(response.headers()["content-type"]).toContain("xml");
    const body = await response.text();
    expect(body).toContain("<rss");
    expect(body).toContain("<title>Joshua Ellis</title>");
    expect(body).toContain(`<link>${SITE}${POST}</link>`);
    // Posts with a front-matter description carry it into their item.
    expect(body).toMatch(/<item>.*<description>\S/su);
  });

  test("sitemap lists static and on-demand pages", async ({ request }) => {
    const index = await request.get("/sitemap-index.xml");
    expect(index.status()).toBe(200);
    expect(await index.text()).toContain(`${SITE}/sitemap-0.xml`);

    const sitemap = await (await request.get("/sitemap-0.xml")).text();
    for (const path of [
      "/",
      "/blog",
      "/projects",
      "/projects/pact-python",
      "/resume",
      "/contact",
      POST,
    ]) {
      expect(sitemap).toContain(`<loc>${SITE}${path}</loc>`);
    }
    expect(sitemap).not.toContain("404");
  });
});

test.describe("Cache-Control", () => {
  for (const { path, maxAge } of CACHED) {
    test(`${path} is cacheable`, async ({ request }) => {
      const response = await request.get(path);
      const header = response.headers()["cache-control"] ?? "";
      expect(header).toContain("public");
      expect(header).toContain(`max-age=${maxAge}`);
      expect(header).toContain("stale-while-revalidate=");
    });
  }
});
