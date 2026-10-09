<script lang="ts">
  let {
    group = $bindable(""),
    value,
    name,
    label,
  }: {
    group: string;
    value: string;
    name: string;
    label: string;
  } = $props();
</script>

<label class="flex items-center gap-2 cursor-pointer group">
  <input type="radio" {name} {value} bind:group class="peer sr-only" />
  <div
    class="relative h-5 w-5 border-2 border-graphite-700 transition-colors duration-200 peer-checked:border-white before:absolute before:left-1/2 before:top-1/2 before:h-2.5 before:w-2.5 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:content-[''] before:scale-0 before:transition-transform before:duration-200 peer-checked:before:scale-100"
  ></div>
  <span class="transition-colors duration-150 group-hover:text-white/90">
    {label}
  </span>
</label>
