// biome-ignore-all lint/style/useNamingConvention: test fixtures mirror the GitHub API response shape
import { describe, expect, test } from "vitest";
import fallbackJson from "../data/github-stats-fallback.json" with {
  type: "json",
};
import {
  contributionWindow,
  fallbackGithubStats,
  parseActivityItems,
  parseContributionTotals,
} from "./github-stats.ts";

const DATE_RE = /^\d{4}-\d{2}-\d{2}$/u;

// MARK: parseContributionTotals

describe("parseContributionTotals", () => {
  test("sums calendar total and restricted contributions", () => {
    const contributions = {
      contributionCalendar: { totalContributions: 2133 },
      restrictedContributionsCount: 485,
      totalCommitContributions: 843,
      totalPullRequestContributions: 189,
      totalIssueContributions: 29,
    };

    const result = parseContributionTotals(contributions);

    expect(result.total).toBe(2618);
    expect(result.commits).toBe(843);
    expect(result.prs).toBe(189);
    expect(result.issues).toBe(29);
  });

  test("restricted contributions are ADDED to total (not ignored)", () => {
    const contributions = {
      contributionCalendar: { totalContributions: 100 },
      restrictedContributionsCount: 50,
      totalCommitContributions: 80,
      totalPullRequestContributions: 10,
      totalIssueContributions: 10,
    };

    const { total } = parseContributionTotals(contributions);

    expect(total).toBe(150);
    expect(total).toBeGreaterThan(100);
  });

  test("works when restricted count is zero", () => {
    const contributions = {
      contributionCalendar: { totalContributions: 500 },
      restrictedContributionsCount: 0,
      totalCommitContributions: 400,
      totalPullRequestContributions: 60,
      totalIssueContributions: 40,
    };

    const { total } = parseContributionTotals(contributions);

    expect(total).toBe(500);
  });

  test("throws on missing totalContributions", () => {
    const contributions = {
      contributionCalendar: {},
      restrictedContributionsCount: 0,
      totalCommitContributions: 0,
      totalPullRequestContributions: 0,
      totalIssueContributions: 0,
    };

    expect(() => parseContributionTotals(contributions)).toThrow(
      "totalContributions missing",
    );
  });

  test("throws on missing restrictedContributionsCount", () => {
    const contributions = {
      contributionCalendar: { totalContributions: 100 },
      totalCommitContributions: 0,
      totalPullRequestContributions: 0,
      totalIssueContributions: 0,
    };

    expect(() => parseContributionTotals(contributions)).toThrow(
      "restrictedContributionsCount missing",
    );
  });
});

// MARK: parseActivityItems

describe("parseActivityItems", () => {
  test("parses merged PR correctly", () => {
    const body = {
      items: [
        {
          title: "feat: merge me",
          html_url: "https://github.com/owner/repo/pull/1",
          repository_url: "https://api.github.com/repos/owner/repo",
          created_at: "2026-04-20T10:00:00Z",
          state: "closed",
          pull_request: { merged_at: "2026-04-21T10:00:00Z" },
        },
      ],
    };

    const items = parseActivityItems(body);

    expect(items).toHaveLength(1);
    expect(items[0].kind).toBe("pull_request");
    expect(items[0].state).toBe("merged");
    expect(items[0].title).toBe("feat: merge me");
    expect(items[0].repo).toBe("owner/repo");
    expect(items[0].url).toBe("https://github.com/owner/repo/pull/1");
  });

  test("parses open PR correctly", () => {
    const body = {
      items: [
        {
          title: "feat: wip",
          html_url: "https://github.com/owner/repo/pull/2",
          repository_url: "https://api.github.com/repos/owner/repo",
          created_at: "2026-04-18T08:00:00Z",
          state: "open",
          pull_request: { merged_at: null },
        },
      ],
    };

    const [item] = parseActivityItems(body);

    expect(item.kind).toBe("pull_request");
    expect(item.state).toBe("open");
  });

  test("parses closed (not merged) PR correctly", () => {
    const body = {
      items: [
        {
          title: "feat: abandoned",
          html_url: "https://github.com/owner/repo/pull/3",
          repository_url: "https://api.github.com/repos/owner/repo",
          created_at: "2026-04-17T08:00:00Z",
          state: "closed",
          pull_request: { merged_at: null },
        },
      ],
    };

    const [item] = parseActivityItems(body);

    expect(item.kind).toBe("pull_request");
    expect(item.state).toBe("closed");
  });

  test("parses open issue correctly (no pull_request field)", () => {
    const body = {
      items: [
        {
          title: "Bug: crash on startup",
          html_url: "https://github.com/owner/repo/issues/7",
          repository_url: "https://api.github.com/repos/owner/repo",
          created_at: "2026-03-10T12:00:00Z",
          state: "open",
        },
      ],
    };

    const [item] = parseActivityItems(body);

    expect(item.kind).toBe("issue");
    expect(item.state).toBe("open");
    expect(item.title).toBe("Bug: crash on startup");
  });

  test("parses closed issue correctly", () => {
    const body = {
      items: [
        {
          title: "Bug: fixed",
          html_url: "https://github.com/owner/repo/issues/8",
          repository_url: "https://api.github.com/repos/owner/repo",
          created_at: "2026-03-05T12:00:00Z",
          state: "closed",
        },
      ],
    };

    const [item] = parseActivityItems(body);

    expect(item.kind).toBe("issue");
    expect(item.state).toBe("closed");
  });

  test("strips api.github.com prefix from repo URL", () => {
    const body = {
      items: [
        {
          title: "feat: something",
          html_url: "https://github.com/JP-Ellis/myrepo/pull/1",
          repository_url: "https://api.github.com/repos/JP-Ellis/myrepo",
          created_at: "2026-04-01T00:00:00Z",
          state: "open",
          pull_request: { merged_at: null },
        },
      ],
    };

    const [item] = parseActivityItems(body);

    expect(item.repo).toBe("JP-Ellis/myrepo");
  });

  test("throws on missing items field", () => {
    expect(() => parseActivityItems({})).toThrow("issue search items missing");
  });
});

