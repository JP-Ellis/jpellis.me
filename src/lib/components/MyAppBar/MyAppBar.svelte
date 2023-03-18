<script lang="ts">
  import { AppBar, LightSwitch } from "@skeletonlabs/skeleton";
</script>

<AppBar>
  <!-- Lead Section -->
  <svelte:fragment slot="lead">
    <span class="text-xl">
      Joshua
      <strong class="text-xl uppercase">Ellis</strong>
    </span>
  </svelte:fragment>

  <!-- Trail Section -->
  <svelte:fragment slot="trail">
    <LightSwitch />
    <a
      class="btn variant-ghost-surface"
      href="https://github.com/JP-Ellis"
      target="_blank"
    >
      GitHub
    </a>
    <a
      class="btn variant-ghost-surface"
      href="mailto:website@jpellis.me"
      target="_blank"
    >
      Email
    </a>
    <a
      class="btn variant-ghost-surface"
      href="https://au.linkedin.com/in/joshuapellis"
      target="_blank"
    >
      LinkedIn
    </a>
  </svelte:fragment>
</AppBar>
