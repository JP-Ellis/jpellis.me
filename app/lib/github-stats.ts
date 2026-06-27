// biome-ignore-all lint/style/noExcessiveLinesPerFile: cohesive GitHub API module: types, fetch, parse, and fallback belong together
// biome-ignore-all lint/style/useExportsLast: types are declared and exported up-front so the rest of the module can reference them; relocating them below the functions hurts readability
// biome-ignore-all lint/style/noContinue: guard-clause early-skip in parsing loops is clearer than nesting

import fallbackJson from "../data/github-stats-fallback.json" with {
  type: "json",
};

// MARK: Types

export interface ContributionDay {
  date: string;
  count: number;
}

export interface ContributionWeek {
  days: ContributionDay[];
}

export type ActivityKind = "commit" | "pull_request" | "issue";
export type ActivityState = "open" | "closed" | "merged" | null;

export interface ActivityItem {
  kind: ActivityKind;
  repo: string;
  title: string;
  url: string;
  state: ActivityState;
  createdAt: string;
}

export interface GitHubStats {
  fetchedAt: string;
  totalContributions: number;
  commitContributions: number;
  prContributions: number;
  issueContributions: number;
  publicRepos: number;
  periodFrom: string;
  periodTo: string;
  contributionWeeks: ContributionWeek[];
  recentActivity: ActivityItem[];
}

// MARK: Fallback JSON types

interface FallbackJson {
  commitContributions: number;
  prContributions: number;
  issueContributions: number;
  publicRepos: number;
  recentActivity: Array<{
    kind: string;
    repo: string;
    title: string;
    url: string;
    state: string | null;
    createdAt: string;
  }>;
}

const fallback = fallbackJson as FallbackJson;

// MARK: Constants

const MS_PER_DAY = 86_400_000;
const CONTRIBUTION_WINDOW_DAYS = 365;
const DAYS_PER_WEEK = 7;

/** Number of most-recent commits to surface in the activity feed. */
const MAX_RECENT_COMMITS = 6;
/** Number of most-recent PR/issue items to surface in the activity feed. */
const MAX_RECENT_ACTIVITY = 4;

// Parameters for the deterministic linear-congruential generator used to
// synthesise a plausible-looking fallback contribution grid.
const LCG_MULTIPLIER = 9301n;
const LCG_INCREMENT = 49297n;
const LCG_MODULUS = 233_280n;
const LCG_MODULUS_NUMBER = 233_280;

const FALLBACK_GRID_WEEKS = 53;
const ACTIVITY_BURST_START_WEEK = 32;
const ACTIVITY_BURST_END_WEEK = 44;
const ACTIVITY_INTENSITY_EXPONENT = 1.6;
const ACTIVITY_BURST_THRESHOLD = 0.4;
const ACTIVITY_BURST_BONUS = 0.3;
const MAX_INTENSITY = 1.0;
const MAX_CONTRIBUTION_COUNT = 10;

// MARK: GraphQL query

function buildGraphqlQuery(from: string, to: string): string {
  return `{
  user(login: "JP-Ellis") {
    contributionsCollection(from: "${from}", to: "${to}") {
      contributionCalendar {
        totalContributions
        weeks {
          contributionDays { date contributionCount }
        }
      }
      restrictedContributionsCount
      totalCommitContributions
      totalPullRequestContributions
      totalIssueContributions
    }
    repositories(privacy: PUBLIC) { totalCount }
  }
}`;
}

// MARK: Parser helpers

function parseContributionWeeks(weeksJson: unknown): ContributionWeek[] {
  if (!Array.isArray(weeksJson)) {
    throw new Error("weeks not an array");
  }
  return weeksJson.map((week: unknown) => {
    const w = week as Record<string, unknown>;
    const daysJson = w.contributionDays;
    if (!Array.isArray(daysJson)) {
      throw new Error("contributionDays not an array");
    }
    const days: ContributionDay[] = daysJson.map((day: unknown) => {
      const d = day as Record<string, unknown>;
      const { date } = d;
      if (typeof date !== "string") {
        throw new Error("date missing");
      }
      const count = d.contributionCount;
      if (typeof count !== "number") {
        throw new Error("contributionCount missing");
      }
      return { date, count };
    });
    return { days };
  });
}

function classifyActivityItem(i: Record<string, unknown>): {
  kind: ActivityKind;
  state: ActivityState;
} {
  if (!("pull_request" in i)) {
    const state: ActivityState = i.state === "open" ? "open" : "closed";
    return { kind: "issue", state };
  }
  const pr = i.pull_request as Record<string, unknown>;
  const merged = typeof pr.merged_at === "string";
  const rawState = typeof i.state === "string" ? i.state : "open";
  let state: ActivityState;
  if (rawState === "open") {
    state = "open";
  } else if (merged) {
    state = "merged";
  } else {
    state = "closed";
  }
  return { kind: "pull_request", state };
}

