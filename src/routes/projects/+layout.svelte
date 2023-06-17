<script lang="ts">
  import { storeHighlightJs } from "@skeletonlabs/skeleton";
  import hljs from "highlight.js";
  import "highlight.js/styles/github-dark.css";
  import "katex/dist/katex.min.css";

  // Highlight.js
  storeHighlightJs.set(hljs);
</script>

<div class="flex flex-col gap-10 p-6 md:p-12 max-w-[120ch] mx-auto">
  <slot />
</div>
