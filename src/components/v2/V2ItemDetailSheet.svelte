<script lang="ts">
  import { iosFocusFix } from '$lib/iosFocusFix';
  import { i18n } from '$lib/i18n';
  import type { V2Tag, V2TodoItem } from '../../types';
  import V2BottomSheet from './V2BottomSheet.svelte';
  import V2TagEditor from './V2TagEditor.svelte';

  type MaybePromise = void | Promise<void>;

  interface Props {
    show: boolean;
    item: V2TodoItem | null;
    availableTags?: V2Tag[];
    isSaving?: boolean;
    onSaveDetails: (
      id: number,
      text: string,
      memo: string | null,
      tagNames?: string[]
    ) => MaybePromise;
    onClose: () => void;
  }

  let {
    show,
    item,
    availableTags = [],
    isSaving = false,
    onSaveDetails,
    onClose
  }: Props = $props();

  let draftText = $state('');
  let draftMemo = $state('');
  let draftTagNames = $state<string[]>([]);
  let preparedItemId = $state<number | null>(null);

  let trimmedText = $derived(draftText.trim());
  let trimmedMemo = $derived(draftMemo.trim());
  let isVisible = $derived(show && item !== null);

  $effect(() => {
    if (!isVisible || !item) {
      draftText = '';
      draftMemo = '';
      draftTagNames = [];
      preparedItemId = null;
      return;
    }

    if (preparedItemId !== item.id) {
      draftText = item.text;
      draftMemo = item.memo ?? '';
      draftTagNames = item.tags.map((tag) => tag.name);
      preparedItemId = item.id;
    }
  });

  function handleClose(): void {
    if (isSaving) return;
    onClose();
  }

  async function submitEdit(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    if (!item || !trimmedText || isSaving) return;

    try {
      await onSaveDetails(item.id, trimmedText, trimmedMemo || null, draftTagNames);
      onClose();
    } catch {
      // The v2 store owns the visible error banner; keep the sheet open.
    }
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
          use:iosFocusFix
          bind:value={draftText}
          class="min-h-12 rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-4 text-base text-[var(--color-ink)] outline-none transition-colors focus:bg-[var(--color-canvas)]"
          placeholder={i18n.t('v2ItemTextPlaceholder')}
          aria-label={i18n.t('v2ItemTextLabel')}
          autocomplete="off"
          disabled={isSaving}
        />
      </label>

      <label class="flex flex-col gap-2">
        <span class="text-sm font-semibold text-[var(--color-ink)]">{i18n.t('v2ItemMemoLabel')}</span>
        <textarea
          use:iosFocusFix
          bind:value={draftMemo}
          class="min-h-28 resize-none rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-4 py-3 text-base leading-6 text-[var(--color-ink)] outline-none transition-colors focus:bg-[var(--color-canvas)]"
          placeholder={i18n.t('v2ItemMemoPlaceholder')}
          aria-label={i18n.t('v2ItemMemoLabel')}
          disabled={isSaving}
        ></textarea>
      </label>

      <label class="flex flex-col gap-2">
        <span class="text-sm font-semibold text-[var(--color-ink)]">{i18n.t('v2ItemTagsLabel')}</span>
        <V2TagEditor
          tagNames={draftTagNames}
          {availableTags}
          disabled={isSaving}
          onChange={(tagNames) => {
            draftTagNames = tagNames;
          }}
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
