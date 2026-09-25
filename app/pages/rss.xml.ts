import rss from "@astrojs/rss";
import type { APIContext } from "astro";
import { getPosts } from "../lib/posts.ts";

export async function GET(context: APIContext): Promise<Response> {
  const posts = await getPosts();

  return rss({
    title: "JP Ellis",
    description: "Posts by JP Ellis",
    site: context.site ?? "https://jpellis.me",
    items: posts.map((p) => ({
      title: p.data.title,
      pubDate: p.data.date,
      description: p.data.description,
      link: p.data.source ?? `/blog/${p.id}`,
    })),
  });
}
