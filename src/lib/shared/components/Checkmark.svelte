<div class="success-checkmark">
  <svg xmlns="http://w3.org" viewBox="0 0 52 52" class="checkmark-svg">
    <circle class="checkmark-circle" cx="26" cy="26" r="25" fill="none" />
    <path
      class="checkmark-check"
      fill="none"
      d="M14.1 27.2l7.1 7.2 16.7-16.8"
    />
  </svg>
</div>

<style>
  .success-checkmark {
    width: 120px;
    height: 120px;
  }

  .checkmark-svg {
    width: 100%;
    height: 100%;
    display: block;
  }

  .checkmark-circle {
    stroke: #fff;
    stroke-width: 2;
    stroke-miterlimit: 10;
    stroke-dasharray: 166;
    stroke-dashoffset: 166;
    animation: strokeCircle 0.6s cubic-bezier(0.65, 0, 0.45, 1) forwards;
  }

  .checkmark-check {
    stroke: #fff;
    stroke-width: 3;
    stroke-linecap: round;
    stroke-dasharray: 48;
    stroke-dashoffset: 48;
    animation: strokeCheck 0.3s cubic-bezier(0.65, 0, 0.45, 1) 0.3s forwards;
  }

  @keyframes strokeCircle {
    100% {
      stroke-dashoffset: 0;
    }
  }

  @keyframes strokeCheck {
    100% {
      stroke-dashoffset: 0;
    }
  }
</style>
