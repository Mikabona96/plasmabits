<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";
  import { fly } from "svelte/transition";
  import MimeTypeModal from "$lib/service-menu/components/MimeTypeModal.svelte";
  import ResultScreen from "$lib/service-menu/components/ResultScreen.svelte";
  import { mimeTypeCategories } from "$lib/service-menu/mock/mimeTypeCategories";
  import type { CreateDesktopActionResult } from "$lib/service-menu/types";
  import {
    createOption,
    selectFile,
    selectIcon,
  } from "$lib/service-menu/utils/index.js";
  import Button from "$lib/shared/components/Button.svelte";
  import Input from "$lib/shared/components/Input.svelte";
  import RadioButton from "$lib/shared/components/RadioButton.svelte";
  import { cn } from "$lib/shared/utils/cn";

  let { class: className }: HTMLAttributes<HTMLDivElement> = $props();

  let newOptionState = $state({
    type: "Service",
    mimeType: [] as string[],
    name: "",
    icon: "",
    exec: "",
  });

  let launchMode = $state<"command" | "file">("file");
  let responseState = $state<CreateDesktopActionResult | null>(null);

  let showResult = $state(false);

  const resetFields = () => {
    showResult = false;
    responseState = null;
    launchMode = "file";
    newOptionState = {
      type: "Service",
      mimeType: [] as string[],
      name: "",
      icon: "",
      exec: "",
    };
  };
</script>

<!-- Outer wrapper with relative positioning so screen slides overlay cleanly -->
<div class="relative w-full overflow-hidden min-h-112.5">
  {#if showResult && responseState}
    <!-- Result screen: slides in from the right (+100px) -->
    <ResultScreen {responseState} {resetFields} />
  {:else}
    <!-- Form screen: slides out to the left (-100px) when opening result -->
    <div
      in:fly={{ x: -100, duration: 300, delay: 150 }}
      out:fly={{ x: -100, duration: 200 }}
      class={cn(
        "flex flex-col items-center w-full max-w-100 mx-auto gap-2",
        className,
      )}
    >
      <span class="text-2xl"> [Desktop Action] </span>
      <Input
        class="w-full"
        placeholder="Option Name"
        bind:value={newOptionState.name}
      />

      <div class="flex items-center gap-2 w-full">
        <Input
          class="w-full"
          placeholder="Command"
          bind:value={newOptionState.exec}
        />
        <span> or </span>

        <Button onclick={() => selectFile(newOptionState)}>Browse</Button>
      </div>

      <div class="flex self-start justify-between w-full gap-2 my-2">
        <RadioButton
          bind:group={launchMode}
          name="launchMode"
          value="file"
          label="Open selected file or folder"
        />
        <RadioButton
          bind:group={launchMode}
          name="launchMode"
          value="command"
          label="Launch application only"
        />
      </div>

      <div class="flex items-center gap-2 w-full">
        <Input
          class="flex flex-1"
          placeholder="Icon"
          bind:value={newOptionState.icon}
        />
        <span> or </span>
        <Button onclick={() => selectIcon(newOptionState)}>Select Icon</Button>
      </div>

      <span class="text-2xl mt-6">[Desktop Entry]</span>
      <Input
        class="w-full"
        placeholder="Type"
        bind:value={newOptionState.type}
      />
      <MimeTypeModal
        categories={mimeTypeCategories}
        bind:value={newOptionState.mimeType}
      />

      <button
        disabled={!newOptionState.name || !newOptionState.exec}
        class="mt-6 w-full bg-white text-lg disabled:bg-graphite-400 disabled:text-graphite-950 text-graphite-950 px-6 py-2 cursor-pointer transition-all duration-200 hover:enabled:opacity-90 active:enabled:scale-95"
        onclick={async () => {
          responseState = await createOption(newOptionState, launchMode);
          showResult = true;
        }}
      >
        Create Option
      </button>
    </div>
  {/if}
</div>
