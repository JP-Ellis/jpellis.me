import { describe, expect, it } from "vitest";
import { loadResume } from "./resume.ts";

describe("resume data", () => {
  const { roles, publications, honours } = loadResume();

  it("has exactly 9 roles", () => {
    expect(roles).toHaveLength(9);
  });

  it("has exactly 1 featured role", () => {
    const featured = roles.filter((r) => r.featured === true);
    expect(featured).toHaveLength(1);
  });

  it("has exactly 3 publications", () => {
    expect(publications).toHaveLength(3);
  });

  it("has exactly 3 honours", () => {
    expect(honours).toHaveLength(3);
  });
});
