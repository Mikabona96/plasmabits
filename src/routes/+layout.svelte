<script lang="ts">
  import { page } from "$app/state";
  import Dolphin from "$lib/shared/icons/Dolphin.svelte";
  import Gear from "$lib/shared/icons/Gear.svelte";
  import "../style.css";

  let { children } = $props();
  const isMainPage = $derived(page.url.pathname === "/");
</script>

<div class="flex text-white h-screen w-full overflow-hidden">
  <nav class="flex flex-col w-55 h-full bg-white/60 backdrop-blur-lg"></nav>
  <main
    class="flex flex-col min-h-0 bg-graphite-950 flex-1 font-Inter overflow-hidden"
  >
    {@render children()}
  </main>
</div>
