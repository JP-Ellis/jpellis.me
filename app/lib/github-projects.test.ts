// biome-ignore-all lint/style/useNamingConvention: test fixtures mirror the GitHub API response shape
import { describe, expect, test } from "vitest";
import {
  fallbackProjectStats,
  parseCommit,
  parseCommitsResponse,
  parseReleaseResponse,
  parseRepoResponse,
} from "./github-projects.ts";

// MARK: parseRepoResponse

describe("parseRepoResponse", () => {
  test("extracts all four fields from a repo response", () => {
    const body = {
      stargazers_count: 158,
      forks_count: 22,
      open_issues_count: 5,
      watchers_count: 12,
    };

    const result = parseRepoResponse("JP-Ellis/tikz-feynman", body);

    expect(result.stars).toBe(158);
    expect(result.forks).toBe(22);
    expect(result.openIssues).toBe(5);
    expect(result.watchers).toBe(12);
  });

  test("returns zero counts when fields are 0", () => {
    const body = {
      stargazers_count: 0,
      forks_count: 0,
      open_issues_count: 0,
      watchers_count: 0,
    };

    const result = parseRepoResponse("owner/repo", body);

    expect(result.stars).toBe(0);
    expect(result.forks).toBe(0);
    expect(result.openIssues).toBe(0);
    expect(result.watchers).toBe(0);
  });

  test("throws when stargazers_count is missing", () => {
    const body = {
      forks_count: 5,
      open_issues_count: 0,
      watchers_count: 1,
    };

    expect(() => parseRepoResponse("owner/repo", body)).toThrow(
      "stargazers_count",
    );
  });

  test("throws when forks_count is missing", () => {
    const body = {
      stargazers_count: 100,
      open_issues_count: 0,
      watchers_count: 1,
    };

    expect(() => parseRepoResponse("owner/repo", body)).toThrow("forks_count");
  });

  test("throws when open_issues_count is missing", () => {
    const body = {
      stargazers_count: 100,
      forks_count: 5,
      watchers_count: 1,
    };

    expect(() => parseRepoResponse("owner/repo", body)).toThrow(
      "open_issues_count",
    );
  });

  test("throws when watchers_count is missing", () => {
    const body = {
      stargazers_count: 100,
      forks_count: 5,
      open_issues_count: 0,
    };

    expect(() => parseRepoResponse("owner/repo", body)).toThrow(
      "watchers_count",
    );
  });
});

// MARK: parseReleaseResponse

describe("parseReleaseResponse", () => {
  test("extracts release info from a valid release response", () => {
    const body = {
      tag_name: "v3.1.0",
      published_at: "2025-01-14T10:00:00Z",
      html_url: "https://github.com/JP-Ellis/tikz-feynman/releases/tag/v3.1.0",
    };

    const result = parseReleaseResponse(body);

    expect(result).not.toBeNull();
    expect(result!.tag).toBe("v3.1.0");
    expect(result!.date).toBe("2025-01-14T10:00:00Z");
    expect(result!.url).toBe(
      "https://github.com/JP-Ellis/tikz-feynman/releases/tag/v3.1.0",
    );
  });

  test("returns null for a 404-style response", () => {
    const body = { message: "Not Found" };

    const result = parseReleaseResponse(body);

    expect(result).toBeNull();
  });

  test("returns null when tag_name is missing", () => {
    const body = {
      published_at: "2025-01-14T10:00:00Z",
      html_url: "https://github.com/owner/repo/releases/tag/v1.0.0",
    };

    expect(parseReleaseResponse(body)).toBeNull();
  });

  test("returns null when published_at is missing", () => {
    const body = {
      tag_name: "v1.0.0",
      html_url: "https://github.com/owner/repo/releases/tag/v1.0.0",
    };

    expect(parseReleaseResponse(body)).toBeNull();
  });

  test("returns null when html_url is missing", () => {
    const body = {
      tag_name: "v1.0.0",
      published_at: "2025-01-14T10:00:00Z",
    };

    expect(parseReleaseResponse(body)).toBeNull();
  });

  test("returns null for an empty object", () => {
    expect(parseReleaseResponse({})).toBeNull();
  });
});

