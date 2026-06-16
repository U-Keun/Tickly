<script lang="ts">
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

  let normalizedDetail = $derived(normalizeV2RepeatDetail(repeatType, repeatDetail));

  function selectRepeatType(nextType: V2RepeatType): void {
    if (disabled) return;

    const nextDetail =
      nextType === repeatType
        ? normalizedDetail
        : nextType === 'weekly'
          ? [new Date().getDay()]
          : nextType === 'monthly'
            ? [new Date().getDate()]
            : [];

    void onChange(nextType, normalizeV2RepeatDetail(nextType, nextDetail));
  }

  function toggleDetail(value: number): void {
    if (disabled) return;

    const nextDetail = normalizedDetail.includes(value)
      ? normalizedDetail.filter((item) => item !== value)
      : [...normalizedDetail, value];

    void onChange(repeatType, normalizeV2RepeatDetail(repeatType, nextDetail));
  }
</script>

<div class="flex flex-col gap-3 rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] p-3">
  <div class="flex items-center gap-2 text-sm font-semibold text-[var(--color-ink)]">
    <Repeat2 size={16} strokeWidth={2.4} aria-hidden="true" />
    <span>{i18n.t('v2ItemRepeatLabel')}</span>
  </div>

  <div class="grid grid-cols-2 gap-2">
    {#each V2_REPEAT_TYPES as type (type)}
      <button
        type="button"
        class={`min-h-11 rounded-[12px] px-3 text-sm font-semibold transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${
          repeatType === type
            ? 'border-2 border-[var(--color-ink)] bg-[var(--color-accent-sky)] text-[var(--color-ink)]'
            : 'border-2 border-[var(--color-stroke)] bg-[var(--color-white)] text-[var(--color-ink-muted)] hover:bg-[var(--color-canvas)]'
        }`}
        {disabled}
        aria-pressed={repeatType === type}
        onclick={() => selectRepeatType(type)}
      >
        {i18n.t(repeatLabelKeys[type])}
      </button>
    {/each}
  </div>

  {#if repeatType === 'weekly'}
    <div class="flex flex-col gap-2">
      <span class="text-xs font-semibold text-[var(--color-ink-muted)]">{i18n.t('repeatDaysLabel')}</span>
      <div class="grid grid-cols-7 gap-1.5">
        {#each V2_WEEKDAY_VALUES as day (day)}
          <button
            type="button"
            class={`min-h-10 rounded-[10px] text-xs font-bold transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${
              normalizedDetail.includes(day)
                ? 'border-2 border-[var(--color-ink)] bg-[var(--color-accent-sky)] text-[var(--color-ink)]'
                : 'border-2 border-[var(--color-stroke)] bg-[var(--color-white)] text-[var(--color-ink-muted)] hover:bg-[var(--color-canvas)]'
            }`}
            {disabled}
            aria-pressed={normalizedDetail.includes(day)}
            onclick={() => toggleDetail(day)}
          >
            {i18n.t(weekdayLabelKeys[day])}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  {#if repeatType === 'monthly'}
    <div class="flex flex-col gap-2">
      <span class="text-xs font-semibold text-[var(--color-ink-muted)]">{i18n.t('repeatDatesLabel')}</span>
      <div class="grid grid-cols-7 gap-1.5">
        {#each V2_MONTH_DAY_VALUES as day (day)}
          <button
            type="button"
            class={`min-h-9 rounded-[10px] text-xs font-bold transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${
              normalizedDetail.includes(day)
                ? 'border-2 border-[var(--color-ink)] bg-[var(--color-accent-sky)] text-[var(--color-ink)]'
                : 'border-2 border-[var(--color-stroke)] bg-[var(--color-white)] text-[var(--color-ink-muted)] hover:bg-[var(--color-canvas)]'
            }`}
            {disabled}
            aria-pressed={normalizedDetail.includes(day)}
            onclick={() => toggleDetail(day)}
          >
            {day}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>
