export interface CachedStats<T> {
  data: T;
  fetchedAt: number;
}

// biome-ignore lint/style/useNamingConvention: SWR = stale-while-revalidate (domain acronym)
export async function readWithSWR<T>(opts: {
  kv: {
    get: (k: string, t?: "json") => Promise<unknown>;
    put: (k: string, v: string) => Promise<void>;
  };
  key: string;
  maxAgeMs: number;
  refresh: () => Promise<T>;
  waitUntil: (p: Promise<unknown>) => void;
  now?: () => number;
}): Promise<{ data: T | null; stale: boolean }> {
  const now = (opts.now ?? Date.now)();
  const cached = (await opts.kv.get(opts.key, "json")) as CachedStats<T> | null;
  const fresh = cached !== null && now - cached.fetchedAt < opts.maxAgeMs;
  if (!(cached && fresh)) {
    opts.waitUntil(
      opts
        .refresh()
        .then((data) =>
          opts.kv.put(
            opts.key,
            JSON.stringify({ data, fetchedAt: now } satisfies CachedStats<T>),
          ),
        ),
    );
  }
  return { data: cached?.data ?? null, stale: cached !== null && !fresh };
}