// MARK: parseCommit

describe("parseCommit", () => {
  test("returns CommitInfo for a human commit", () => {
    const commit = {
      sha: "a1b2c3d4e5f6",
      commit: {
        message: "Fix diagram spacing\n\nLonger description here",
        author: { name: "Joshua Ellis", date: "2025-05-01T09:00:00Z" },
      },
      author: { login: "JP-Ellis", type: "User" },
      html_url: "https://github.com/JP-Ellis/tikz-feynman/commit/a1b2c3d",
    };

    const result = parseCommit(commit);

    expect(result).not.toBeNull();
    expect(result!.sha).toBe("a1b2c3d"); // 7-char short SHA
    expect(result!.message).toBe("Fix diagram spacing"); // first line only
    expect(result!.author).toBe("JP-Ellis"); // GitHub login for Users
    expect(result!.date).toBe("2025-05-01T09:00:00Z");
    expect(result!.url).toBe(
      "https://github.com/JP-Ellis/tikz-feynman/commit/a1b2c3d",
    );
  });

  test("returns null for a bot commit (author.type === 'Bot')", () => {
    const commit = {
      sha: "abc1234567890",
      commit: {
        message: "chore(deps): update deps\nMore details",
        author: { name: "renovate[bot]", date: "2025-05-01T09:00:00Z" },
      },
      author: { login: "renovate[bot]", type: "Bot" },
      html_url: "https://github.com/owner/repo/commit/abc1234",
    };

    expect(parseCommit(commit)).toBeNull();
  });

  test("treats null author (unlinked GitHub account) as human", () => {
    const commit = {
      sha: "abc1234567890",
      commit: {
        message: "Initial commit",
        author: { name: "Someone", date: "2025-05-01T09:00:00Z" },
      },
      author: null,
      html_url: "https://github.com/owner/repo/commit/abc1234",
    };

    const result = parseCommit(commit);

    expect(result).not.toBeNull();
    expect(result!.author).toBe("Someone"); // falls back to git committer name
  });

  test("truncates message longer than 72 chars and appends ellipsis", () => {
    const longMsg = `${"A".repeat(80)}\nSecond line`;
    const commit = {
      sha: "a1b2c3d4e5f6",
      commit: {
        message: longMsg,
        author: { name: "JP-Ellis", date: "2025-01-01T00:00:00Z" },
      },
      author: { login: "JP-Ellis", type: "User" },
      html_url: "https://example.com/commit/abc",
    };

    const result = parseCommit(commit);

    expect(result).not.toBeNull();
    expect(result!.message.endsWith("…")).toBe(true);
    // 72 truncated chars + "…" ellipsis = 73 chars total
    expect([...result!.message].length).toBe(73);
  });

  test("handles multi-byte chars in truncation (char-based, not byte-based)", () => {
    const longMsg = `${"🚀".repeat(80)}\nSecond line`;
    const commit = {
      sha: "a1b2c3d4e5f6",
      commit: {
        message: longMsg,
        author: { name: "JP-Ellis", date: "2025-01-01T00:00:00Z" },
      },
      author: { login: "JP-Ellis", type: "User" },
      html_url: "https://example.com/commit/abc",
    };

    const result = parseCommit(commit);

    expect(result).not.toBeNull();
    expect(result!.message.endsWith("…")).toBe(true);
    expect([...result!.message].length).toBe(73);
  });

  test("does not append ellipsis when message is exactly 72 chars", () => {
    const msg72 = "B".repeat(72);
    const commit = {
      sha: "a1b2c3d4e5f6",
      commit: {
        message: msg72,
        author: { name: "JP-Ellis", date: "2025-01-01T00:00:00Z" },
      },
      author: { login: "JP-Ellis", type: "User" },
      html_url: "https://example.com/commit/abc",
    };

    const result = parseCommit(commit);

    expect(result).not.toBeNull();
    expect(result!.message).toBe(msg72);
    expect(result!.message.endsWith("…")).toBe(false);
  });

  test("produces a 7-char sha", () => {
    const commit = {
      sha: "abcdef1234567890",
      commit: {
        message: "Some commit",
        author: { name: "JP-Ellis", date: "2025-01-01T00:00:00Z" },
      },
      author: { login: "JP-Ellis", type: "User" },
      html_url: "https://example.com/commit/abc",
    };

    const result = parseCommit(commit);

    expect(result!.sha).toBe("abcdef1");
    expect(result!.sha).toHaveLength(7);
  });
});

