import rss from "@astrojs/rss";
import type { APIContext } from "astro";
import { getPosts } from "../lib/posts.ts";
import { SITE_DESCRIPTION, SITE_NAME } from "../lib/site.ts";

export async function GET(context: APIContext): Promise<Response> {
  const posts = await getPosts();

  return rss({
    title: SITE_NAME,
    description: SITE_DESCRIPTION,
    site: context.site ?? "https://jpellis.me",
    trailingSlash: false,
    items: posts.map((p) => ({
      title: p.data.title,
      pubDate: p.data.date,
      description: p.data.description,
      link: p.data.source ?? `/blog/${p.id}`,
    })),
  });
}