// MARK: Transport

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
  return resp.json() as Promise<Record<string, unknown>>;
}

async function restGet(
  url: string,
  token: string,
  extraHeaders?: Record<string, string>,
): Promise<unknown> {
  const resp = await fetch(url, {
    headers: {
      // biome-ignore lint/style/useNamingConvention: standard HTTP header name
      Authorization: `Bearer ${token}`,
      "User-Agent": "jpellis-me/1.0",
      ...extraHeaders,
    },
  });
  if (!resp.ok) {
    throw new Error(`REST status ${resp.status}`);
  }
  return resp.json();
}

async function fetchRecentCommits(
  token: string,
  limit: number,
): Promise<ActivityItem[]> {
  const perPage = limit * 2;
  const url = `https://api.github.com/search/commits?q=author:JP-Ellis+is:public&sort=author-date&order=desc&per_page=${perPage}`;
  const body = (await restGet(url, token, {
    // biome-ignore lint/style/useNamingConvention: standard HTTP header name
    Accept: "application/vnd.github.cloak-preview",
  })) as Record<string, unknown>;

  const { items } = body;
  if (!Array.isArray(items)) {
    throw new Error("commit search items missing");
  }

  const result: ActivityItem[] = [];
  for (const item of items) {
    const i = item as Record<string, unknown>;
    const htmlUrl = i.html_url;
    if (typeof htmlUrl !== "string") {
      continue;
    }

    const repository = i.repository as Record<string, unknown> | undefined;
    const repo = repository?.full_name;
    if (typeof repo !== "string") {
      continue;
    }

    const commit = i.commit as Record<string, unknown> | undefined;
    const message = (commit?.message as string | undefined) ?? "";
    const title = message.split("\n")[0] ?? "";
    if (!title) {
      continue;
    }

    const author = commit?.author as Record<string, unknown> | undefined;
    const dateStr = author?.date;
    if (typeof dateStr !== "string") {
      continue;
    }

    result.push({
      kind: "commit",
      repo,
      title,
      url: htmlUrl,
      state: null,
      createdAt: new Date(dateStr).toISOString(),
    });
  }

  return result;
}

