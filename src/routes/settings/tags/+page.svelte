<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { fade, slide } from 'svelte/transition';
  import { Hash, Pencil, Trash2 } from '@lucide/svelte';

  import type { TagSummary } from '../../../types';
  import SettingsGroup from '../../../components/settings/SettingsGroup.svelte';
  import SettingsShell from '../../../components/settings/SettingsShell.svelte';
  import BottomSheet from '../../../components/checklist/BottomSheet.svelte';
  import ConfirmModal from '../../../components/checklist/ConfirmModal.svelte';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';
  import * as checklistApi from '$lib/api/checklistApi';

  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));
  let tagSummaries = $state<TagSummary[]>([]);
  let isLoading = $state(true);
  let errorMessage = $state<string | null>(null);
  let tagPendingRename = $state<TagSummary | null>(null);
  let renameDraft = $state('');
  let isRenaming = $state(false);
  let tagPendingDeletion = $state<TagSummary | null>(null);
  let deletingTagId = $state<number | null>(null);
  let trimmedRenameDraft = $derived(renameDraft.trim());
  let canSaveRename = $derived(
    Boolean(tagPendingRename) &&
      trimmedRenameDraft.length > 0 &&
      trimmedRenameDraft !== tagPendingRename?.tag.name &&
      !isRenaming
  );

  async function loadTagSummaries(): Promise<void> {
    isLoading = true;
    errorMessage = null;

    try {
      tagSummaries = await checklistApi.getTagSummaries();
    } catch (error) {
      const nextError = error instanceof Error ? error : new Error(String(error));
      errorMessage = nextError.message;
      console.error('Failed to load tag summaries.', error);
    } finally {
      isLoading = false;
    }
  }

  function openRenameSheet(summary: TagSummary): void {
    if (isRenaming || deletingTagId !== null) return;

    tagPendingRename = summary;
    renameDraft = summary.tag.name;
  }

  function closeRenameSheet(): void {
    if (isRenaming) return;

    tagPendingRename = null;
    renameDraft = '';
  }

  async function submitRename(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    if (!tagPendingRename || !canSaveRename) return;

    isRenaming = true;
    errorMessage = null;

    try {
      await checklistApi.renameTag(tagPendingRename.tag.id, trimmedRenameDraft);
      await loadTagSummaries();
      tagPendingRename = null;
      renameDraft = '';
    } catch (error) {
      const nextError = error instanceof Error ? error : new Error(String(error));
      errorMessage = nextError.message;
      console.error('Failed to rename tag.', error);
    } finally {
      isRenaming = false;
    }
  }

  function requestDeleteTag(summary: TagSummary): void {
    if (isRenaming || deletingTagId !== null) return;

    tagPendingDeletion = summary;
  }

  function cancelDeleteTag(): void {
    if (deletingTagId !== null) return;

    tagPendingDeletion = null;
  }

  async function confirmDeleteTag(): Promise<void> {
    if (!tagPendingDeletion || deletingTagId !== null) return;

    const tagId = tagPendingDeletion.tag.id;
    deletingTagId = tagId;
    errorMessage = null;

    try {
      await checklistApi.deleteTag(tagId);
      tagSummaries = tagSummaries.filter((summary) => summary.tag.id !== tagId);
      tagPendingDeletion = null;
    } catch (error) {
      const nextError = error instanceof Error ? error : new Error(String(error));
      errorMessage = nextError.message;
      console.error('Failed to delete tag.', error);
    } finally {
      deletingTagId = null;
    }
  }

  onMount(() => {
    void loadTagSummaries();
  });
</script>

<SettingsShell
  title={i18n.t('checklistTagManageTitle')}
  onBack={() => void goto(settingsPathWithReturnTo('/settings', returnTo))}
