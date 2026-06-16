<script lang="ts">
  import { iosFocusFix } from '$lib/iosFocusFix';
  import { i18n } from '$lib/i18n';
  import {
    parseV2RepeatDetail,
    stringifyV2RepeatDetail
  } from '$lib/v2/v2Repeat';
  import type { V2RepeatType, V2Tag, V2TodoItem } from '../../types';
  import V2BottomSheet from './V2BottomSheet.svelte';
  import V2RepeatEditor from './V2RepeatEditor.svelte';
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
      tagNames?: string[],
      repeatType?: V2RepeatType,
      repeatDetail?: string | null,
      reminderAt?: string | null
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
  let draftRepeatType = $state<V2RepeatType>('none');
  let draftRepeatDetail = $state<number[]>([]);
  let draftReminderAt = $state('');
  let preparedItemId = $state<number | null>(null);

  let trimmedText = $derived(draftText.trim());
  let trimmedMemo = $derived(draftMemo.trim());
  let isVisible = $derived(show && item !== null);
  let isReminderFocused = $state(false);

  $effect(() => {
    if (!isVisible || !item) {
      draftText = '';
      draftMemo = '';
      draftTagNames = [];
      draftRepeatType = 'none';
      draftRepeatDetail = [];
      draftReminderAt = '';
      preparedItemId = null;
      return;
    }

    if (preparedItemId !== item.id) {
      draftText = item.text;
      draftMemo = item.memo ?? '';
      draftTagNames = item.tags.map((tag) => tag.name);
      draftRepeatType = item.repeat_type;
      draftRepeatDetail = parseV2RepeatDetail(item.repeat_detail);
      draftReminderAt = item.reminder_at ?? '';
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
      await onSaveDetails(
        item.id,
        trimmedText,
        trimmedMemo || null,
        draftTagNames,
        draftRepeatType,
        stringifyV2RepeatDetail(draftRepeatType, draftRepeatDetail),
        draftReminderAt || null
      );
      onClose();
    } catch {
      // The v2 store owns the visible error banner; keep the sheet open.
    }
  }
</script>

<V2BottomSheet
  show={isVisible}
  title={i18n.t('v2EditItemDetails')}
  preferredHeight={680}
  onClose={handleClose}
>
  {#snippet footer()}
    <div class="flex gap-[10px]">
      <button
        type="submit"
        form="v2-item-detail-form"
        class="min-h-12 flex-1 rounded-[14px] bg-[var(--color-accent-sky-strong)] px-4 text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-accent-sky)] disabled:cursor-not-allowed disabled:opacity-50"
        disabled={!trimmedText || isSaving}
      >
        {i18n.t('v2SaveItem')}
      </button>
      <button
        type="button"
        class="min-h-12 flex-1 rounded-[14px] bg-[var(--color-canvas)] px-4 text-sm font-semibold text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-mist)] disabled:cursor-not-allowed disabled:opacity-50"
        disabled={isSaving}
        onclick={handleClose}
      >
        {i18n.t('cancel')}
      </button>
    </div>
  {/snippet}

  {#if item}
    <form id="v2-item-detail-form" class="flex flex-col gap-4" onsubmit={submitEdit}>
      <label class="flex flex-col">
        <span class="sr-only">{i18n.t('v2ItemTextLabel')}</span>
        <input
          use:iosFocusFix
          bind:value={draftText}
          class="min-h-[52px] rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-[14px] text-base text-[var(--color-ink)] outline-none transition-colors focus:bg-[var(--color-canvas)]"
          placeholder={i18n.t('v2ItemTextPlaceholder')}
          aria-label={i18n.t('v2ItemTextLabel')}
          autocomplete="off"
          disabled={isSaving}
        />
      </label>

      <label class="flex flex-col">
        <span class="sr-only">{i18n.t('v2ItemMemoLabel')}</span>
        <textarea
          use:iosFocusFix
          bind:value={draftMemo}
          class="min-h-[112px] resize-none rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-[14px] py-3 text-base leading-6 text-[var(--color-ink)] outline-none transition-colors focus:bg-[var(--color-canvas)]"
          placeholder={i18n.t('v2ItemMemoPlaceholder')}
          aria-label={i18n.t('v2ItemMemoLabel')}
          disabled={isSaving}
        ></textarea>
      </label>

      <label class="flex flex-col">
        <span class="sr-only">{i18n.t('v2ItemTagsLabel')}</span>
        <V2TagEditor
          tagNames={draftTagNames}
          {availableTags}
          disabled={isSaving}
          onChange={(tagNames) => {
            draftTagNames = tagNames;
          }}
        />
      </label>

      <V2RepeatEditor
        repeatType={draftRepeatType}
        repeatDetail={draftRepeatDetail}
        disabled={isSaving}
        onChange={(repeatType, repeatDetail) => {
          draftRepeatType = repeatType;
          draftRepeatDetail = repeatDetail;
        }}
      />

      <label class="relative flex flex-col">
        <span class="sr-only">{i18n.t('v2ItemReminderLabel')}</span>
        <input
          use:iosFocusFix
          type="time"
          bind:value={draftReminderAt}
          class={`min-h-[52px] rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-[14px] text-base outline-none transition-colors focus:bg-[var(--color-canvas)] ${
            draftReminderAt || isReminderFocused
              ? 'text-[var(--color-ink)]'
              : 'text-transparent'
          }`}
          aria-label={i18n.t('v2ItemReminderLabel')}
          title={i18n.t('v2ItemReminderPlaceholder')}
          disabled={isSaving}
          onfocus={() => {
            isReminderFocused = true;
          }}
          onblur={() => {
            isReminderFocused = false;
          }}
        />
        {#if !draftReminderAt && !isReminderFocused}
          <span class="pointer-events-none absolute left-[14px] top-1/2 -translate-y-1/2 text-base text-[var(--color-ink-muted)] opacity-55">
            {i18n.t('v2ItemReminderPlaceholder')}
          </span>
        {/if}
      </label>

    </form>
  {/if}
</V2BottomSheet>
