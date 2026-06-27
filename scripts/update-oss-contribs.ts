// Regenerates app/data/oss-contribs.ts from live GitHub contribution data.
//
// Run from the repository root:
//
//   mise run oss-contribs:update
//   # or: aubx tsx scripts/update-oss-contribs.ts
//
// Requires a GitHub token: GITHUB_TOKEN / GH_TOKEN, or an authenticated `gh`
// CLI (the script falls back to `gh auth token`).
//
// It:
//   1. Fetches your commit + PR contributions for the past year (GraphQL).
//   2. Drops own/work-org repos, repos already in PROJECTS, private repos,
//      and repos with no PRs.
//   3. Fetches star counts and sorts by popularity (then slug).
//   4. Preserves any custom display names from the current file.
//   5. Rewrites app/data/oss-contribs.ts.

import { execFileSync } from "node:child_process";
import { writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { OSS_CONTRIBS } from "../app/data/oss-contribs.ts";
import { PROJECTS } from "../app/data/projects.ts";

// MARK: Configuration

const GITHUB_USER = "JP-Ellis";

// Orgs/owners excluded entirely (own repos and work orgs). Repos already in
// PROJECTS are excluded separately regardless of org.
const SKIP_ORGS = new Set(["JP-Ellis", "hep-rs", "pactflow"]);

// Minimum PRs to qualify for the list.
const MIN_PRS = 1;

const DAYS_PER_YEAR = 365;
const MS_PER_DAY = 86_400_000;
const THOUSANDS = 10_000;

const OUT_PATH = join(
  dirname(fileURLToPath(import.meta.url)),
  "..",
  "app/data/oss-contribs.ts",
);

interface RepoActivity {
  commits: number;
  prs: number;
  isPrivate: boolean;
}

// MARK: Token

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

// MARK: GitHub API

async function graphql(
  query: string,
  token: string,
): Promise<Record<string, unknown>> {
  const resp = await fetch("https://api.github.com/graphql", {
    method: "POST",
    headers: {
      // biome-ignore lint/style/useNamingConvention: standard HTTP header name
      Authorization: `Bearer ${token}`,
      "User-Agent": "jpellis-me/1.0",
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ query }),
  });
  if (!resp.ok) {
    throw new Error(`GraphQL status ${resp.status}`);
  }
  const body = (await resp.json()) as Record<string, unknown>;
  return (body.data ?? {}) as Record<string, unknown>;
}

function accumulate(
  activity: Map<string, RepoActivity>,
  list: unknown,
  field: "commits" | "prs",
): void {
  if (!Array.isArray(list)) {
    return;
  }
  for (const raw of list) {
    const item = raw as Record<string, unknown>;
    const repo = item.repository as Record<string, unknown> | undefined;
    const slug = repo?.nameWithOwner;
    if (typeof slug !== "string" || slug === "") {
      // biome-ignore lint/style/noContinue: guard-clause skip for malformed items
      continue;
    }
    const isPrivate = repo?.isPrivate === true;
    const contributions = item.contributions as
      | Record<string, unknown>
      | undefined;
    const count =
      typeof contributions?.totalCount === "number"
        ? contributions.totalCount
        : 0;
    const entry = activity.get(slug) ?? { commits: 0, prs: 0, isPrivate };
    entry[field] += count;
    entry.isPrivate = isPrivate;
    activity.set(slug, entry);
  }
}

async function fetchContributions(
  token: string,
  from: string,
  to: string,
): Promise<Map<string, RepoActivity>> {
  const query = `{
  user(login: "${GITHUB_USER}") {
    contributionsCollection(from: "${from}", to: "${to}") {
      commitContributionsByRepository(maxRepositories: 100) {
        repository { nameWithOwner isPrivate }
        contributions { totalCount }
      }
      pullRequestContributionsByRepository(maxRepositories: 100) {
        repository { nameWithOwner isPrivate }
        contributions { totalCount }
      }
    }
  }
}`;

  const data = await graphql(query, token);
  const user = data.user as Record<string, unknown> | undefined;
  const collection = user?.contributionsCollection as
    | Record<string, unknown>
    | undefined;

  const activity = new Map<string, RepoActivity>();
  accumulate(activity, collection?.commitContributionsByRepository, "commits");
  accumulate(activity, collection?.pullRequestContributionsByRepository, "prs");
  return activity;
}

