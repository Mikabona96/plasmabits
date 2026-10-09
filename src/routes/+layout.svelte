<script lang="ts">
  import { page } from "$app/state";
  import Dolphin from "$lib/shared/icons/Dolphin.svelte";
  import Gear from "$lib/shared/icons/Gear.svelte";
  import "../style.css";

  let { children } = $props();
  const isMainPage = $derived(page.url.pathname === "/");
</script>

<div
  class="flex flex-col text-white h-screen w-full max-w-162.5 bg-graphite-950 overflow-hidden"
>
  <main
    class="flex flex-col min-h-0 flex-1 font-WDXLLubrifontJPN-Regular overflow-hidden"
  >
    {@render children()}
  </main>

  <nav
    class="shrink-0 h-20 w-full bg-graphite-900 flex items-center justify-around"
  >
    <a href="/" class="flex flex-col items-center gap-1">
      <div class="relative">
        <Dolphin size={44} />
        <Gear size={20} class="absolute bottom-0 right-0" />
      </div>
      <span class="font-WDXLLubrifontJPN-Regular text-sm"> Service Menu </span>
    </a>
  </nav>
</div>
