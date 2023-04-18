<script lang="ts">
  import { Code2, ServerCog } from "@steeze-ui/lucide-icons";
  import { Python, Rust } from "@steeze-ui/simple-icons";
  import { Icon } from "@steeze-ui/svelte-icon";
  import Img from "@zerodevx/svelte-img";

  import headshot from "$lib/assets/images/joshua-ellis/headshot-square-2022.jpg?run";
  import portrait from "$lib/assets/images/joshua-ellis/portrait-2022.png?run&lqip=0";

  const descriptions = [
    { icon: Code2, text: "Software Engineer" },
    { icon: ServerCog, text: "Data Engineer" },
    { icon: Python, text: "Python Developer" },
    { icon: Rust, text: "Rust Developer" },
  ];
</script>

<div class="hero">
  <div class="hero-image">
    <Img class="headshot" src="{headshot}" alt="Joshua Ellis" />
    <Img class="portrait" src="{portrait}" alt="Joshua Ellis" />
  </div>
  <div class="hero-text">
    <div>
      <h1>Joshua <strong style="font-variant: small-caps;">Ellis</strong></h1>
      <ul class="list-inside">
        {#each descriptions as { icon, text }}
          <li>
            <Icon src="{icon}" />{text}
          </li>
        {/each}
      </ul>
    </div>
  </div>
</div>
<hr class="divider" />

<style lang="postcss">
  hr {
    @apply border-b-4 border-primary-500;
  }

  .hero {
    /* By default, use the full screen of the page, but prevent really short
     * screens from clipping the content.
     */
    @apply container mx-auto;
    @apply flex flex-col md:flex-row justify-center items-center;
    @apply h-[calc(100vh-74px)] max-md:min-h-[650px];
    @apply pt-8;

    /* Vertically align the image and text on the page (either within the
     * top/bottom half, or side-by-side)
     */
    .hero-image,
    .hero-text {
      @apply relative;
      @apply flex flex-col items-center justify-center;
      @apply h-1/2;
    }

    /* For the image container, avoid any overflow of the image.
     */
    .hero-image {
      @apply p-8;
      @apply overflow-clip;
    }

    .hero-text {
      @apply min-w-[25ch];
    }

    :global(.headshot) {
      @apply mx-auto;
      @apply aspect-square rounded-full;
      @apply max-w-[30ch];
    }

    :global(.portrait) {
      @apply mx-auto;
      @apply hidden object-contain;
    }

    /* at md, switch to horizontal alignment of the two segments */
    @media screen(md) {
      @apply pt-0;

      .hero-image,
      .hero-text {
        @apply w-1/2 h-full;
      }
    }

    /* at lg, hide the headshot and show the portrait instead */
    @media screen(lg) {
      .hero-image {
        @apply p-0;
        @apply justify-between before:content-[''];
      }

      :global(.headshot) {
        @apply hidden;
      }

      :global(.portrait) {
        @apply block;
      }
    }
  }

  h1 {
    @apply pb-4;
  }
  ul {
    li {
      @apply my-2;
      @apply flex items-center;
      @apply text-xl md:text-2xl;

      :global(svg) {
        @apply mr-3;
        @apply w-4 md:w-6;
      }
    }
  }
</style>
