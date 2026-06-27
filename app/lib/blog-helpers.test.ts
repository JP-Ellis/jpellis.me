import { describe, expect, it } from "vitest";
import { formatDate, sourceDomain, splitExcerpt } from "./blog.ts";

describe("formatDate", () => {
  it("formats 2025-12-04 as '4 Dec 2025'", () => {
    expect(formatDate("2025-12-04")).toBe("4 Dec 2025");
  });

  it("formats 2024-04-11 as '4 Apr 2024' (strips leading zero from day)", () => {
    expect(formatDate("2024-04-11")).toBe("11 Apr 2024");
  });

  it("formats 2026-03-01 as '1 Mar 2026'", () => {
    expect(formatDate("2026-03-01")).toBe("1 Mar 2026");
  });

  it("strips leading zero from day: 2024-04-04 → '4 Apr 2024'", () => {
    expect(formatDate("2024-04-04")).toBe("4 Apr 2024");
  });

  it("returns malformed input unchanged", () => {
    expect(formatDate("not-a-date")).toBe("not-a-date");
  });
});

describe("sourceDomain", () => {
  it("extracts host from https URL", () => {
    expect(sourceDomain("https://github.com/JP-Ellis/x")).toBe("github.com");
  });

  it("extracts host from long path URL", () => {
    expect(
      sourceDomain(
        "https://pact-foundation.github.io/pact-python/blog/2024/12/30/functional-arguments/",
      ),
    ).toBe("pact-foundation.github.io");
  });

  it("returns undefined for string with no '//'", () => {
    expect(sourceDomain("no-slashes-here")).toBeUndefined();
  });
});

describe("splitExcerpt", () => {
  it("returns intro HTML when body has <!-- more --> marker", async () => {
    const body =
      "## Intro\n\nThis is the intro paragraph.\n\n<!-- more -->\n\n## Rest\n\nThis is after the marker.";
    const result = await splitExcerpt(body);
    expect(result).toContain("This is the intro paragraph.");
    expect(result).not.toContain("This is after the marker.");
  });

  it("does not include the marker itself in the excerpt", async () => {
    const body = "Hello world.\n\n<!-- more -->\n\nMore content.";
    const result = await splitExcerpt(body);
    expect(result).not.toContain("<!-- more -->");
  });

  it("returns <p>description</p> when no marker and description provided", async () => {
    const body = "Some content with no more marker.";
    const result = await splitExcerpt(body, "A short description.");
    expect(result).toBe("<p>A short description.</p>");
  });

  it("returns empty string when no marker and no description", async () => {
    const body = "Some content with no more marker.";
    const result = await splitExcerpt(body);
    expect(result).toBe("");
  });

  it("renders markdown to HTML in the excerpt", async () => {
    const body = "**Bold text** and `code`.\n\n<!-- more -->\n\nRest.";
    const result = await splitExcerpt(body);
    expect(result).toContain("<strong>Bold text</strong>");
    expect(result).toContain("<code>code</code>");
  });
});
