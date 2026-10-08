<script lang="ts">
  let { children } = $props();
  type TabState = "create" | "edit";
  let tabState = $state<TabState>("create");
</script>

<div class="flex h-full w-full flex-col text-white overflow-hidden">
  <h1 class="text-3xl pl-10 pt-10 flex-none">Edit Dolphin Context Menu</h1>

  <div
    class="mt-10 flex w-fit self-center overflow-hidden rounded-3xl border border-graphite-400 text-lg flex-none"
  >
    <button
      onclick={() => (tabState = "create")}
      class={[
        "cursor-pointer px-4 py-1",
        tabState === "create"
          ? "rounded-3xl rounded-tl-none rounded-bl-none bg-white text-black"
          : "text-white",
      ]}
    >
      Create New Option
    </button>
    <button
      onclick={() => (tabState = "edit")}
      class={[
        "cursor-pointer px-4 py-1",
        tabState === "edit"
          ? "rounded-3xl rounded-tr-none rounded-br-none bg-white text-black"
          : "text-white",
      ]}
    >
      Edit Existing Option
    </button>
  </div>

  <div class="flex-1 overflow-y-auto w-full px-10 pb-10 mt-6">
    {@render children()}
  </div>
</div>
