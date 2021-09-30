<script lang="ts">
  export let to: string;
  export let blank: boolean = false;
  export let opener: boolean = false;
  export let referrer: boolean = false;

  const href = to;
  const target = blank ? "_blank" : undefined;
  const rel = [
    ["noopener", !opener],
    ["noreferrer", !referrer],
  ]
    .filter(([, cond]) => cond)
    .map(([v]) => v)
    .join(" ");
</script>

<a {href} {target} {rel} class={$$props.class}>
  <slot />
</a>
