<script lang="ts">
  import { onDestroy, onMount, tick } from 'svelte';
  import { flip } from 'svelte/animate';
  import { Check, SlidersHorizontal } from '@lucide/svelte';
  import { dndzone } from 'svelte-dnd-action';
  import type { DndEvent } from 'svelte-dnd-action';

  import { i18n } from '$lib/i18n';
  import type { V2Category } from '../../types';

  type MaybePromise = void | Promise<void>;
  const LONG_PRESS_MS = 500;
  const LONG_PRESS_CANCEL_DISTANCE_PX = 10;
  const REORDER_FLIP_DURATION_MS = 180;

  interface Props {
    categories: V2Category[];
    selectedCategoryId: number | null;
    isReorderMode?: boolean;
    isReorderBusy?: boolean;
    onSelectCategory: (id: number) => MaybePromise;
    onManageCategory: (category: V2Category) => void;
    onEnterReorderMode: () => void;
    onFinishReorderMode: () => void;
    onReorderConsider: (categories: V2Category[]) => void;
    onReorderFinalize: (categories: V2Category[]) => MaybePromise;
  }

  let {
    categories,
    selectedCategoryId,
    isReorderMode = false,
    isReorderBusy = false,
    onSelectCategory,
    onManageCategory,
    onEnterReorderMode,
    onFinishReorderMode,
    onReorderConsider,
    onReorderFinalize
  }: Props = $props();

  let selectedCategory = $derived(
    categories.find((category) => category.id === selectedCategoryId) ?? null
  );
  let categorySignature = $derived(categories.map((category) => category.id).join(','));
  let segmentTrack = $state<HTMLDivElement | null>(null);
  let longPressTimer = $state<ReturnType<typeof setTimeout> | null>(null);
  let longPressPointerId = $state<number | null>(null);
  let longPressStartX = $state(0);
  let longPressStartY = $state(0);
  let suppressNextClick = $state(false);
  let indicatorLeft = $state(0);
  let indicatorTop = $state(0);
  let indicatorWidth = $state(0);
  let indicatorHeight = $state(0);
  let isIndicatorReady = $state(false);
  let prefersReducedMotion = $state(false);
  let indicatorTransition = $derived(
    prefersReducedMotion
      ? 'none'
      : 'transform 220ms cubic-bezier(0.4, 0, 0.2, 1), width 220ms cubic-bezier(0.4, 0, 0.2, 1), height 220ms cubic-bezier(0.4, 0, 0.2, 1), opacity 120ms ease-out'
  );
  let reorderFlipDuration = $derived(prefersReducedMotion ? 0 : REORDER_FLIP_DURATION_MS);

  function selectedSegment(): HTMLDivElement | null {
    if (!segmentTrack || selectedCategoryId === null) return null;
    return segmentTrack.querySelector<HTMLDivElement>(`[data-category-id="${selectedCategoryId}"]`);
  }

  async function updateIndicator(shouldScroll = false): Promise<void> {
    await tick();

    const segment = selectedSegment();
    if (!segment) {
      isIndicatorReady = false;
      return;
    }

    indicatorLeft = segment.offsetLeft;
    indicatorTop = segment.offsetTop;
    indicatorWidth = segment.offsetWidth;
    indicatorHeight = segment.offsetHeight;
    isIndicatorReady = true;

    if (shouldScroll) {
      segment.scrollIntoView({
        block: 'nearest',
        inline: 'nearest',
        behavior: prefersReducedMotion ? 'auto' : 'smooth'
      });
    }
  }

  function handleResize(): void {
    void updateIndicator(false);
  }

  async function selectCategory(id: number): Promise<void> {
    if (suppressNextClick) {
      suppressNextClick = false;
      return;
    }
    if (isReorderMode) return;

    try {
      await onSelectCategory(id);
    } catch {
      // The v2 store owns the visible error banner; keep the rail state stable.
    }
  }

  function clearLongPressTimer(): void {
    if (!longPressTimer) return;
    clearTimeout(longPressTimer);
    longPressTimer = null;
  }

  function removeLongPressListeners(): void {
    window.removeEventListener('pointermove', handleLongPressMove);
    window.removeEventListener('pointerup', handleLongPressEnd);
    window.removeEventListener('pointercancel', handleLongPressEnd);
  }

  function cancelLongPress(): void {
    clearLongPressTimer();
    longPressPointerId = null;
    removeLongPressListeners();
  }

  function handleLongPressMove(event: PointerEvent): void {
    if (event.pointerId !== longPressPointerId) return;

    const movedX = Math.abs(event.clientX - longPressStartX);
    const movedY = Math.abs(event.clientY - longPressStartY);
    if (movedX > LONG_PRESS_CANCEL_DISTANCE_PX || movedY > LONG_PRESS_CANCEL_DISTANCE_PX) {
      cancelLongPress();
    }
  }

  function handleLongPressEnd(event: PointerEvent): void {
    if (event.pointerId !== longPressPointerId) return;
    cancelLongPress();
  }

  function startLongPress(event: PointerEvent): void {
    if (event.button !== 0 || isReorderMode || isReorderBusy || categories.length <= 1) return;

    cancelLongPress();
    longPressPointerId = event.pointerId;
    longPressStartX = event.clientX;
    longPressStartY = event.clientY;

    window.addEventListener('pointermove', handleLongPressMove, { passive: true });
    window.addEventListener('pointerup', handleLongPressEnd);
    window.addEventListener('pointercancel', handleLongPressEnd);

    longPressTimer = setTimeout(() => {
      clearLongPressTimer();
      removeLongPressListeners();
      longPressPointerId = null;
      suppressNextClick = true;
      onEnterReorderMode();
      window.setTimeout(() => {
        suppressNextClick = false;
      }, 700);
    }, LONG_PRESS_MS);
  }

  function keepDraggedCategoryQuiet(element: HTMLElement | undefined): void {
    if (!element) return;
    element.style.outline = 'none';
  }

  function handleReorderConsider(event: CustomEvent<DndEvent<V2Category>>): void {
    onReorderConsider(event.detail.items);
  }

  async function handleReorderFinalize(event: CustomEvent<DndEvent<V2Category>>): Promise<void> {
    try {
      await onReorderFinalize(event.detail.items);
    } catch {
      // The v2 store owns the visible error banner; keep the rail interactive.
    }
  }

  $effect(() => {
    selectedCategoryId;
    categorySignature;
    isReorderMode;
    void updateIndicator(!isReorderMode);
  });

  onMount(() => {
    const motionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
    const handleMotionPreferenceChange = (event: MediaQueryListEvent): void => {
      prefersReducedMotion = event.matches;
      void updateIndicator(false);
    };

    prefersReducedMotion = motionQuery.matches;
    motionQuery.addEventListener('change', handleMotionPreferenceChange);
    window.addEventListener('resize', handleResize);
    void updateIndicator(false);

    return () => {
      motionQuery.removeEventListener('change', handleMotionPreferenceChange);
      window.removeEventListener('resize', handleResize);
    };
  });

  onDestroy(() => {
    cancelLongPress();
  });
