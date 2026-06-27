import { describe, expect, it } from "vitest";
import {
  formatRelativeDate,
  formatShortDate,
  formatStars,
} from "./projects.ts";

describe("formatStars", () => {
  it("returns plain digits below 1000", () => {
    expect(formatStars(0)).toBe("0");
    expect(formatStars(158)).toBe("158");
    expect(formatStars(664)).toBe("664");
    expect(formatStars(999)).toBe("999");
  });

  it("formats 1000 as 1.0k", () => {
    expect(formatStars(1000)).toBe("1.0k");
  });

  it("formats 1400 as 1.4k", () => {
    expect(formatStars(1400)).toBe("1.4k");
  });

  it("formats large values correctly", () => {
    expect(formatStars(10_000)).toBe("10.0k");
    expect(formatStars(98_516)).toBe("98.5k");
  });
});

describe("formatShortDate", () => {
  it("formats a full ISO 8601 timestamp", () => {
    expect(formatShortDate("2025-01-14T00:00:00Z")).toBe("14 Jan 2025");
  });

  it("formats a plain date string", () => {
    expect(formatShortDate("2024-04-01")).toBe("1 Apr 2024");
  });

  it("returns input with fewer than 3 dash-parts as-is", () => {
    expect(formatShortDate("2025-01")).toBe("2025-01");
    expect(formatShortDate("nodashes")).toBe("nodashes");
  });
});

describe("formatRelativeDate", () => {
  const now = new Date("2025-06-01T12:00:00Z");

  it("returns 'just now' for 30 seconds ago", () => {
    const dt = new Date(now.getTime() - 30_000);
    expect(formatRelativeDate(dt.toISOString(), now)).toBe("just now");
  });

  it("returns 'Xh ago' for 2 hours ago", () => {
    const dt = new Date(now.getTime() - 2 * 3_600_000);
    expect(formatRelativeDate(dt.toISOString(), now)).toBe("2h ago");
  });

  it("returns 'Xd ago' for 3 days ago", () => {
    const dt = new Date(now.getTime() - 3 * 86_400_000);
    expect(formatRelativeDate(dt.toISOString(), now)).toBe("3d ago");
  });

  it("returns 'Xw ago' for 2 weeks ago", () => {
    const dt = new Date(now.getTime() - 14 * 86_400_000);
    expect(formatRelativeDate(dt.toISOString(), now)).toBe("2w ago");
  });

  it("returns 'Xmo ago' for 4 months ago", () => {
    const dt = new Date(now.getTime() - 120 * 86_400_000);
    expect(formatRelativeDate(dt.toISOString(), now)).toBe("4mo ago");
  });

  it("returns 'Xy ago' for 2 years ago", () => {
    const dt = new Date(now.getTime() - 730 * 86_400_000);
    expect(formatRelativeDate(dt.toISOString(), now)).toBe("2y ago");
  });

  it("returns malformed input as-is", () => {
    expect(formatRelativeDate("not-a-date", now)).toBe("not-a-date");
  });
});
