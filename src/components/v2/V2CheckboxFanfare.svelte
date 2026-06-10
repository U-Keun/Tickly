<svg class="fanfareRays" aria-hidden="true" viewBox="0 0 26 24">
  <path class="fanfareRay fanfareRayOne" d="M6 20 L6 15.5" />
  <path class="fanfareRay fanfareRayTwo" d="M11 18 L15 14" />
  <path class="fanfareRay fanfareRayThree" d="M16 20 L20.5 20" />
</svg>

<style>
  .fanfareRays {
    display: block;
    width: 26px;
    height: 24px;
    color: currentColor;
    overflow: visible;
    pointer-events: none;
  }

  .fanfareRay {
    fill: none;
    stroke: currentColor;
    stroke-dasharray: 10;
    stroke-dashoffset: 10;
    stroke-linecap: round;
    stroke-width: 2.25;
    opacity: 0;
    --fanfare-x: 0px;
    --fanfare-y: -3px;
    animation: fanfare-ray 280ms cubic-bezier(0, 0, 0.3, 1) forwards;
    animation-delay: var(--fanfare-delay);
  }

  .fanfareRayOne {
    --fanfare-delay: 170ms;
    --fanfare-y: -3px;
  }

  .fanfareRayTwo {
    --fanfare-delay: 190ms;
    --fanfare-x: 2px;
    --fanfare-y: -2px;
  }

  .fanfareRayThree {
    --fanfare-delay: 210ms;
    --fanfare-x: 3px;
    --fanfare-y: 0px;
  }

  @keyframes fanfare-ray {
    0% {
      opacity: 0;
      stroke-dashoffset: 10;
      transform: translate(0, 0);
    }

    18% {
      opacity: 1;
      stroke-dashoffset: 10;
      transform: translate(0, 0);
    }

    56% {
      opacity: 1;
      stroke-dashoffset: 0;
      transform: translate(0, 0);
    }

    100% {
      opacity: 0;
      stroke-dashoffset: 0;
      transform: translate(var(--fanfare-x), var(--fanfare-y));
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .fanfareRay {
      animation: none;
      opacity: 0;
    }
  }
</style>
