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

<div
  class="container mx-auto flex flex-col md:flex-row justify-center items-center h-[calc(100vh-74px)] max-md:min-h-[650px]"
>
  <div
    class="flex flex-col items-center justify-center h-1/2 md:h-full md:w-1/2 p-8 lg:p-0 overflow-clip lg:justify-between lg:before:content-['']"
  >
    <Img
      class="lg:hidden aspect-square rounded-full max-w-[30ch]"
      src="{headshot}"
      alt="Joshua Ellis"
    />
    <Img
      class="max-lg:hidden mx-auto object-container"
      src="{portrait}"
      alt="Joshua Ellis"
    />
  </div>
  <div
    class="flex flex-col items-center justify-center h-1/2 p-8 md:h-full md:w-1/2"
  >
    <div>
      <h1 class="h1 pb-4">
        Joshua <strong style="font-variant: small-caps;">Ellis</strong>
      </h1>
      <ul class="list-inside">
        {#each descriptions as { icon, text }}
          <li class="my-2 flex items-center text-xl md:text-2xl">
            <Icon src="{icon}" class="mr-3 w-4 md:w-6" />{text}
          </li>
        {/each}
      </ul>
    </div>
  </div>
</div>
<hr class="divider border-b-4 border-primary-500" />
