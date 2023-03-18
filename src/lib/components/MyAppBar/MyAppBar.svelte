<script lang="ts">
  import { AppBar, LightSwitch } from "@skeletonlabs/skeleton";
  import Image from "svimg";
</script>

<AppBar>
  <!-- Lead Section -->
  <svelte:fragment slot="lead">
    <div class="flex items-center space-x-4">
      <!-- Headshot -->
      <Image
        class="rounded-full w-10 h-10 aspect-square"
        src="headshot.jpg"
        alt="Joshua Ellis"
        width="64"
      />
      <!-- Name -->
      <span class="text-xl hidden lg:inline-block">
        Joshua
        <strong class="text-xl uppercase">Ellis</strong>
      </span>
    </div>
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
