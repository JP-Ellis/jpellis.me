// Refreshes the fallback JSON data baked into the site.
//
// Writes two files from live GitHub API data, reusing the exact functions the
// running site calls so this script doubles as an end-to-end check of them:
//   - app/data/github-stats-fallback.json     (via fetchGithubStats)
//   - app/data/github-projects-fallback.json  (via fetchProjectStats)
//
// Run from the repository root:
//
//   mise run data:update
//   # or: aubx tsx scripts/update-fallback-data.ts
//
// Requires a GitHub token: GITHUB_TOKEN / GH_TOKEN, or an authenticated `gh`
// CLI (the script falls back to `gh auth token`).

import { execFileSync } from "node:child_process";
import { writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { PROJECTS } from "../app/data/projects.ts";
import { fetchProjectStats } from "../app/lib/github-projects.ts";
import { fetchGithubStats } from "../app/lib/github-stats.ts";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const STATS_PATH = join(ROOT, "app/data/github-stats-fallback.json");
const PROJECTS_PATH = join(ROOT, "app/data/github-projects-fallback.json");

function resolveToken(): string {
  const fromEnv = process.env.GITHUB_TOKEN ?? process.env.GH_TOKEN;
  if (fromEnv) {
    return fromEnv;
  }
  try {
    return execFileSync("gh", ["auth", "token"], { encoding: "utf8" }).trim();
  } catch (error) {
    throw new Error(
      "No GITHUB_TOKEN/GH_TOKEN set and `gh auth token` failed. " +
        "Export a token or run `gh auth login`.",
      { cause: error },
    );
  }
}

async function writeJson(path: string, value: unknown): Promise<void> {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function updateStats(token: string): Promise<void> {
  console.error("Fetching GitHub stats via fetchGithubStats…");
  const stats = await fetchGithubStats(token);

  // The fallback only stores the contribution counts and recent activity; the
  // contribution grid is regenerated synthetically by fallbackGithubStats().
  await writeJson(STATS_PATH, {
    commitContributions: stats.commitContributions,
    prContributions: stats.prContributions,
    issueContributions: stats.issueContributions,
    publicRepos: stats.publicRepos,
    recentActivity: stats.recentActivity,
  });

  console.error(
    `  → ${stats.recentActivity.length} activity items; ` +
      `${stats.commitContributions}/${stats.prContributions}/${stats.issueContributions} ` +
      `commit/PR/issue contributions; ${stats.publicRepos} public repos`,
  );
}

async function updateProjects(token: string): Promise<void> {
  const slugs = PROJECTS.flatMap((project) =>
    project.link?.kind === "github" ? [project.link.slug] : [],
  );

  console.error(
    `Fetching project stats via fetchProjectStats for ${slugs.length} repos…`,
  );
  const { repos } = await fetchProjectStats(token, slugs);
  const bySlug = new Map(repos.map((repo) => [repo.slug, repo]));

  // Keep the minimal slug/stars/forks shape, in the declared project order.
  // The richer per-repo detail is fetched live on project pages; the fallback
  // only needs what the home "year in code" band renders offline.
  const fallback = slugs
    .map((slug) => bySlug.get(slug))
    .filter((repo) => repo !== undefined)
    .map((repo) => ({ slug: repo.slug, stars: repo.stars, forks: repo.forks }));

  await writeJson(PROJECTS_PATH, fallback);

  const missing = slugs.filter((slug) => !bySlug.has(slug));
  if (missing.length > 0) {
    console.error(`  warning: no data for ${missing.join(", ")}`);
  }
  console.error(`  → ${fallback.length}/${slugs.length} repos`);
}

async function main(): Promise<void> {
  const token = resolveToken();
  await updateStats(token);
  await updateProjects(token);
}

main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});
