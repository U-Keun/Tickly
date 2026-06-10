<script lang="ts">
  import { tick } from 'svelte';
  import { ArrowLeft, Plus, Search, X } from '@lucide/svelte';

  import type { V2Tag } from '../../types';
  import { iosFocusFix } from '$lib/iosFocusFix';
  import { i18n } from '$lib/i18n';
  import {
    getActiveV2TagToken,
    normalizeV2TagName,
    normalizeV2TagNames,
    removeV2TagToken,
    suggestV2Tags,
    type V2InlineTagToken
  } from '$lib/v2/v2TagInput';
  import V2TagSuggestionBoard from './V2TagSuggestionBoard.svelte';

  type MaybePromise = void | Promise<void>;
  type CommandMode = 'add' | 'search';

  interface Props {
    disabled?: boolean;
    initialInput?: string;
    mode?: CommandMode;
    searchQuery?: string;
    availableTags?: V2Tag[];
    onAddItem: (text: string, tagNames?: string[]) => MaybePromise;
    onEnterSearch?: () => MaybePromise;
    onExitSearch?: () => MaybePromise;
    onSearchQueryChange?: (query: string) => void;
    onSearchInputFocus?: () => void;
  }

  let {
    disabled = false,
    initialInput = '',
    mode = 'add',
    searchQuery = '',
    availableTags = [],
    onAddItem,
    onEnterSearch,
    onExitSearch,
    onSearchQueryChange,
    onSearchInputFocus
  }: Props = $props();

  let addInput = $state('');
  let isSubmitting = $state(false);
  let inputElement = $state<HTMLInputElement | null>(null);
  let activeTagToken = $state<V2InlineTagToken | null>(null);
  let selectedTagNames = $state<string[]>([]);
  let didApplyInitialInput = $state(false);
  let lastMode = $state<CommandMode>('add');
  let inputValue = $derived(mode === 'search' ? searchQuery : addInput);
  let trimmedInput = $derived(inputValue.trim());
  let selectedTagKeys = $derived(
    new Set(selectedTagNames.map((tagName) => tagName.toLocaleLowerCase()))
  );
  let availableTagKeys = $derived(
    new Set(availableTags.map((tag) => tag.name.toLocaleLowerCase()))
  );
  let activeTagName = $derived(normalizeV2TagName(activeTagToken?.query ?? ''));
  let tagSuggestions = $derived(
    activeTagToken
      ? suggestV2Tags(availableTags, activeTagToken.query, 6).filter(
          (tag) => !selectedTagKeys.has(tag.name.toLocaleLowerCase())
        )
      : []
  );
  let showTagSuggestions = $derived(
    mode === 'add' && (activeTagToken !== null || selectedTagNames.length > 0)
  );
  let createTagSuggestionName = $derived(
    activeTagName &&
      !selectedTagKeys.has(activeTagName.toLocaleLowerCase()) &&
      !availableTagKeys.has(activeTagName.toLocaleLowerCase())
      ? activeTagName
      : null
  );
  let submitTagNames = $derived(normalizeV2TagNames(selectedTagNames));
  let canSubmitAdd = $derived(
    mode === 'add' &&
      addInput.trim().length > 0 &&
      activeTagToken === null &&
      !disabled &&
      !isSubmitting
  );

  $effect(() => {
    if (didApplyInitialInput) return;
    addInput = initialInput;
    didApplyInitialInput = true;
  });

  $effect(() => {
    if (mode === lastMode) return;
    lastMode = mode;
    focusInput();
  });

  function focusInput(): void {
    setTimeout(() => inputElement?.focus({ preventScroll: true }), 0);
  }

  function clearInput(): void {
    if (mode === 'search') {
      onSearchQueryChange?.('');
    } else {
      addInput = '';
      selectedTagNames = [];
      activeTagToken = null;
    }
    focusInput();
  }

  function updateActiveTagToken(): void {
    if (mode !== 'add' || !inputElement) {
      activeTagToken = null;
      return;
    }

    activeTagToken = getActiveV2TagToken(addInput, inputElement.selectionStart);
  }

  function handleInput(event: Event): void {
    const value = (event.currentTarget as HTMLInputElement).value;
    if (mode === 'search') {
      onSearchQueryChange?.(value);
    } else {
      addInput = value;
      activeTagToken = getActiveV2TagToken(
        value,
        (event.currentTarget as HTMLInputElement).selectionStart
      );
    }
  }

  function handleFocus(): void {
    if (mode === 'search') {
      onSearchInputFocus?.();
      return;
    }

    updateActiveTagToken();
  }

  async function handleSubmit(): Promise<void> {
    if (mode === 'search') return;
    if (!canSubmitAdd) return;

    isSubmitting = true;
    try {
      await onAddItem(addInput.trim(), submitTagNames);
      addInput = '';
      selectedTagNames = [];
      activeTagToken = null;
    } catch {
      // The v2 store owns the visible error banner; keep the draft in place.
    } finally {
      isSubmitting = false;
      focusInput();
    }
  }

  function handleAddButtonClick(): void {
    if (canSubmitAdd) {
      void handleSubmit();
      return;
    }

    focusInput();
  }

  async function prepareTagInput(): Promise<void> {
    if (mode !== 'add' || disabled || isSubmitting) return;

    const caretIndex = inputElement?.selectionStart ?? addInput.length;
    const currentToken = getActiveV2TagToken(addInput, caretIndex);

    if (currentToken) {
      activeTagToken = currentToken;
      focusInput();
      return;
    }

    const before = addInput.slice(0, caretIndex);
    const after = addInput.slice(caretIndex);
    const prefix = before.length === 0 || /\s$/u.test(before) ? '' : ' ';
    const suffix = after.length === 0 || /^\s/u.test(after) ? '' : ' ';
    const nextCaretIndex = before.length + prefix.length + 1;

    addInput = `${before}${prefix}#${suffix}${after}`;
    await tick();
    inputElement?.focus({ preventScroll: true });
    inputElement?.setSelectionRange(nextCaretIndex, nextCaretIndex);
    updateActiveTagToken();
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key !== 'Enter') return;

    event.preventDefault();
    void handleSubmit();
  }

  async function selectTagSuggestion(tag: V2Tag): Promise<void> {
    await addSelectedTag(tag.name);
  }

  async function createTagSuggestion(tagName: string): Promise<void> {
    await addSelectedTag(tagName);
  }

  async function addSelectedTag(tagName: string): Promise<void> {
    if (!activeTagToken) return;

    const replacement = removeV2TagToken(addInput, activeTagToken);
    addInput = replacement.value;
    selectedTagNames = normalizeV2TagNames([...selectedTagNames, tagName]);
    activeTagToken = null;
    await tick();
    inputElement?.focus({ preventScroll: true });
    inputElement?.setSelectionRange(replacement.caretIndex, replacement.caretIndex);
    updateActiveTagToken();
  }

  function removeSelectedTag(tagName: string): void {
    const keyToRemove = tagName.toLocaleLowerCase();
    selectedTagNames = selectedTagNames.filter(
      (selectedTagName) => selectedTagName.toLocaleLowerCase() !== keyToRemove
    );
    focusInput();
  }
