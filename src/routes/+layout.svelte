<script lang="ts">
  import "$lib/styles/theme.postcss";

  import { page } from "$app/stores";
  import {
    arrow,
    autoUpdate,
    computePosition,
    flip,
    offset,
    shift,
  } from "@floating-ui/dom";
  import {
    AppShell,
    Drawer,
    storeHighlightJs,
    storePopup,
  } from "@skeletonlabs/skeleton";
  import "@skeletonlabs/skeleton/styles/skeleton.css";
  import hljs from "highlight.js";
  import "highlight.js/styles/github-dark.css";

  import Footer from "$lib/components/Footer/Footer.svelte";
  import Navbar from "$lib/components/Navbar/Navbar.svelte";
  import Sidebar from "$lib/components/Sidebar/Sidebar.svelte";
  import "$lib/styles/global.postcss";

  // Highlight.js
  storeHighlightJs.set(hljs);

  // Popup shared storage settings
  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

  // Meta tags
  interface PageMeta {
    title: string;
    description: string;
    image: {
      url: string;
      alt: string;
      type: string;
    };
  }
  const metaDefaults = {
    title: "Joshua Ellis",
    description:
      "Joshua Ellis is a software and data engineer based in Australia with a passion for solving problems. With a background in theoretical particle physics, Josh is a quick learner and loves programming in Python and Rust.",
    image: {
      url: "/images/joshua-ellis/headshot-square-2022.jpg",
      alt: "Headshot of a beared man in a burgundy suit wearing glasses.",
      type: "image/jpg",
    },
  } satisfies PageMeta;
  const meta = {
    title: metaDefaults.title,
    description: metaDefaults.description,
    image: metaDefaults.image,
  } satisfies PageMeta;
  page.subscribe(() => {
    meta.title = metaDefaults.title;
    meta.description = metaDefaults.description;
    meta.image = metaDefaults.image;
  });
</script>

<svelte:head>
  <title>{meta.title}</title>

  <!-- meta tags -->
  <meta name="title" content="{meta.title}" />
  <meta name="description" content="{meta.description}" />
  <meta name="author" content="Joshua Ellis" />
  <meta name="theme-color" content="#D43400" />
  <meta name="image" content="{meta.image.url}" />

  <!-- open graph (https://ogp.me) -->
  <meta property="og:type" content="website" />
  <meta property="og:site_name" content="Joshua Ellis" />
  <meta property="og:url" content="https://jpellis.me{$page.url.pathname}" />
  <meta property="og:title" content="{meta.title}" />
  <meta property="og:description" content="{meta.description}" />
  <meta property="og:image" content="{meta.image.url}" />
  <meta property="og:image:url" content="{meta.image.url}" />
  <meta property="og:image:alt" content="{meta.image.alt}" />
  <meta property="og:image:type" content="{meta.image.type}" />
</svelte:head>

<!-- Overlays -->
<Drawer class="w-[50%]">
  <Sidebar class="w-[80px]" />
</Drawer>

<!-- App Shell -->
<AppShell>
  <!-- Header -->
  <svelte:fragment slot="header">
    <Navbar />
  </svelte:fragment>

  <!-- Navigation bar (on left) -->
  <svelte:fragment slot="sidebarLeft">
    <Sidebar class="hidden lg:grid overflow-hidden" />
  </svelte:fragment>

  <!-- Page Content -->
  <slot />

  <!-- Footer -->
  <svelte:fragment slot="pageFooter">
    <Footer />
  </svelte:fragment>
</AppShell>
