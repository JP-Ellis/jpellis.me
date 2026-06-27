import { getCollection } from "astro:content";
import rss from "@astrojs/rss";
import type { APIContext } from "astro";

export async function GET(context: APIContext): Promise<Response> {
  const posts = (await getCollection("blog"))
    .filter((p) => !p.data.draft)
    .sort((a, b) => Number(b.data.date) - Number(a.data.date));

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
