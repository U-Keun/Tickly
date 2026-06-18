<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { fade, slide } from 'svelte/transition';
  import { ArchiveRestore, FileText, Tags, Trash2 } from '@lucide/svelte';

  import type { V2ArchivedItem } from '../../../types';
  import V2SettingsGroup from '../../../components/settings/V2SettingsGroup.svelte';
  import V2SettingsShell from '../../../components/settings/V2SettingsShell.svelte';
  import V2ConfirmModal from '../../../components/v2/V2ConfirmModal.svelte';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';
  import * as v2ChecklistApi from '$lib/api/v2ChecklistApi';

  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));
  let archivedItems = $state<V2ArchivedItem[]>([]);
  let isLoading = $state(true);
  let errorMessage = $state<string | null>(null);
  let restoringItemId = $state<number | null>(null);
  let deletingItemId = $state<number | null>(null);
  let itemPendingDeletion = $state<V2ArchivedItem | null>(null);

  function formatArchivedAt(value: string | null): string {
    if (!value) return '';

    const date = new Date(value);
    if (Number.isNaN(date.getTime())) {
      return value.slice(0, 10);
    }

    const locale = i18n.locale === 'ko' ? 'ko-KR' : i18n.locale === 'ja' ? 'ja-JP' : 'en-US';
    return new Intl.DateTimeFormat(locale, {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    }).format(date);
  }

  async function loadArchivedItems(): Promise<void> {
    isLoading = true;
    errorMessage = null;

    try {
      archivedItems = await v2ChecklistApi.v2GetArchivedItems();
    } catch (error) {
      const nextError = error instanceof Error ? error : new Error(String(error));
      errorMessage = nextError.message;
      console.error('Failed to load v2 archived items.', error);
    } finally {
      isLoading = false;
    }
  }

  async function restoreArchivedItem(id: number): Promise<void> {
    if (restoringItemId !== null || deletingItemId !== null) return;

    restoringItemId = id;
    errorMessage = null;

    try {
      await v2ChecklistApi.v2RestoreArchivedItem(id);
      archivedItems = archivedItems.filter((archivedItem) => archivedItem.item.id !== id);
    } catch (error) {
      const nextError = error instanceof Error ? error : new Error(String(error));
      errorMessage = nextError.message;
      console.error('Failed to restore v2 archived item.', error);
    } finally {
      restoringItemId = null;
    }
  }

  function requestDeleteArchivedItem(archivedItem: V2ArchivedItem): void {
    if (restoringItemId !== null || deletingItemId !== null) return;
    itemPendingDeletion = archivedItem;
  }

  function cancelDeleteArchivedItem(): void {
    if (deletingItemId !== null) return;
    itemPendingDeletion = null;
  }

  async function confirmDeleteArchivedItem(): Promise<void> {
    if (!itemPendingDeletion || deletingItemId !== null || restoringItemId !== null) return;

    const itemId = itemPendingDeletion.item.id;
    deletingItemId = itemId;
    errorMessage = null;

    try {
      await v2ChecklistApi.v2DeleteArchivedItem(itemId);
      archivedItems = archivedItems.filter((archivedItem) => archivedItem.item.id !== itemId);
      itemPendingDeletion = null;
    } catch (error) {
      const nextError = error instanceof Error ? error : new Error(String(error));
      errorMessage = nextError.message;
      console.error('Failed to delete v2 archived item.', error);
    } finally {
      deletingItemId = null;
    }
  }

  onMount(() => {
    void loadArchivedItems();
  });
</script>

<V2SettingsShell
  title={i18n.t('v2ArchiveManageTitle')}
  onBack={() => void goto(settingsPathWithReturnTo('/settings', returnTo))}
