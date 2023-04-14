<script lang="ts">
  import { faGithub, faLinkedin } from "@fortawesome/free-brands-svg-icons";
  import { faEnvelope } from "@fortawesome/free-solid-svg-icons";
  import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

  let outerClass = "";
  export let linkClass = "btn variant-ghost-primary";
  export let iconClass = "text-xl";
  export let labelClass = "";
  export { outerClass as class };
</script>

<div class="{outerClass}">
  <a class="{linkClass}" href="https://github.com/JP-Ellis" target="_blank">
    <span><FontAwesomeIcon icon="{faGithub}" class="{iconClass}" /></span>
    <span class="{labelClass}">GitHub</span>
  </a>
  <a class="{linkClass}" href="mailto:website@jpellis.me" target="_blank">
    <span><FontAwesomeIcon icon="{faEnvelope}" class="{iconClass}" /></span>
    <span class="{labelClass}">Email</span>
  </a>
  <a
    class="{linkClass}"
    href="https://au.linkedin.com/in/joshuapellis"
    target="_blank"
  >
    <span><FontAwesomeIcon icon="{faLinkedin}" class="{iconClass}" /></span>
    <span class="{labelClass}">LinkedIn</span>
  </a>
</div>
