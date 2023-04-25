<script lang="ts">
  import { page } from "$app/stores";
</script>

{#if $page.error}
  <div
    class="flex flex-col justify-center align-center mx-auto h-[calc(100vh-74px)]"
  >
    <div class="text-center text-xl pb-6">There was an error</div>
    <div class="flex flex-row content-center">
      <h1 class="w-1/2 inline-block text-end border-r pr-4 border-primary-500">
        {$page.status}
      </h1>
      <h2 class="w-1/2 inline-block pl-4">{$page.error.message}</h2>
    </div>
  </div>
{:else}
  <div
    class="flex flex-col justify-center align-center mx-auto h-[calc(100vh-74px)]"
  >
    <div class="text-center text-xl border-b border-primary-500 pb-6 mx-auto">
      There was an error
    </div>
    <h1 class="text-center mx-auto pt-6">
      {$page.status}
    </h1>
  </div>
{/if}
