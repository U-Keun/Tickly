<script lang="ts">
  import { onDestroy } from 'svelte';
  import { fade } from 'svelte/transition';
  import { Pencil, Trash2 } from '@lucide/svelte';

  import type { V2TodoItem } from '../../types';
  import { i18n } from '$lib/i18n';

  type MaybePromise = void | Promise<void>;

  interface Props {
    item: V2TodoItem;
    isReorderMode?: boolean;
    isFirst?: boolean;
    isLast?: boolean;
    initialEditing?: boolean;
    onToggleItem: (id: number) => MaybePromise;
    onUpdateItemText: (id: number, text: string) => MaybePromise;
    onRequestDeleteItem: (item: V2TodoItem) => MaybePromise;
    onMoveItem: (id: number, delta: number) => MaybePromise;
  }

  let {
    item,
    isReorderMode = false,
    isFirst = false,
    isLast = false,
    initialEditing = false,
    onToggleItem,
    onUpdateItemText,
    onRequestDeleteItem,
    onMoveItem
  }: Props = $props();

  let isEditing = $state(false);
  let draftText = $state('');
  let isSaving = $state(false);
  let displayedDone = $state(false);
  let textDone = $state(false);
  let tickPulse = $state(false);
  let tickAnimationKey = $state(0);
  let isToggling = $state(false);
  let inputElement = $state<HTMLInputElement | null>(null);
  let didApplyInitialEditing = $state(false);
  let lastSyncedItemId = $state<number | null>(null);
  let lastSyncedDone = $state<boolean | null>(null);
  let textDoneTimer: ReturnType<typeof setTimeout> | null = null;
  let tickPulseTimer: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    if (didApplyInitialEditing) return;
    if (initialEditing) {
      isEditing = true;
      draftText = item.text;
    }
    didApplyInitialEditing = true;
  });

  $effect(() => {
    if (item.id === lastSyncedItemId && item.done === lastSyncedDone) return;
    setDisplayedDone(item.done, false);
    lastSyncedItemId = item.id;
    lastSyncedDone = item.done;
  });

  onDestroy(() => {
    clearTextDoneTimer();
    clearTickPulseTimer();
  });

  function clearTextDoneTimer(): void {
    if (!textDoneTimer) return;
    clearTimeout(textDoneTimer);
    textDoneTimer = null;
  }

  function clearTickPulseTimer(): void {
    if (!tickPulseTimer) return;
    clearTimeout(tickPulseTimer);
    tickPulseTimer = null;
  }

  function focusInput(): void {
    setTimeout(() => inputElement?.focus(), 0);
  }

  function beginEdit(): void {
    isEditing = true;
    draftText = item.text;
    focusInput();
  }

  function cancelEdit(): void {
    isEditing = false;
    draftText = '';
  }

  function setDisplayedDone(nextDone: boolean, shouldAnimate: boolean): void {
    displayedDone = nextDone;
    clearTextDoneTimer();

    if (!nextDone) {
      textDone = false;
      tickPulse = false;
      clearTickPulseTimer();
      return;
    }

    if (!shouldAnimate) {
      textDone = true;
      tickPulse = false;
      return;
    }

    textDone = false;
    tickPulse = true;
    tickAnimationKey += 1;

    textDoneTimer = setTimeout(() => {
      textDone = true;
      textDoneTimer = null;
    }, 70);

    clearTickPulseTimer();
    tickPulseTimer = setTimeout(() => {
      tickPulse = false;
      tickPulseTimer = null;
    }, 180);
  }

  async function handleToggleItem(): Promise<void> {
    if (isToggling) return;

    setDisplayedDone(!displayedDone, !displayedDone);
    isToggling = true;
    try {
      await onToggleItem(item.id);
    } finally {
      isToggling = false;
    }
  }

  async function submitEdit(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    const trimmedText = draftText.trim();
    if (!trimmedText || isSaving) return;

    isSaving = true;
    try {
      await onUpdateItemText(item.id, trimmedText);
      isEditing = false;
      draftText = '';
    } finally {
      isSaving = false;
    }
  }
</script>

<article
  class={`rounded-[0_24px_0_24px] border-2 border-[var(--color-ink)] px-2.5 py-2 shadow-sm transition-colors ${
    displayedDone
      ? 'bg-[var(--color-canvas)]'
      : 'bg-[var(--color-paper)]'
  }`}
