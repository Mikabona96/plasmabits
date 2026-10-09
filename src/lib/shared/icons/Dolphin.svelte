<script lang="ts">
  import type { SVGAttributes } from "svelte/elements";

  interface IProps extends SVGAttributes<SVGSVGElement> {
    size?: number;
  }

  let { size = 44, ...rest } = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 64 64"
  {...rest}
>
  <defs>
    <linearGradient id="linearGradient3">
      <stop offset="1" stop-color="#e5fbff" stop-opacity="1" />
      <stop offset="1" stop-color="#ababab" stop-opacity="0.99539232" />
    </linearGradient>

    <linearGradient id="linearGradient82546">
      <stop offset="0" stop-color="#000" stop-opacity="0.39814615" />
      <stop offset="1" stop-color="#000" stop-opacity="1" />
    </linearGradient>

    <linearGradient id="linearGradient39400">
      <stop offset="0" stop-color="#000" stop-opacity="1" />
      <stop offset="1" stop-color="#147cdc" stop-opacity="0" />
    </linearGradient>

    <linearGradient id="linearGradient919">
      <stop offset="0" stop-color="#fff" stop-opacity="1" />
      <stop offset="1" stop-color="#fff" stop-opacity="0" />
    </linearGradient>

    <linearGradient id="linearGradient4393">
      <stop offset="0" stop-color="#000" stop-opacity="1" />
      <stop offset="1" stop-color="#000" stop-opacity="0" />
    </linearGradient>

    <linearGradient id="linearGradient4227">
      <stop offset="0" stop-color="#f5f5f5" stop-opacity="1" />
      <stop offset="1" stop-color="#f9f9f9" stop-opacity="1" />
    </linearGradient>

    <linearGradient id="linearGradient4274">
      <stop offset="0" stop-color="#fff" stop-opacity="1" />
      <stop offset="1" stop-color="#fff" stop-opacity="0.48760331" />
    </linearGradient>

    <linearGradient
      id="linearGradient921"
      x1="339.668"
      x2="357.403"
      y1="472.302"
      y2="502.195"
      gradientTransform="translate(-317.668 -444.302)"
      gradientUnits="userSpaceOnUse"
      href="#linearGradient919"
    />

    <linearGradient
      id="linearGradient39402"
      x1="376.818"
      x2="337.818"
      y1="549.95"
      y2="519.95"
      gradientTransform="translate(-315.818 -491.95)"
      gradientUnits="userSpaceOnUse"
      href="#linearGradient39400"
    />

    <linearGradient
      id="linearGradient49568"
      x1="443.571"
      x2="443.571"
      y1="510.798"
      y2="526.798"
      gradientTransform="translate(-353.793 -518.24)"
      gradientUnits="userSpaceOnUse"
      href="#linearGradient4393"
    />

    <linearGradient
      id="linearGradient76299"
      x1="390.571"
      x2="442.571"
      y1="498.298"
      y2="498.298"
      gradientTransform="translate(-384.571 -480.798)"
      gradientUnits="userSpaceOnUse"
      href="#linearGradient4274"
    />

    <linearGradient
      id="linearGradient76301"
      x1="396.571"
      x2="426.571"
      y1="498.798"
      y2="511.798"
      gradientTransform="translate(-384.571 -482.798)"
      gradientUnits="userSpaceOnUse"
      href="#linearGradient4227"
    />

    <linearGradient
      id="linearGradient82548"
      x1="357.489"
      x2="384.419"
      y1="494.852"
      y2="541.495"
      gradientTransform="translate(-326.571 -483.798)"
      gradientUnits="userSpaceOnUse"
      href="#linearGradient82546"
    />

    <linearGradient
      id="linearGradient2"
      x1="410.571"
      x2="432.071"
      y1="512.798"
      y2="552.423"
      gradientTransform="translate(-385.897 -483.798)"
      gradientUnits="userSpaceOnUse"
      href="#linearGradient3"
    />

    <marker
      id="marker1511"
      markerHeight="14.026"
      markerWidth="16.165"
      orient="auto"
      refX="8.082"
      refY="7.013"
    >
      <path
        d="m27.135 57.452-16.157.49 7.654-14.238Z"
        opacity="0.507"
        fill="red"
        transform="rotate(1.546 1632.558 -370.734)"
      />
    </marker>

    <style>
      .ColorScheme-Highlight {
        color: #3593e6;
      }
    </style>
  </defs>

  <path
    d="M4 6v27c0 .554.446 1 1 1h54c.554 0 1-.446 1-1V11.01l-.002-.01H60v-1H32l-4-4H4Z"
    class="ColorScheme-Highlight"
    fill="currentColor"
  />

  <path
    d="M4 6v27c0 .554.446 1 1 1h54c.554 0 1-.446 1-1V11.01l-.002-.01H60v-1H32l-4-4H4Z"
    opacity="0.35"
    fill="url(#linearGradient82548)"
  />

  <rect
    width="52"
    height="15"
    x="6"
    y="15"
    rx="1"
    ry="1"
    fill="url(#linearGradient76301)"
  />

  <path d="M3 28v1h20l3-5-4 4z" opacity="0.3" fill="#fff" fill-rule="evenodd" />

  <path d="m28 6 3 5h29v-1H32Z" opacity="0.3" fill="#fff" fill-rule="evenodd" />

  <path
    d="M7 15c-.554 0-1 .446-1 1v1c0-.554.446-1 1-1h50c.554 0 1 .446 1 1v-1c0-.554-.446-1-1-1z"
    opacity="0.85"
    fill="url(#linearGradient76299)"
  />

  <path
    d="m26 24-4 4H3v1h.002L3 29.012V57c0 .554.446 1 1 1h56c.554 0 1-.446 1-1V24z"
    class="ColorScheme-Highlight"
    fill="currentColor"
  />

  <path
    d="m26 24-4 4H3v1h.002L3 29.012V57c0 .554.446 1 1 1h56c.554 0 1-.446 1-1V24z"
    opacity="0.1"
    fill="url(#linearGradient921)"
  />

  <path
    d="m26 24-4 4H3v1h.002L3 29.012V57c0 .554.446 1 1 1h56c.554 0 1-.446 1-1V24z"
    opacity="0.3"
    fill="url(#linearGradient39402)"
  />

  <path d="M3 28v1h20l3-5-4 4z" opacity="0.3" fill="#fff" fill-rule="evenodd" />

  <path
    d="M37.903 32.621c-2.82.019-6.311.819-10.557 3.123-9.704 5.267-12.34 14.618-19.424 22.223l33.176.02c.408-7.787 4.205-10.102 10.117-16.903 0 0 3.744-3.308 2.908-6.422-.884-3.297-7.735.705-8.5.215-1.58-1.014-4.095-2.28-7.72-2.256zm15.44 3c-1.774 2.324-3.974 4.572-6.966 7.328-2.917 2.688-7.93 6.512-8.164 6.11-.063-.11 5.143-4.09 7.658-6.413a462.54 462.54 0 0 0 7.471-7.025zm-21.481 11.66c.76 0 1.375.756 1.375 1.688 0 .932-.616 1.687-1.375 1.687-.76 0-1.375-.755-1.375-1.687s.615-1.688-1.375-1.688z"
    fill="#fff"
    fill-opacity="0.40000001"
  />

  <path
    d="M4 58c-.554 0-1-.446-1-1v-1c0 .554.446 1 1 1h56c.554 0 1-.446 1-1v1c0 .554-.446 1-1 1z"
    opacity="0.3"
    fill="#000"
  />
</svg>