</script>

<div class="flex w-full min-w-0 items-center rounded-[16px] border-2 border-[var(--color-stroke)] bg-[rgb(255_255_255_/_0.7)] p-1">
  <div class="min-w-0 flex-1 basis-0">
    <div class="scrollbar-hide min-w-0 overflow-x-auto">
      <div class="relative min-w-full">
        <div
          class="pointer-events-none absolute left-0 top-0 z-0 rounded-[12px] border-2 border-[var(--color-ink)] bg-[var(--color-accent-sky)] shadow-sm will-change-transform"
          style={`transform: translate3d(${indicatorLeft}px, ${indicatorTop}px, 0); width: ${indicatorWidth}px; height: ${indicatorHeight}px; opacity: ${isIndicatorReady ? 1 : 0}; transition: ${indicatorTransition};`}
          aria-hidden="true"
        ></div>

        <div
          bind:this={segmentTrack}
          use:dndzone={{
            items: categories,
            flipDurationMs: reorderFlipDuration,
            type: 'v2-categories',
            dragDisabled: !isReorderMode || isReorderBusy || categories.length <= 1,
            morphDisabled: true,
            dropFromOthersDisabled: true,
            dropTargetStyle: { outline: 'none' },
            dropTargetClasses: [],
            delayTouchStart: 120,
            transformDraggedElement: keepDraggedCategoryQuiet
          }}
          onconsider={handleReorderConsider}
          onfinalize={(event) => void handleReorderFinalize(event)}
          class="relative z-10 flex min-w-full gap-1"
          aria-label={i18n.t('v2Categories')}
        >
          {#each categories as category (category.id)}
            {@const isSelected = category.id === selectedCategoryId}
            <div
              animate:flip={{ duration: reorderFlipDuration }}
              class="category-segment shrink-0 outline-none focus:outline-none focus-visible:outline-none"
              data-category-id={category.id}
            >
              <button
                type="button"
                class={`relative flex min-h-11 max-w-44 items-center justify-center rounded-[12px] border-2 border-transparent px-4 text-sm font-semibold transition-colors disabled:cursor-not-allowed disabled:opacity-70 ${
                  isSelected
                    ? 'text-[var(--color-ink)]'
                    : 'text-[var(--color-ink-muted)] hover:bg-[var(--color-canvas)] hover:text-[var(--color-ink)] active:bg-[var(--color-mist)]'
                } ${isReorderMode ? 'category-wiggle cursor-grab active:cursor-grabbing' : ''}`}
                aria-current={isSelected ? 'true' : undefined}
                aria-disabled={isReorderMode}
                title={isReorderMode ? i18n.t('v2CategoryOrderHint') : category.name}
                onpointerdown={startLongPress}
                onclick={() => void selectCategory(category.id)}
              >
                <span class="min-w-0 truncate">{category.name}</span>
              </button>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </div>

  <div class="mx-1 h-8 w-px shrink-0 bg-[var(--color-stroke)]" aria-hidden="true"></div>

  <div class="flex shrink-0 items-center gap-1.5">
    {#if isReorderMode}
      <button
        type="button"
        class="flex h-10 items-center justify-center gap-1.5 rounded-[13px] border-2 border-[var(--color-ink)] bg-[var(--color-white)] px-2.5 text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-accent-sky)] disabled:cursor-not-allowed disabled:opacity-45"
        aria-label={i18n.t('v2FinishCategoryOrder')}
        title={i18n.t('v2FinishCategoryOrder')}
        disabled={isReorderBusy}
        onclick={onFinishReorderMode}
      >
        <Check size={17} strokeWidth={2.6} aria-hidden="true" />
        <span>{i18n.t('v2FinishCategoryOrder')}</span>
      </button>
    {:else}
      <button
        type="button"
        class="flex h-10 w-10 items-center justify-center rounded-full border-2 border-[var(--color-stroke)] bg-[rgb(255_255_255_/_0.76)] text-[var(--color-ink)] transition-colors hover:border-[var(--color-ink)] hover:bg-[var(--color-canvas)] disabled:cursor-not-allowed disabled:opacity-40"
        aria-label={i18n.t('v2ManageCategory')}
        title={i18n.t('v2ManageCategory')}
        disabled={!selectedCategory}
        onclick={() => selectedCategory && onManageCategory(selectedCategory)}
      >
        <SlidersHorizontal size={20} strokeWidth={2.4} aria-hidden="true" />
      </button>
    {/if}
  </div>
</div>

<style>
  @keyframes v2-category-wiggle {
    0%,
    100% {
      transform: rotate(-1.2deg) translateY(0);
    }

    50% {
      transform: rotate(1.2deg) translateY(-1px);
    }
  }

  .category-wiggle {
    animation: v2-category-wiggle 720ms ease-in-out infinite;
    transform-origin: center;
  }

  .category-segment:nth-child(2n) > .category-wiggle {
    animation-delay: -140ms;
  }

  .category-segment:nth-child(3n) > .category-wiggle {
    animation-delay: -260ms;
  }

  @media (prefers-reduced-motion: reduce) {
    .category-wiggle {
      animation: none;
    }
  }
</style>
