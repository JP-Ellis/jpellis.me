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
  import MyFooter from "$lib/components/MyFooter/MyFooter.svelte";
  import "$lib/styles/global.postcss";

  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<!-- App Shell -->
<AppShell>
  <!-- Header -->
  <svelte:fragment slot="header">
    <MyAppBar />
  </svelte:fragment>
  <svelte:fragment slot="sidebarLeft" />
  <!-- Page Content -->
  <div class="container mx-auto">
    <slot />
  </div>
  <!-- Footer -->
  <svelte:fragment slot="pageFooter">
    <MyFooter />
  </svelte:fragment>
</AppShell>
