<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import { cn } from "../utils/cn";

  interface IProps extends HTMLInputAttributes {
    value?: string;
    placeholder?: string;
  }
  let {
    value = $bindable(""),
    placeholder,
    class: className,
    ...rest
  }: IProps = $props();
</script>

<input
  class={cn(
    "px-2 py-2 text-white placeholder:text-graphite-500 border-2 border-graphite-700 hover:border-white/70 focus:outline-none focus:ring-2 focus:ring-white transition-colors duration-200",
    className,
  )}
  bind:value
  {placeholder}
  {...rest}
/>
