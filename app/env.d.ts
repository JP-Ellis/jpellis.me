// biome-ignore-all lint/style/useNamingConvention: env names match the wrangler.toml bindings
declare module "cloudflare:workers" {
  interface KVNamespace {
    get: (key: string, type?: "json") => Promise<unknown>;
    put: (key: string, value: string) => Promise<void>;
  }

  export const env: {
    GITHUB_STATS?: KVNamespace;
    PROJECTS_STATS?: KVNamespace;
    GITHUB_TOKEN: string;
  };
}
