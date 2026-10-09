<script lang="ts">
  import { fly, slide } from "svelte/transition";
  import type { CreateDesktopActionResult } from "$lib/service-menu/types";
  import Button from "$lib/shared/components/Button.svelte";
  import Checkmark from "$lib/shared/components/Checkmark.svelte";
  import ErrorCross from "$lib/shared/components/ErrorCross.svelte";

  let {
    responseState,
    resetFields,
  }: {
    responseState: CreateDesktopActionResult;
    resetFields: () => void;
  } = $props();

  let isLogsOpen = $state(false);
</script>

<div
  in:fly={{ x: 100, duration: 300, delay: 150 }}
  out:fly={{ x: 100, duration: 200 }}
  class="absolute inset-0 flex flex-col items-center justify-center w-full h-full mx-auto gap-2"
>
  {#if responseState.success}
    <Checkmark />
  {:else}
    <ErrorCross />
  {/if}

  <div class="flex mt-4 items-end gap-4">
    <span class="text-2xl">
      Status: {responseState.success ? "Success" : "Failed"}
    </span>
    <button
      onclick={() => (isLogsOpen = !isLogsOpen)}
      class="underline cursor-pointer mb-0.5 transition-colors duration-150 hover:opacity-80"
    >
      {isLogsOpen ? "hide logs" : "show logs"}
    </button>
  </div>

  {#if isLogsOpen}
    <div
      transition:slide={{ duration: 250 }}
      class="mt-4 h-60 py-2 px-4 w-full bg-graphite-700 overflow-y-auto gap-y-4"
    >
      {#each responseState?.steps as step}
        <div
          class="grid grid-cols-[150px_1fr] items-start justify-items-start gap-2"
        >
          <span class="text-white">{step.name}:</span>
          <span
            class={[
              step.success
                ? "text-green-500"
                : step.success === null
                  ? "text-yellow-500"
                  : "text-red-500",
            ]}
          >
            {step.message}
          </span>
        </div>
      {/each}
    </div>
  {/if}

  <Button
    class="mt-6 bg-white text-lg px-6 py-1"
    onclick={() => {
      resetFields();
      isLogsOpen = false;
    }}
  >
    OK
  </Button>
</div>
