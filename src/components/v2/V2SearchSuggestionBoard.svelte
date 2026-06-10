<script lang="ts">
  import { Search } from '@lucide/svelte';

  import type { V2ItemSearchResult } from '../../types';
  import { i18n } from '$lib/i18n';

  type MaybePromise = void | Promise<void>;

  interface Props {
    query: string;
    results?: V2ItemSearchResult[];
    isLoading?: boolean;
    onSelectResult: (result: V2ItemSearchResult) => MaybePromise;
  }

  let { query, results = [], isLoading = false, onSelectResult }: Props = $props();

  let normalizedQuery = $derived(query.trim().toLocaleLowerCase());

  function getMemoSearchSnippet(result: V2ItemSearchResult): string | null {
    const memo = result.item.memo?.trim();
    if (!memo || !normalizedQuery) return null;
    if (result.item.text.toLocaleLowerCase().includes(normalizedQuery)) return null;
    if (!memo.toLocaleLowerCase().includes(normalizedQuery)) return null;

    return memo.length > 72 ? `${memo.slice(0, 72).trim()}...` : memo;
  }

  function getTagSearchSnippet(result: V2ItemSearchResult): string | null {
    if (!normalizedQuery) return null;
    if (result.item.text.toLocaleLowerCase().includes(normalizedQuery)) return null;
    if ((result.item.memo ?? '').toLocaleLowerCase().includes(normalizedQuery)) return null;

    const matchedTag = result.item.tags.find((tag) =>
      tag.name.toLocaleLowerCase().includes(normalizedQuery)
    );

    return matchedTag ? `#${matchedTag.name}` : null;
  }
</script>

<div
  class="max-h-[min(52vh,360px)] overflow-y-auto rounded-[5px_18px_5px_18px] border-[3px] border-[var(--color-ink)] bg-[var(--color-white)] p-2 shadow-xl"
  role="region"
  aria-label={i18n.t('v2SearchSuggestions')}
>
  <div class="flex items-center gap-2 px-2 pb-1 pt-1 text-xs font-semibold text-[var(--color-ink-muted)]">
    <Search size={14} strokeWidth={2.4} aria-hidden="true" />
    <span>{i18n.t('v2SearchSuggestions')}</span>
  </div>

  {#if isLoading}
    <div class="flex min-h-11 items-center rounded-[12px] px-3 text-sm text-[var(--color-ink-muted)]">
      {i18n.t('v2Searching')}
    </div>
  {:else if results.length === 0}
    <div class="flex min-h-11 items-center rounded-[12px] px-3 text-sm text-[var(--color-ink-muted)]">
      {i18n.t('v2NoSearchResultsTemplate')(query)}
    </div>
  {:else}
    <div class="flex flex-col gap-1">
      {#each results as result (result.item.id)}
        {@const memoSnippet = getMemoSearchSnippet(result)}
        {@const tagSnippet = getTagSearchSnippet(result)}
        <button
          type="button"
          class="flex min-h-11 min-w-0 items-center gap-3 rounded-[12px] px-3 py-1.5 text-left transition-colors hover:bg-[var(--color-canvas)] active:bg-[var(--color-mist)]"
          onclick={() => void onSelectResult(result)}
        >
          <span class="min-w-0 flex-1">
            <span
              class={`block truncate text-sm font-semibold ${
                result.item.done
                  ? 'text-[var(--color-ink-muted)] line-through decoration-2'
                  : 'text-[var(--color-ink)]'
              }`}
            >
              {result.item.text}
            </span>
            {#if memoSnippet}
              <span class="mt-0.5 block truncate text-xs font-medium text-[var(--color-ink-muted)]">
                {i18n.t('v2MemoSearchSnippetTemplate')(memoSnippet)}
              </span>
            {:else if tagSnippet}
              <span class="mt-0.5 block truncate text-xs font-medium text-[var(--color-ink-muted)]">
                {i18n.t('v2TagSearchSnippetTemplate')(tagSnippet)}
              </span>
            {/if}
          </span>
          <span class="max-w-28 shrink-0 truncate rounded-full bg-[var(--color-paper)] px-2.5 py-1 text-xs font-semibold text-[var(--color-ink-muted)]">
            {result.category.name}
          </span>
        </button>
      {/each}
    </div>
  {/if}
</div>
