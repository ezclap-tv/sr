<script>
  import Github from "@jprochazk/svelte-material-icons/Github.svelte";
  import { Link, Icon } from "./lib";
</script>

<footer class="fixed bottom-0 box flex justify-end w-full">
  <Link
    to="https://github.com/ezclap-tv"
    class="flex items-center  opacity-[0.5] hover:opacity-[1]"
  >
    <Icon icon={Github} class="mr-0.5 w-8 text-brand-secondary" />
    <span class="font-semibold text-brand-secondary">GitHub</span>
  </Link>
</footer>
