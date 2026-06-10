<script lang="ts">
  import { iosFocusFix } from '$lib/iosFocusFix';
  import { i18n } from '$lib/i18n';
  import type { V2Category } from '../../types';
  import V2BottomSheet from './V2BottomSheet.svelte';

  type MaybePromise = void | Promise<void>;
  type Mode = 'create' | 'rename';

  interface Props {
    show: boolean;
    mode: Mode;
    category: V2Category | null;
    isSaving?: boolean;
    onSave: (name: string) => MaybePromise;
    onClose: () => void;
  }

  let {
    show,
    mode,
    category,
    isSaving = false,
    onSave,
    onClose
  }: Props = $props();

  let draftName = $state('');
  let preparedKey = $state('');

  let trimmedName = $derived(draftName.trim());
  let isVisible = $derived(show && (mode === 'create' || category !== null));
  let sheetKey = $derived(`${mode}-${category?.id ?? 'new'}`);
  let title = $derived(
    mode === 'create' ? i18n.t('v2CreateCategoryTitle') : i18n.t('v2RenameCategoryTitle')
  );

  $effect(() => {
    if (!isVisible) {
      draftName = '';
      preparedKey = '';
      return;
    }

    if (preparedKey !== sheetKey) {
      draftName = mode === 'rename' ? (category?.name ?? '') : '';
      preparedKey = sheetKey;
    }
  });

  function handleClose(): void {
    if (isSaving) return;
    onClose();
  }

  async function submitCategory(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    if (!trimmedName || isSaving) return;

    try {
      await onSave(trimmedName);
      onClose();
    } catch {
      // The v2 store owns the visible error banner; keep the sheet open.
    }
  }
</script>

<V2BottomSheet
  show={isVisible}
  {title}
  onClose={handleClose}
>
  <form class="flex flex-col gap-4" onsubmit={submitCategory}>
    <label class="flex flex-col gap-2">
      <span class="text-sm font-semibold text-[var(--color-ink)]">{i18n.t('v2CategoryNameLabel')}</span>
      <input
        use:iosFocusFix
        bind:value={draftName}
        class="min-h-12 rounded-[14px] border-2 border-[var(--color-ink)] bg-[var(--color-paper)] px-4 text-base text-[var(--color-ink)] outline-none transition-colors focus:bg-[var(--color-canvas)]"
        placeholder={i18n.t('v2NewCategoryPlaceholder')}
        aria-label={i18n.t('v2CategoryNameLabel')}
        autocomplete="off"
        disabled={isSaving}
      />
    </label>

    <div class="flex gap-2">
      <button
        type="submit"
        class="min-h-11 flex-1 rounded-[12px] bg-[var(--color-accent-sky-strong)] px-4 text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-accent-sky)] disabled:cursor-not-allowed disabled:opacity-50"
        disabled={!trimmedName || isSaving}
      >
        {i18n.t('v2SaveCategory')}
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
</V2BottomSheet>
