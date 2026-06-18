<script lang="ts">
  import { tick } from 'svelte';
  import { Flame, Repeat2 } from '@lucide/svelte';

  import type { StreakHeatmap } from '../../types';
  import { i18n } from '$lib/i18n';
  import { parseRepeatDetail } from '$lib/checklist/repeat';

  interface Props {
    heatmap: StreakHeatmap;
  }

  let { heatmap }: Props = $props();

  let heatmapScrollElement = $state<HTMLDivElement | null>(null);
  let lastScrolledItemId = $state<number | null>(null);
  let logByDate = $derived(new Map(heatmap.logs.map((log) => [log.completed_on, log])));
  let heatmapDates = $derived(buildHeatmapDates());
  let repeatSummary = $derived(formatRepeatSummary());

  $effect(() => {
    const container = heatmapScrollElement;
    const itemId = heatmap.item.id;
    if (!container || lastScrolledItemId === itemId) return;

    lastScrolledItemId = itemId;
    void scrollHeatmapToCurrent(container);
  });

  async function scrollHeatmapToCurrent(container: HTMLDivElement): Promise<void> {
    await tick();
    requestAnimationFrame(() => {
      container.scrollLeft = container.scrollWidth - container.clientWidth;
    });
  }

  function buildHeatmapDates(): string[] {
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    const dates: string[] = [];

    for (let offset = 364; offset >= 0; offset -= 1) {
      const date = new Date(today);
      date.setDate(today.getDate() - offset);
      dates.push(formatDateKey(date));
    }

    return dates;
  }

  function formatDateKey(date: Date): string {
    const year = date.getFullYear();
    const month = `${date.getMonth() + 1}`.padStart(2, '0');
    const day = `${date.getDate()}`.padStart(2, '0');
    return `${year}-${month}-${day}`;
  }

  function intensityClass(level: number): string {
    if (level >= 8) return 'bg-[var(--color-accent-peach-strong)]';
    if (level >= 5) return 'bg-[var(--color-accent-peach)]';
    if (level >= 3) return 'bg-[var(--color-accent-sky-strong)]';
    if (level >= 1) return 'bg-[var(--color-accent-sky)]';
    return 'bg-[var(--color-mist)]';
  }

  function formatRepeatSummary(): string {
    if (heatmap.item.repeat_type === 'daily') return i18n.t('repeatDaily');
    if (heatmap.item.repeat_type === 'none') return i18n.t('checklistStreakDailyCadence');

    const detail = parseRepeatDetail(heatmap.item.repeat_detail);
    if (heatmap.item.repeat_type === 'weekly') {
      const weekdayLabelKeys = ['sun', 'mon', 'tue', 'wed', 'thu', 'fri', 'sat'] as const;
      const labels = detail
        .filter((day) => day >= 0 && day < weekdayLabelKeys.length)
        .map((day) => i18n.t(weekdayLabelKeys[day]))
        .join(', ');
      return labels ? `${i18n.t('repeatWeekly')} · ${labels}` : i18n.t('repeatWeekly');
    }

    if (heatmap.item.repeat_type === 'monthly') {
      return detail.length > 0
        ? `${i18n.t('repeatMonthly')} · ${detail.join(', ')}`
        : i18n.t('repeatMonthly');
    }

    return i18n.t('checklistStreakDailyCadence');
  }
</script>

<article class="rounded-[6px_24px_6px_24px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] p-3 shadow-sm">
  <div class="flex items-start gap-3">
    <span class="grid h-10 w-10 shrink-0 place-items-center rounded-full bg-[var(--color-white)] text-[var(--color-ink)]">
      <Flame size={20} strokeWidth={2.4} aria-hidden="true" />
    </span>

    <div class="min-w-0 flex-1">
      <p class="line-clamp-2 break-words text-base font-semibold leading-snug text-[var(--color-ink)]">
        {heatmap.item.text}
      </p>
      <div class="mt-1 flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1 text-xs font-semibold text-[var(--color-ink-muted)]">
        <span class="truncate">{heatmap.category.name}</span>
        <span class="inline-flex items-center gap-1">
          <Repeat2 size={12} strokeWidth={2.4} aria-hidden="true" />
          <span>{repeatSummary}</span>
        </span>
      </div>
    </div>
  </div>

  <div class="mt-3 grid grid-cols-3 gap-2 text-center">
    <div class="rounded-[12px] bg-[var(--color-white)] px-2 py-2">
      <p class="text-[11px] font-semibold text-[var(--color-ink-muted)]">{i18n.t('checklistStreakTotalDays')}</p>
      <p class="mt-0.5 text-lg font-bold text-[var(--color-ink)]">{heatmap.total_days}</p>
    </div>
    <div class="rounded-[12px] bg-[var(--color-white)] px-2 py-2">
      <p class="text-[11px] font-semibold text-[var(--color-ink-muted)]">{i18n.t('checklistStreakCurrent')}</p>
      <p class="mt-0.5 text-lg font-bold text-[var(--color-ink)]">{heatmap.current_streak}</p>
    </div>
    <div class="rounded-[12px] bg-[var(--color-white)] px-2 py-2">
      <p class="text-[11px] font-semibold text-[var(--color-ink-muted)]">{i18n.t('checklistStreakLongest')}</p>
      <p class="mt-0.5 text-lg font-bold text-[var(--color-ink)]">{heatmap.longest_streak}</p>
    </div>
  </div>

  <div
    bind:this={heatmapScrollElement}
    class="mt-3 overflow-x-auto pb-1"
    aria-label={i18n.t('checklistStreakHeatmapLabel')}
  >
    <div class="grid w-max grid-flow-col grid-rows-7 gap-[3px]">
      {#each heatmapDates as date (date)}
        {@const log = logByDate.get(date)}
        <span
          class={`h-2.5 w-2.5 rounded-[3px] ${intensityClass(log?.combo_intensity ?? 0)}`}
          title={log ? `${date} · ${log.completed_count}` : date}
          aria-label={log ? `${date} ${i18n.t('completed')}` : `${date} ${i18n.t('notCompleted')}`}
        ></span>
      {/each}
    </div>
  </div>
</article>
