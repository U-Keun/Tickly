<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { Plus, SlidersHorizontal } from '@lucide/svelte';

  import { i18n } from '$lib/i18n';
  import type { V2Category } from '../../types';

  type MaybePromise = void | Promise<void>;

  interface Props {
    categories: V2Category[];
    selectedCategoryId: number | null;
    onSelectCategory: (id: number) => MaybePromise;
    onCreateCategory: () => void;
    onManageCategory: (category: V2Category) => void;
  }

  let {
    categories,
    selectedCategoryId,
    onSelectCategory,
    onCreateCategory,
    onManageCategory
  }: Props = $props();

  let selectedCategory = $derived(
    categories.find((category) => category.id === selectedCategoryId) ?? null
  );
  let categorySignature = $derived(categories.map((category) => category.id).join(','));
  let segmentTrack = $state<HTMLDivElement | null>(null);
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

  function selectedButton(): HTMLButtonElement | null {
    if (!segmentTrack || selectedCategoryId === null) return null;
    return segmentTrack.querySelector<HTMLButtonElement>(`[data-category-id="${selectedCategoryId}"]`);
  }

  async function updateIndicator(shouldScroll = false): Promise<void> {
    await tick();

    const button = selectedButton();
    if (!button) {
      isIndicatorReady = false;
      return;
    }

    indicatorLeft = button.offsetLeft;
    indicatorTop = button.offsetTop;
    indicatorWidth = button.offsetWidth;
    indicatorHeight = button.offsetHeight;
    isIndicatorReady = true;

    if (shouldScroll) {
      button.scrollIntoView({
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
    try {
      await onSelectCategory(id);
    } catch {
      // The v2 store owns the visible error banner; keep the rail state stable.
    }
  }

  $effect(() => {
    selectedCategoryId;
    categorySignature;
    void updateIndicator(true);
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
</script>

<div class="flex w-full items-center gap-2">
  <div class="min-w-0 flex-1 rounded-[16px] border-2 border-[var(--color-stroke)] bg-[rgb(255_255_255_/_0.7)] p-1">
    <div bind:this={segmentTrack} class="scrollbar-hide relative flex min-w-0 gap-1 overflow-x-auto">
      <div
        class="pointer-events-none absolute left-0 top-0 z-0 rounded-[12px] border-2 border-[var(--color-ink)] bg-[var(--color-accent-sky)] shadow-sm will-change-transform"
        style={`transform: translate3d(${indicatorLeft}px, ${indicatorTop}px, 0); width: ${indicatorWidth}px; height: ${indicatorHeight}px; opacity: ${isIndicatorReady ? 1 : 0}; transition: ${indicatorTransition};`}
        aria-hidden="true"
      ></div>

      {#each categories as category (category.id)}
        {@const isSelected = category.id === selectedCategoryId}
        <button
          type="button"
          class={`relative z-10 flex min-h-10 max-w-44 shrink-0 items-center justify-center rounded-[12px] border-2 border-transparent px-4 text-sm font-semibold transition-colors disabled:cursor-not-allowed disabled:opacity-70 ${
            isSelected
              ? 'text-[var(--color-ink)]'
              : 'text-[var(--color-ink-muted)] hover:bg-[var(--color-canvas)] hover:text-[var(--color-ink)] active:bg-[var(--color-mist)]'
          }`}
          data-category-id={category.id}
          aria-current={isSelected ? 'true' : undefined}
          title={category.name}
          onclick={() => void selectCategory(category.id)}
        >
          <span class="min-w-0 truncate">{category.name}</span>
        </button>
      {/each}
    </div>
  </div>

  <div class="flex shrink-0 items-center gap-2">
    <button
      type="button"
      class="flex h-11 w-11 items-center justify-center rounded-full border-2 border-[var(--color-ink)] bg-[var(--color-white)] text-[var(--color-ink)] transition-colors hover:bg-[var(--color-accent-sky)] disabled:cursor-not-allowed disabled:opacity-45"
      aria-label={i18n.t('v2AddCategory')}
      title={i18n.t('v2AddCategory')}
      onclick={onCreateCategory}
    >
      <Plus size={22} strokeWidth={2.6} aria-hidden="true" />
    </button>

    <button
      type="button"
      class="flex h-11 w-11 items-center justify-center rounded-full border-2 border-[var(--color-stroke)] bg-[var(--color-white)] text-[var(--color-ink)] transition-colors hover:border-[var(--color-ink)] hover:bg-[var(--color-canvas)] disabled:cursor-not-allowed disabled:opacity-40"
      aria-label={i18n.t('v2ManageCategory')}
      title={i18n.t('v2ManageCategory')}
      disabled={!selectedCategory}
      onclick={() => selectedCategory && onManageCategory(selectedCategory)}
    >
      <SlidersHorizontal size={21} strokeWidth={2.4} aria-hidden="true" />
    </button>
  </div>
</div>