// MARK: parseCommitsResponse

describe("parseCommitsResponse", () => {
  test("filters bots and caps at 5 results", () => {
    const commits: Record<string, unknown>[] = [];
    // 5 human commits
    for (let i = 0; i < 5; i += 1) {
      commits.push({
        sha: `human${i}abcdef`,
        commit: {
          message: `Human commit ${i}`,
          author: { name: "JP-Ellis", date: "2025-05-01T09:00:00Z" },
        },
        author: { login: "JP-Ellis", type: "User" },
        html_url: "https://example.com/commit/abc",
      });
    }
    // 2 bot commits
    for (let i = 0; i < 2; i += 1) {
      commits.push({
        sha: `bot${i}abcdefgh`,
        commit: {
          message: `chore(deps): bot commit ${i}`,
          author: { name: "renovate[bot]", date: "2025-05-01T09:00:00Z" },
        },
        author: { login: "renovate[bot]", type: "Bot" },
        html_url: "https://example.com/commit/bot",
      });
    }

    const result = parseCommitsResponse(commits);

    expect(result).toHaveLength(5);
    expect(result.every((c) => !c.author.includes("bot"))).toBe(true);
  });

  test("returns fewer than 5 when fewer non-bot commits exist", () => {
    const commits = [
      {
        sha: "human1abcdef",
        commit: {
          message: "Only commit",
          author: { name: "JP-Ellis", date: "2025-05-01T09:00:00Z" },
        },
        author: { login: "JP-Ellis", type: "User" },
        html_url: "https://example.com/commit/abc",
      },
    ];

    const result = parseCommitsResponse(commits);

    expect(result).toHaveLength(1);
  });

  test("returns empty array for empty input", () => {
    expect(parseCommitsResponse([])).toEqual([]);
  });

  test("caps at 5 even with many human commits", () => {
    const commits = Array.from({ length: 20 }, (_, i) => ({
      sha: `abc${i}defgh`,
      commit: {
        message: `Commit ${i}`,
        author: { name: "JP-Ellis", date: "2025-05-01T09:00:00Z" },
      },
      author: { login: "JP-Ellis", type: "User" },
      html_url: "https://example.com/commit/abc",
    }));

    const result = parseCommitsResponse(commits);

    expect(result).toHaveLength(5);
  });
});

// MARK: fallbackProjectStats

describe("fallbackProjectStats", () => {
  test("returns a valid ProjectsStats shape", () => {
    const stats = fallbackProjectStats();

    expect(stats).toHaveProperty("fetchedAt");
    expect(stats).toHaveProperty("repos");
    expect(Array.isArray(stats.repos)).toBe(true);
  });

  test("fetchedAt is a recent ISO timestamp", () => {
    const before = Date.now();
    const stats = fallbackProjectStats();
    const after = Date.now();

    const ts = new Date(stats.fetchedAt).getTime();
    expect(ts).toBeGreaterThanOrEqual(before);
    expect(ts).toBeLessThanOrEqual(after);
  });

  test("repos array is non-empty", () => {
    const stats = fallbackProjectStats();

    expect(stats.repos.length).toBeGreaterThan(0);
  });

  test("each repo has required fields", () => {
    const stats = fallbackProjectStats();

    for (const repo of stats.repos) {
      expect(typeof repo.slug).toBe("string");
      expect(repo.slug.length).toBeGreaterThan(0);
      expect(typeof repo.stars).toBe("number");
      expect(typeof repo.forks).toBe("number");
    }
  });

  test("contains known repos from fallback data", () => {
    const stats = fallbackProjectStats();
    const slugs = stats.repos.map((r) => r.slug);

    expect(slugs).toContain("JP-Ellis/tikz-feynman");
    expect(slugs).toContain("pact-foundation/pact-python");
  });
});
