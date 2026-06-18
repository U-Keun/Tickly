<script lang="ts">
  import { Flame } from '@lucide/svelte';
  import { cubicOut } from 'svelte/easing';
  import { fade, slide } from 'svelte/transition';

  import type { RepeatType, Tag, TodoItem } from '../../types';
  import { iosFocusFix } from '$lib/iosFocusFix';
  import { i18n } from '$lib/i18n';
  import {
    parseRepeatDetail,
    stringifyRepeatDetail
  } from '$lib/checklist/repeat';
  import BottomSheet from './BottomSheet.svelte';
  import RepeatEditor from './RepeatEditor.svelte';
  import TagEditor from './TagEditor.svelte';

  type MaybePromise = void | Promise<void>;

  interface Props {
    show: boolean;
    item: TodoItem | null;
    availableTags?: Tag[];
    isSaving?: boolean;
    onSaveDetails: (
      id: number,
      text: string,
      memo: string | null,
      tagNames?: string[],
      repeatType?: RepeatType,
      repeatDetail?: string | null,
      reminderAt?: string | null,
      trackStreak?: boolean
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
  let draftRepeatType = $state<RepeatType>('none');
  let draftRepeatDetail = $state<number[]>([]);
  let draftReminderAt = $state('');
  let draftTrackStreak = $state(false);
  let preparedItemId = $state<number | null>(null);

  let trimmedText = $derived(draftText.trim());
  let trimmedMemo = $derived(draftMemo.trim());
  let isVisible = $derived(show && item !== null);
  let canTrackStreak = $derived(draftRepeatType !== 'none');
  let isReminderFocused = $state(false);

  $effect(() => {
    if (!isVisible || !item) {
      draftText = '';
      draftMemo = '';
      draftTagNames = [];
      draftRepeatType = 'none';
      draftRepeatDetail = [];
      draftReminderAt = '';
      draftTrackStreak = false;
      preparedItemId = null;
      return;
    }

    if (preparedItemId !== item.id) {
      draftText = item.text;
      draftMemo = item.memo ?? '';
      draftTagNames = item.tags.map((tag) => tag.name);
      draftRepeatType = item.repeat_type;
      draftRepeatDetail = parseRepeatDetail(item.repeat_detail);
      draftReminderAt = item.reminder_at ?? '';
      draftTrackStreak = item.track_streak;
      preparedItemId = item.id;
    }
  });

  $effect(() => {
    if (!canTrackStreak && draftTrackStreak) {
      draftTrackStreak = false;
    }
  });

  function handleClose(): void {
    if (isSaving) return;
    onClose();
  }

  function toggleDraftTrackStreak(): void {
    if (!canTrackStreak || isSaving) return;
    draftTrackStreak = !draftTrackStreak;
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
        stringifyRepeatDetail(draftRepeatType, draftRepeatDetail),
        draftReminderAt || null,
        canTrackStreak && draftTrackStreak
      );
      onClose();
    } catch {
      // The checklist store owns the visible error banner; keep the sheet open.
    }
  }
</script>

<BottomSheet
  show={isVisible}
  title={i18n.t('checklistEditItemDetails')}
  preferredHeight={680}
  onClose={handleClose}
