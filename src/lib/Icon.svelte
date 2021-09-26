<script lang="ts">
  export let icon: any;

  const props = Object.filter($$props, ["icon"]);
</script>

<svelte:component this={icon} width={null} height={null} {...props} />
