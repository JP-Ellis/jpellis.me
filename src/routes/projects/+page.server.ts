import { Octokit } from "@octokit/rest";
import { compile as compileMarkdown } from "mdsvex";

import type { PageServerLoad } from "./$types";

// Prerender this page at build time, so that the GitHub API doesn't get
// spammed with requests every time the page is loaded.
export const prerender = true;

const octokit = new Octokit({
  auth: process.env.GITHUB_TOKEN,
});
const me = "JP-Ellis";

interface GitHubStats {
  repos: number;
  stars: number;
  forks: number;
}

interface ProjectMetadata {
  title: string;
  slug: string;
  blurb: string;
  github?: string;
}

// Fetch statistics about my GitHub projects, tallying up the number of stars
// and forks for each project.
async function fetchGitHubStats(
  fetch: (
    input: RequestInfo | URL,
    init?: RequestInit | undefined,
  ) => Promise<Response>,
): Promise<GitHubStats> {
  const iterator = octokit.paginate.iterator(octokit.rest.repos.listForUser, {
    username: me,
    perPage: 100,
    request: { fetch },
  });

  const stats = { repos: 0, stars: 0, forks: 0 } satisfies GitHubStats;

  for await (const { data: repos } of iterator) {
    for (const repo of repos) {
      if (repo.fork) continue;

      stats.repos += 1;
      stats.stars += repo.stargazers_count ?? 0;
      stats.forks += repo.forks_count ?? 0;
    }
  }

  return stats;
}

async function parseProjectMetadata(
  data: ProjectMetadata,
): Promise<ProjectMetadata> {
  const compiled = await compileMarkdown(data.blurb);
  return { ...data, blurb: compiled?.code ?? "" };
}

async function parseProjects(): Promise<ProjectMetadata[]> {
  const modules = import.meta.glob("./*/+page.svx");
  return Promise.all(
    Object.values(modules).map((module) =>
      module()
        .then((value) => {
          if (value && typeof value === "object" && "metadata" in value) {
            return value.metadata as ProjectMetadata;
          }
          throw new Error("No metadata found");
        })
        .then(parseProjectMetadata),
    ),
  );
}

export const load = (async ({ fetch }) => {
  const github = await fetchGitHubStats(fetch);
  const projects = await parseProjects();
  return { github, projects };
}) satisfies PageServerLoad;
