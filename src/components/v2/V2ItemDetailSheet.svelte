<script lang="ts">
  import { i18n } from '$lib/i18n';
  import type { V2TodoItem } from '../../types';
  import V2BottomSheet from './V2BottomSheet.svelte';

  type MaybePromise = void | Promise<void>;

  interface Props {
    show: boolean;
    item: V2TodoItem | null;
    isSaving?: boolean;
    onSaveText: (id: number, text: string) => MaybePromise;
    onClose: () => void;
  }

  let { show, item, isSaving = false, onSaveText, onClose }: Props = $props();

  let draftText = $state('');
  let inputElement = $state<HTMLInputElement | null>(null);
  let preparedItemId = $state<number | null>(null);
  let focusedItemId = $state<number | null>(null);

  let trimmedText = $derived(draftText.trim());
  let isVisible = $derived(show && item !== null);

  $effect(() => {
    if (!isVisible || !item) {
      draftText = '';
      preparedItemId = null;
      focusedItemId = null;
      return;
    }

    if (preparedItemId !== item.id) {
      draftText = item.text;
      preparedItemId = item.id;
      focusedItemId = null;
    }

    if (focusedItemId !== item.id) {
      focusedItemId = item.id;
      setTimeout(() => inputElement?.focus(), 0);
    }
  });

  function handleClose(): void {
    if (isSaving) return;
    onClose();
  }

  async function submitEdit(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    if (!item || !trimmedText || isSaving) return;

    await onSaveText(item.id, trimmedText);
    onClose();
  }
</script>

<V2BottomSheet
  show={isVisible}
  title={i18n.t('v2EditItemDetails')}
  onClose={handleClose}
>
  {#if item}
    <form class="flex flex-col gap-4" onsubmit={submitEdit}>
      <label class="flex flex-col gap-2">
        <span class="text-sm font-semibold text-[var(--color-ink)]">{i18n.t('v2ItemTextLabel')}</span>
        <input
          bind:this={inputElement}
          bind:value={draftText}
          class="min-h-12 rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-4 text-base text-[var(--color-ink)] outline-none transition-colors focus:bg-[var(--color-canvas)]"
          placeholder={i18n.t('v2ItemTextPlaceholder')}
          aria-label={i18n.t('v2ItemTextLabel')}
          autocomplete="off"
          disabled={isSaving}
        />
      </label>

      <div class="flex gap-2">
        <button
          type="submit"
          class="min-h-11 flex-1 rounded-[12px] bg-[var(--color-accent-sky-strong)] px-4 text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-accent-sky)] disabled:cursor-not-allowed disabled:opacity-50"
          disabled={!trimmedText || isSaving}
        >
          {i18n.t('v2SaveItem')}
        </button>
        <button
          type="button"
          class="min-h-11 flex-1 rounded-[12px] bg-[var(--color-canvas)] px-4 text-sm font-semibold text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-mist)] disabled:cursor-not-allowed disabled:opacity-50"
          disabled={isSaving}
          onclick={handleClose}
        >
          {i18n.t('cancel')}
        </button>
      </div>
    </form>
  {/if}
</V2BottomSheet>