>
  {#snippet footer()}
    <div class="flex gap-[10px]">
      <button
        type="submit"
        form="item-detail-form"
        class="min-h-12 flex-1 rounded-[14px] bg-[var(--color-accent-sky-strong)] px-4 text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-accent-sky)] disabled:cursor-not-allowed disabled:opacity-50"
        disabled={!trimmedText || isSaving}
      >
        {i18n.t('checklistSaveItem')}
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
    <form id="item-detail-form" class="flex flex-col gap-4" onsubmit={submitEdit}>
      <label class="flex flex-col">
        <span class="sr-only">{i18n.t('checklistItemTextLabel')}</span>
        <input
          use:iosFocusFix
          bind:value={draftText}
          class="min-h-[52px] rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-[14px] text-base text-[var(--color-ink)] outline-none transition-colors focus:bg-[var(--color-canvas)]"
          placeholder={i18n.t('checklistItemTextPlaceholder')}
          aria-label={i18n.t('checklistItemTextLabel')}
          autocomplete="off"
          disabled={isSaving}
        />
      </label>

      <label class="flex flex-col">
        <span class="sr-only">{i18n.t('checklistItemMemoLabel')}</span>
        <textarea
          use:iosFocusFix
          bind:value={draftMemo}
          class="min-h-[112px] resize-none rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-[14px] py-3 text-base leading-6 text-[var(--color-ink)] outline-none transition-colors focus:bg-[var(--color-canvas)]"
          placeholder={i18n.t('checklistItemMemoPlaceholder')}
          aria-label={i18n.t('checklistItemMemoLabel')}
          disabled={isSaving}
        ></textarea>
      </label>

      <label class="flex flex-col">
        <span class="sr-only">{i18n.t('checklistItemTagsLabel')}</span>
        <TagEditor
          tagNames={draftTagNames}
          {availableTags}
          disabled={isSaving}
          onChange={(tagNames) => {
            draftTagNames = tagNames;
          }}
        />
      </label>

      <RepeatEditor
        repeatType={draftRepeatType}
        repeatDetail={draftRepeatDetail}
        disabled={isSaving}
        onChange={(repeatType, repeatDetail) => {
          draftRepeatType = repeatType;
          draftRepeatDetail = repeatDetail;
        }}
      />

      <label class="relative flex flex-col">
        <span class="sr-only">{i18n.t('checklistItemReminderLabel')}</span>
        <input
          use:iosFocusFix
          type="time"
          bind:value={draftReminderAt}
          class={`min-h-[52px] rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-[14px] text-base outline-none transition-colors focus:bg-[var(--color-canvas)] ${
            draftReminderAt || isReminderFocused
              ? 'text-[var(--color-ink)]'
              : 'text-transparent'
          }`}
          aria-label={i18n.t('checklistItemReminderLabel')}
          title={i18n.t('checklistItemReminderPlaceholder')}
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
            {i18n.t('checklistItemReminderPlaceholder')}
          </span>
        {/if}
      </label>

      <div
        class={`flex min-h-[52px] flex-col gap-2 overflow-hidden rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-[14px] py-2 transition-opacity ${
          canTrackStreak ? '' : 'opacity-75'
        }`}
      >
        <button
          type="button"
          class="flex w-full items-center gap-3 text-left disabled:cursor-not-allowed"
          role="switch"
          aria-checked={draftTrackStreak}
          aria-describedby={!canTrackStreak ? 'track-streak-requires-repeat' : undefined}
          disabled={!canTrackStreak || isSaving}
          onclick={toggleDraftTrackStreak}
        >
          <span class="grid h-9 w-9 shrink-0 place-items-center rounded-full bg-[var(--color-white)] text-[var(--color-ink-muted)]">
            <Flame size={18} strokeWidth={2.4} aria-hidden="true" />
          </span>
          <span class="min-w-0 flex-1 text-base font-semibold text-[var(--color-ink)]">
            {i18n.t('checklistTrackStreak')}
          </span>
          <span
            class={`relative h-8 w-[52px] shrink-0 rounded-full border-2 transition-colors ${
              draftTrackStreak
                ? 'border-[var(--color-ink)] bg-[var(--color-accent-sky)]'
                : 'border-[var(--color-stroke)] bg-[var(--color-white)]'
            }`}
            aria-hidden="true"
          >
            <span
              class={`absolute top-1/2 h-6 w-6 -translate-y-1/2 rounded-full border-2 border-[var(--color-ink)] bg-[var(--color-paper)] transition-transform ${
                draftTrackStreak ? 'translate-x-[22px]' : 'translate-x-1'
              }`}
            ></span>
          </span>
        </button>
        {#if !canTrackStreak}
          <div
            in:slide={{ duration: 230, easing: cubicOut }}
            out:slide={{ duration: 210, delay: 90, easing: cubicOut }}
          >
            <span
              id="track-streak-requires-repeat"
              class="block pl-12 text-xs leading-5 text-[var(--color-ink-muted)]"
              in:fade={{ duration: 140, delay: 105, easing: cubicOut }}
              out:fade={{ duration: 90, easing: cubicOut }}
            >
              {i18n.t('checklistTrackStreakRequiresRepeat')}
            </span>
          </div>
        {/if}
      </div>

    </form>
  {/if}
</BottomSheet>
