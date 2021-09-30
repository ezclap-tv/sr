<script context="module" lang="ts">
  const state = {
    [0]: "stopped",
    [1]: "playing",
    [2]: "paused",
    [3]: "buffering",
  } as const;
  type State = typeof state;
  type States = State[keyof State];

  export type Quality =
    | YT.VideoQualitySmall
    | YT.VideoQualityMedium
    | YT.VideoQualityLarge
    | YT.VideoQualityHighRes
    | YT.VideoQualityHD720
    | YT.VideoQualityHD1080;

  type EventBase = { player: YT.Player };
  type EventData<Data> = EventBase & { data: Data };

  export type ReadyEvent = CustomEvent<EventBase>;
  export type ErrorEvent = CustomEvent<EventData<YT.PlayerError>>;
  export type StateEvent = CustomEvent<EventData<States>>;
  export type QualityEvent = CustomEvent<EventData<Quality>>;
  export type RateEvent = CustomEvent<EventData<number>>;
</script>

<script lang="ts">
  import { onDestroy, createEventDispatcher } from "svelte";
  import { nonce } from "../util";
  import { load } from "./ytapi";

  export let options: YT.PlayerOptions = {};

  /**
   * Get the YouTube player instance.
   * Don't store this anywhere!
   */
  export const player = () => _player;

  function event(data?: any): EventData<any> {
    return { player: _player!, data };
  }
  const dispatch = createEventDispatcher<{
    ready: EventBase;
    error: EventData<YT.PlayerError>;
    state: EventData<States>;
    quality: EventData<Quality>;
    rate: EventData<number>;
  }>();

  let id = "player-" + nonce(5);
  let _player: YT.Player | null = null;
  $: createPlayer(options);

  let loading = false;
  function createPlayer(options: YT.PlayerOptions) {
    if (loading) return;
    loading = true;
    load().then(() => {
      _player = new YT.Player(id, options);
      _player.addEventListener("onReady", (e: YT.PlayerEvent) =>
        dispatch("ready", event())
      );
      _player.addEventListener(
        "onError",
        (e: YT.PlayerEvent & { data: YT.PlayerError }) =>
          dispatch("error", event(e.data))
      );
      _player.addEventListener(
        "onStateChange",
        (e: YT.PlayerEvent & { data: keyof State }) =>
          state[e.data] && dispatch("state", event(state[e.data]))
      );
      _player.addEventListener(
        "onPlaybackQualityChange",
        (e: YT.PlayerEvent & { data: Quality }) =>
          dispatch("quality", event(e.data))
      );
      _player.addEventListener(
        "onPlaybackRateChange",
        (e: YT.PlayerEvent & { data: number }) =>
          dispatch("rate", event(e.data))
      );
      loading = false;
    });
  }

  onDestroy(() => {
    if (_player) _player.destroy();
  });
</script>

<div class="h-full w-full bg-none m-0 p-0">
  <div {id} />
</div>
