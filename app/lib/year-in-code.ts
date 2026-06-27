// biome-ignore-all lint/style/useExportsLast: module is organized into MARK sections; exports sit with their section's helpers
import type { ActivityItem, GitHubStats } from "./github-stats.ts";

// MARK: Cell level

const LEVEL_RATIO_LOW = 0.25;
const LEVEL_RATIO_MID = 0.5;
const LEVEL_RATIO_HIGH = 0.75;
// Ascending intensity thresholds; the cell level is how many a ratio clears.
const LEVEL_RATIOS = [LEVEL_RATIO_LOW, LEVEL_RATIO_MID, LEVEL_RATIO_HIGH];

export function cellLevelFromCount(count: number, max: number): number {
  if (max === 0 || count === 0) {
    return 0;
  }
  const ratio = count / max;
  return LEVEL_RATIOS.filter((threshold) => ratio >= threshold).length + 1;
}

// MARK: Grid

export function buildGridLevels(stats: GitHubStats): number[][] {
  const counts = stats.contributionWeeks.flatMap((week) =>
    week.days.map((day) => day.count),
  );
  const max = Math.max(1, ...counts);
  return stats.contributionWeeks.map((week) =>
    week.days.map((day) => cellLevelFromCount(day.count, max)),
  );
}

// MARK: Relative time

const SECS_PER_MINUTE = 60;
const SECS_PER_HOUR = 3600;
const SECS_PER_DAY = 86_400;
const SECS_PER_WEEK = 604_800;
const MS_PER_SECOND = 1000;

export function timeAgo(date: Date | string, now: Date): string {
  const then = date instanceof Date ? date : new Date(date);
  const secs = Math.max(0, (now.getTime() - then.getTime()) / MS_PER_SECOND);
  if (secs < SECS_PER_HOUR) {
    return `${Math.max(1, Math.floor(secs / SECS_PER_MINUTE))}m`;
  }
  if (secs < SECS_PER_DAY) {
    return `${Math.floor(secs / SECS_PER_HOUR)}h`;
  }
  if (secs < SECS_PER_WEEK) {
    return `${Math.floor(secs / SECS_PER_DAY)}d`;
  }
  return `${Math.floor(secs / SECS_PER_WEEK)}w`;
}

// MARK: Dedup

export function dedupCommitsAgainstPrs(
  activity: ActivityItem[],
): ActivityItem[] {
  const prTitles = new Set(
    activity
      .filter((item) => item.kind === "pull_request")
      .map((item) => item.title),
  );
  return activity.filter(
    (item) => !(item.kind === "commit" && prTitles.has(item.title)),
  );
}
