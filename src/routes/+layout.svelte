<script lang="ts">
  import Dolphin from "#lib/icons/Dolphin.svelte";
  import Gear from "#lib/icons/Gear.svelte";
  import { page } from "$app/state";
  import "../style.css";

  let { children } = $props();
  const isMainPage = $derived(page.url.pathname === "/");
</script>

<div
  class="flex flex-col h-screen w-full max-w-162.5 bg-graphite-950 overflow-hidden"
>
  <main
    class="flex flex-col min-h-0 flex-1 font-WDXLLubrifontJPN-Regular overflow-hidden"
  >
    {@render children()}
  </main>

  <nav
    class="shrink-0 h-20 w-full bg-graphite-900 flex items-center justify-around"
  >
    <a href="/service-menu" class="flex flex-col items-center gap-1 text-white">
      <div class="relative">
        <Dolphin size={44} />
        <Gear size={20} class="absolute bottom-0 right-0 text-white" />
      </div>
      <span class="font-WDXLLubrifontJPN-Regular text-sm"> Service Menu </span>
    </a>
  </nav>
</div>
