<script lang="ts">
  let {
    option = $bindable(""),
    value,
    groupName: name,
    title,
    description,
  }: {
    option: string;
    value: string;
    groupName: string;
    title: string;
    description: string;
  } = $props();
</script>

<label
  class={[
    "h-full group cursor-pointer relative flex flex-col gap-1 px-4 py-4 border-t-2 border-b-2 transition-colors duration-200",
    "before:absolute before:inset-y-0 before:left-0 before:w-0.5 before:content-[''] before:bg-[repeating-linear-gradient(to_bottom,var(--bc),var(--bc)_12px,transparent_12px,transparent_18px)]",
    "after:absolute after:inset-y-0 after:right-0 after:w-0.5 after:content-[''] after:bg-[repeating-linear-gradient(to_bottom,var(--bc),var(--bc)_12px,transparent_12px,transparent_18px)]",
    option === value
      ? "border-t-white border-b-white [--bc:var(--color-white)]"
      : "border-t-graphite-700 hover:border-t-white/70 border-b-graphite-700 hover:border-b-white/70 [--bc:var(--color-graphite-700)] hover:[--bc:color-mix(in_srgb,var(--color-white)_70%,transparent)]",
  ]}
>
  <div class="flex items-start gap-2 cursor-pointer">
    <input
      type="radio"
      {name}
      {value}
      class="peer sr-only"
      bind:group={option}
    />
    <div
      class={[
        "relative h-5 w-5 border-2 transition-colors duration-200 peer-checked:border-white before:absolute before:left-1/2 before:top-1/2 before:h-2.5 before:w-2.5 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:content-[''] before:scale-0 before:transition-transform before:duration-200 peer-checked:before:scale-100",
        option === value
          ? ""
          : "border-graphite-700 group-hover:border-white/70",
      ]}
    ></div>
    <span
      class={[
        "transition-colors duration-150",
        option === value
          ? "text-white"
          : "text-graphite-500 group-hover:text-white/70",
      ]}
    >
      {title}
    </span>
  </div>
  <span
    class={[
      "max-w-40 transition-colors duration-150",
      option === value
        ? "text-white"
        : "text-graphite-500 group-hover:text-white/70",
    ]}
  >
    {description}
  </span>
</label>
