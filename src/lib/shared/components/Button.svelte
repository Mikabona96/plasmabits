<script lang="ts">
  import type { Snippet } from "svelte";
  import { cn } from "../utils/cn";

  interface IProps {
    children: Snippet;
    onclick?: () => void;
    variant?: "primary" | "outlined";
    class?: string;
    type?: "button" | "submit" | "reset";
    disabled?: boolean;
  }
  let {
    children,
    onclick,
    variant = "primary",
    class: className,
    type,
    disabled,
  }: IProps = $props();
</script>

<button
  {disabled}
  class={cn(
    "px-2 py-2 cursor-pointer transition-all duration-200 active:scale-95",
    {
      "bg-white text-graphite-950 disabled:bg-graphite-400":
        variant === "primary",
      "border border-graphite-700 text-white cursor-pointer hover:bg-graphite-700":
        variant === "outlined",
    },
    className,
  )}
  {type}
  {onclick}
>
  {@render children()}
</button>
