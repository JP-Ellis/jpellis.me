<script lang="ts">
  import {
    AppBar,
    LightSwitch,
    type PopupSettings,
    drawerStore,
    popup,
  } from "@skeletonlabs/skeleton";
  import { Bars3, ChevronDown } from "@steeze-ui/heroicons";
  import { Icon } from "@steeze-ui/svelte-icon";
  import Img from "@zerodevx/svelte-img";

  import headshot from "$lib/assets/images/joshua-ellis/headshot-square-2022.jpg?as=run";
  import Socials from "$lib/components/Socials/Socials.svelte";

  // Links popup
  const popupSettings = {
    event: "click",
    target: "linksPopup",
    placement: "bottom-end",
  } as PopupSettings;
</script>

<AppBar shadow="shadow-xl">
  <!-- Lead Section -->
  <svelte:fragment slot="lead">
    <!-- Sidebar Sandwich -->
    <button
      class="flex items-center lg:hidden"
      aria-label="Open navigation sidebar"
      on:click="{() => drawerStore.open()}"
    >
      <Icon
        src="{Bars3}"
        size="lg"
        class="w-10 mr-8"
        width="100%"
        height="100%"
      />
    </button>
    <!-- Headshot -->
    <a href="/" class="flex items-center space-x-4">
      <Img
        class="rounded-full aspect-square w-10"
        src="{headshot}"
        alt="Joshua Ellis"
      />
      <!-- Name -->
      <span class="text-xl hidden lg:inline-block">
        Joshua
        <strong class="text-xl" style="font-variant: small-caps">Ellis</strong>
      </span>
    </a>
  </svelte:fragment>

  <!-- Trail Section -->
  <svelte:fragment slot="trail">
    <LightSwitch />
    <Socials
      class="hidden sm:inline-block"
      labelClass="hidden lg:inline-block"
    />
    <div class="inline-block sm:hidden">
      <button
        class="btn-icon variant-ghost-primary p-2"
        aria-label="Open social links popup"
        use:popup="{popupSettings}"
      >
        <Icon src="{ChevronDown}" size="lg" width="100%" height="100%" />
      </button>
      <div data-popup="linksPopup" class="card p-2 w-40 shadow-xl">
        <Socials
          class="flex flex-col"
          linkClass="p-2"
          iconClass="h-8 w-1/3 inline-block"
          labelClass="w-2/3"
        />
      </div>
    </div>
  </svelte:fragment>
</AppBar>
