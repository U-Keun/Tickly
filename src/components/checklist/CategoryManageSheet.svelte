<script lang="ts">
  import { GripHorizontal, Pencil, Plus, Trash2 } from '@lucide/svelte';

  import { i18n } from '$lib/i18n';
  import type { Category } from '../../types';
  import BottomSheet from './BottomSheet.svelte';

  interface Props {
    show: boolean;
    category: Category | null;
    isOnlyCategory?: boolean;
    isBusy?: boolean;
    onCreate: () => void;
    onRename: () => void;
    onEditOrder: () => void;
    onDeleteRequest: () => void;
    onClose: () => void;
  }

  let {
    show,
    category,
    isOnlyCategory = false,
    isBusy = false,
    onCreate,
    onRename,
    onEditOrder,
    onDeleteRequest,
    onClose
  }: Props = $props();

  let isVisible = $derived(show && category !== null);

  function handleClose(): void {
    if (isBusy) return;
    onClose();
  }
</script>

<BottomSheet
  show={isVisible}
  title={i18n.t('checklistManageCategory')}
  description={category?.name ?? ''}
  onClose={handleClose}
>
  {#if category}
    <div class="flex flex-col gap-3">
      <button
        type="button"
        class="flex min-h-11 items-center gap-3 rounded-[14px] bg-[var(--color-paper)] px-4 text-left text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] disabled:cursor-not-allowed disabled:opacity-50"
        disabled={isBusy}
        onclick={onCreate}
      >
        <Plus size={20} strokeWidth={2.4} aria-hidden="true" />
        <span>{i18n.t('checklistAddCategory')}</span>
      </button>

      <button
        type="button"
        class="flex min-h-11 items-center gap-3 rounded-[14px] bg-[var(--color-paper)] px-4 text-left text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] disabled:cursor-not-allowed disabled:opacity-50"
        disabled={isBusy}
        onclick={onRename}
      >
        <Pencil size={20} strokeWidth={2.4} aria-hidden="true" />
        <span>{i18n.t('checklistRenameCategoryActionTemplate')(category.name)}</span>
      </button>

      <button
        type="button"
        class="flex min-h-11 items-center gap-3 rounded-[14px] bg-[var(--color-paper)] px-4 text-left text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] disabled:cursor-not-allowed disabled:opacity-50"
        disabled={isBusy || isOnlyCategory}
        onclick={onEditOrder}
      >
        <GripHorizontal size={20} strokeWidth={2.4} aria-hidden="true" />
        <span>{i18n.t('checklistEditCategoryOrder')}</span>
      </button>

      <button
        type="button"
        class="flex min-h-11 items-center gap-3 rounded-[14px] bg-[var(--color-accent-peach)] px-4 text-left text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-accent-peach-strong)] disabled:cursor-not-allowed disabled:opacity-45"
        disabled={isBusy || isOnlyCategory}
        aria-label={i18n.t('checklistDeleteCategory')}
        title={i18n.t('checklistDeleteCategory')}
        onclick={onDeleteRequest}
      >
        <Trash2 size={20} strokeWidth={2.4} aria-hidden="true" />
        <span>{i18n.t('checklistDeleteCategory')}</span>
      </button>
    </div>
  {/if}
</BottomSheet>
