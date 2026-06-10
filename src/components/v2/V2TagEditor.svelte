<script lang="ts">
  import { Hash, X } from '@lucide/svelte';

  import type { V2Tag } from '../../types';
  import { i18n } from '$lib/i18n';
  import {
    normalizeV2TagName,
    normalizeV2TagNames,
    suggestV2Tags
  } from '$lib/v2/v2TagInput';

  type MaybePromise = void | Promise<void>;

  interface Props {
    tagNames?: string[];
    availableTags?: V2Tag[];
    disabled?: boolean;
    onChange: (tagNames: string[]) => MaybePromise;
  }

  let {
    tagNames = [],
    availableTags = [],
    disabled = false,
    onChange
  }: Props = $props();

  let draftTag = $state('');
  let normalizedTagNames = $derived(normalizeV2TagNames(tagNames));
  let draftSuggestions = $derived(
    suggestV2Tags(availableTags, draftTag, 5).filter(
      (tag) =>
        !normalizedTagNames
          .map((name) => name.toLocaleLowerCase())
          .includes(tag.name.toLocaleLowerCase())
    )
  );

  function commitTag(name = draftTag): void {
    const normalizedName = normalizeV2TagName(name);
    if (!normalizedName) return;

    const nextNames = normalizeV2TagNames([...normalizedTagNames, normalizedName]);
    draftTag = '';
    void onChange(nextNames);
  }

  function removeTag(name: string): void {
    const key = name.toLocaleLowerCase();
    void onChange(normalizedTagNames.filter((tagName) => tagName.toLocaleLowerCase() !== key));
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === 'Enter' || event.key === ',' || event.key === ' ') {
      event.preventDefault();
      commitTag();
    }

    if (event.key === 'Backspace' && !draftTag && normalizedTagNames.length > 0) {
      event.preventDefault();
      void onChange(normalizedTagNames.slice(0, -1));
    }
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex min-h-12 flex-wrap items-center gap-2 rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-3 py-2 focus-within:bg-[var(--color-canvas)]">
    {#each normalizedTagNames as tagName (tagName.toLocaleLowerCase())}
      <span class="inline-flex min-h-8 max-w-full items-center gap-1 rounded-full bg-[var(--color-white)] px-2.5 text-sm font-semibold text-[var(--color-ink)]">
        <Hash size={13} strokeWidth={2.5} aria-hidden="true" />
        <span class="truncate">{tagName}</span>
        <button
          type="button"
          class="grid h-7 w-7 place-items-center rounded-full text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-canvas)] disabled:opacity-50"
          aria-label={i18n.t('v2RemoveTagTemplate')(tagName)}
          title={i18n.t('v2RemoveTagTemplate')(tagName)}
          disabled={disabled}
          onclick={() => removeTag(tagName)}
        >
          <X size={14} strokeWidth={2.5} aria-hidden="true" />
        </button>
      </span>
    {/each}

    <input
      bind:value={draftTag}
      class="min-h-8 min-w-28 flex-1 bg-transparent text-base text-[var(--color-ink)] outline-none placeholder:text-[var(--color-ink-muted)] disabled:opacity-50"
      placeholder={normalizedTagNames.length === 0 ? i18n.t('v2ItemTagsPlaceholder') : i18n.t('v2AddAnotherTag')}
      aria-label={i18n.t('v2ItemTagsLabel')}
      autocomplete="off"
      {disabled}
      onkeydown={handleKeydown}
    />
  </div>

  {#if draftTag.trim() && draftSuggestions.length > 0}
    <div class="flex flex-wrap gap-2">
      {#each draftSuggestions as tag (tag.id)}
        <button
          type="button"
          class="inline-flex min-h-8 items-center gap-1 rounded-full bg-[var(--color-canvas)] px-3 text-xs font-semibold text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-mist)] disabled:opacity-50"
          disabled={disabled}
          onclick={() => commitTag(tag.name)}
        >
          <Hash size={12} strokeWidth={2.4} aria-hidden="true" />
          <span>{tag.name}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>
