<script lang="ts">
  import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
  import Img from "@zerodevx/svelte-img";

  import headshot from "$lib/assets/images/headshot.jpg?run";
</script>

<div class="flex flex-col md:flex-row items-center align-center m-4">
  <div class="md:max-lg:w-1/2 lg:w-1/3 p-4">
    <Img
      class="rounded-full aspect-square max-w-[400px] w-full mx-auto"
      src="{headshot}"
      alt="Joshua Ellis"
    />
  </div>
  <div class="md:max-lg:w-1/2 md:w-2/3 text-justify max-w-prose mx-auto">
    <p>
      I'm Joshua Ellis, a data engineer with a passion for programming.
      Currently, I work in the Data & Cloud team at KPMG where I build robust
      and scalable data pipelines. Before that, I studied a PhD at Melbourne
      University where I developed a tool to calculate the matter-anti-matter
      asymmetry from the fundamental interactions between particles. I'm also
      the author of TikZ-Feynman, a (now popular) LaTeX package for drawing
      Feynman diagrams.
    </p>
    <p>
      Apart from my work, I love to code in my free time. Python and Rust are my
      go-to languages, and I enjoy tinkering with new tools and technologies to
      stay on top of the latest developments.
    </p>
    <p>
      This website is my way of sharing my experiences and projects with the
      world. Feel free to browse around and get in touch if you want to chat
      about data engineering, programming, or anything else that interests you.
    </p>
  </div>
</div>

<style lang="postcss">
  p {
    @apply my-4;
  }
</style>
