import { expect, test } from "vitest";
import type {
  ActivityItem,
  ActivityKind,
  GitHubStats,
} from "./github-stats.ts";
import {
  buildGridLevels,
  cellLevelFromCount,
  dedupCommitsAgainstPrs,
  timeAgo,
} from "./year-in-code.ts";

// MARK: cellLevelFromCount

test("cellLevelFromCount edge cases", () => {
  expect(cellLevelFromCount(0, 0)).toBe(0);
  expect(cellLevelFromCount(0, 10)).toBe(0);
  expect(cellLevelFromCount(10, 10)).toBe(4);
  expect(cellLevelFromCount(1, 10)).toBe(1); // ratio 0.10 -> level 1
  expect(cellLevelFromCount(3, 10)).toBe(2); // ratio 0.30 -> level 2
  expect(cellLevelFromCount(6, 10)).toBe(3); // ratio 0.60 -> level 3
  expect(cellLevelFromCount(8, 10)).toBe(4); // ratio 0.80 -> level 4
});

// MARK: buildGridLevels

test("buildGridLevels normalises correctly", () => {
  const stats: GitHubStats = {
    fetchedAt: "2025-01-07T00:00:00Z",
    totalContributions: 10,
    commitContributions: 8,
    prContributions: 1,
    issueContributions: 1,
    publicRepos: 1,
    periodFrom: "2025-01-01",
    periodTo: "2025-01-07",
    contributionWeeks: [
      {
        days: [
          { date: "2025-01-01", count: 0 },
          { date: "2025-01-02", count: 10 },
        ],
      },
    ],
    recentActivity: [],
  };
  const grid = buildGridLevels(stats);
  expect(grid.length).toBe(1);
  expect(grid[0][0]).toBe(0); // count 0 -> level 0
  expect(grid[0][1]).toBe(4); // count 10/10 = max -> level 4
});

test("buildGridLevels defaults max to 1 when all counts are zero", () => {
  const stats: GitHubStats = {
    fetchedAt: "2025-01-07T00:00:00Z",
    totalContributions: 0,
    commitContributions: 0,
    prContributions: 0,
    issueContributions: 0,
    publicRepos: 0,
    periodFrom: "2025-01-01",
    periodTo: "2025-01-07",
    contributionWeeks: [{ days: [{ date: "2025-01-01", count: 0 }] }],
    recentActivity: [],
  };
  const grid = buildGridLevels(stats);
  expect(grid[0][0]).toBe(0);
});

// MARK: timeAgo

test("timeAgo boundaries with injected now", () => {
  const now = new Date("2025-06-01T00:00:00Z");
  const ago = (ms: number): Date => new Date(now.getTime() - ms);
  expect(timeAgo(ago(30 * 1000), now)).toBe("1m"); // 30s -> floors to 1m minimum
  expect(timeAgo(ago(90 * 1000), now)).toBe("1m"); // 90s -> 1m
  expect(timeAgo(ago(30 * 60 * 1000), now)).toBe("30m"); // 30m
  expect(timeAgo(ago(2 * 3600 * 1000), now)).toBe("2h"); // 2h
  expect(timeAgo(ago(3 * 86_400 * 1000), now)).toBe("3d"); // 3d
  expect(timeAgo(ago(2 * 604_800 * 1000), now)).toBe("2w"); // 2w
});

test("timeAgo clamps future dates to 1m", () => {
  const now = new Date("2025-06-01T00:00:00Z");
  const future = new Date(now.getTime() + 60 * 1000);
  expect(timeAgo(future, now)).toBe("1m");
});

test("timeAgo accepts ISO string input", () => {
  const now = new Date("2025-06-01T00:00:00Z");
  expect(timeAgo("2025-05-30T00:00:00Z", now)).toBe("2d");
});

// MARK: dedupCommitsAgainstPrs

function makeItem(kind: ActivityKind, title: string): ActivityItem {
  return {
    kind,
    repo: "owner/repo",
    title,
    url: "https://example.com",
    state: null,
    createdAt: "2025-01-01T00:00:00Z",
  };
}

test("dedup removes commit when PR has same title", () => {
  const activity = [
    makeItem("commit", "feat: add thing"),
    makeItem("pull_request", "feat: add thing"),
    makeItem("commit", "fix: unrelated"),
  ];
  const result = dedupCommitsAgainstPrs(activity);
  expect(result.length).toBe(2);
  expect(
    result.some(
      (i) => i.kind === "pull_request" && i.title === "feat: add thing",
    ),
  ).toBe(true);
  expect(
    result.some((i) => i.kind === "commit" && i.title === "fix: unrelated"),
  ).toBe(true);
});

test("dedup keeps commit when no matching PR", () => {
  const activity = [
    makeItem("commit", "feat: standalone commit"),
    makeItem("pull_request", "feat: different title"),
  ];
  const result = dedupCommitsAgainstPrs(activity);
  expect(result.length).toBe(2);
});

test("dedup keeps issue even when title matches commit", () => {
  const activity = [
    makeItem("commit", "fix: bug"),
    makeItem("issue", "fix: bug"),
  ];
  const result = dedupCommitsAgainstPrs(activity);
  expect(result.length).toBe(2);
});
