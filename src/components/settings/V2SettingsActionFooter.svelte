<script lang="ts">
  import { i18n } from '$lib/i18n';

  type MaybePromise = void | Promise<void>;

  interface Props {
    onSave: () => MaybePromise;
    disabled?: boolean;
  }

  let { onSave, disabled = false }: Props = $props();
  let isSaving = $state(false);

  async function handleSave() {
    if (disabled || isSaving) return;

    isSaving = true;
    try {
      await onSave();
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="shrink-0 pt-3">
  <button
    type="button"
    class="flex min-h-12 w-full items-center justify-center rounded-[6px_18px_6px_18px] border-2 border-ink bg-accent-sky px-4 text-[16px] font-semibold leading-6 text-ink transition-colors hover:bg-accent-sky-strong active:bg-accent-sky disabled:cursor-not-allowed disabled:border-stroke disabled:bg-canvas disabled:text-ink-muted"
    disabled={disabled || isSaving}
    onclick={handleSave}
  >
    {isSaving ? i18n.t('saving') : i18n.t('save')}
  </button>
</div>
