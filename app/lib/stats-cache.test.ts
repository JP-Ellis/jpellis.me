import { afterEach, expect, test, vi } from "vitest";
import { readWithSWR } from "./stats-cache.ts";

afterEach(() => {
  vi.restoreAllMocks();
});

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

test("a failing refresh is reported and never rejects", async () => {
  const kv = fakeKv({});
  const err = new Error("GraphQL status 401");
  const refresh = vi.fn().mockRejectedValue(err);
  const logged = vi.spyOn(console, "error").mockImplementation(() => undefined);
  const waits: Promise<unknown>[] = [];

  const r = await readWithSWR({
    kv,
    key: "fail-reported",
    maxAgeMs: 10_000,
    refresh,
    waitUntil: (p) => waits.push(p),
    now: () => 1000,
  });

  expect(r).toEqual({ data: null, stale: false });
  await expect(Promise.all(waits)).resolves.toBeDefined();
  expect(logged).toHaveBeenCalledWith(
    expect.stringContaining("fail-reported"),
    err,
  );
});

test("concurrent misses share a single refresh", async () => {
  const kv = fakeKv({});
  let resolveRefresh: ((v: number) => void) | undefined;
  const refresh = vi.fn(
    () =>
      new Promise<number>((res) => {
        resolveRefresh = res;
      }),
  );
  const waits: Promise<unknown>[] = [];
  const read = () =>
    readWithSWR({
      kv,
      key: "single-flight",
      maxAgeMs: 10_000,
      refresh,
      waitUntil: (p) => waits.push(p),
      now: () => 1000,
    });

  await Promise.all([read(), read(), read()]);
  resolveRefresh?.(7);
  await Promise.all(waits);

  expect(refresh).toHaveBeenCalledTimes(1);
  expect(JSON.parse(kv._store.get("single-flight") as string).data).toBe(7);
});

test("a failed refresh is not retried until the cooldown expires", async () => {
  const kv = fakeKv({});
  const refresh = vi.fn().mockRejectedValue(new Error("boom"));
  vi.spyOn(console, "error").mockImplementation(() => undefined);
  const waits: Promise<unknown>[] = [];
  const read = (now: number) =>
    readWithSWR({
      kv,
      key: "cooldown",
      maxAgeMs: 10_000,
      refresh,
      waitUntil: (p) => waits.push(p),
      cooldownMs: 60_000,
      now: () => now,
    });

  await read(1000);
  await Promise.all(waits);
  expect(refresh).toHaveBeenCalledTimes(1);

  await read(30_000);
  await Promise.all(waits);
  expect(refresh).toHaveBeenCalledTimes(1);

  await read(61_001);
  await Promise.all(waits);
  expect(refresh).toHaveBeenCalledTimes(2);
});
