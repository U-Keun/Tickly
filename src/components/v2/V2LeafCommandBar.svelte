<script lang="ts">
  import { i18n } from '$lib/i18n';

  type MaybePromise = void | Promise<void>;

  interface Props {
    disabled?: boolean;
    initialInput?: string;
    onAddItem: (text: string) => MaybePromise;
  }

  let { disabled = false, initialInput = '', onAddItem }: Props = $props();

  let input = $state('');
  let isSubmitting = $state(false);
  let inputElement = $state<HTMLInputElement | null>(null);
  let didApplyInitialInput = $state(false);

  $effect(() => {
    if (didApplyInitialInput) return;
    input = initialInput;
    didApplyInitialInput = true;
  });

  function focusInput(): void {
    setTimeout(() => inputElement?.focus(), 0);
  }

  function clearInput(): void {
    input = '';
    focusInput();
  }

  async function handleSubmit(): Promise<void> {
    const trimmedInput = input.trim();
    if (!trimmedInput || disabled || isSubmitting) return;

    isSubmitting = true;
    try {
      await onAddItem(trimmedInput);
      input = '';
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
    class="flex min-h-[64px] items-center gap-2.5 rounded-[0_24px_0_24px] border-[3px] border-[var(--color-ink)] bg-[var(--color-white)] px-2.5 py-2 shadow-sm"
    aria-disabled={disabled}
  >
    <div
      class="grid h-11 w-11 flex-shrink-0 place-items-center rounded-full text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-canvas)] active:bg-[var(--color-canvas)]"
      aria-hidden="true"
    >
      <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2.2"
          d="M6.5 17.5c6.4.3 10.9-4.2 11-11-6.8.1-11.3 4.6-11 11z"
        />
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2.2"
          d="M7.5 16.5 17 7"
        />
      </svg>
    </div>

    <input
      bind:this={inputElement}
      bind:value={input}
      type="text"
      class="min-h-11 min-w-0 flex-1 bg-transparent text-base text-[var(--color-ink)] outline-none placeholder:text-[var(--color-ink-muted)] disabled:opacity-50"
      placeholder={i18n.t('v2NewItemPlaceholder')}
      aria-label={i18n.t('v2NewItemPlaceholder')}
      autocomplete="off"
      {disabled}
      onkeydown={handleKeydown}
    />

    {#if input.trim()}
      <button
        type="button"
        class="grid h-11 w-11 flex-shrink-0 place-items-center rounded-[14px] text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-canvas)]"
        aria-label={i18n.t('v2ClearInput')}
        title={i18n.t('v2ClearInput')}
        onclick={clearInput}
      >
        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2.4"
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
      </button>
    {/if}

    <button
      type="button"
      class="grid h-11 w-11 flex-shrink-0 place-items-center rounded-[14px] bg-[var(--color-accent-sky-strong)] text-[var(--color-white)] transition-transform hover:scale-105 disabled:cursor-not-allowed disabled:opacity-40"
      aria-label={i18n.t('v2AddItem')}
      title={i18n.t('v2AddItem')}
      disabled={!input.trim() || disabled || isSubmitting}
      onclick={handleSubmit}
    >
      <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2.4"
          d="M12 5v14M5 12h14"
        />
      </svg>
    </button>
  </div>
</div>