</script>

<div class="relative flex-shrink-0">
  <div
    class="flex min-h-[64px] items-center gap-2 rounded-[6px_24px_6px_24px] border-[3px] border-[var(--color-ink)] bg-[var(--color-white)] px-2.5 py-2 shadow-sm"
    aria-disabled={disabled}
  >
    {#if mode === 'search'}
      <div
        class="grid h-10 w-10 flex-shrink-0 place-items-center rounded-full text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] active:bg-[var(--color-canvas)]"
      >
        <button
          type="button"
          class="grid h-10 w-10 place-items-center rounded-full transition-colors hover:bg-[var(--color-canvas)]"
          aria-label={i18n.t('v2ExitSearch')}
          title={i18n.t('v2ExitSearch')}
          onclick={() => void onExitSearch?.()}
        >
          <ArrowLeft size={22} strokeWidth={2.5} aria-hidden="true" />
        </button>
      </div>
    {:else}
      <button
        type="button"
        class={`grid h-11 w-11 flex-shrink-0 place-items-center rounded-[12px] border-2 text-[var(--color-ink)] transition-colors disabled:cursor-not-allowed disabled:opacity-40 ${
          canSubmitAdd
            ? 'border-[var(--color-ink)] bg-[var(--color-accent-sky)] hover:bg-[var(--color-accent-sky-strong)] active:bg-[var(--color-accent-sky-strong)]'
            : 'border-[var(--color-stroke)] bg-[var(--color-white)] hover:border-[var(--color-ink)] hover:bg-[var(--color-canvas)] active:bg-[var(--color-mist)]'
        }`}
        aria-label={i18n.t('v2AddItem')}
        title={i18n.t('v2AddItem')}
        disabled={disabled || isSubmitting}
        onclick={handleAddButtonClick}
      >
        <Plus size={23} strokeWidth={2.5} aria-hidden="true" />
      </button>
    {/if}

    <input
      use:iosFocusFix
      bind:this={inputElement}
      value={inputValue}
      type="text"
      class="min-h-11 min-w-0 flex-1 bg-transparent text-base text-[var(--color-ink)] outline-none placeholder:text-[var(--color-ink-muted)] disabled:opacity-50"
      placeholder={mode === 'search' ? i18n.t('v2SearchPlaceholder') : i18n.t('v2NewItemPlaceholder')}
      aria-label={mode === 'search' ? i18n.t('v2SearchPlaceholder') : i18n.t('v2NewItemPlaceholder')}
      autocomplete="off"
      {disabled}
      oninput={handleInput}
      onfocus={handleFocus}
      onkeyup={updateActiveTagToken}
      onclick={updateActiveTagToken}
      onkeydown={handleKeydown}
    />

    {#if mode === 'search' && trimmedInput}
      <button
        type="button"
        class="grid h-10 w-10 flex-shrink-0 place-items-center rounded-[14px] text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-canvas)]"
        aria-label={i18n.t('v2ClearInput')}
        title={i18n.t('v2ClearInput')}
        onclick={clearInput}
      >
        <X size={18} strokeWidth={2.5} aria-hidden="true" />
      </button>
    {/if}

    {#if mode === 'add'}
      <div class="flex flex-shrink-0 items-center gap-1.5">
        <button
          type="button"
          class="grid h-10 w-10 place-items-center rounded-[13px] text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] active:bg-[var(--color-mist)] disabled:cursor-not-allowed disabled:opacity-40"
          aria-label={i18n.t('v2AddTag')}
          title={i18n.t('v2AddTag')}
          disabled={disabled || isSubmitting}
          onclick={() => void prepareTagInput()}
        >
          <span class="text-xl font-black leading-none" aria-hidden="true">#</span>
        </button>
        <button
          type="button"
          class="grid h-10 w-10 place-items-center rounded-[13px] text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] active:bg-[var(--color-mist)] disabled:cursor-not-allowed disabled:opacity-40"
          aria-label={i18n.t('v2EnterSearch')}
          title={i18n.t('v2EnterSearch')}
          disabled={disabled}
          onclick={() => void onEnterSearch?.()}
        >
          <Search size={20} strokeWidth={2.5} aria-hidden="true" />
        </button>
      </div>
    {/if}
  </div>

  {#if showTagSuggestions}
    <div class="absolute left-0 right-0 top-[calc(100%+8px)] z-40">
      <V2TagSuggestionBoard
        query={activeTagToken?.query ?? ''}
        suggestions={tagSuggestions}
        createTagName={createTagSuggestionName}
        {selectedTagNames}
        onSelectTag={selectTagSuggestion}
        onCreateTag={createTagSuggestion}
        onRemoveSelectedTag={removeSelectedTag}
      />
    </div>
  {/if}
</div>
