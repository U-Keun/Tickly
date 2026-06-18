<script lang="ts">
  import { cubicOut } from 'svelte/easing';
  import { fade, fly } from 'svelte/transition';
  import { RefreshCw, X } from '@lucide/svelte';

  import { i18n } from '$lib/i18n';
  import type { GraphData, TodoItem } from '../../types';
  import GraphCanvas from './GraphCanvas.svelte';

  type MaybePromise<T = void> = T | Promise<T>;

  interface Props {
    show: boolean;
    data: GraphData | null;
    initialSelectedItemId?: number | null;
    isLoading?: boolean;
    errorMessage?: string | null;
    onRefresh?: () => MaybePromise;
    onItemEdit: (itemId: number) => MaybePromise;
    onItemToggle: (itemId: number) => MaybePromise<TodoItem>;
    onClose: () => MaybePromise;
  }

  let {
    show,
    data,
    initialSelectedItemId = null,
    isLoading = false,
    errorMessage = null,
    onRefresh,
    onItemEdit,
    onItemToggle,
    onClose
  }: Props = $props();

  let graphSignature = $derived(
    data
      ? [
          [...data.categories]
            .sort((a, b) => a.id - b.id)
            .map((category) => `${category.id}:${category.name}`)
            .join('|'),
          [...data.items]
            .sort((a, b) => a.id - b.id)
            .map(
              (item) =>
                `${item.id}:${item.text}:${item.category_id}:${item.tags
                  .map((tag) => tag.name)
                  .sort()
                  .join(',')}`
            )
            .join('|'),
          [...data.tag_edges]
            .sort((a, b) => a.tag_id - b.tag_id || a.item_id - b.item_id)
            .map((edge) => `${edge.tag_id}:${edge.item_id}`)
            .join('|')
        ].join('::')
      : 'empty'
  );
</script>

{#if show}
  <div
    class="fixed inset-0 z-[90] flex flex-col bg-[var(--color-canvas)]/95 text-[var(--color-ink)] backdrop-blur-sm"
    transition:fade={{ duration: 140 }}
  >
    <header class="flex shrink-0 items-center gap-3 px-5 pb-3 pt-[max(1rem,var(--safe-area-top))]">
      <span class="grid h-11 w-11 place-items-center rounded-full border-2 border-[var(--color-ink)] bg-[var(--color-white)]">
        <svg
          class="h-6 w-6 text-[var(--color-ink)]"
          viewBox="0 0 20 20"
          fill="none"
          aria-hidden="true"
        >
          <path
            d="M12.2 4.2H16.6V13.1M6.4 10.5H9V15.7H13.2"
            stroke="currentColor"
            stroke-width="2.05"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <circle cx="9.8" cy="4" r="2.8" stroke="currentColor" stroke-width="2.05" />
          <circle cx="3.8" cy="10.5" r="2.8" stroke="currentColor" stroke-width="2.05" />
          <circle cx="16" cy="15.8" r="2.8" stroke="currentColor" stroke-width="2.05" />
        </svg>
      </span>
      <div class="min-w-0 flex-1">
        <h2 class="text-xl font-bold leading-tight">{i18n.t('checklistGraphOverlayTitle')}</h2>
        <p class="mt-0.5 text-sm font-semibold text-[var(--color-ink-muted)]">
          {i18n.t('checklistGraphOverlaySubtitle')}
        </p>
      </div>
      {#if onRefresh}
        <button
          type="button"
          class="grid h-11 w-11 place-items-center rounded-full bg-[var(--color-white)] text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-paper)] disabled:cursor-not-allowed disabled:opacity-50"
          aria-label={i18n.t('retry')}
          title={i18n.t('retry')}
          disabled={isLoading}
          onclick={() => void onRefresh()}
        >
          <RefreshCw size={21} strokeWidth={2.4} />
        </button>
      {/if}
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

    <div class="relative min-h-0 flex-1 overflow-hidden px-5 pb-[max(1.25rem,var(--safe-area-bottom))]">
      {#if isLoading}
        <div class="grid h-full place-items-center text-sm font-semibold text-[var(--color-ink-muted)]">
          {i18n.t('loading')}
        </div>
      {:else if errorMessage}
        <div
          class="rounded-[6px_24px_6px_24px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] p-4"
          transition:fly={{ y: 8, duration: 180, easing: cubicOut }}
        >
          <p class="text-base font-bold">{i18n.t('checklistGraphLoadErrorTitle')}</p>
          <p class="mt-1 text-sm font-semibold text-[var(--color-ink-muted)]">{errorMessage}</p>
        </div>
      {:else if !data || data.items.length === 0}
        <div
          class="rounded-[6px_24px_6px_24px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] p-5 text-center"
          transition:fly={{ y: 8, duration: 180, easing: cubicOut }}
        >
          <p class="text-lg font-bold">{i18n.t('checklistGraphEmptyTitle')}</p>
          <p class="mt-1 text-sm font-semibold leading-6 text-[var(--color-ink-muted)]">
            {i18n.t('checklistGraphEmptyMessage')}
          </p>
        </div>
      {:else}
        <div class="relative h-full overflow-hidden rounded-[6px_24px_6px_24px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)]">
          {#key graphSignature}
            <GraphCanvas {data} {initialSelectedItemId} {onItemEdit} {onItemToggle} />
          {/key}
        </div>
      {/if}
    </div>
  </div>
{/if}
