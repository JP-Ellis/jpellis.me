import process from "node:process";
import type { PlaywrightTestConfig } from "@playwright/test";
import { devices } from "@playwright/test";

const IS_CI: boolean = Boolean(process.env["CI"]);
// E2E_PORT lets parallel checkouts each run their own Worker.
const PORT: string = process.env["E2E_PORT"] ?? "8787";
const BASE_URL = `http://localhost:${PORT}/`;

const browsers = [
  { name: "Desktop Chrome", use: devices["Desktop Chrome"] },
  { name: "Desktop Firefox", use: devices["Desktop Firefox"] },
  { name: "Desktop Safari", use: devices["Desktop Safari"] },
  { name: "Mobile Chrome", use: devices["Pixel 5"] },
  { name: "Mobile Safari", use: devices["iPhone 12"] },
];

const ciBrowsers = new Set(["Desktop Chrome", "Mobile Chrome"]);
const selectedBrowsers = IS_CI
  ? browsers.filter((browser) => ciBrowsers.has(browser.name))
  : browsers;

export const config: PlaywrightTestConfig = {
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: IS_CI,
  retries: IS_CI ? 2 : 0,
  workers: IS_CI ? 1 : "100%",
  reporter: IS_CI ? "github" : "html",
  use: {
    // biome-ignore lint/style/useNamingConvention: Playwright config key
    baseURL: BASE_URL,
    trace: "on-first-retry",
  },
  projects: selectedBrowsers.flatMap(({ name, use }) =>
    (["light", "dark"] as const).map((colorScheme) => ({
      name: `${name} (${colorScheme})`,
      use: { ...use, colorScheme },
    })),
  ),
  webServer: {
    command: `cd .. && aube run build && aubx wrangler dev --config dist/server/wrangler.json --port ${PORT}`,
    url: BASE_URL,
    reuseExistingServer: !IS_CI,
    timeout: 120_000,
  },
};

export default config;
