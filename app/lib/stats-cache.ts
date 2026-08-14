interface Kv {
  get: (k: string, t?: "json") => Promise<unknown>;
  put: (k: string, v: string) => Promise<void>;
}

interface RefreshOpts<T> {
  kv: Kv;
  key: string;
  refresh: () => Promise<T>;
  waitUntil: (p: Promise<unknown>) => void;
  cooldownMs?: number;
  now?: () => number;
}

/**
 * How long to wait before retrying a key whose refresh threw. Without this a
 * persistently failing upstream is retried on every single request, which both
 * hammers the API and guarantees it keeps failing.
 */
const DEFAULT_COOLDOWN_MS = 60_000;

/** Refreshes currently running in this isolate, keyed by cache key. */
const inFlight = new Map<string, Promise<unknown>>();

/** Earliest time a key may be refreshed again after a failure, keyed by cache key. */
const retryAfter = new Map<string, number>();

function scheduleRefresh<T>(opts: RefreshOpts<T>, now: number): void {
  const { key } = opts;

  const blockedUntil = retryAfter.get(key);
  if (blockedUntil !== undefined && now < blockedUntil) {
    return;
  }

  const running = inFlight.get(key);
  if (running) {
    opts.waitUntil(running);
    return;
  }

  const clock = opts.now ?? Date.now;
  const pending = opts
    .refresh()
    .then((data) =>
      opts.kv.put(
        key,
        JSON.stringify({
          data,
          fetchedAt: clock(),
        } satisfies CachedStats<T>),
      ),
    )
    .then(() => {
      retryAfter.delete(key);
    })
    .catch((err: unknown) => {
      retryAfter.set(key, now + (opts.cooldownMs ?? DEFAULT_COOLDOWN_MS));
      // biome-ignore lint/suspicious/noConsole: Worker observability captures console output; this is the only signal that a background refresh is failing.
      console.error(`[stats-cache] refresh failed for "${key}":`, err);
    })
    .finally(() => {
      if (inFlight.get(key) === pending) {
        inFlight.delete(key);
      }
    });

  inFlight.set(key, pending);
  opts.waitUntil(pending);
}

export interface CachedStats<T> {
  data: T;
  fetchedAt: number;
}

// biome-ignore lint/style/useNamingConvention: SWR = stale-while-revalidate (domain acronym)
export async function readWithSWR<T>(
  opts: RefreshOpts<T> & { maxAgeMs: number },
): Promise<{ data: T | null; stale: boolean }> {
  const now = (opts.now ?? Date.now)();
  const cached = (await opts.kv.get(opts.key, "json")) as CachedStats<T> | null;
  const fresh = cached !== null && now - cached.fetchedAt < opts.maxAgeMs;
  if (!(cached && fresh)) {
    scheduleRefresh(opts, now);
  }
  return { data: cached?.data ?? null, stale: cached !== null && !fresh };
}
