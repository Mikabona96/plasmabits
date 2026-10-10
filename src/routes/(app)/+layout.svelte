<script lang="ts">
  import Userplus from "$lib/shared/icons/Userplus.svelte";

  let { children } = $props();
</script>

<div class="flex justify-between items-end">
  <h1 class="text-2xl font-bold flex-none">Custom Dolphin Actions</h1>
  <button
    class="flex items-center gap-1 px-4 py-1 text-white bg-blue-wash rounded-lg"
  >
    <Userplus />
    Add New Item</button
  >
</div>

<div class="flex w-full flex-col">
  {@render children()}
</div>
