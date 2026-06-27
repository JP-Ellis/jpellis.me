// biome-ignore-all lint/style/noExcessiveLinesPerFile: cohesive GitHub API module: types, fetch, parse, and fallback belong together
// biome-ignore-all lint/style/useExportsLast: types are declared and exported up-front so the rest of the module can reference them; relocating them below the functions hurts readability

import fallbackJson from "../data/github-projects-fallback.json" with {
  type: "json",
};

// MARK: Types

export interface ReleaseInfo {
  tag: string;
  date: string;
  url: string;
}

export interface CommitInfo {
  sha: string;
  message: string;
  date: string;
  author: string;
  url: string;
}

export interface RepoStats {
  slug: string;
  stars: number;
  forks: number;
  openIssues: number;
  watchers: number;
  latestRelease: ReleaseInfo | null;
  recentCommits: CommitInfo[];
  openPrs: number;
}

export interface ProjectsStats {
  fetchedAt: string;
  repos: RepoStats[];
}

// MARK: Fallback JSON type

interface FallbackRepo {
  slug: string;
  stars: number;
  forks: number;
  openIssues?: number;
  watchers?: number;
  latestRelease?: {
    tag: string;
    date: string;
    url: string;
  } | null;
  recentCommits?: Array<{
    sha: string;
    message: string;
    date: string;
    author: string;
    url: string;
  }>;
  openPrs?: number;
}

// MARK: Constants

/** Length of the abbreviated commit SHA shown in the UI. */
const SHORT_SHA_LENGTH = 7;
/** Maximum commit-message length before truncation with an ellipsis. */
const MAX_MESSAGE_LENGTH = 72;
/** Cap on the number of human commits surfaced per repo. */
const MAX_COMMITS_PER_REPO = 5;
/** HTTP status returned when a repo has no published release. */
const HTTP_NOT_FOUND = 404;

// MARK: Pure parsers

/**
 * Parses stars, forks, open_issues, and watchers from a GitHub REST repo response.
 * Mirrors parse_repo_response in fetch.rs — uses watchers_count (not subscribers_count).
 *
 * @throws when any required numeric field is missing or not a number.
 */
export function parseRepoResponse(
  slug: string,
  body: Record<string, unknown>,
): { stars: number; forks: number; openIssues: number; watchers: number } {
  const stars = body.stargazers_count;
  if (typeof stars !== "number") {
    throw new Error(`${slug}: stargazers_count missing`);
  }
  const forks = body.forks_count;
  if (typeof forks !== "number") {
    throw new Error(`${slug}: forks_count missing`);
  }
  const openIssues = body.open_issues_count;
  if (typeof openIssues !== "number") {
    throw new Error(`${slug}: open_issues_count missing`);
  }
  const watchers = body.watchers_count;
  if (typeof watchers !== "number") {
    throw new Error(`${slug}: watchers_count missing`);
  }
  return { stars, forks, openIssues, watchers };
}

/**
 * Parses a GitHub releases/latest response into ReleaseInfo.
 * Returns null if any required field is absent (including 404-style responses).
 * Maps: tag_name→tag, published_at→date, html_url→url.
 */
export function parseReleaseResponse(
  body: Record<string, unknown>,
): ReleaseInfo | null {
  const tag = body.tag_name;
  const date = body.published_at;
  const url = body.html_url;
  if (
    typeof tag !== "string" ||
    typeof date !== "string" ||
    typeof url !== "string"
  ) {
    return null;
  }
  return { tag, date, url };
}

/**
 * Parses a single commit from the GitHub commits list API.
 *
 * Bot-detection rule (mirrors parse_commit in fetch.rs):
 *   author object is present AND author.type === "Bot" → null (bot)
 *   author is null/absent → treat as human (not linked to a GitHub account)
 *
 * SHA: first 7 characters.
 * Message: first line only, truncated at 72 chars with "…" appended if longer.
 * Author: GitHub login for linked accounts; falls back to commit.author.name.
 */
export function parseCommit(
  commit: Record<string, unknown>,
): CommitInfo | null {
  // Bot detection: author object present AND type is "Bot"
  const authorObj = commit.author;
  if (
    authorObj !== null &&
    typeof authorObj === "object" &&
    (authorObj as Record<string, unknown>).type === "Bot"
  ) {
    return null;
  }

  const fullSha = commit.sha;
  if (typeof fullSha !== "string") {
    return null;
  }
  const sha = [...fullSha].slice(0, SHORT_SHA_LENGTH).join("");

  const commitData = commit.commit as Record<string, unknown> | undefined;
  const fullMessage =
    typeof commitData?.message === "string" ? commitData.message : "";
  const firstLine = (fullMessage.split("\n")[0] ?? "").trim();
  const chars = [...firstLine];
  const message =
    chars.length > MAX_MESSAGE_LENGTH
      ? `${chars.slice(0, MAX_MESSAGE_LENGTH).join("")}…`
      : firstLine;

  const commitAuthor = commitData?.author as
    | Record<string, unknown>
    | undefined;
  const date = typeof commitAuthor?.date === "string" ? commitAuthor.date : "";

  // For linked GitHub accounts use the login; for null author fall back to git name
  let author: string;
  if (
    authorObj !== null &&
    typeof authorObj === "object" &&
    typeof (authorObj as Record<string, unknown>).login === "string"
  ) {
    author = (authorObj as Record<string, unknown>).login as string;
  } else {
    author =
      typeof commitAuthor?.name === "string" ? commitAuthor.name : "unknown";
  }

  const url = typeof commit.html_url === "string" ? commit.html_url : "";

  return { sha, message, date, author, url };
}

