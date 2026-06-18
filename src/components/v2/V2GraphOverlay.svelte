<script lang="ts">
  import { cubicOut } from 'svelte/easing';
  import { fade, fly } from 'svelte/transition';
  import { RefreshCw, X } from '@lucide/svelte';

  import { i18n } from '$lib/i18n';
  import type { V2GraphData } from '../../types';
  import V2GraphCanvas from './V2GraphCanvas.svelte';

  type MaybePromise = void | Promise<void>;

  interface Props {
    show: boolean;
    data: V2GraphData | null;
    isLoading?: boolean;
    errorMessage?: string | null;
    onRefresh?: () => MaybePromise;
    onItemSelect: (itemId: number) => MaybePromise;
    onClose: () => MaybePromise;
  }

  let {
    show,
    data,
    isLoading = false,
    errorMessage = null,
    onRefresh,
    onItemSelect,
    onClose
  }: Props = $props();

  let graphSignature = $derived(
    data
      ? [
          data.categories.map((category) => `${category.id}:${category.name}`).join('|'),
          data.items
            .map(
              (item) =>
                `${item.id}:${item.text}:${item.done}:${item.category_id}:${item.tags.map((tag) => tag.name).join(',')}`
            )
            .join('|'),
          data.tag_edges.map((edge) => `${edge.tag_id}:${edge.item_id}`).join('|')
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
        <span class="relative h-6 w-6" aria-hidden="true">
          <span class="absolute left-1 top-1 h-2 w-2 rounded-full border-2 border-[var(--color-ink)]"></span>
          <span class="absolute right-1 top-1 h-2 w-2 rounded-full border-2 border-[var(--color-ink)]"></span>
          <span class="absolute bottom-1 left-1/2 h-2 w-2 -translate-x-1/2 rounded-full border-2 border-[var(--color-ink)]"></span>
          <span class="absolute left-[7px] top-[9px] h-[2px] w-[11px] rotate-[24deg] rounded-full bg-[var(--color-ink)]"></span>
          <span class="absolute right-[7px] top-[9px] h-[2px] w-[11px] -rotate-[24deg] rounded-full bg-[var(--color-ink)]"></span>
        </span>
      </span>
      <div class="min-w-0 flex-1">
        <h2 class="text-xl font-bold leading-tight">{i18n.t('v2GraphOverlayTitle')}</h2>
        <p class="mt-0.5 text-sm font-semibold text-[var(--color-ink-muted)]">
          {i18n.t('v2GraphOverlaySubtitle')}
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
          <p class="text-base font-bold">{i18n.t('v2GraphLoadErrorTitle')}</p>
          <p class="mt-1 text-sm font-semibold text-[var(--color-ink-muted)]">{errorMessage}</p>
        </div>
      {:else if !data || data.items.length === 0}
        <div
          class="rounded-[6px_24px_6px_24px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] p-5 text-center"
          transition:fly={{ y: 8, duration: 180, easing: cubicOut }}
        >
          <p class="text-lg font-bold">{i18n.t('v2GraphEmptyTitle')}</p>
          <p class="mt-1 text-sm font-semibold leading-6 text-[var(--color-ink-muted)]">
            {i18n.t('v2GraphEmptyMessage')}
          </p>
        </div>
      {:else}
        <div class="relative h-full overflow-hidden rounded-[6px_24px_6px_24px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)]">
          {#key graphSignature}
            <V2GraphCanvas {data} {onItemSelect} />
          {/key}
        </div>
      {/if}
    </div>
  </div>
{/if}
