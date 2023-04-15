<script lang="ts">
  import { Envelope } from "@steeze-ui/heroicons";
  import { Github, Linkedin } from "@steeze-ui/simple-icons";
  import { Icon } from "@steeze-ui/svelte-icon";

  let outerClass = "";
  export let linkClass = "btn variant-ghost-primary";
  export let iconClass = "h-7";
  export let labelClass = "";
  export { outerClass as class };
</script>

<div class="{outerClass}">
  <a class="{linkClass}" href="https://github.com/JP-Ellis" target="_blank">
    <span><Icon src="{Github}" class="{iconClass}" /></span>
    <span class="{labelClass}">GitHub</span>
  </a>
  <a class="{linkClass}" href="mailto:website@jpellis.me" target="_blank">
    <span><Icon src="{Envelope}" class="{iconClass}" /></span>
    <span class="{labelClass}">Email</span>
  </a>
  <a
    class="{linkClass}"
    href="https://au.linkedin.com/in/joshuapellis"
    target="_blank"
  >
    <span><Icon src="{Linkedin}" class="{iconClass}" /></span>
    <span class="{labelClass}">LinkedIn</span>
  </a>
</div>
