import { type CollectionEntry, getCollection } from "astro:content";

export type Post = CollectionEntry<"blog">;

/** Drafts show in the dev server and are left out of production builds. */
function isVisible(post: Post): boolean {
  return import.meta.env.DEV || !post.data.draft;
}

/** Blog posts visible in this build, newest first. */
export async function getPosts(): Promise<Post[]> {
  const posts = await getCollection("blog", isVisible);
  return posts.sort((a, b) => b.data.date.getTime() - a.data.date.getTime());
}
