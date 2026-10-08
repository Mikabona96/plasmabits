<script lang="ts">
  import { onMount, tick } from "svelte";

  let { children } = $props();

  type TabState = "create" | "edit";
  let tabState = $state<TabState>("create");

  // DOM references for dynamic measurement
  let containerRef = $state<HTMLDivElement | null>(null);
  let createBtnRef = $state<HTMLButtonElement | null>(null);
  let editBtnRef = $state<HTMLButtonElement | null>(null);

  // Dynamic position and width for square indicator
  let indicatorStyle = $state({
    left: 0,
    width: 0,
  });

  // Calculate indicator position using bounding rects
  async function updateIndicator() {
    await tick(); // Wait for DOM/i18n updates

    const activeBtn = tabState === "create" ? createBtnRef : editBtnRef;

    if (activeBtn && containerRef) {
      const containerRect = containerRef.getBoundingClientRect();
      const btnRect = activeBtn.getBoundingClientRect();

      indicatorStyle = {
        left: btnRect.left - containerRect.left,
        width: btnRect.width,
      };
    }
  }

  // Recalculate indicator position when active tab changes
  $effect(() => {
    const currentTab = tabState;
    updateIndicator();
  });

  // Observe container size changes (window resize, locale changes)
  onMount(() => {
    updateIndicator();

    const resizeObserver = new ResizeObserver(() => {
      updateIndicator();
    });

    if (containerRef) {
      resizeObserver.observe(containerRef);
    }

    return () => resizeObserver.disconnect();
  });
</script>

<div class="flex h-full w-full flex-col text-white overflow-hidden">
  <h1 class="text-4xl pl-10 pt-10 flex-none">Edit Dolphin Context Menu</h1>

  <!-- Sharp Square Tabs Navigation Container -->
  <div
    bind:this={containerRef}
    class="relative mt-10 flex w-fit self-center border border-graphite-400 text-lg flex-none bg-graphite-800/40 backdrop-blur-sm"
  >
    <!-- Sharp Sliding Background Block -->
    <div
      class="absolute top-0 bottom-0 bg-white transition-all duration-300 ease-[cubic-bezier(0.25,1,0.5,1)]"
      style="left: {indicatorStyle.left -
        1}px; width: {indicatorStyle.width}px;"
    ></div>

    <!-- Tab Buttons -->
    <button
      bind:this={createBtnRef}
      onclick={() => (tabState = "create")}
      class={[
        "relative z-10 cursor-pointer px-6 py-2 transition-colors duration-300 font-medium select-none",
        tabState === "create" ? "text-black" : "text-white/70 hover:text-white",
      ]}
    >
      Create New Option
    </button>

    <button
      bind:this={editBtnRef}
      onclick={() => (tabState = "edit")}
      class={[
        "relative z-10 cursor-pointer px-6 py-2 transition-colors duration-300 font-medium select-none",
        tabState === "edit" ? "text-black" : "text-white/70 hover:text-white",
      ]}
    >
      Edit Existing Option
    </button>
  </div>

  <div class="flex-1 overflow-y-auto w-full px-10 pb-10 mt-6">
    {@render children()}
  </div>
</div>
