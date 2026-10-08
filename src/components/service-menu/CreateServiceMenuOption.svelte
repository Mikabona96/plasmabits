<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import type { HTMLAttributes } from "svelte/elements";
  import { fly, slide } from "svelte/transition";
  import Checkmark from "#components/Checkmark.svelte";
  import ErrorCross from "#components/ErrorCross.svelte";
  import Input from "#components/Input.svelte";
  import MimeTypeModal from "./MimeTypeModal.svelte";
  import { mimeTypeCategories } from "./mimeTypeCategories";
  import type { AppMetadata, CreateDesktopActionResult } from "./types";

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
  let isLogsOpen = $state(false);
  let showResult = $state(false);

  async function selectFile() {
    const selected = await open({
      multiple: false,
      directory: false,
      defaultPath: "/usr/share/applications",
    });

    if (typeof selected !== "string") return;

    const metadata = await invoke<AppMetadata | null>("get_app_metadata", {
      desktopFile: selected,
    });

    if (!metadata) return;

    if (metadata.name) newOptionState.name = `Open with ${metadata.name}`;
    if (metadata.icon) newOptionState.icon = metadata.icon;
    if (metadata.exec) newOptionState.exec = metadata.exec;
  }

  async function selectIcon() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Images", extensions: ["png", "svg"] }],
    });

    if (typeof selected !== "string") return;
    newOptionState.icon = selected;
  }

  function shellQuote(value: string): string {
    return `'${value.replace(/'/g, `'\\''`)}'`;
  }

  function buildExec(command: string, mode: "command" | "file") {
    if (mode === "file") return `${command} %f`;
    return `sh -c ${shellQuote(command)}`;
  }

  const createOption = async () => {
    const option = {
      name: newOptionState.name,
      icon: newOptionState.icon,
      exec: buildExec(newOptionState.exec, launchMode),
      type: newOptionState.type,
      mimeType: newOptionState.mimeType.join(";"),
    };

    try {
      const result = await invoke<CreateDesktopActionResult>(
        "create_desktop_action",
        { action: option },
      );

      responseState = result;
      showResult = true;
    } catch (error) {
      if (error instanceof Error) {
        console.error(error.message);
      }
      responseState = { success: false, steps: [] };
      showResult = true;
    }
  };

  const resetFields = () => {
    showResult = false;
    isLogsOpen = false;
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

      <button
        class="mt-6 bg-white text-lg disabled:bg-graphite-400 disabled:text-graphite-950 text-graphite-950 px-6 py-1 cursor-pointer transition-transform active:scale-95"
        onclick={resetFields}
      >
        OK
      </button>
    </div>
  {:else}
    <!-- Form screen: slides out to the left (-100px) when opening result -->
    <div
      in:fly={{ x: -100, duration: 300, delay: 150 }}
      out:fly={{ x: -100, duration: 200 }}
      class={[
        "flex flex-col items-center w-full max-w-100 mx-auto gap-2",
        className,
      ]}
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
        <button
          class="bg-white text-graphite-950 px-2 py-2 cursor-pointer transition-transform active:scale-95"
          type="button"
          onclick={selectFile}
        >
          Browse
        </button>
      </div>

      <div class="flex self-start justify-between w-full gap-2 my-2">
        <label class="flex items-center gap-2 cursor-pointer group">
          <input
            type="radio"
            name="launchMode"
            value="file"
            bind:group={launchMode}
            class="peer sr-only"
          />
          <div
            class="relative h-5 w-5 border-2 border-graphite-700 transition-colors duration-200 peer-checked:border-white before:absolute before:left-1/2 before:top-1/2 before:h-2.5 before:w-2.5 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:content-[''] before:scale-0 before:transition-transform before:duration-200 peer-checked:before:scale-100"
          ></div>
          <span
            class="transition-colors duration-150 group-hover:text-white/90"
          >
            Open selected file or folder
          </span>
        </label>
        <label class="flex items-center gap-2 cursor-pointer group">
          <input
            type="radio"
            name="launchMode"
            value="command"
            bind:group={launchMode}
            class="peer sr-only"
          />
          <div
            class="relative h-5 w-5 border-2 border-graphite-700 transition-colors duration-200 peer-checked:border-white before:absolute before:left-1/2 before:top-1/2 before:h-2.5 before:w-2.5 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:content-[''] before:scale-0 before:transition-transform before:duration-200 peer-checked:before:scale-100"
          ></div>
          <span
            class="transition-colors duration-150 group-hover:text-white/90"
          >
            Launch application only
          </span>
        </label>
      </div>

      <div class="flex items-center gap-2 w-full">
        <Input
          class="flex flex-1"
          placeholder="Icon"
          bind:value={newOptionState.icon}
        />
        <span> or </span>
        <button
          class="bg-white text-graphite-950 px-2 py-2 cursor-pointer transition-transform active:scale-95"
          type="button"
          onclick={selectIcon}
        >
          Select Icon
        </button>
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
        onclick={createOption}
      >
        Create Option
      </button>
    </div>
  {/if}
</div>
