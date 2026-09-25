// biome-ignore-all lint/style/useNamingConvention: env names match the wrangler.toml bindings
declare module "cloudflare:workers" {
  interface KVNamespace {
    get: (key: string, type?: "json") => Promise<unknown>;
    put: (key: string, value: string) => Promise<void>;
  }

  export const env: {
    GITHUB_STATS?: KVNamespace;
    PROJECTS_STATS?: KVNamespace;
    /** Unset in CI and in local builds without `.dev.vars`. */
    GITHUB_TOKEN?: string;
  };
}
