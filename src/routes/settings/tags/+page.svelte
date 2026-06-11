<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';
  import SettingsLayout from '../../../components/SettingsLayout.svelte';
  import ConfirmModal from '../../../components/ConfirmModal.svelte';
  import type { Tag } from '../../../types';
  import { appStore } from '$lib/stores';

  let tags = $state<Tag[]>([]);
  let deleteTarget = $state<Tag | null>(null);
  let showDeleteConfirm = $state(false);
  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));

  onMount(async () => {
    await appStore.loadAllTags();
    tags = appStore.allTags;
  });

  function requestDelete(tag: Tag) {
    deleteTarget = tag;
    showDeleteConfirm = true;
  }

  async function confirmDelete() {
    if (!deleteTarget) return;
    await appStore.deleteTag(deleteTarget.id);
    tags = appStore.allTags;
    showDeleteConfirm = false;
    deleteTarget = null;
  }

  function cancelDelete() {
    showDeleteConfirm = false;
    deleteTarget = null;
  }
</script>

<SettingsLayout title={i18n.t('tagManage')} onBack={() => goto(settingsPathWithReturnTo('/settings', returnTo))}>
  {#if tags.length === 0}
    <div class="empty-state">
      <p>{i18n.t('tagEmpty')}</p>
    </div>
  {:else}
    <div class="tag-list">
      {#each tags as tag (tag.id)}
        <div class="tag-row">
          <span class="tag-name">{tag.name}</span>
          <button
            type="button"
            class="delete-btn"
            onclick={() => requestDelete(tag)}
            aria-label="Delete"
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="3 6 5 6 21 6"></polyline>
              <path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"></path>
            </svg>
          </button>
        </div>
      {/each}
    </div>
  {/if}

  <ConfirmModal
    show={showDeleteConfirm}
    title={i18n.t('delete')}
    message={deleteTarget ? i18n.t('tagDeleteConfirmTemplate')(deleteTarget.name) : ''}
    confirmLabel={i18n.t('delete')}
    cancelLabel={i18n.t('cancel')}
    confirmStyle="danger"
    onConfirm={confirmDelete}
    onCancel={cancelDelete}
  />
</SettingsLayout>

<style>
  .empty-state {
    text-align: center;
    padding: 32px 16px;
    color: var(--color-ink-muted);
    font-size: 14px;
  }

  .tag-list {
    background: var(--color-canvas);
    border-radius: 12px;
    overflow: hidden;
  }

  .tag-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
  }

  .tag-row:not(:last-child) {
    border-bottom: 1px solid var(--color-stroke);
  }

  .tag-name {
    font-size: 15px;
    color: var(--color-ink);
  }

  .delete-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-ink-muted);
    border-radius: 8px;
    transition: background 0.15s;
  }

  .delete-btn:hover {
    background: var(--color-accent-peach);
    color: var(--color-ink);
  }
</style>
