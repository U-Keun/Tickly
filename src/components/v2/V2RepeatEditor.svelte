<script lang="ts">
  import { onDestroy, untrack } from 'svelte';
  import { Repeat2 } from '@lucide/svelte';

  import type { V2RepeatType } from '../../types';
  import { i18n } from '$lib/i18n';
  import {
    V2_MONTH_DAY_VALUES,
    V2_REPEAT_TYPES,
    V2_WEEKDAY_VALUES,
    normalizeV2RepeatDetail
  } from '$lib/v2/v2Repeat';

  type MaybePromise = void | Promise<void>;

  interface Props {
    repeatType: V2RepeatType;
    repeatDetail?: number[];
    disabled?: boolean;
    onChange: (repeatType: V2RepeatType, repeatDetail: number[]) => MaybePromise;
  }

  let {
    repeatType,
    repeatDetail = [],
    disabled = false,
    onChange
  }: Props = $props();

  const repeatLabelKeys: Record<V2RepeatType, 'repeatNone' | 'repeatDaily' | 'repeatWeekly' | 'repeatMonthly'> = {
    none: 'repeatNone',
    daily: 'repeatDaily',
    weekly: 'repeatWeekly',
    monthly: 'repeatMonthly'
  };
  const weekdayLabelKeys = ['sun', 'mon', 'tue', 'wed', 'thu', 'fri', 'sat'] as const;

  let visualRepeatType = $state<V2RepeatType>('none');
  let visualRepeatDetail = $state<number[]>([]);
  let normalizedDetail = $derived(normalizeV2RepeatDetail(visualRepeatType, visualRepeatDetail));
  let currentDetailType = $derived(detailTypeFor(visualRepeatType));
  let renderedDetailType = $state<Extract<V2RepeatType, 'weekly' | 'monthly'> | null>(null);
  let renderedNormalizedDetail = $derived(
    renderedDetailType ? normalizeV2RepeatDetail(renderedDetailType, repeatDetail) : []
  );
  let detailShellElement = $state<HTMLElement | null>(null);
  let detailContentElement = $state<HTMLElement | null>(null);
  let detailContentHeight = $state(0);
  let didInitializeDetailAnimation = false;
  let lastAnimatedDetailOpen = false;
  let lastAnimatedDetailHeight = 0;
  let detailAnimationFrame: number | null = null;

  $effect(() => {
    if (
      visualRepeatType !== repeatType ||
      normalizeV2RepeatDetail(visualRepeatType, visualRepeatDetail).join(',') !==
        normalizeV2RepeatDetail(repeatType, repeatDetail).join(',')
    ) {
      visualRepeatType = repeatType;
      visualRepeatDetail = repeatDetail;
    }
  });

  $effect(() => {
    const targetType = currentDetailType;
    if (targetType) {
      renderedDetailType = targetType;
    }
  });

  $effect(() => {
    const isOpen = currentDetailType !== null;
    const height = detailContentHeight;
    const shell = detailShellElement;
    const content = detailContentElement;

    untrack(() => {
      syncDetailAnimation(shell, content, isOpen, height);
    });
  });

  onDestroy(() => {
    cancelDetailAnimation();
  });

  function detailTypeFor(type: V2RepeatType): Extract<V2RepeatType, 'weekly' | 'monthly'> | null {
    return type === 'weekly' || type === 'monthly' ? type : null;
  }

  function measureDetailContent(node: HTMLElement) {
    const updateHeight = () => {
      detailContentHeight = node.scrollHeight;
    };

    detailContentElement = node;
    updateHeight();

    const observer = new ResizeObserver(updateHeight);
    observer.observe(node);

    return {
      destroy() {
        observer.disconnect();
        if (detailContentElement === node) {
          detailContentElement = null;
        }
      }
    };
  }

  function syncDetailAnimation(
    shell: HTMLElement | null,
    content: HTMLElement | null,
    isOpen: boolean,
    targetHeight: number
  ): void {
    if (!shell) return;

    const shouldReduceMotion =
      typeof window !== 'undefined' && window.matchMedia('(prefers-reduced-motion: reduce)').matches;

    if (!didInitializeDetailAnimation || shouldReduceMotion) {
      cancelDetailAnimation();
      shell.style.height = `${isOpen ? targetHeight : 0}px`;
      if (content) {
        content.style.opacity = isOpen ? '1' : '0';
        content.style.transform = isOpen ? 'translateY(0)' : 'translateY(-4px)';
      }
      didInitializeDetailAnimation = true;
      lastAnimatedDetailOpen = isOpen;
      lastAnimatedDetailHeight = targetHeight;
      return;
    }

    if (isOpen === lastAnimatedDetailOpen && targetHeight === lastAnimatedDetailHeight) return;

    const fromHeight = shell.getBoundingClientRect().height;
    const toHeight = isOpen ? targetHeight : 0;

    animateDetailFrameByFrame(shell, content, isOpen, fromHeight, toHeight);

    lastAnimatedDetailOpen = isOpen;
    lastAnimatedDetailHeight = targetHeight;
  }

  function cancelDetailAnimation(): void {
    if (detailAnimationFrame !== null) {
      cancelAnimationFrame(detailAnimationFrame);
      detailAnimationFrame = null;
    }
  }

  function easeOutCubic(progress: number): number {
    return 1 - Math.pow(1 - progress, 3);
  }

  function clampProgress(value: number): number {
    return Math.min(Math.max(value, 0), 1);
  }

  function animateDetailFrameByFrame(
    shell: HTMLElement,
    content: HTMLElement | null,
    isOpen: boolean,
    fromHeight: number,
    toHeight: number
  ): void {
    cancelDetailAnimation();

    const shellDelay = isOpen ? 0 : 120;
    const shellDuration = isOpen ? 320 : 240;
    const contentDelay = isOpen ? 120 : 0;
    const contentDuration = isOpen ? 190 : 150;
    const totalDuration = Math.max(shellDelay + shellDuration, contentDelay + contentDuration);
    let startedAt: number | null = null;

    const step = (timestamp: number) => {
      startedAt ??= timestamp;
      const elapsed = timestamp - startedAt;

      const shellProgress = easeOutCubic(clampProgress((elapsed - shellDelay) / shellDuration));
      const currentHeight = fromHeight + (toHeight - fromHeight) * shellProgress;
      shell.style.height = `${currentHeight}px`;

      if (content) {
        const contentProgress = easeOutCubic(clampProgress((elapsed - contentDelay) / contentDuration));
        const opacity = isOpen ? contentProgress : 1 - contentProgress;
        const translateY = isOpen ? (1 - contentProgress) * 8 : -4 * contentProgress;
        content.style.opacity = `${opacity}`;
        content.style.transform = `translateY(${translateY}px)`;
      }

      if (elapsed < totalDuration) {
        detailAnimationFrame = requestAnimationFrame(step);
        return;
      }

      shell.style.height = `${toHeight}px`;
      if (content) {
        content.style.opacity = isOpen ? '1' : '0';
        content.style.transform = isOpen ? 'translateY(0)' : 'translateY(-4px)';
      }
      detailAnimationFrame = null;
    };

    detailAnimationFrame = requestAnimationFrame(step);
  }

  function selectRepeatType(nextType: V2RepeatType): void {
    if (disabled) return;

    const nextDetail =
      nextType === visualRepeatType
        ? normalizedDetail
        : nextType === 'weekly'
          ? [new Date().getDay()]
          : nextType === 'monthly'
            ? [new Date().getDate()]
            : [];

    const normalizedNextDetail = normalizeV2RepeatDetail(nextType, nextDetail);
    visualRepeatType = nextType;
    visualRepeatDetail = normalizedNextDetail;
    void onChange(nextType, normalizedNextDetail);
  }

  function toggleDetail(value: number): void {
    if (disabled) return;

    const nextDetail = normalizedDetail.includes(value)
      ? normalizedDetail.filter((item) => item !== value)
      : [...normalizedDetail, value];

    const normalizedNextDetail = normalizeV2RepeatDetail(visualRepeatType, nextDetail);
    visualRepeatDetail = normalizedNextDetail;
    void onChange(visualRepeatType, normalizedNextDetail);
  }
