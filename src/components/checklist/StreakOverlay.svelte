<script lang="ts">
  import { cubicOut } from 'svelte/easing';
  import { fade, fly } from 'svelte/transition';
  import { Flame, X } from '@lucide/svelte';

  import { i18n } from '$lib/i18n';
  import type { StreakHeatmap } from '../../types';
  import StreakCard from './StreakCard.svelte';

  type MaybePromise = void | Promise<void>;

  interface Props {
    show: boolean;
    heatmaps?: StreakHeatmap[];
    isLoading?: boolean;
    errorMessage?: string | null;
    onRefresh?: () => MaybePromise;
    onClose: () => MaybePromise;
  }

  let {
    show,
    heatmaps = [],
    isLoading = false,
    errorMessage = null,
    onRefresh,
    onClose
  }: Props = $props();
</script>

{#if show}
  <div
    class="fixed inset-0 z-[90] flex flex-col bg-[var(--color-canvas)]/95 text-[var(--color-ink)] backdrop-blur-sm"
    transition:fade={{ duration: 140 }}
  >
    <header class="flex shrink-0 items-center gap-3 px-5 pb-3 pt-[max(1rem,var(--safe-area-top))]">
      <span class="grid h-11 w-11 place-items-center rounded-full border-2 border-[var(--color-ink)] bg-[var(--color-white)]">
        <Flame size={22} strokeWidth={2.4} aria-hidden="true" />
      </span>
      <div class="min-w-0 flex-1">
        <h2 class="text-xl font-bold leading-tight">{i18n.t('checklistStreakOverlayTitle')}</h2>
        <p class="mt-0.5 text-sm font-semibold text-[var(--color-ink-muted)]">
          {i18n.t('checklistStreakOverlaySubtitle')}
        </p>
      </div>
      <button
        type="button"
        class="grid h-11 w-11 place-items-center rounded-full bg-[var(--color-white)] text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-paper)]"
        aria-label={i18n.t('close')}
        title={i18n.t('close')}
        onclick={() => void onClose()}
      >
        <X size={22} strokeWidth={2.4} />
      </button>
    </header>

    <div class="min-h-0 flex-1 overflow-y-auto px-5 pb-[max(1.25rem,var(--safe-area-bottom))]">
      {#if isLoading}
        <div class="grid min-h-[280px] place-items-center text-sm font-semibold text-[var(--color-ink-muted)]">
          {i18n.t('loading')}
        </div>
      {:else if errorMessage}
        <div
          class="rounded-[6px_24px_6px_24px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] p-4"
          transition:fly={{ y: 8, duration: 180, easing: cubicOut }}
        >
          <p class="text-base font-bold">{i18n.t('checklistStreakLoadErrorTitle')}</p>
          <p class="mt-1 text-sm font-semibold text-[var(--color-ink-muted)]">{errorMessage}</p>
          {#if onRefresh}
            <button
              type="button"
              class="mt-4 min-h-11 rounded-[14px] bg-[var(--color-accent-sky-strong)] px-4 text-sm font-bold text-[var(--color-ink)]"
              onclick={() => void onRefresh()}
            >
              {i18n.t('retry')}
            </button>
          {/if}
        </div>
      {:else if heatmaps.length === 0}
        <div
          class="rounded-[6px_24px_6px_24px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] p-5 text-center"
          transition:fly={{ y: 8, duration: 180, easing: cubicOut }}
        >
          <div class="mx-auto grid h-12 w-12 place-items-center rounded-full bg-[var(--color-white)]">
            <Flame size={24} strokeWidth={2.4} aria-hidden="true" />
          </div>
          <p class="mt-3 text-lg font-bold">{i18n.t('checklistStreakOverlayEmptyTitle')}</p>
          <p class="mt-1 text-sm font-semibold leading-6 text-[var(--color-ink-muted)]">
            {i18n.t('checklistStreakOverlayEmptyMessage')}
          </p>
        </div>
      {:else}
        <div class="flex flex-col gap-3">
          {#each heatmaps as heatmap (heatmap.item.id)}
            <StreakCard {heatmap} />
          {/each}
        </div>
      {/if}
    </div>
  </div>
{/if}
