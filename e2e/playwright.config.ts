/** biome-ignore-all lint/style/useNamingConvention: configuration uses camelCase */
/** biome-ignore-all lint/style/noMagicNumbers: configuration file */
/** biome-ignore-all lint/nursery/noTernary: more succinct for configuration */
/** biome-ignore-all lint/style/noDefaultExport: Playwright requires a default export */
/** biome-ignore-all lint/nursery/noRedundantDefaultExport: Playwright requires a default export */
/** biome-ignore-all lint/style/noProcessEnv: process.env is idiomatic for CI detection */
/** biome-ignore-all lint/complexity/useLiteralKeys: bracket notation for env keys is conventional */

import process from "node:process";
import type { PlaywrightTestConfig } from "@playwright/test";
import { devices } from "@playwright/test";

const IS_CI: boolean = Boolean(process.env["CI"]);

export const config: PlaywrightTestConfig = {
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: IS_CI,
  retries: IS_CI ? 2 : 0,
  workers: IS_CI ? 1 : "100%",
  reporter: IS_CI ? "github" : "html",
  use: {
    baseURL: "http://localhost:8787/",
    trace: "on-first-retry",
  },
  projects: [
    { name: "Desktop Chrome", use: devices["Desktop Chrome"] },
    { name: "Desktop Firefox", use: devices["Desktop Firefox"] },
    { name: "Desktop Safari", use: devices["Desktop Safari"] },
    { name: "Mobile Chrome", use: devices["Pixel 5"] },
    { name: "Mobile Safari", use: devices["iPhone 12"] },
    { name: "Microsoft Edge", use: { channel: "msedge" } },
    { name: "Google Chrome", use: { channel: "chrome" } },
  ].flatMap(({ name, use }) =>
    (["light", "dark"] as const).map((colorScheme) => ({
      name: `${name} (${colorScheme})`,
      use: { ...use, colorScheme },
    })),
  ),
  webServer: {
    command: "mise run watch",
    url: "http://localhost:8787/",
    reuseExistingServer: !IS_CI,
  },
};

export default config;
