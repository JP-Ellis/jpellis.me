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

  import AppBar from "$lib/components/AppBar/AppBar.svelte";
  import Footer from "$lib/components/Footer/Footer.svelte";
  import "$lib/styles/global.postcss";

  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<!-- App Shell -->
<AppShell>
  <!-- Header -->
  <svelte:fragment slot="header">
    <AppBar />
  </svelte:fragment>
  <svelte:fragment slot="sidebarLeft" />
  <!-- Page Content -->
  <div class="container mx-auto">
    <slot />
  </div>
  <!-- Footer -->
  <svelte:fragment slot="pageFooter">
    <Footer />
  </svelte:fragment>
</AppShell>
