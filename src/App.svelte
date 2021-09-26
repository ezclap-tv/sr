<script lang="ts">
  import Router from "svelte-spa-router";
  import Header from "./Header.svelte";
  import Footer from "./Footer.svelte";

  import Home from "./views/Home.svelte";

  const routes = {
    "/": Home,
  };
</script>

<Header />
<main class="ml-[15%] mr-[15%] box">
  <Router {routes} />
</main>
<Footer />

<style global>
  html,
  body,
  #app {
    @apply h-full;
    @apply bg-brand-dark;
    @apply text-brand-lighter;
  }

  svg > path {
    color: currentColor;
    fill: currentColor;
  }

  .box {
    @apply px-4 py-3;
  }
</style>