</script>

<div class="flex flex-col gap-[10px] rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] p-3">
  <div class="flex items-center gap-[7px] text-sm font-semibold text-[var(--color-ink)]">
    <Repeat2 size={17} strokeWidth={2.4} aria-hidden="true" />
    <span>{i18n.t('v2ItemRepeatLabel')}</span>
  </div>

  <div class="grid grid-cols-4 gap-2">
    {#each V2_REPEAT_TYPES as type (type)}
      <button
        type="button"
        class={`min-h-11 overflow-hidden whitespace-nowrap rounded-[12px] px-1.5 text-[13px] font-semibold leading-4 transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${
          visualRepeatType === type
            ? 'border-2 border-[var(--color-ink)] bg-[var(--color-accent-sky)] text-[var(--color-ink)]'
            : 'border border-[var(--color-stroke)] bg-[var(--color-white)] text-[var(--color-ink-muted)] hover:bg-[var(--color-canvas)]'
        }`}
        {disabled}
        aria-pressed={visualRepeatType === type}
        onclick={() => selectRepeatType(type)}
      >
        <span class="block truncate whitespace-nowrap">{i18n.t(repeatLabelKeys[type])}</span>
      </button>
    {/each}
  </div>

  {#if renderedDetailType}
    <div
      bind:this={detailShellElement}
      class:repeat-detail-shell-open={currentDetailType !== null}
      class="repeat-detail-shell"
    >
      {#key renderedDetailType}
        <div use:measureDetailContent class="repeat-detail-content flex flex-col gap-[7px]">
          {#if renderedDetailType === 'weekly'}
            <span class="text-xs font-semibold text-[var(--color-ink-muted)]">{i18n.t('repeatDaysLabel')}</span>
            <div class="grid grid-cols-7 gap-1.5">
              {#each V2_WEEKDAY_VALUES as day (day)}
                <button
                  type="button"
                  class={`min-h-10 rounded-[10px] text-xs font-bold transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${
                    renderedNormalizedDetail.includes(day)
                      ? 'border-2 border-[var(--color-ink)] bg-[var(--color-accent-sky)] text-[var(--color-ink)]'
                      : 'border border-[var(--color-stroke)] bg-[var(--color-white)] text-[var(--color-ink-muted)] hover:bg-[var(--color-canvas)]'
                  }`}
                  {disabled}
                  aria-pressed={renderedNormalizedDetail.includes(day)}
                  onclick={() => toggleDetail(day)}
                >
                  {i18n.t(weekdayLabelKeys[day])}
                </button>
              {/each}
            </div>
          {:else}
            <span class="text-xs font-semibold text-[var(--color-ink-muted)]">{i18n.t('repeatDatesLabel')}</span>
            <div class="grid grid-cols-7 gap-1.5">
              {#each V2_MONTH_DAY_VALUES as day (day)}
                <button
                  type="button"
                  class={`min-h-9 rounded-[10px] text-xs font-bold transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${
                    renderedNormalizedDetail.includes(day)
                      ? 'border-2 border-[var(--color-ink)] bg-[var(--color-accent-sky)] text-[var(--color-ink)]'
                      : 'border border-[var(--color-stroke)] bg-[var(--color-white)] text-[var(--color-ink-muted)] hover:bg-[var(--color-canvas)]'
                  }`}
                  {disabled}
                  aria-pressed={renderedNormalizedDetail.includes(day)}
                  onclick={() => toggleDetail(day)}
                >
                  {day}
                </button>
              {/each}
            </div>
          {/if}
        </div>
      {/key}
    </div>
  {/if}
</div>

<style>
  .repeat-detail-shell {
    height: 0;
    overflow: hidden;
  }

  .repeat-detail-content {
    min-height: 0;
    opacity: 0;
    transform: translateY(-4px);
  }

  .repeat-detail-shell-open .repeat-detail-content {
    opacity: 1;
    transform: translateY(0);
  }

  @media (prefers-reduced-motion: reduce) {
    .repeat-detail-shell,
    .repeat-detail-content {
      animation: none;
    }
  }
</style>
