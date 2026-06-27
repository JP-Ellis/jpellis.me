import { expect, test, vi } from "vitest";
import { readWithSWR } from "./stats-cache.ts";

function fakeKv(initial: Record<string, string>) {
  const store = new Map(Object.entries(initial));
  return {
    get(k: string, t?: "json"): Promise<unknown> {
      const v = store.get(k);
      if (v === undefined) {
        return Promise.resolve(null);
      }
      return Promise.resolve(t === "json" ? JSON.parse(v) : v);
    },
    put(k: string, v: string): Promise<void> {
      store.set(k, v);
      return Promise.resolve();
    },
    _store: store,
  };
}

test("returns fresh data without refreshing", async () => {
  const kv = fakeKv({ s: JSON.stringify({ data: 1, fetchedAt: 1000 }) });
  const refresh = vi.fn();
  const waits: Promise<unknown>[] = [];
  const r = await readWithSWR({
    kv,
    key: "s",
    maxAgeMs: 10_000,
    refresh,
    waitUntil: (p) => waits.push(p),
    now: () => 5000,
  });
  expect(r).toEqual({ data: 1, stale: false });
  expect(refresh).not.toHaveBeenCalled();
});

test("returns stale data and schedules background refresh", async () => {
  const kv = fakeKv({ s: JSON.stringify({ data: 1, fetchedAt: 0 }) });
  const refresh = vi.fn().mockResolvedValue(2);
  const waits: Promise<unknown>[] = [];
  const r = await readWithSWR({
    kv,
    key: "s",
    maxAgeMs: 10_000,
    refresh,
    waitUntil: (p) => waits.push(p),
    now: () => 999_999,
  });
  expect(r).toEqual({ data: 1, stale: true });
  expect(waits.length).toBe(1);
  await Promise.all(waits);
  expect(JSON.parse(kv._store.get("s") as string).data).toBe(2);
});
