<script lang="ts">
  import "$lib/styles/theme.postcss";

  import {
    arrow,
    autoUpdate,
    computePosition,
    flip,
    offset,
    shift,
  } from "@floating-ui/dom";
  import { AppShell, storePopup } from "@skeletonlabs/skeleton";
  import "@skeletonlabs/skeleton/styles/all.css";

  import MyAppBar from "$lib/components/MyAppBar/MyAppBar.svelte";
  import "$lib/styles/global.postcss";

  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<!-- App Shell -->
<AppShell>
  <svelte:fragment slot="header">
    <MyAppBar />
  </svelte:fragment>
  <!-- Page Route Content -->
  <slot />
</AppShell>