>
  <div class="flex flex-col gap-5">
    {#if errorMessage}
      <div class="rounded-md border border-accent-peach-strong bg-accent-peach px-3 py-2 text-sm text-ink">
        {errorMessage}
      </div>
    {/if}

    <SettingsGroup
      title={i18n.t('checklistTagManageGroupTitle')}
      description={i18n.t('checklistTagManageDescription')}
    >
      {#if isLoading}
        <div class="flex min-h-24 items-center px-4 py-4 text-sm font-medium text-ink-muted">
          {i18n.t('checklistLoading')}
        </div>
      {:else if tagSummaries.length === 0}
        <div class="px-4 py-6 text-sm leading-6 text-ink-muted">
          <p class="font-semibold text-ink">{i18n.t('checklistTagManageEmptyTitle')}</p>
          <p class="mt-1">{i18n.t('checklistTagManageEmptyDescription')}</p>
        </div>
      {:else}
        <div class="divide-y divide-stroke">
          {#each tagSummaries as summary (summary.tag.id)}
            <div class="overflow-hidden" out:slide|local={{ duration: 210 }}>
              <article class="min-w-0 px-3 py-3" out:fade|local={{ duration: 120 }}>
                <div class="flex min-w-0 items-center gap-3">
                  <div
                    class="flex h-10 w-10 shrink-0 items-center justify-center rounded-[6px_14px_6px_14px] bg-accent-mint text-ink"
                    aria-hidden="true"
                  >
                    <Hash size={20} strokeWidth={2.4} />
                  </div>

                  <div class="min-w-0 flex-1">
                    <div class="flex min-w-0 items-center gap-2">
                      <h2 class="min-w-0 flex-1 truncate text-[16px] font-semibold leading-6 text-ink">
                        #{summary.tag.name}
                      </h2>
                      <span
                        class="shrink-0 rounded-full bg-canvas px-2.5 py-1 text-[11px] font-semibold leading-4 text-ink-muted"
                      >
                        {i18n.t('checklistTagItemCountTemplate')(summary.item_count)}
                      </span>
                    </div>
                  </div>

                  <div class="flex shrink-0 items-center gap-1">
                    <button
                      type="button"
                      class="flex h-11 w-11 items-center justify-center rounded-[12px] text-ink-muted transition-colors hover:bg-accent-sky hover:text-ink disabled:cursor-not-allowed disabled:opacity-50"
                      aria-label={i18n.t('checklistRenameTag')}
                      disabled={isRenaming || deletingTagId !== null}
                      onclick={() => openRenameSheet(summary)}
                    >
                      <Pencil size={19} strokeWidth={2.2} aria-hidden="true" />
                    </button>
                    <button
                      type="button"
                      class="flex h-11 w-11 items-center justify-center rounded-[12px] text-ink-muted transition-colors hover:bg-accent-peach hover:text-ink disabled:cursor-not-allowed disabled:opacity-50"
                      aria-label={i18n.t('checklistDeleteTag')}
                      disabled={isRenaming || deletingTagId !== null}
                      onclick={() => requestDeleteTag(summary)}
                    >
                      <Trash2 size={19} strokeWidth={2.2} aria-hidden="true" />
                    </button>
                  </div>
                </div>
              </article>
            </div>
          {/each}
        </div>
      {/if}
    </SettingsGroup>
  </div>
</SettingsShell>

<BottomSheet
  show={tagPendingRename !== null}
  title={i18n.t('checklistRenameTagTitle')}
  onClose={closeRenameSheet}
>
  {#snippet footer()}
    <div class="flex gap-[10px]">
      <button
        type="submit"
        form="tag-rename-form"
        class="min-h-12 flex-1 rounded-[14px] bg-accent-sky-strong px-4 text-sm font-semibold text-ink transition-colors hover:bg-accent-sky disabled:cursor-not-allowed disabled:opacity-50"
        disabled={!canSaveRename}
      >
        {isRenaming ? i18n.t('checklistRenamingTag') : i18n.t('checklistSaveTag')}
      </button>
      <button
        type="button"
        class="min-h-12 flex-1 rounded-[14px] bg-canvas px-4 text-sm font-semibold text-ink-muted transition-colors hover:bg-mist disabled:cursor-not-allowed disabled:opacity-50"
        disabled={isRenaming}
        onclick={closeRenameSheet}
      >
        {i18n.t('cancel')}
      </button>
    </div>
  {/snippet}

  <form id="tag-rename-form" class="flex flex-col gap-3" onsubmit={submitRename}>
    <label class="flex flex-col gap-2">
      <span class="text-sm font-semibold leading-5 text-ink">{i18n.t('checklistTagNameLabel')}</span>
      <input
        bind:value={renameDraft}
        class="min-h-[52px] rounded-[14px] border-2 border-ink bg-paper px-[14px] text-base text-ink outline-none transition-colors focus:bg-canvas"
        placeholder={i18n.t('checklistTagNamePlaceholder')}
        aria-label={i18n.t('checklistTagNameLabel')}
        disabled={isRenaming}
      />
    </label>
  </form>
</BottomSheet>

<ConfirmModal
  show={tagPendingDeletion !== null}
  title={i18n.t('checklistDeleteTagConfirmTitle')}
  message={tagPendingDeletion
    ? i18n.t('checklistDeleteTagConfirmMessageTemplate')(
        tagPendingDeletion.tag.name,
        tagPendingDeletion.item_count
      )
    : ''}
  confirmLabel={deletingTagId === null ? i18n.t('checklistDeleteTag') : i18n.t('checklistDeletingTag')}
  tone="danger"
  isBusy={deletingTagId !== null}
  onConfirm={confirmDeleteTag}
  onCancel={cancelDeleteTag}
/>