async function fetchRecentActivity(
  token: string,
  limit: number,
): Promise<ActivityItem[]> {
  const prUrl = `https://api.github.com/search/issues?q=author:JP-Ellis+is:pull-request+is:public&sort=created&order=desc&per_page=${limit}`;
  const issueUrl = `https://api.github.com/search/issues?q=author:JP-Ellis+is:issue+is:public&sort=created&order=desc&per_page=${limit}`;

  const [prBody, issueBody] = await Promise.all([
    restGet(prUrl, token),
    restGet(issueUrl, token),
  ]);

  const prs = parseActivityItems(prBody);
  const issues = parseActivityItems(issueBody);

  const combined = [...prs, ...issues];
  combined.sort(
    (a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime(),
  );
  return combined.slice(0, limit);
}

// MARK: Contribution window

export function contributionWindow(now: Date): { from: Date; to: Date } {
  const from = new Date(now.getTime() - CONTRIBUTION_WINDOW_DAYS * MS_PER_DAY);
  return { from, to: now };
}

// MARK: Fallback grid

function fallbackGrid(startDate: Date): ContributionWeek[] {
  let s = 11n;
  const lcg = (): number => {
    s = (s * LCG_MULTIPLIER + LCG_INCREMENT) % LCG_MODULUS;
    return Number(s) / LCG_MODULUS_NUMBER;
  };

  const weeks: ContributionWeek[] = [];
  for (let w = 0; w < FALLBACK_GRID_WEEKS; w += 1) {
    const days: ContributionDay[] = [];
    for (let d = 0; d < DAYS_PER_WEEK; d += 1) {
      const v = lcg() ** ACTIVITY_INTENSITY_EXPONENT;
      const burst =
        w > ACTIVITY_BURST_START_WEEK &&
        w < ACTIVITY_BURST_END_WEEK &&
        lcg() > ACTIVITY_BURST_THRESHOLD
          ? ACTIVITY_BURST_BONUS
          : 0;
      const raw = Math.trunc(
        Math.min(v + burst, MAX_INTENSITY) * MAX_CONTRIBUTION_COUNT,
      );
      const offsetMs = (w * DAYS_PER_WEEK + d) * MS_PER_DAY;
      const date = new Date(startDate.getTime() + offsetMs)
        .toISOString()
        .slice(0, 10);
      days.push({ date, count: raw });
    }
    weeks.push({ days });
  }
  return weeks;
}

// MARK: Exports

export function parseContributionTotals(contributions: unknown): {
  total: number;
  commits: number;
  prs: number;
  issues: number;
} {
  const c = contributions as Record<string, unknown>;
  const calendar = c.contributionCalendar as
    | Record<string, unknown>
    | undefined;
  const calendarTotal = calendar?.totalContributions;
  if (typeof calendarTotal !== "number") {
    throw new Error("totalContributions missing");
  }
  const restricted = c.restrictedContributionsCount;
  if (typeof restricted !== "number") {
    throw new Error("restrictedContributionsCount missing");
  }
  const commits = c.totalCommitContributions;
  if (typeof commits !== "number") {
    throw new Error("totalCommitContributions missing");
  }
  const prs = c.totalPullRequestContributions;
  if (typeof prs !== "number") {
    throw new Error("totalPullRequestContributions missing");
  }
  const issues = c.totalIssueContributions;
  if (typeof issues !== "number") {
    throw new Error("totalIssueContributions missing");
  }
  return {
    total: calendarTotal + restricted,
    commits,
    prs,
    issues,
  };
}

export function parseActivityItems(body: unknown): ActivityItem[] {
  const b = body as Record<string, unknown>;
  const { items } = b;
  if (!Array.isArray(items)) {
    throw new Error("issue search items missing");
  }

  const result: ActivityItem[] = [];
  for (const item of items) {
    const i = item as Record<string, unknown>;
    const {
      title,
      html_url: htmlUrl,
      repository_url: repositoryUrl,
      created_at: createdAt,
    } = i;

    if (
      typeof title !== "string" ||
      typeof htmlUrl !== "string" ||
      typeof repositoryUrl !== "string" ||
      typeof createdAt !== "string"
    ) {
      continue;
    }

    const repo = repositoryUrl.replace("https://api.github.com/repos/", "");
    const { kind, state } = classifyActivityItem(i);

    result.push({
      kind,
      repo,
      title,
      url: htmlUrl,
      state,
      createdAt,
    });
  }

  return result;
}

export async function fetchGithubStats(token: string): Promise<GitHubStats> {
  const now = new Date();
  const { from: windowStart } = contributionWindow(now);
  const periodFrom = windowStart.toISOString().slice(0, 10);
  const periodTo = now.toISOString().slice(0, 10);

  const from = `${periodFrom}T00:00:00Z`;
  const to = `${periodTo}T23:59:59Z`;

  const query = buildGraphqlQuery(from, to);
  const gql = await graphql(query, token);

  const data = gql.data as Record<string, unknown>;
  const user = data.user as Record<string, unknown>;
  const contributions = user.contributionsCollection;
  const { total, commits, prs, issues } =
    parseContributionTotals(contributions);

  const repositories = user.repositories as Record<string, unknown>;
  const publicRepos = repositories.totalCount;
  if (typeof publicRepos !== "number") {
    throw new Error("totalCount missing");
  }

  const calendar = (contributions as Record<string, unknown>)
    .contributionCalendar as Record<string, unknown>;
  const contributionWeeks = parseContributionWeeks(calendar.weeks);

  const recentCommits = await fetchRecentCommits(token, MAX_RECENT_COMMITS);
  const otherActivity = await fetchRecentActivity(token, MAX_RECENT_ACTIVITY);

  const recentActivity = [...recentCommits, ...otherActivity];
  recentActivity.sort(
    (a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime(),
  );

  return {
    fetchedAt: now.toISOString(),
    totalContributions: total,
    commitContributions: commits,
    prContributions: prs,
    issueContributions: issues,
    publicRepos,
    periodFrom,
    periodTo,
    contributionWeeks,
    recentActivity,
  };
}

export function fallbackGithubStats(): GitHubStats {
  const now = new Date();
  const todayStr = now.toISOString().slice(0, 10);

  const { from: windowStart } = contributionWindow(now);
  const periodFrom = windowStart.toISOString().slice(0, 10);

  const contributionWeeks = fallbackGrid(windowStart);
  const totalContributions = contributionWeeks
    .flatMap((w) => w.days)
    .reduce((sum, d) => sum + d.count, 0);

  const recentActivity: ActivityItem[] = fallback.recentActivity.map((a) => ({
    kind: a.kind as ActivityKind,
    repo: a.repo,
    title: a.title,
    url: a.url,
    state: a.state as ActivityState,
    createdAt: a.createdAt,
  }));

  return {
    fetchedAt: now.toISOString(),
    totalContributions,
    commitContributions: fallback.commitContributions,
    prContributions: fallback.prContributions,
    issueContributions: fallback.issueContributions,
    publicRepos: fallback.publicRepos,
    periodFrom,
    periodTo: todayStr,
    contributionWeeks,
    recentActivity,
  };
}
