<script lang="ts">
  import { faCaretDown } from "@fortawesome/free-solid-svg-icons";
  import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
  import {
    AppBar,
    LightSwitch,
    type PopupSettings,
    popup,
  } from "@skeletonlabs/skeleton";
  import Img from "@zerodevx/svelte-img";

  import headshot from "$lib/assets/images/2022_headshot_square.jpg?run";
  import MyLinks from "$lib/components/MyLinks/MyLinks.svelte";

  // Links popup
  const popupSettings = {
    event: "click",
    target: "linksPopup",
  } as PopupSettings;
</script>

<AppBar shadow="shadow-xl">
  <!-- Lead Section -->
  <svelte:fragment slot="lead">
    <div class="flex items-center space-x-4">
      <!-- Headshot -->
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
    </div>
  </svelte:fragment>

  <!-- Trail Section -->
  <svelte:fragment slot="trail">
    <LightSwitch />
    <MyLinks
      class="hidden sm:inline-block"
      labelClass="hidden lg:inline-block"
    />
    <div class="inline-block sm:hidden">
      <button class="btn-icon" use:popup="{popupSettings}">
        <FontAwesomeIcon icon="{faCaretDown}" size="lg" />
      </button>
      <div data-popup="linksPopup" class="card p-2 w-40 shadow-xl">
        <MyLinks
          class="flex flex-col"
          linkClass="p-2"
          iconClass="text-xl w-1/3"
          labelClass="w-2/3"
        />
      </div>
    </div>
  </svelte:fragment>
</AppBar>
