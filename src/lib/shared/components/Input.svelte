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
    "px-2 py-0.5 text-midnight placeholder:text-midnight rounded-md border bg- border-silver hover:border-blue-500 focus:outline-none focus:ring-2 focus:ring-red-500 transition-colors duration-200",
    className,
  )}
  bind:value
  {placeholder}
  {...rest}
/>
