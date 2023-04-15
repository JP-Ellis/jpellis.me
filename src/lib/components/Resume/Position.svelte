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

<div class="position">
  <div class="header">
    <div class="title">
      <h2>{title}</h2>
      {#if subtitle}
        <h3>{subtitle}</h3>
      {/if}
    </div>
    <div class="date">
      <h4>
        {#if end}
          {end.toLocaleDateString("en-US", { year: "numeric", month: "short" })}
        {:else}
          {"Present"}
        {/if}
      </h4>
      <h4 class="date">
        {start.toLocaleDateString("en-US", { year: "numeric", month: "short" })}
      </h4>
    </div>
  </div>

  <div class="body">
    <slot />

    <div class="keywords">
      {#each keywords as keyword}
        <span class="keyword">{keyword}</span>
      {/each}
    </div>
  </div>
</div>

<style lang="postcss">
  .position {
    @apply flex flex-col;
    @apply ml-4 mb-8;
  }

  .header {
    @apply flex flex-col md:flex-row;
    @apply md:justify-between;
    @apply pb-2 mb-2;
  }

  .title {
    @apply flex flex-col grow-[2];
  }

  .date {
    @apply flex flex-col md:text-right;
  }

  .body {
    @apply flex flex-col;

    :global(ul) {
      @apply list-disc list-outside;
      @apply pl-4;

      :global(li) {
        @apply mb-4;
      }
    }
  }

  .keywords {
    @apply hidden md:flex;
    @apply flex-row flex-wrap;
    @apply gap-1;
    @apply pt-6;

    .keyword {
      @apply text-sm;
      @apply text-surface-700-200-token bg-surface-200-700-token;
      @apply rounded-md;
      @apply p-1;
    }
  }
</style>
