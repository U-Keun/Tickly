<script lang="ts">
  import { GripHorizontal, Pencil, Trash2 } from '@lucide/svelte';

  import { i18n } from '$lib/i18n';
  import type { V2Category } from '../../types';
  import V2BottomSheet from './V2BottomSheet.svelte';

  interface Props {
    show: boolean;
    category: V2Category | null;
    isOnlyCategory?: boolean;
    isBusy?: boolean;
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

<V2BottomSheet
  show={isVisible}
  title={i18n.t('v2ManageCategory')}
  description={category?.name ?? ''}
  onClose={handleClose}
>
  {#if category}
    <div class="flex flex-col gap-3">
      <button
        type="button"
        class="flex min-h-11 items-center gap-3 rounded-[14px] bg-[var(--color-paper)] px-4 text-left text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] disabled:cursor-not-allowed disabled:opacity-50"
        disabled={isBusy}
        onclick={onRename}
      >
        <Pencil size={20} strokeWidth={2.4} aria-hidden="true" />
        <span>{i18n.t('v2EditCategory')}</span>
      </button>

      <button
        type="button"
        class="flex min-h-11 items-center gap-3 rounded-[14px] bg-[var(--color-paper)] px-4 text-left text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] disabled:cursor-not-allowed disabled:opacity-50"
        disabled={isBusy || isOnlyCategory}
        onclick={onEditOrder}
      >
        <GripHorizontal size={20} strokeWidth={2.4} aria-hidden="true" />
        <span>{i18n.t('v2EditCategoryOrder')}</span>
      </button>

      <button
        type="button"
        class="flex min-h-11 items-center gap-3 rounded-[14px] bg-[var(--color-accent-peach)] px-4 text-left text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-accent-peach-strong)] disabled:cursor-not-allowed disabled:opacity-45"
        disabled={isBusy || isOnlyCategory}
        aria-label={i18n.t('v2DeleteCategory')}
        title={i18n.t('v2DeleteCategory')}
        onclick={onDeleteRequest}
      >
        <Trash2 size={20} strokeWidth={2.4} aria-hidden="true" />
        <span>{i18n.t('v2DeleteCategory')}</span>
      </button>
    </div>
  {/if}
</V2BottomSheet>
