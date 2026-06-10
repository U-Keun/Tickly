<script lang="ts">
  import { ArrowLeft, Plus, Search, X } from '@lucide/svelte';

  import { iosFocusFix } from '$lib/iosFocusFix';
  import { i18n } from '$lib/i18n';

  type MaybePromise = void | Promise<void>;
  type CommandMode = 'add' | 'search';

  interface Props {
    disabled?: boolean;
    initialInput?: string;
    mode?: CommandMode;
    searchQuery?: string;
    onAddItem: (text: string) => MaybePromise;
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
    onAddItem,
    onEnterSearch,
    onExitSearch,
    onSearchQueryChange,
    onSearchInputFocus
  }: Props = $props();

  let addInput = $state('');
  let isSubmitting = $state(false);
  let inputElement = $state<HTMLInputElement | null>(null);
  let didApplyInitialInput = $state(false);
  let lastMode = $state<CommandMode>('add');
  let inputValue = $derived(mode === 'search' ? searchQuery : addInput);
  let trimmedInput = $derived(inputValue.trim());

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
    }
    focusInput();
  }

  function handleInput(event: Event): void {
    const value = (event.currentTarget as HTMLInputElement).value;
    if (mode === 'search') {
      onSearchQueryChange?.(value);
    } else {
      addInput = value;
    }
  }

  async function handleSubmit(): Promise<void> {
    if (mode === 'search') return;
    if (!trimmedInput || disabled || isSubmitting) return;

    isSubmitting = true;
    try {
      await onAddItem(trimmedInput);
      addInput = '';
    } catch {
      // The v2 store owns the visible error banner; keep the draft in place.
    } finally {
      isSubmitting = false;
      focusInput();
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key !== 'Enter') return;

    event.preventDefault();
    void handleSubmit();
  }
</script>

<div class="flex-shrink-0">
  <div
    class="flex min-h-[64px] items-center gap-2.5 rounded-[6px_24px_6px_24px] border-[3px] border-[var(--color-ink)] bg-[var(--color-white)] px-2.5 py-2 shadow-sm"
    aria-disabled={disabled}
  >
    <div
      class="grid h-11 w-11 flex-shrink-0 place-items-center rounded-full text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] active:bg-[var(--color-canvas)]"
    >
      {#if mode === 'search'}
        <button
          type="button"
          class="grid h-11 w-11 place-items-center rounded-full transition-colors hover:bg-[var(--color-canvas)]"
          aria-label={i18n.t('v2ExitSearch')}
          title={i18n.t('v2ExitSearch')}
          onclick={() => void onExitSearch?.()}
        >
          <ArrowLeft size={22} strokeWidth={2.5} aria-hidden="true" />
        </button>
      {:else}
        <button
          type="button"
          class="grid h-11 w-11 place-items-center rounded-full transition-colors hover:bg-[var(--color-canvas)]"
          aria-label={i18n.t('v2EnterSearch')}
          title={i18n.t('v2EnterSearch')}
          disabled={disabled}
          onclick={() => void onEnterSearch?.()}
        >
          <Search size={21} strokeWidth={2.5} aria-hidden="true" />
        </button>
      {/if}
    </div>

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
      onfocus={() => mode === 'search' && onSearchInputFocus?.()}
      onkeydown={handleKeydown}
    />

    {#if trimmedInput}
      <button
        type="button"
        class="grid h-11 w-11 flex-shrink-0 place-items-center rounded-[14px] text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-canvas)]"
        aria-label={i18n.t('v2ClearInput')}
        title={i18n.t('v2ClearInput')}
        onclick={clearInput}
      >
        <X size={18} strokeWidth={2.5} aria-hidden="true" />
      </button>
    {/if}

    {#if mode === 'add'}
      <button
        type="button"
        class="grid h-11 w-11 flex-shrink-0 place-items-center rounded-[14px] bg-[var(--color-accent-sky-strong)] text-[var(--color-white)] transition-transform hover:scale-105 disabled:cursor-not-allowed disabled:opacity-40"
        aria-label={i18n.t('v2AddItem')}
        title={i18n.t('v2AddItem')}
        disabled={!trimmedInput || disabled || isSubmitting}
        onclick={handleSubmit}
      >
        <Plus size={25} strokeWidth={2.5} aria-hidden="true" />
      </button>
    {/if}
  </div>
</div>
