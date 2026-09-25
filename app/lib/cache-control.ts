/**
 * Cache-Control values for the on-demand (SSR) routes. Browsers and shared
 * caches serve a response fresh for `max-age` seconds, then keep serving it
 * for up to a day while revalidating in the background.
 */

const STALE_WHILE_REVALIDATE_SECS = 86_400;
const FIVE_MINUTES_SECS = 300;
const FIFTEEN_MINUTES_SECS = 900;
const ONE_HOUR_SECS = 3600;

/** Builds a public Cache-Control value with a day of stale-while-revalidate. */
export function cacheControl(maxAgeSecs: number): string {
  return `public, max-age=${maxAgeSecs}, stale-while-revalidate=${STALE_WHILE_REVALIDATE_SECS}`;
}

/** Pages whose content changes only on deploy (the blog index). */
export const CACHE_CONTENT = cacheControl(FIVE_MINUTES_SECS);

/** The home page, whose stats refresh daily. */
export const CACHE_DAILY_STATS = cacheControl(FIFTEEN_MINUTES_SECS);

/** Project pages, whose stats refresh weekly. */
export const CACHE_WEEKLY_STATS = cacheControl(ONE_HOUR_SECS);
