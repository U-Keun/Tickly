<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { cubicOut } from 'svelte/easing';
  import { fade, fly } from 'svelte/transition';
  import type { Snippet } from 'svelte';

  interface Props {
    show: boolean;
    title: string;
    description?: string;
    preferredHeight?: number | null;
    onClose: () => void;
    children?: Snippet;
    footer?: Snippet;
  }

  let {
    show,
    title,
    description = '',
    preferredHeight = null,
    onClose,
    children,
    footer
  }: Props = $props();
  let wasShown = $state(false);
  let viewportBottomGap = $state(0);
  let keyboardBottomInset = $state(0);
  let sheetMaxHeight = $state(720);
  let viewportUpdateTimers: number[] = [];

  function isIOSViewport(): boolean {
    if (typeof navigator === 'undefined') return false;

    return (
      /iPad|iPhone|iPod/.test(navigator.userAgent) ||
      (navigator.platform === 'MacIntel' && navigator.maxTouchPoints > 1)
    );
  }

  function measureViewportMetrics(): {
    reservedGap: number;
    keyboardInset: number;
    sheetMaxHeight: number;
  } {
    if (typeof window === 'undefined') {
      return { reservedGap: 0, keyboardInset: 0, sheetMaxHeight: 720 };
    }

    const visualViewport = window.visualViewport;
    const visualBottomGap = visualViewport
      ? window.innerHeight - visualViewport.offsetTop - visualViewport.height
      : 0;
    const reservedBottomGap = isIOSViewport() ? window.outerHeight - window.innerHeight : 0;
    const keyboardInset = visualViewport ? Math.max(0, Math.round(visualBottomGap)) : 0;
    const visibleHeight = visualViewport?.height ?? window.innerHeight;

    return {
      reservedGap: keyboardInset > 0 ? 0 : Math.max(0, Math.round(reservedBottomGap)),
      keyboardInset,
      sheetMaxHeight: Math.max(120, Math.floor(visibleHeight * 0.86))
    };
  }

  function updateViewportBottomGap(): void {
    const metrics = measureViewportMetrics();
    viewportBottomGap = metrics.reservedGap;
    keyboardBottomInset = metrics.keyboardInset;
    sheetMaxHeight = metrics.sheetMaxHeight;
  }

  function clearViewportUpdateTimers(): void {
    viewportUpdateTimers.forEach((timer) => window.clearTimeout(timer));
    viewportUpdateTimers = [];
  }

  function scheduleViewportGapUpdates(): void {
    if (typeof window === 'undefined') return;

    clearViewportUpdateTimers();
    updateViewportBottomGap();
    viewportUpdateTimers = [60, 180, 360].map((delay) =>
      window.setTimeout(updateViewportBottomGap, delay)
    );
  }

  function blurActiveEditable(): void {
    const activeElement = document.activeElement;
    if (!(activeElement instanceof HTMLElement)) return;

    const isEditable =
      activeElement instanceof HTMLInputElement ||
      activeElement instanceof HTMLTextAreaElement ||
      activeElement instanceof HTMLSelectElement ||
      activeElement.isContentEditable;

    if (isEditable) {
      activeElement.blur();
    }
  }

  function setRootSheetBackground(isOpen: boolean): void {
    document.documentElement.classList.toggle('bottom-sheet-open', isOpen);
  }

  $effect(() => {
    if (show && !wasShown) {
      blurActiveEditable();
      scheduleViewportGapUpdates();
    }

    if (!show && wasShown) {
      clearViewportUpdateTimers();
      viewportBottomGap = 0;
      keyboardBottomInset = 0;
      sheetMaxHeight = 720;
    }

    setRootSheetBackground(show);
    wasShown = show;
  });

  onMount(() => {
    const visualViewport = window.visualViewport;

    window.addEventListener('resize', scheduleViewportGapUpdates);
    visualViewport?.addEventListener('resize', scheduleViewportGapUpdates);
    visualViewport?.addEventListener('scroll', updateViewportBottomGap);

    return () => {
      clearViewportUpdateTimers();
      window.removeEventListener('resize', scheduleViewportGapUpdates);
      visualViewport?.removeEventListener('resize', scheduleViewportGapUpdates);
      visualViewport?.removeEventListener('scroll', updateViewportBottomGap);
    };
  });

  onDestroy(clearViewportUpdateTimers);
  onDestroy(() => setRootSheetBackground(false));

  function handleKeydown(event: KeyboardEvent): void {
    if (!show || event.key !== 'Escape') return;

    event.preventDefault();
    onClose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <div
    class="fixed left-0 right-0 top-0 z-50 overflow-hidden bg-black/45"
    style={`height: calc(100dvh + ${viewportBottomGap + keyboardBottomInset}px);`}
    role="presentation"
    onclick={onClose}
    transition:fade={{ duration: 180, easing: cubicOut }}
  >
    <div
      class="fixed left-0 right-0 top-0 flex items-end justify-center overflow-visible px-3 pt-4"
      style={`bottom: ${keyboardBottomInset}px; transition: bottom 180ms cubic-bezier(0.22, 1, 0.36, 1);`}
    >
      <div
        class="relative w-full max-w-[440px] overflow-visible text-[var(--color-ink)]"
        role="presentation"
        onclick={(event) => event.stopPropagation()}
        onkeydown={(event) => event.stopPropagation()}
        in:fly={{ y: 28, duration: 260, easing: cubicOut }}
        out:fly={{ y: 24, duration: 190, easing: cubicOut }}
      >
        <div
          class="relative z-10 flex flex-col overflow-hidden rounded-[6px_24px_6px_24px] border-[2.5px] border-[var(--color-ink)] bg-[var(--color-white)] shadow-[0_-4px_18px_rgba(0,0,0,0.14)]"
          style={preferredHeight
            ? `height: min(${preferredHeight}px, ${sheetMaxHeight}px, 720px); max-height: min(${sheetMaxHeight}px, 720px);`
            : `max-height: min(${sheetMaxHeight}px, 720px);`}
          role="dialog"
          aria-modal="true"
          aria-label={title}
          tabindex="-1"
        >
          <header class="px-6 pb-[22px] pt-6">
            <h2 class="text-[22px] font-semibold leading-7 text-[var(--color-ink)]">{title}</h2>
            {#if description}
              <p class="mt-2 whitespace-pre-line text-sm leading-6 text-[var(--color-ink-muted)]">
                {description}
              </p>
            {/if}
          </header>

          {#if children}
            <div class={`min-h-0 overflow-y-auto px-6 ${footer ? 'pb-3' : 'pb-6'}`}>
              {@render children()}
            </div>
          {/if}

          {#if footer}
            <div class="shrink-0 px-6 pb-6 pt-3">
              {@render footer()}
            </div>
          {/if}
        </div>

        {#if viewportBottomGap > 0}
          <div
            class="pointer-events-none absolute left-1 right-1 top-full bg-[var(--color-white)]"
            style={`height: ${viewportBottomGap}px;`}
            aria-hidden="true"
          ></div>
        {/if}
      </div>
    </div>
  </div>
{/if}
