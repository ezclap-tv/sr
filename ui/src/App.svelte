<script lang="ts">
  import Router from "svelte-spa-router";
  import wrap from "svelte-spa-router/wrap";
  import Footer from "./Footer.svelte";

  import Home from "./views/Home.svelte";

  const routes = {
    "/": Home,
    "/player/:room": wrap({
      asyncComponent: () => import("./views/Player.svelte"),
    }),
  };
</script>

<Router {routes} />
<Footer />

<style global>
  html,
  body,
  #app {
    @apply h-full bg-brand-dark text-brand-lighter;
  }

  svg > path {
    color: currentColor;
    fill: currentColor;
  }

  .box {
    @apply px-4 py-3;
  }
</style>