>
  <div class="flex flex-col gap-5">
    {#if errorMessage}
      <div class="rounded-md border border-accent-peach-strong bg-accent-peach px-3 py-2 text-sm text-ink">
        {errorMessage}
      </div>
    {/if}

    <V2SettingsGroup
      title={i18n.t('v2ArchiveManageGroupTitle')}
      description={i18n.t('v2ArchiveManageDescription')}
    >
      {#if isLoading}
        <div class="flex min-h-24 items-center px-4 py-4 text-sm font-medium text-ink-muted">
          {i18n.t('v2Loading')}
        </div>
      {:else if archivedItems.length === 0}
        <div class="px-4 py-6 text-sm leading-6 text-ink-muted">
          <p class="font-semibold text-ink">{i18n.t('v2ArchiveManageEmptyTitle')}</p>
          <p class="mt-1">{i18n.t('v2ArchiveManageEmptyDescription')}</p>
        </div>
      {:else}
        <div class="divide-y divide-stroke">
          {#each archivedItems as archivedItem (archivedItem.item.id)}
            {@const item = archivedItem.item}
            {@const tags = item.tags.map((tag) => `#${tag.name}`).join(' ')}
            {@const memo = item.memo?.trim() ?? ''}
            {@const archivedAt = formatArchivedAt(item.archived_at)}
            <div class="overflow-hidden" out:slide|local={{ duration: 210 }}>
              <article class="min-w-0 px-3 py-3" out:fade|local={{ duration: 120 }}>
                <div class="flex min-w-0 items-start gap-3">
                  <div
                    class="mt-1 flex h-10 w-10 shrink-0 items-center justify-center rounded-[6px_14px_6px_14px] bg-accent-sky text-ink"
                    aria-hidden="true"
                  >
                    <ArchiveRestore size={20} strokeWidth={2.2} />
                  </div>

                  <div class="min-w-0 flex-1">
                    <div class="flex min-w-0 items-center gap-2">
                      <h2 class="min-w-0 flex-1 truncate text-[16px] font-semibold leading-6 text-ink">
                        {item.text}
                      </h2>
                      <span class="shrink-0 rounded-full bg-canvas px-2.5 py-1 text-[11px] font-semibold leading-4 text-ink-muted">
                        {archivedItem.category.name}
                      </span>
                    </div>

                    {#if archivedAt}
                      <p class="mt-1 text-xs font-semibold leading-5 text-ink-muted">
                        {i18n.t('v2ArchivedAtTemplate')(archivedAt)}
                      </p>
                    {/if}

                    {#if memo}
                      <p class="mt-2 max-h-10 overflow-hidden text-sm leading-5 text-ink-muted">
                        <FileText class="mr-1 inline-block align-[-2px]" size={14} strokeWidth={2.2} />
                        {memo}
                      </p>
                    {/if}

                    {#if tags}
                      <p class="mt-1 truncate text-xs font-semibold leading-5 text-ink-muted">
                        <Tags class="mr-1 inline-block align-[-2px]" size={13} strokeWidth={2.2} />
                        {tags}
                      </p>
                    {/if}
                  </div>
                </div>

                <div class="mt-3 grid grid-cols-[1fr_auto] gap-2">
                  <button
                    type="button"
                    class="min-h-11 rounded-[12px] bg-accent-sky-strong px-4 text-sm font-semibold text-ink transition-colors hover:bg-accent-sky disabled:cursor-not-allowed disabled:opacity-50"
                    disabled={restoringItemId !== null || deletingItemId !== null}
                    onclick={() => void restoreArchivedItem(item.id)}
                  >
                    {restoringItemId === item.id
                      ? i18n.t('v2RestoringArchivedItem')
                      : i18n.t('v2RestoreArchivedItem')}
                  </button>

                  <button
                    type="button"
                    class="flex min-h-11 w-12 items-center justify-center rounded-[12px] bg-canvas text-ink-muted transition-colors hover:bg-accent-peach disabled:cursor-not-allowed disabled:opacity-50"
                    aria-label={i18n.t('v2DeleteArchivedItem')}
                    disabled={restoringItemId !== null || deletingItemId !== null}
                    onclick={() => requestDeleteArchivedItem(archivedItem)}
                  >
                    <Trash2 size={19} strokeWidth={2.2} aria-hidden="true" />
                  </button>
                </div>
              </article>
            </div>
          {/each}
        </div>
      {/if}
    </V2SettingsGroup>
  </div>
</V2SettingsShell>

<V2ConfirmModal
  show={itemPendingDeletion !== null}
  title={i18n.t('v2DeleteArchivedConfirmTitle')}
  message={itemPendingDeletion
    ? i18n.t('v2DeleteArchivedConfirmMessageTemplate')(itemPendingDeletion.item.text)
    : ''}
  confirmLabel={deletingItemId === null ? i18n.t('v2DeleteArchivedItem') : i18n.t('v2DeletingArchivedItem')}
  tone="danger"
  isBusy={deletingItemId !== null}
  onConfirm={confirmDeleteArchivedItem}
  onCancel={cancelDeleteArchivedItem}
/>
