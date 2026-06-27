import { describe, expect, it } from "vitest";
import { PROJECTS } from "./projects.ts";

describe("PROJECTS data", () => {
  it("has exactly 14 entries", () => {
    expect(PROJECTS).toHaveLength(14);
  });

  it("pact-python is a github link to pact-foundation/pact-python", () => {
    const [entry] = PROJECTS;
    expect(entry.name).toBe("pact-python");
    expect(entry.link).toEqual({
      kind: "github",
      slug: "pact-foundation/pact-python",
    });
  });

  it("azure data accelerator has link: null", () => {
    const entry = PROJECTS.find((p) => p.name === "azure data accelerator");
    expect(entry).toBeDefined();
    expect(entry!.link).toBeNull();
  });

  it("pactflow-ai has an external link", () => {
    const entry = PROJECTS.at(-1);
    expect(entry?.name).toBe("pactflow-ai");
    expect(entry?.link).toEqual({
      kind: "external",
      url: "https://pactflow.io/ai/",
    });
  });

  it("order matches the Rust source exactly", () => {
    const names = PROJECTS.map((p) => p.name);
    expect(names).toEqual([
      "pact-python",
      "tikz-feynman",
      "rust-skiplist",
      "mathematica-notebook-filter",
      "simpler-wick",
      "boltzmann-solver",
      "dotfiles",
      "jpellis.me",
      "borrow-checker",
      "amber-api",
      "enphase-api",
      "repo-manage",
      "azure data accelerator",
      "pactflow-ai",
    ]);
  });
});
