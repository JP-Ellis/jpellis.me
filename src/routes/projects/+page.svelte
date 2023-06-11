<script lang="ts">
  import type { PageData } from "./$types";

  // eslint-disable-next-line init-declarations
  export let data: PageData;
</script>

<h1 class="h1">Projects</h1>

<p class="text-xl lg:text-2xl">
  {#await data.github}
    I have a passion for open source software and have contributed to a number
    of projects. I have created 18 repositories on GitHub which have received
    286 stars and have been forked 47 times.
  {:then value}
    I have a passion for open source software and have contributed to a number
    of projects. I have created {value.repos} repositories on GitHub which have received
    {value.stars} stars and have been forked
    {value.forks} times.
  {:catch}
    I have a passion for open source software and have contributed to a number
    of projects. I have created 18 repositories on GitHub which have received
    286 stars and have been forked 47 times.
  {/await}
</p>

<!-- Use 2-column grid -->
<div class="grid grid-cols-1 lg:grid-cols-2 grid-flow-row gap-10">
  <a class="card p-4 variant-glass-primary" href="/projects/tikz-feynman/">
    <header class="card-header">
      <h2 class="h2">Ti<i>k</i>Z-Feynman</h2>
    </header>
    <section class="p-4">
      Ti<i>k</i>Z-Feynman is a LaTeX package that simplifies the generation of
      Feynman diagrams within LaTeX. It utilizes the Ti<i>k</i>Z package and its
      graph placement algorithms to automate vertex placement. Despite its
      automation, Ti<i>k</i>Z-Feynman still allows precise vertex placement for
      complex diagrams. The package is available on CTAN and includes
      comprehensive documentation with tutorials and examples. Contributions are
      welcome on its GitHub repository. The README provides a glimpse of how
      effortless and customizable the diagrams can be, with more examples
      available in the documentation.
    </section>
  </a>

  <a class="card p-4 variant-glass-primary" href="/projects/simpler-wick/">
    <header class="card-header">
      <h2 class="h2">Simpler-Wick</h2>
    </header>
    <section class="p-4">
      Simpler-Wick is a LaTeX package designed to simplify the process of
      typesetting Wick contractions in mathematical expressions. It offers an
      intuitive and streamlined syntax, allowing users to easily represent
      contractions between operators. Simpler-Wick automates the placement of
      contraction lines, eliminating the need for manual adjustments. The
      package comes with comprehensive documentation, providing clear
      instructions and examples for its usage. Simpler-Wick is open source and
      can be accessed on CTAN, making it readily available for LaTeX users. With
      Simpler-Wick, the task of typesetting Wick contractions becomes
      effortless, enabling researchers and mathematicians to focus on their
      content rather than the technicalities.
    </section>
  </a>

  <a class="card p-4 variant-glass-primary" href="/projects/rust-skiplist/">
    <header class="card-header">
      <h2 class="h2">Rust-Skiplist</h2>
    </header>
    <section class="p-4">
      SkipList is a Rust package that provides an efficient implementation of a
      skip list data structure. Skip lists are a type of sorted linked list that
      allow for fast searching, insertion, and deletion operations with a
      logarithmic time complexity. The SkipList package offers a simple and
      intuitive API, allowing developers to easily integrate skip lists into
      their Rust projects. It provides methods for inserting, removing, and
      searching for elements in the list. The package is well-documented,
      providing clear explanations and examples to guide users. With SkipList,
      Rust developers can leverage the power of skip lists to efficiently manage
      sorted data structures in their applications.
    </section>
  </a>
</div>
