<script lang="ts">
  import YoutubePlayer from "../lib/YoutubePlayer.svelte";
  import type { ReadyEvent, StateEvent } from "../lib/YoutubePlayer.svelte";
  export let params: Record<string, string> = {};

  let room: string = params.room;

  function handleReady(e: ReadyEvent) {
    console.log(e.detail);
  }

  function handleState(e: StateEvent) {
    console.log(e.detail);
  }
</script>

<YoutubePlayer on:ready={handleReady} on:state={handleState} />
