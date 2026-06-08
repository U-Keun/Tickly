<script lang="ts">
  import { i18n } from '$lib/i18n';
  import V2ModalShell from './V2ModalShell.svelte';

  type MaybePromise = void | Promise<void>;
  type Tone = 'danger' | 'primary';

  interface Props {
    show: boolean;
    title: string;
    message: string;
    confirmLabel: string;
    cancelLabel?: string;
    tone?: Tone;
    isBusy?: boolean;
    onConfirm: () => MaybePromise;
    onCancel: () => MaybePromise;
  }

  let {
    show,
    title,
    message,
    confirmLabel,
    cancelLabel = i18n.t('cancel'),
    tone = 'primary',
    isBusy = false,
    onConfirm,
    onCancel
  }: Props = $props();

  let actionClass = $derived(
    tone === 'danger'
      ? 'bg-[var(--color-accent-peach-strong)] hover:bg-[var(--color-accent-peach)]'
      : 'bg-[var(--color-accent-sky-strong)] hover:bg-[var(--color-accent-sky)]'
  );

  async function handleConfirm(): Promise<void> {
    if (isBusy) return;
    await onConfirm();
  }
</script>

<V2ModalShell {show} {title} description={message} onClose={onCancel}>
  <div class="flex gap-2">
    <button
      type="button"
      class={`min-h-11 flex-1 rounded-[12px] px-4 text-sm font-semibold text-[var(--color-ink)] transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${actionClass}`}
      disabled={isBusy}
      onclick={() => void handleConfirm()}
    >
      {confirmLabel}
    </button>
    <button
      type="button"
      class="min-h-11 flex-1 rounded-[12px] bg-[var(--color-canvas)] px-4 text-sm font-semibold text-[var(--color-ink-muted)] transition-colors hover:bg-[var(--color-mist)]"
      onclick={onCancel}
    >
      {cancelLabel}
    </button>
  </div>
</V2ModalShell>