// MARK: contributionWindow

describe("contributionWindow", () => {
  test("returns exactly 365-day window in non-leap year", () => {
    const now = new Date("2027-03-01T00:00:00Z");
    const { from, to } = contributionWindow(now);

    const diffMs = to.getTime() - from.getTime();
    const days = diffMs / 86_400_000;

    expect(days).toBe(365);
  });

  test("from date is 365 days before to date (non-leap year)", () => {
    const now = new Date("2027-03-01T00:00:00Z");
    const { from, to } = contributionWindow(now);

    const fromStr = from.toISOString().slice(0, 10);
    expect(fromStr).toBe("2026-03-01");
    expect(to.toISOString().slice(0, 10)).toBe("2027-03-01");
  });

  test("from date is 365 days before to date (leap-year-spanning)", () => {
    const now = new Date("2026-03-01T00:00:00Z");
    const { from, to } = contributionWindow(now);

    const diffMs = to.getTime() - from.getTime();
    const days = diffMs / 86_400_000;

    expect(days).toBe(365);
    const fromStr = from.toISOString().slice(0, 10);
    expect(fromStr).toBe("2025-03-01");
  });
});

// MARK: fallbackGithubStats

describe("fallbackGithubStats", () => {
  test("returns a valid GitHubStats shape", () => {
    const stats = fallbackGithubStats();

    expect(typeof stats.fetchedAt).toBe("string");
    expect(typeof stats.totalContributions).toBe("number");
    expect(typeof stats.commitContributions).toBe("number");
    expect(typeof stats.prContributions).toBe("number");
    expect(typeof stats.issueContributions).toBe("number");
    expect(typeof stats.publicRepos).toBe("number");
    expect(typeof stats.periodFrom).toBe("string");
    expect(typeof stats.periodTo).toBe("string");
    expect(Array.isArray(stats.contributionWeeks)).toBe(true);
    expect(Array.isArray(stats.recentActivity)).toBe(true);
  });

  test("scalar counts match fallback.json values", () => {
    const stats = fallbackGithubStats();

    expect(stats.commitContributions).toBe(fallbackJson.commitContributions);
    expect(stats.prContributions).toBe(fallbackJson.prContributions);
    expect(stats.issueContributions).toBe(fallbackJson.issueContributions);
    expect(stats.publicRepos).toBe(fallbackJson.publicRepos);
  });

  test("contributionWeeks has 53 weeks each with 7 days", () => {
    const { contributionWeeks } = fallbackGithubStats();

    expect(contributionWeeks).toHaveLength(53);
    for (const week of contributionWeeks) {
      expect(week.days).toHaveLength(7);
    }
  });

  test("totalContributions is sum of all day counts", () => {
    const stats = fallbackGithubStats();
    const sum = stats.contributionWeeks
      .flatMap((w) => w.days)
      .reduce((acc, d) => acc + d.count, 0);

    expect(stats.totalContributions).toBe(sum);
  });

  test("recentActivity items have correct ActivityItem shape", () => {
    const { recentActivity } = fallbackGithubStats();

    expect(recentActivity.length).toBeGreaterThan(0);
    for (const item of recentActivity) {
      expect(["commit", "pull_request", "issue"]).toContain(item.kind);
      expect(typeof item.repo).toBe("string");
      expect(typeof item.title).toBe("string");
      expect(typeof item.url).toBe("string");
      expect(typeof item.createdAt).toBe("string");
    }
  });

  test("period dates are YYYY-MM-DD strings", () => {
    const { periodFrom, periodTo } = fallbackGithubStats();

    expect(periodFrom).toMatch(DATE_RE);
    expect(periodTo).toMatch(DATE_RE);
  });
});
