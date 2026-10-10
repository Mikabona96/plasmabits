<script lang="ts">
  let { children } = $props();
</script>

<h1 class="text-white text-4xl pl-10 pt-10 flex-none">
  Edit Dolphin Context Menu
</h1>

<div class="flex h-full w-full flex-col text-white overflow-hidden">
  <div class="flex-1 overflow-y-auto w-full px-10 pb-10 mt-6">
    {@render children()}
  </div>
</div>