// Fetches star counts for all slugs in a single aliased GraphQL query.
async function fetchStars(
  token: string,
  slugs: string[],
): Promise<Map<string, number>> {
  if (slugs.length === 0) {
    return new Map();
  }

  const fields = slugs
    .map((slug, i) => {
      const [owner, name] = slug.split("/");
      return owner && name
        ? `  r${i}: repository(owner: "${owner}", name: "${name}") { stargazerCount }`
        : "";
    })
    .filter((field) => field !== "")
    .join("\n");
  const data = await graphql(`{\n${fields}\n}`, token);

  const stars = new Map<string, number>();
  slugs.forEach((slug, i) => {
    const node = data[`r${i}`] as Record<string, unknown> | undefined;
    const count = node?.stargazerCount;
    if (typeof count === "number") {
      stars.set(slug, count);
    }
  });
  return stars;
}

// MARK: Output

function numberLiteral(n: number): string {
  // Match the repo style: group thousands for 5+ digit numbers.
  return n < THOUSANDS
    ? String(n)
    : n.toLocaleString("en-US").replace(/,/gu, "_");
}

function renderFile(
  entries: { slug: string; name: string; stars: number }[],
): string {
  const lines = entries.map(
    (e) =>
      `  { slug: ${JSON.stringify(e.slug)}, name: ${JSON.stringify(e.name)}, stars: ${numberLiteral(e.stars)} },`,
  );
  return `export const OSS_CONTRIBS: { slug: string; name: string; stars: number }[] = [\n${lines.join("\n")}\n];\n`;
}

// MARK: Main

async function main(): Promise<void> {
  const token = resolveToken();

  const tracked = new Set(
    PROJECTS.flatMap((project) =>
      project.link?.kind === "github" ? [project.link.slug] : [],
    ),
  );
  const existingNames = new Map(OSS_CONTRIBS.map((c) => [c.slug, c.name]));

  const now = Date.now();
  const from = new Date(now - DAYS_PER_YEAR * MS_PER_DAY).toISOString();
  const to = new Date(now).toISOString();

  console.error(`Fetching contributions ${from} → ${to} for @${GITHUB_USER}…`);
  const activity = await fetchContributions(token, from, to);

  const candidates = [...activity.entries()]
    .filter(([slug, data]) => {
      const org = slug.split("/")[0] ?? "";
      return (
        !(SKIP_ORGS.has(org) || tracked.has(slug) || data.isPrivate) &&
        data.prs >= MIN_PRS
      );
    })
    .map(([slug]) => slug);

  console.error(`Fetching star counts for ${candidates.length} repos…`);
  const stars = await fetchStars(token, candidates);

  candidates.sort((a, b) => {
    const byStars = (stars.get(b) ?? 0) - (stars.get(a) ?? 0);
    return byStars === 0 ? a.localeCompare(b) : byStars;
  });

  const entries = candidates.map((slug) => ({
    slug,
    name: existingNames.get(slug) ?? slug.split("/")[1] ?? slug,
    stars: stars.get(slug) ?? 0,
  }));

  await writeFile(OUT_PATH, renderFile(entries));

  console.error(
    `Updated app/data/oss-contribs.ts — ${entries.length} entries:`,
  );
  for (const entry of entries) {
    const isNew = existingNames.has(entry.slug) ? "" : "  [new]";
    console.error(`  ${entry.slug} (${entry.stars}★)${isNew}`);
  }
}

main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});