>
  <div class="flex min-h-10 items-center gap-2.5">
    <button
      type="button"
      class={`grid h-10 w-10 flex-shrink-0 place-items-center border-2 transition-colors ${
        displayedDone
          ? 'rounded-[12px] border-[var(--color-ink)] bg-[var(--color-white)] text-[var(--color-ink)]'
          : 'rounded-[12px] border-[var(--color-ink)] bg-[var(--color-white)] text-transparent hover:bg-[var(--color-canvas)] active:bg-[var(--color-canvas)]'
      }`}
      class:tickPop={tickPulse}
      aria-pressed={displayedDone}
      aria-label={displayedDone ? i18n.t('v2RestoreItem') : i18n.t('v2CompleteItem')}
      title={displayedDone ? i18n.t('v2RestoreItem') : i18n.t('v2CompleteItem')}
      onclick={() => void handleToggleItem()}
    >
      {#if displayedDone}
        {#key tickAnimationKey}
          <svg class="tickCheck h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="3"
              d="m5 12.5 4.2 4.2L19 7"
            />
          </svg>
        {/key}
      {/if}
    </button>

    {#if isEditing}
      <form
        class="flex min-w-0 flex-1 items-center gap-1.5"
        onsubmit={submitEdit}
      >
        <input
          bind:this={inputElement}
          bind:value={draftText}
          class="min-h-10 min-w-0 flex-1 bg-transparent px-0 text-[15px] leading-5 text-[var(--color-ink)] outline-none"
          aria-label={i18n.t('v2EditItem')}
          autocomplete="off"
        />
        <div class="flex flex-shrink-0 items-center gap-1.5" in:fade={{ duration: 120 }}>
          <button
            type="submit"
            class="grid h-10 w-10 place-items-center rounded-[12px] bg-[var(--color-accent-mint-strong)] text-[var(--color-ink)] disabled:cursor-not-allowed disabled:opacity-40"
            aria-label={i18n.t('v2SaveItem')}
            title={i18n.t('v2SaveItem')}
            disabled={!draftText.trim() || isSaving}
          >
            <svg class="h-[18px] w-[18px]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2.6"
                d="m5 12.5 4.2 4.2L19 7"
              />
            </svg>
          </button>
          <button
            type="button"
            class="grid h-10 w-10 place-items-center rounded-[12px] bg-transparent text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] active:bg-[var(--color-canvas)]"
            aria-label={i18n.t('cancel')}
            title={i18n.t('cancel')}
            onclick={cancelEdit}
          >
            <svg class="h-[18px] w-[18px]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2.6"
                d="M6 18 18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>
      </form>
    {:else}
      <div
        class="flex min-w-0 flex-1 items-center gap-1.5"
      >
        <p
          class={`min-w-0 flex-1 truncate pr-1 text-[15px] leading-5 ${
            textDone
              ? 'text-[var(--color-ink-muted)]'
              : 'text-[var(--color-ink)]'
          }`}
          title={item.text}
        >
          <span class="tickText" class:tickTextDone={textDone}>{item.text}</span>
        </p>

        <div class="flex flex-shrink-0 items-center gap-1.5" in:fade={{ duration: 120 }}>
          <button
            type="button"
            class="grid h-10 w-10 place-items-center rounded-[12px] bg-transparent text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] active:bg-[var(--color-canvas)]"
            aria-label={i18n.t('v2EditItem')}
            title={i18n.t('v2EditItem')}
            onclick={beginEdit}
          >
            <Pencil size={22} strokeWidth={2.3} />
          </button>
          <button
            type="button"
            class="grid h-10 w-10 place-items-center rounded-[12px] bg-transparent text-[var(--color-ink)] transition-colors hover:bg-[var(--color-accent-peach)] active:bg-[var(--color-accent-peach)]"
            aria-label={i18n.t('v2DeleteItem')}
            title={i18n.t('v2DeleteItem')}
            onclick={() => onRequestDeleteItem(item)}
          >
            <Trash2 size={22} strokeWidth={2.3} />
          </button>

          {#if isReorderMode}
            <button
              type="button"
              class="grid h-10 w-10 place-items-center rounded-[12px] border border-[var(--color-stroke)] bg-[var(--color-paper)] text-[var(--color-ink)] disabled:opacity-40"
              aria-label={i18n.t('v2MoveUp')}
              title={i18n.t('v2MoveUp')}
              disabled={isFirst}
              onclick={() => onMoveItem(item.id, -1)}
            >
              <svg class="h-[18px] w-[18px]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2.4"
                  d="m7 14 5-5 5 5"
                />
              </svg>
            </button>
            <button
              type="button"
              class="grid h-10 w-10 place-items-center rounded-[12px] border border-[var(--color-stroke)] bg-[var(--color-paper)] text-[var(--color-ink)] disabled:opacity-40"
              aria-label={i18n.t('v2MoveDown')}
              title={i18n.t('v2MoveDown')}
              disabled={isLast}
              onclick={() => onMoveItem(item.id, 1)}
            >
              <svg class="h-[18px] w-[18px]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2.4"
                  d="m7 10 5 5 5-5"
                />
              </svg>
            </button>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</article>

<style>
  .tickPop {
    animation: tick-pop 180ms cubic-bezier(0.2, 0.9, 0.25, 1.25);
  }

  .tickCheck path {
    stroke-dasharray: 24;
    stroke-dashoffset: 24;
    animation: tick-draw 170ms ease-out forwards;
  }

  .tickText {
    position: relative;
    display: inline-block;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    vertical-align: top;
  }

  .tickText::after {
    position: absolute;
    right: 0;
    left: 0;
    top: 52%;
    height: 2px;
    content: '';
    background: currentColor;
    border-radius: 999px;
    transform: scaleX(0);
    transform-origin: left center;
  }

  .tickTextDone::after {
    animation: tick-line 320ms cubic-bezier(0.2, 0.8, 0.2, 1) forwards;
  }

  @keyframes tick-pop {
    0% {
      transform: scale(0.96);
    }
    55% {
      transform: scale(1.06);
    }
    100% {
      transform: scale(1);
    }
  }

  @keyframes tick-draw {
    to {
      stroke-dashoffset: 0;
    }
  }

  @keyframes tick-line {
    to {
      transform: scaleX(1);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .tickPop {
      animation: none;
    }

    .tickCheck path {
      animation: none;
      stroke-dashoffset: 0;
    }

    .tickTextDone::after {
      animation: none;
      transform: scaleX(1);
    }
  }
</style>
