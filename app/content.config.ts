import { defineCollection } from "astro:content";
import { glob } from "astro/loaders";
import { z } from "astro/zod";

const blog = defineCollection({
  loader: glob({ pattern: "**/*.md", base: "./app/content/blog" }),
  schema: z.object({
    title: z.string(),
    date: z.coerce.date(),
    tags: z.array(z.string()).default([]),
    source: z.string().url().optional(),
    description: z.string().optional(),
    draft: z.boolean().default(false),
  }),
});

const projects = defineCollection({
  loader: glob({ pattern: "*.md", base: "./app/content/projects" }),
  schema: z.object({
    title: z.string(),
    slug: z.string(),
    github: z.string(),
    tagline: z.string(),
    activity: z
      .object({
        release: z.boolean().default(true),
        recentCommits: z.boolean().default(true),
        openPrs: z.boolean().default(true),
      })
      .default({ release: true, recentCommits: true, openPrs: true }),
  }),
});

export const collections = { blog, projects };
