<script lang="ts">
  // Stores
  // import { storeCurrentUrl } from "$docs/stores/stores";
  import { page } from "$app/stores";
  import { AppRail, AppRailAnchor, drawerStore } from "@skeletonlabs/skeleton";
  import {
    ChatBubbleBottomCenterText,
    Home,
    RocketLaunch,
  } from "@steeze-ui/heroicons";
  import { Icon } from "@steeze-ui/svelte-icon";

  // eslint-disable-next-line init-declarations
  let basePath: string | undefined;
  page.subscribe((pg) => {
    basePath = pg.url.pathname.split("/")[1];
  });

  let outerClass = "";
  export { outerClass as class };

  interface Anchor {
    name: string;
    icon: typeof Home;
    href: string;
    basePath: string;
  }

  const anchors: Anchor[] = [
    {
      name: "Home",
      icon: Home,
      href: "/",
      basePath: "",
    },
    {
      name: "Projects",
      icon: RocketLaunch,
      href: "/projects/",
      basePath: "projects",
    },
    {
      name: "Blog",
      icon: ChatBubbleBottomCenterText,
      href: "/blog/",
      basePath: "blog",
    },
  ];
</script>

<div class="h-full bg-surface-100-800-token {outerClass}">
  <AppRail background="bg-transparent">
    {#each anchors as anchor}
      <AppRailAnchor
        href="{anchor.href}"
        selected="{basePath === anchor.basePath}"
        on:click="{() => drawerStore.close()}"
      >
        <svelte:fragment slot="lead">
          <Icon src="{anchor.icon}" width="50%" />
        </svelte:fragment>
        <span>{anchor.name}</span>
      </AppRailAnchor>
    {/each}
  </AppRail>
</div>
