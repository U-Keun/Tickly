<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { Cloud, RefreshCw } from '@lucide/svelte';

  import SettingsGroup from '../../../components/settings/SettingsGroup.svelte';
  import SettingsShell from '../../../components/settings/SettingsShell.svelte';
  import { icloudSyncStore } from '$lib/checklist/icloudSyncStore.svelte';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';

  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));
  let isLoading = $state(true);
  let isToggling = $state(false);
  let relativeNow = $state(Date.now());
  let lastSyncedLabel = $derived(formatLastSyncedAt(icloudSyncStore.lastSyncedAt));

  function formatLastSyncedAt(value: string | null): string {
    if (!value) return i18n.t('icloudSyncNeverSynced');

    const syncedAt = Date.parse(value);
    if (!Number.isFinite(syncedAt)) return value;

    const elapsedMinutes = Math.max(0, Math.floor((relativeNow - syncedAt) / 60000));
    if (elapsedMinutes < 1) {
      return i18n.t('icloudSyncJustNow');
    }
    if (elapsedMinutes < 60) {
      return i18n.t('icloudSyncMinutesAgoTemplate')(elapsedMinutes);
    }

    const elapsedHours = Math.floor(elapsedMinutes / 60);
    if (elapsedHours < 24) {
      return i18n.t('icloudSyncHoursAgoTemplate')(elapsedHours);
    }

    return i18n.t('icloudSyncDaysAgoTemplate')(Math.floor(elapsedHours / 24));
  }

  function backToSettings(): void {
    void goto(settingsPathWithReturnTo('/settings', returnTo));
  }

  async function toggleSync(): Promise<void> {
    if (isToggling) return;
    isToggling = true;
    try {
      await icloudSyncStore.setEnabled(!icloudSyncStore.enabled);
    } finally {
      isToggling = false;
    }
  }

  onMount(() => {
    const relativeTimer = window.setInterval(() => {
      relativeNow = Date.now();
    }, 60000);

    void icloudSyncStore
      .loadStatus()
      .catch(() => undefined)
      .finally(() => {
        isLoading = false;
      });

    return () => {
      window.clearInterval(relativeTimer);
    };
  });
</script>

<SettingsShell title={i18n.t('icloudSyncTitle')} onBack={backToSettings}>
  <div class="flex flex-col gap-5">
    <SettingsGroup
      title={i18n.t('icloudSyncGroupTitle')}
      description={i18n.t('icloudSyncDescription')}
    >
      <div class="px-3 py-3">
        <div
          class={`flex min-h-[52px] flex-col gap-2 overflow-hidden rounded-[14px] border-2 border-ink bg-paper px-[14px] py-2 transition-opacity ${
            icloudSyncStore.canUseNative ? '' : 'opacity-75'
          }`}
        >
          <button
            type="button"
            class="flex w-full items-center gap-3 text-left disabled:cursor-not-allowed"
            role="switch"
            aria-checked={icloudSyncStore.enabled}
            aria-label={i18n.t('icloudSyncToggle')}
            disabled={isLoading || isToggling || icloudSyncStore.isSyncing || (!icloudSyncStore.enabled && !icloudSyncStore.canUseNative)}
            onclick={toggleSync}
          >
            <span class="grid h-9 w-9 shrink-0 place-items-center rounded-full bg-white text-ink-muted">
              <Cloud size={18} strokeWidth={2.4} aria-hidden="true" />
            </span>
            <span class="min-w-0 flex-1">
              <span class="block text-base font-semibold leading-6 text-ink">
                {icloudSyncStore.enabled ? i18n.t('icloudSyncOn') : i18n.t('icloudSyncOff')}
              </span>
              <span class="block text-[13px] font-medium leading-5 text-ink-muted">
                {icloudSyncStore.availabilityMessage || i18n.t('icloudSyncPrivateDatabaseHint')}
              </span>
            </span>
            <span
              class={`relative h-8 w-[52px] shrink-0 rounded-full border-2 transition-colors ${
                icloudSyncStore.enabled
                  ? 'border-ink bg-accent-sky'
                  : 'border-stroke bg-white'
              }`}
              aria-hidden="true"
            >
              <span
                class={`absolute top-1/2 h-6 w-6 -translate-y-1/2 rounded-full border-2 border-ink bg-paper transition-transform ${
                  icloudSyncStore.enabled ? 'translate-x-[22px]' : 'translate-x-1'
                }`}
              ></span>
            </span>
          </button>
        </div>
      </div>
    </SettingsGroup>

    <SettingsGroup title={i18n.t('icloudSyncStatusTitle')}>
      <div class="space-y-3 px-3 py-3">
        <div class="flex items-center justify-between gap-3">
          <span class="text-[14px] font-semibold leading-5 text-ink-muted">
            {i18n.t('icloudSyncLastSynced')}
          </span>
          <span class="min-w-0 truncate text-right text-[14px] font-semibold leading-5 text-ink">
            {lastSyncedLabel}
          </span>
        </div>

        {#if icloudSyncStore.lastError}
          <p class="rounded-[6px_16px_6px_16px] border border-accent-peach-strong bg-accent-peach/50 px-3 py-2 text-[13px] font-semibold leading-5 text-ink">
            {icloudSyncStore.lastError}
          </p>
        {/if}

        <button
          type="button"
          class="flex min-h-11 w-full items-center justify-center gap-2 rounded-[6px_18px_6px_18px] border-2 border-ink bg-white px-4 text-[15px] font-semibold leading-6 text-ink transition-colors hover:bg-canvas active:bg-accent-sky disabled:cursor-not-allowed disabled:border-stroke disabled:text-ink-muted"
          disabled={!icloudSyncStore.enabled || icloudSyncStore.isSyncing}
          onclick={() => void icloudSyncStore.syncNow()}
        >
          <RefreshCw
            class={icloudSyncStore.isSyncing ? 'animate-spin' : ''}
            size={18}
            strokeWidth={2.4}
            aria-hidden="true"
          />
          {icloudSyncStore.isSyncing ? i18n.t('icloudSyncSyncing') : i18n.t('icloudSyncNow')}
        </button>
      </div>
    </SettingsGroup>
  </div>
</SettingsShell>
