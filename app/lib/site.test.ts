import { describe, expect, it } from "vitest";
import { pageTitle, personJsonLd, publicPath, SITE_NAME } from "./site.ts";

describe("pageTitle", () => {
  it("suffixes a page name with the site name", () => {
    expect(pageTitle("Projects")).toBe("Projects · Joshua Ellis");
  });

  it("uses the bare site name when there is no page name", () => {
    expect(pageTitle()).toBe(SITE_NAME);
    expect(pageTitle("")).toBe(SITE_NAME);
  });
});

describe("publicPath", () => {
  it.each([
    ["/contact.html", "/contact"],
    ["/blog/some-post.html", "/blog/some-post"],
    ["/index.html", "/"],
    ["/", "/"],
    ["/projects/pact-python", "/projects/pact-python"],
    ["/blog", "/blog"],
  ])("maps %s to %s", (input, expected) => {
    expect(publicPath(input)).toBe(expected);
  });
});

describe("personJsonLd", () => {
  it("describes the site owner", () => {
    const data = personJsonLd(new URL("https://jpellis.me"));
    expect(data).toMatchObject({
      "@context": "https://schema.org",
      "@type": "Person",
      name: "Joshua Ellis",
      url: "https://jpellis.me/",
    });
  });
});
