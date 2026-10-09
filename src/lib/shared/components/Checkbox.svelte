<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import { cn } from "../utils/cn";

  let {
    checked = $bindable(false),
    name,
    value,
    disabled = false,
    class: className,
  }: {
    checked?: boolean;
    name?: string;
    value?: string;
    disabled?: boolean;
    class?: string;
  } = $props();
</script>

<label
  class={[
    "relative inline-flex items-center justify-center cursor-pointer select-none",
    disabled && "opacity-50 pointer-events-none",
  ]}
>
  <input
    type="checkbox"
    {name}
    {value}
    bind:checked
    {disabled}
    class="peer sr-only"
  />
  <div
    class={cn(
      "relative h-5 w-5 border-2 border-graphite-700 transition-colors duration-200 hover:border-white/70 peer-checked:border-white before:absolute before:left-1/2 before:top-1/2 before:h-2.5 before:w-2.5 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:content-[''] before:scale-0 before:transition-transform before:duration-200 peer-checked:before:scale-100",
      className,
    )}
  ></div>
</label>
