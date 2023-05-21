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

<section class="flex flex-col md:flex-row justify-center my-8">
  <!--
    Left side (or above on small screens) is the organization name, location
    and logo
  -->
  <div
    class="flex flex-col justify-between p-4 md:w-1/4 md:min-w-[18rem] md:max-w-lg md:h-full md:sticky md:top-0"
  >
    <div>
      <h2>{title}</h2>
      <span class="text-lg md:text-xl">{location}</span>
    </div>
    <div class="logo p-6 w-full max-w-[15rem] max-h-[15rem] self-center">
      <slot name="logo" />
    </div>
  </div>

  <!--
    Right side (or below on small screens) is the main content of the section,
    which consists of a list of positions held at the organization.
  -->
  <div
    class="flex flex-col md:border-l-2 md:border-surface-300-600-token md:text-xl md:max-w-[70ch]"
  >
    <slot />
  </div>
</section>

<style lang="postcss">
  .logo {
    :global(svg) {
      @apply w-full h-full;
    }
  }
</style>
