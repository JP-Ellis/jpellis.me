<script lang="ts">
  import "$lib/styles/theme.postcss";

  import { AppBar, AppShell } from "@skeletonlabs/skeleton";
  import "@skeletonlabs/skeleton/styles/all.css";

  import MyAppBar from "$lib/components/MyAppBar/MyAppBar.svelte";
  import "$lib/styles/global.postcss";
</script>

<!-- App Shell -->
<AppShell>
  <svelte:fragment slot="header">
    <MyAppBar />
  </svelte:fragment>
  <!-- Page Route Content -->
  <slot />
</AppShell>
