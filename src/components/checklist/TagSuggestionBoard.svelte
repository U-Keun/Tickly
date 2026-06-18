<script lang="ts">
  import { Hash } from '@lucide/svelte';

  import type { Tag } from '../../types';
  import { i18n } from '$lib/i18n';

  type MaybePromise = void | Promise<void>;

  interface Props {
    query: string;
    suggestions?: Tag[];
    createTagName?: string | null;
    selectedTagNames?: string[];
    onSelectTag: (tag: Tag) => MaybePromise;
    onCreateTag?: (tagName: string) => MaybePromise;
    onRemoveSelectedTag?: (tagName: string) => MaybePromise;
  }

  let {
    query,
    suggestions = [],
    createTagName = null,
    selectedTagNames = [],
    onSelectTag,
    onCreateTag,
    onRemoveSelectedTag
  }: Props = $props();
</script>

<div
  class="max-h-[min(38vh,280px)] overflow-y-auto rounded-[5px_18px_5px_18px] border-[3px] border-[var(--color-ink)] bg-[var(--color-white)] p-2 shadow-xl"
  role="region"
  aria-label={i18n.t('checklistTagSuggestions')}
>
  <div class="flex items-center gap-2 px-2 pb-1 pt-1 text-xs font-semibold text-[var(--color-ink-muted)]">
    <Hash size={14} strokeWidth={2.4} aria-hidden="true" />
    <span>{i18n.t('checklistTagSuggestions')}</span>
  </div>

  {#if selectedTagNames.length > 0}
    <div
      class="mb-1 flex flex-wrap gap-1.5 rounded-[12px] bg-[var(--color-canvas)] px-2 py-2"
      aria-label={i18n.t('checklistSelectedTags')}
    >
      {#each selectedTagNames as tagName (tagName)}
        <button
          type="button"
          class="inline-flex min-h-8 max-w-full items-center gap-1 rounded-full border border-[var(--color-stroke)] bg-[var(--color-white)] px-2.5 text-xs font-semibold text-[var(--color-ink)]"
          aria-label={i18n.t('checklistRemoveTagTemplate')(tagName)}
          title={i18n.t('checklistRemoveTagTemplate')(tagName)}
          onclick={() => void onRemoveSelectedTag?.(tagName)}
        >
          <span class="truncate">#{tagName}</span>
          <span class="text-[var(--color-ink-muted)]" aria-hidden="true">×</span>
        </button>
      {/each}
    </div>
  {/if}

  {#if createTagName}
    <button
      type="button"
      class="mb-1 flex min-h-11 w-full min-w-0 items-center gap-2 rounded-[12px] border border-dashed border-[var(--color-stroke)] px-3 py-1.5 text-left text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] active:bg-[var(--color-mist)]"
      onclick={() => void onCreateTag?.(createTagName)}
    >
      <Hash size={15} strokeWidth={2.4} aria-hidden="true" />
      <span class="truncate">{i18n.t('checklistCreateTagSuggestionTemplate')(createTagName)}</span>
    </button>
  {/if}

  {#if suggestions.length === 0 && !createTagName && (query || selectedTagNames.length === 0)}
    <div class="flex min-h-11 items-center rounded-[12px] px-3 text-sm text-[var(--color-ink-muted)]">
      {query ? i18n.t('checklistNoTagSuggestionsTemplate')(query) : i18n.t('checklistNoTagsYet')}
    </div>
  {:else if suggestions.length > 0}
    <div class="flex flex-col gap-1">
      {#each suggestions as tag (tag.id)}
        <button
          type="button"
          class="flex min-h-11 min-w-0 items-center gap-2 rounded-[12px] px-3 py-1.5 text-left text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] active:bg-[var(--color-mist)]"
          onclick={() => void onSelectTag(tag)}
        >
          <Hash size={15} strokeWidth={2.4} aria-hidden="true" />
          <span class="truncate">{tag.name}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>
