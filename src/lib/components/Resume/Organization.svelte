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
  export let location = "";
</script>

<section class="organization">
  <!--
    Left side (or above on small screens) is the organization name, location
    and logo
  -->
  <div class="description">
    <div class="title">
      <h2>{title}</h2>
      <h3>{location}</h3>
    </div>
    <div class="logo">
      <slot name="logo" />
    </div>
  </div>

  <!--
    Right side (or below on small screens) is the main content of the section,
    which consists of a list of positions held at the organization.
  -->
  <div class="body">
    <slot />
  </div>
</section>

<style lang="postcss">
  .organization {
    @apply flex flex-col md:flex-row justify-center;
    @apply flex-none;
    @apply my-8;
    /* @apply bg-tertiary-50-900-token; */
  }

  .description {
    @apply flex flex-col justify-between;
    @apply p-4;

    @screen md {
      @apply w-1/4 min-w-[18rem] max-w-lg;
      @apply h-full;
      @apply sticky top-0;
    }
  }

  .logo {
    @apply p-6;
    @apply w-full max-w-[15rem] max-h-[15rem];
    @apply self-center;

    :global(svg) {
      @apply w-full h-full;
    }
  }

  .body {
    @apply flex flex-col;
    @apply border-l-2 border-surface-700-200-token;

    @screen md {
      @apply text-xl max-w-[65ch];
    }
  }
</style>
