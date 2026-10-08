<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

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
  class={[
    "px-2 py-2 bg-graphite-800 text-white placeholder:text-graphite-500 border border-graphite-400 focus:outline-none focus:ring-2 focus:ring-white",
    className,
  ]}
  bind:value
  {placeholder}
  {...rest}
/>
