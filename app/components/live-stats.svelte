<script lang="ts">
// biome-ignore-all lint/correctness/noUnusedVariables: `stats` is rendered in the Svelte template, which Biome does not parse
interface Headline {
  commitContributions: number;
  prContributions: number;
  issueContributions: number;
  publicRepos: number;
}
let { initial }: { initial: Headline } = $props();
let stats = $state<Headline>(initial);
$effect(() => {
  let aborted = false;
  fetch("/api/github-stats")
    .then((r) => r.json())
    .then((d) => {
      if (!aborted && d && typeof d.commitContributions === "number") {
        stats = {
          commitContributions: d.commitContributions,
          prContributions: d.prContributions,
          issueContributions: d.issueContributions,
          publicRepos: d.publicRepos,
        };
      }
    })
    .catch(() => {
      // Network failure: keep the server-rendered initial stats.
    });
  return () => {
    aborted = true;
  };
});
</script>

<p class="stats-headline">
  <em>{stats.commitContributions}</em> commits, <em
    >{stats.prContributions}</em
  > PRs, and <em>{stats.issueContributions}</em> issues across <em
    >{stats.publicRepos}</em
  > repositories.
</p>

<style lang="scss">
  .stats-headline {
    font-family: var(--font-display);
    font-weight: 300;
    font-size: clamp(28px, 4vw, 36px);
    margin-top: var(--space-2);
    letter-spacing: -0.015em;
  }
</style>
