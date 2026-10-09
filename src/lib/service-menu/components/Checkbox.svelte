<script lang="ts">
  let {
    checked = false,
    indeterminate = false,
    disabled = false,
    onchange,
    ariaLabel,
  }: {
    checked?: boolean;
    indeterminate?: boolean;
    disabled?: boolean;
    onchange?: () => void;
    ariaLabel?: string;
  } = $props();
</script>

<label
  class="relative inline-flex items-center justify-center cursor-pointer select-none"
>
  <input
    type="checkbox"
    class="peer sr-only"
    {checked}
    {disabled}
    aria-label={ariaLabel}
    onchange={() => onchange?.()}
  />
  <div
    class={[
      "relative h-5 w-5 border-2 transition-colors duration-200",
      "before:absolute before:left-1/2 before:top-1/2 before:h-2.5 before:w-2.5 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:content-[''] before:transition-transform before:duration-200",
      checked
        ? "border-white before:scale-100"
        : indeterminate
          ? "border-white/70 before:scale-100 before:bg-white/70"
          : "border-graphite-700 hover:border-white/70 before:scale-0",
    ]}
  ></div>
</label>