/**
 * Parses a GitHub commits list response, filtering bots and capping at 5.
 */
export function parseCommitsResponse(body: unknown[]): CommitInfo[] {
  const result: CommitInfo[] = [];
  for (const item of body) {
    if (result.length >= MAX_COMMITS_PER_REPO) {
      break;
    }
    const info = parseCommit(item as Record<string, unknown>);
    if (info !== null) {
      result.push(info);
    }
  }
  return result;
}

// MARK: Transport

async function getJson(url: string, token: string): Promise<unknown> {
  const resp = await fetch(url, {
    headers: {
      // biome-ignore lint/style/useNamingConvention: standard HTTP header name
      Authorization: `Bearer ${token}`,
      "User-Agent": "jpellis-me/1.0",
      // biome-ignore lint/style/useNamingConvention: standard HTTP header name
      Accept: "application/vnd.github+json",
    },
  });
  if (!resp.ok) {
    throw new Error(`GET ${url} returned ${resp.status}`);
  }
  return resp.json();
}

async function fetchLatestRelease(
  token: string,
  slug: string,
): Promise<ReleaseInfo | null> {
  const url = `https://api.github.com/repos/${slug}/releases/latest`;
  const resp = await fetch(url, {
    headers: {
      // biome-ignore lint/style/useNamingConvention: standard HTTP header name
      Authorization: `Bearer ${token}`,
      "User-Agent": "jpellis-me/1.0",
      // biome-ignore lint/style/useNamingConvention: standard HTTP header name
      Accept: "application/vnd.github+json",
    },
  });
  // 404 is expected for repos with no releases
  if (resp.status === HTTP_NOT_FOUND) {
    return null;
  }
  if (!resp.ok) {
    return null;
  }
  const body = (await resp.json()) as Record<string, unknown>;
  return parseReleaseResponse(body);
}

async function fetchRecentCommits(
  token: string,
  slug: string,
): Promise<CommitInfo[]> {
  // Fetch 20 to provide a bot-filtering buffer; cap result at 5 human commits
  const url = `https://api.github.com/repos/${slug}/commits?per_page=20`;
  try {
    const body = (await getJson(url, token)) as unknown[];
    return parseCommitsResponse(body);
  } catch (e) {
    // biome-ignore lint/suspicious/noConsole: server-side fetch fallback logging
    console.warn(`github-projects: fetch commits ${slug}: ${String(e)}`);
    return [];
  }
}

async function fetchOpenPrsCount(token: string, slug: string): Promise<number> {
  const url = `https://api.github.com/repos/${slug}/pulls?state=open&per_page=100`;
  try {
    const body = (await getJson(url, token)) as unknown[];
    return Array.isArray(body) ? body.length : 0;
  } catch (e) {
    // biome-ignore lint/suspicious/noConsole: server-side fetch fallback logging
    console.warn(`github-projects: fetch prs ${slug}: ${String(e)}`);
    return 0;
  }
}

async function fetchSingleRepo(
  token: string,
  slug: string,
): Promise<RepoStats> {
  const repoUrl = `https://api.github.com/repos/${slug}`;
  const repoBody = (await getJson(repoUrl, token)) as Record<string, unknown>;
  const { stars, forks, openIssues, watchers } = parseRepoResponse(
    slug,
    repoBody,
  );

  // Fetch activity concurrently after we know the repo exists
  const [latestRelease, recentCommits, openPrs] = await Promise.all([
    fetchLatestRelease(token, slug),
    fetchRecentCommits(token, slug),
    fetchOpenPrsCount(token, slug),
  ]);

  return {
    slug,
    stars,
    forks,
    openIssues,
    watchers,
    latestRelease,
    recentCommits,
    openPrs,
  };
}

// MARK: Exports

/**
 * Fetches stats and activity for the given repository slugs concurrently.
 * Repos that fail to fetch are logged and skipped (one failure doesn't kill all).
 */
export async function fetchProjectStats(
  token: string,
  slugs: string[],
): Promise<ProjectsStats> {
  const results = await Promise.allSettled(
    slugs.map((slug) => fetchSingleRepo(token, slug)),
  );

  const repos: RepoStats[] = [];
  for (const result of results) {
    if (result.status === "fulfilled") {
      repos.push(result.value);
    } else {
      // biome-ignore lint/suspicious/noConsole: server-side fetch fallback logging
      console.warn(`github-projects: fetch: ${String(result.reason)}`);
    }
  }

  return {
    fetchedAt: new Date().toISOString(),
    repos,
  };
}

/**
 * Returns placeholder ProjectsStats for use when the GitHub API is unavailable.
 * Mirrors fallback_projects_stats() from defaults.rs.
 */
export function fallbackProjectStats(): ProjectsStats {
  const repos: RepoStats[] = (fallbackJson as FallbackRepo[]).map((r) => ({
    slug: r.slug,
    stars: r.stars,
    forks: r.forks,
    openIssues: r.openIssues ?? 0,
    watchers: r.watchers ?? 0,
    latestRelease: r.latestRelease ?? null,
    recentCommits: r.recentCommits ?? [],
    openPrs: r.openPrs ?? 0,
  }));

  return {
    fetchedAt: new Date().toISOString(),
    repos,
  };
}
