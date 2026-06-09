<script lang="ts">
  import { ArrowLeft, ArrowRight, Pencil, Trash2 } from '@lucide/svelte';

  import { i18n } from '$lib/i18n';
  import type { V2Category } from '../../types';
  import V2BottomSheet from './V2BottomSheet.svelte';

  type MaybePromise = void | Promise<void>;

  interface Props {
    show: boolean;
    category: V2Category | null;
    isOnlyCategory?: boolean;
    isFirst?: boolean;
    isLast?: boolean;
    isBusy?: boolean;
    onRename: () => void;
    onDeleteRequest: () => void;
    onMove: (delta: number) => MaybePromise;
    onClose: () => void;
  }

  let {
    show,
    category,
    isOnlyCategory = false,
    isFirst = false,
    isLast = false,
    isBusy = false,
    onRename,
    onDeleteRequest,
    onMove,
    onClose
  }: Props = $props();

  let isVisible = $derived(show && category !== null);

  function handleClose(): void {
    if (isBusy) return;
    onClose();
  }

  async function move(delta: number): Promise<void> {
    if (isBusy) return;

    try {
      await onMove(delta);
    } catch {
      // The v2 store owns the visible error banner; keep the sheet open.
    }
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

      <div class="grid grid-cols-2 gap-2">
        <button
          type="button"
          class="flex min-h-11 items-center justify-center gap-2 rounded-[14px] bg-[var(--color-paper)] px-3 text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] disabled:cursor-not-allowed disabled:opacity-40"
          disabled={isBusy || isFirst}
          aria-label={i18n.t('v2MoveLeft')}
          title={i18n.t('v2MoveLeft')}
          onclick={() => void move(-1)}
        >
          <ArrowLeft size={20} strokeWidth={2.4} aria-hidden="true" />
          <span>{i18n.t('v2MoveLeft')}</span>
        </button>
        <button
          type="button"
          class="flex min-h-11 items-center justify-center gap-2 rounded-[14px] bg-[var(--color-paper)] px-3 text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-canvas)] disabled:cursor-not-allowed disabled:opacity-40"
          disabled={isBusy || isLast}
          aria-label={i18n.t('v2MoveRight')}
          title={i18n.t('v2MoveRight')}
          onclick={() => void move(1)}
        >
          <span>{i18n.t('v2MoveRight')}</span>
          <ArrowRight size={20} strokeWidth={2.4} aria-hidden="true" />
        </button>
      </div>

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
