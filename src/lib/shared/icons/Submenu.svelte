<script lang="ts">
  import type { SVGAttributes } from "svelte/elements";

  interface IProps extends SVGAttributes<SVGSVGElement> {
    size?: number;
  }
  let { size = 20, ...rest }: IProps = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  stroke="currentColor"
  {...rest}
>
  <path
    d="M4 6H20M4 12H14M4 18H9"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
  />
</svg>
