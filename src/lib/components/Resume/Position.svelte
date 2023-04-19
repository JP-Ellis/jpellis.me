<!--
This component is used to create a section in the resume. It takes the following
short properties:

- position: the position held at the company
- company: the company name
- logo: the company logo
- location: the location of the company
- start: the start date of the position
- end: the end date of the position
- keywords: a list of keywords to display in the section

The main slot is used to display the content of the section and will format
paragraph and lists automatically.
-->
<script lang="ts">
  export let title = "";
  export let subtitle: string | null = null;
  export let start = new Date();
  export let end: Date | null = null;
  export let keywords: string[] = [];

  /* eslint-disable */
</script>

<div class="flex flex-col m-4 mb-8">
  <div class="flex flex-col md:flex-row md:justify-between pb-4">
    <div class="flex flex-col font-bold">
      <h2>{title}</h2>
      {#if subtitle}
        <h3>{subtitle}</h3>
      {/if}
    </div>
    <div class="flex flex-col md:text-right">
      <h4>
        {#if end}
          {end.toLocaleDateString("en-US", { year: "numeric", month: "short" })}
        {:else}
          {"Present"}
        {/if}
      </h4>
      <h4>
        {start.toLocaleDateString("en-US", { year: "numeric", month: "short" })}
      </h4>
    </div>
  </div>

  <div class="flex flex-col text-justify">
    <slot />

    <div
      class="hidden md:flex flex-row justify-end max-w-[75%] self-end flex-wrap gap-1 pt-6"
    >
      {#each keywords as keyword}
        <span
          class="text-sm text-surface-700-200-token bg-surface-200-700-token rounded-md p-1"
          >{keyword}</span
        >
      {/each}
    </div>
  </div>
</div>
