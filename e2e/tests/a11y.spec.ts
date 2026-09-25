import AxeBuilder from "@axe-core/playwright";
import { expect, type Page, test } from "@playwright/test";

const PAGES = [
  "/",
  "/projects",
  "/projects/pact-python",
  "/resume",
  "/blog",
  "/blog/asynchronous-message-support",
  "/blog/functional-arguments",
  "/contact",
  "/nope",
];

// Contrast findings accepted for now, each matched by a CSS selector:
// - accent text on the inverted band surface (≈4.1–4.3:1)
// - Shiki's github-light orange and github-dark comment colours
// - the light-theme warning colour on the "wip" project status
// Muted text (--color-muted) is below AA by design and is skipped on its own.
const KNOWN_CONTRAST = [
  ".band .eyebrow:not(.eyebrow--muted)",
  ".band .commit-repo",
  ".astro-code span",
  ".row_status_wip",
];

interface Finding {
  id: string;
  help: string;
  targets: string[];
}

/** True when the node is muted text or one of the known contrast findings. */
function isAcceptedContrast(page: Page, target: string): Promise<boolean> {
  return page
    .locator(target)
    .first()
    .evaluate((el, known) => {
      if (known.some((selector) => el.matches(selector))) {
        return true;
      }
      const probe = document.createElement("span");
      probe.style.color = "var(--color-muted)";
      el.append(probe);
      const muted = getComputedStyle(probe).color;
      probe.remove();
      return getComputedStyle(el).color === muted;
    }, KNOWN_CONTRAST);
}

async function findings(page: Page): Promise<Finding[]> {
  const results = await new AxeBuilder({ page }).analyze();
  const out: Finding[] = [];
  for (const violation of results.violations) {
    const targets: string[] = [];
    for (const node of violation.nodes) {
      const target = node.target.join(" ");
      if (
        violation.id === "color-contrast" &&
        (await isAcceptedContrast(page, target))
      ) {
        continue;
      }
      targets.push(target);
    }
    if (targets.length > 0) {
      out.push({ id: violation.id, help: violation.help, targets });
    }
  }
  return out;
}

test.describe("Accessibility (axe)", () => {
  for (const path of PAGES) {
    test(`${path} has no violations`, async ({ page }) => {
      await page.goto(path);
      expect(await findings(page)).toEqual([]);
    });
  }
});
