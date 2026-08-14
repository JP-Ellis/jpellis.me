import type { APIRoute } from "astro";
import {
  fallbackGithubStats,
  type GitHubStats,
} from "../../lib/github-stats.ts";

export const prerender = false;

export const GET: APIRoute = async () => {
  let stats: GitHubStats = fallbackGithubStats();
  try {
    const { env } = await import("cloudflare:workers");
    if (env?.GITHUB_STATS) {
      const cached = (await env.GITHUB_STATS.get("github/stats", "json")) as {
        data: unknown;
      } | null;
      stats =
        (cached?.data as GitHubStats | undefined) ?? fallbackGithubStats();
    }
  } catch (err) {
    // biome-ignore lint/suspicious/noConsole: Worker observability captures console output; a silent fallback hides a broken refresh.
    console.error("[api/github-stats] KV read failed, using fallback:", err);
    stats = fallbackGithubStats();
  }
  return Response.json(stats);
};
