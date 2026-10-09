<div class="error-cross">
  <svg xmlns="http://w3.org" viewBox="0 0 52 52" class="cross-svg">
    <circle class="cross-circle" cx="26" cy="26" r="25" fill="none" />
    <path class="cross-line line-1" fill="none" d="M16 16l20 20" />
    <path class="cross-line line-2" fill="none" d="M36 16l-20 20" />
  </svg>
</div>

<style>
  .error-cross {
    width: 120px;
    height: 120px;
  }

  .cross-svg {
    width: 100%;
    height: 100%;
    display: block;
  }

  .cross-circle {
    stroke: #fff;
    stroke-width: 2;
    stroke-miterlimit: 10;
    stroke-dasharray: 166;
    stroke-dashoffset: 166;
    animation: strokeCircle 0.6s cubic-bezier(0.65, 0, 0.45, 1) forwards;
  }

  .cross-line {
    stroke: #fff;
    stroke-width: 3;
    stroke-linecap: round;
    stroke-dasharray: 29;
    stroke-dashoffset: 29;
  }

  .cross-line.line-1 {
    animation: strokeLine 0.15s cubic-bezier(0.65, 0, 0.45, 1) 0.6s forwards;
  }

  .cross-line.line-2 {
    animation: strokeLine 0.15s cubic-bezier(0.65, 0, 0.45, 1) 0.75s forwards;
  }

  @keyframes strokeCircle {
    100% {
      stroke-dashoffset: 0;
    }
  }

  @keyframes strokeLine {
    100% {
      stroke-dashoffset: 0;
    }
  }
</style>
