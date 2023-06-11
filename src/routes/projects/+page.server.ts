import { Octokit } from "@octokit/rest";

import type { PageServerLoad } from "./$types";

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

// Fetch statistics about my GitHub projects, tallying up the number of stars
// and forks for each project.
async function fetchGitHubStats(
  fetch: (
    input: RequestInfo | URL,
    init?: RequestInit | undefined
  ) => Promise<Response>
): Promise<GitHubStats> {
  const iterator = octokit.paginate.iterator(octokit.rest.repos.listForUser, {
    username: me,
    perPage: 100,
    request: { fetch },
  });

  const ZERO = 0;
  const stats = { repos: 0, stars: 0, forks: 0 } satisfies GitHubStats;

  for await (const { data: repos } of iterator) {
    for (const repo of repos) {
      if (repo.fork) continue;

      stats.repos += 1;
      stats.stars += repo.stargazers_count ?? ZERO;
      stats.forks += repo.forks_count ?? ZERO;
    }
  }

  return stats;
}

export const load = (async ({ fetch }) => {
  const github = await fetchGitHubStats(fetch);
  return { github };
}) satisfies PageServerLoad;
