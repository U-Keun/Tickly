<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { Cloud, RefreshCw } from '@lucide/svelte';
  import { onMount } from 'svelte';

  import V2SettingsGroup from '../../../components/settings/V2SettingsGroup.svelte';
  import V2SettingsShell from '../../../components/settings/V2SettingsShell.svelte';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';
  import { v2ICloudSyncStore } from '$lib/v2/v2ICloudSyncStore.svelte';

  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));
  let isToggling = $state(false);
  let errorMessage = $state<string | null>(null);
  let canToggleSync = $derived(
    !isToggling &&
      !v2ICloudSyncStore.isLoading &&
      !v2ICloudSyncStore.isSyncing &&
      (v2ICloudSyncStore.status.available || v2ICloudSyncStore.status.enabled)
  );
  let canSyncNow = $derived(
    v2ICloudSyncStore.status.enabled &&
      v2ICloudSyncStore.status.available &&
      !v2ICloudSyncStore.isSyncing
  );

  function formatLastSynced(value: string | null): string {
    if (!value) return i18n.t('v2ICloudSyncNever');
    const date = new Date(value);
    if (Number.isNaN(date.getTime())) return value;

    const locale = i18n.locale === 'ko' ? 'ko-KR' : i18n.locale === 'ja' ? 'ja-JP' : 'en-US';
    return new Intl.DateTimeFormat(locale, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    }).format(date);
  }

  function statusText(status: string): string {
    switch (status) {
      case 'idle':
        return i18n.t('v2ICloudSyncStatusIdle');
      case 'available':
        return i18n.t('v2ICloudSyncStatusAvailable');
      case 'syncing':
        return i18n.t('v2ICloudSyncStatusSyncing');
      case 'synced':
        return i18n.t('v2ICloudSyncStatusSynced');
      case 'disabled':
        return i18n.t('v2ICloudSyncStatusDisabled');
      case 'unavailable':
        return i18n.t('v2ICloudSyncStatusUnavailable');
      case 'unsupported_os':
        return i18n.t('v2ICloudSyncStatusUnsupportedOs');
      case 'no_account':
        return i18n.t('v2ICloudSyncStatusNoAccount');
      case 'restricted':
        return i18n.t('v2ICloudSyncStatusRestricted');
      case 'temporarily_unavailable':
        return i18n.t('v2ICloudSyncStatusTemporarilyUnavailable');
      case 'could_not_determine':
        return i18n.t('v2ICloudSyncStatusCouldNotDetermine');
      case 'timeout':
        return i18n.t('v2ICloudSyncStatusTimeout');
      case 'error':
        return i18n.t('v2ICloudSyncStatusError');
      default:
        return i18n.t('v2ICloudSyncStatusUnknown');
    }
  }

  async function toggleSync(): Promise<void> {
    if (!canToggleSync) return;
    isToggling = true;
    errorMessage = null;
    try {
      await v2ICloudSyncStore.setEnabled(!v2ICloudSyncStore.status.enabled);
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      isToggling = false;
    }
  }

  async function syncNow(): Promise<void> {
    errorMessage = null;
    try {
      await v2ICloudSyncStore.syncNow();
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  onMount(() => {
    void v2ICloudSyncStore.loadStatus().catch((error) => {
      errorMessage = error instanceof Error ? error.message : String(error);
    });
  });
</script>

<V2SettingsShell
  title={i18n.t('v2ICloudSyncTitle')}
  onBack={() => void goto(settingsPathWithReturnTo('/settings', returnTo))}
>
  <div class="flex flex-col gap-5">
    {#if errorMessage || v2ICloudSyncStore.status.error}
      <div class="rounded-md border border-accent-peach-strong bg-accent-peach px-3 py-2 text-sm text-ink">
        {errorMessage ?? v2ICloudSyncStore.status.error}
      </div>
    {/if}

    <V2SettingsGroup
      title={i18n.t('v2ICloudSyncGroupTitle')}
      description={i18n.t('v2ICloudSyncDescription')}
    >
      <div class="px-4 py-4">
        <div class="flex items-start gap-3">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-[6px_14px_6px_14px] bg-accent-sky text-ink"
            aria-hidden="true"
          >
            <Cloud size={20} strokeWidth={2.2} />
          </div>

          <div class="min-w-0 flex-1">
            <p class="text-[16px] font-semibold leading-6 text-ink">
              {i18n.t('v2ICloudSyncToggleTitle')}
            </p>
            <p class="mt-1 text-sm leading-5 text-ink-muted">
              {i18n.t('v2ICloudSyncToggleDescription')}
            </p>
          </div>

          <button
            type="button"
            class={`relative h-8 w-14 shrink-0 rounded-full border-2 transition-colors disabled:cursor-not-allowed disabled:opacity-55 ${
              v2ICloudSyncStore.status.enabled
                ? 'border-ink bg-accent-sky-strong'
                : 'border-stroke bg-canvas'
            }`}
            role="switch"
            aria-checked={v2ICloudSyncStore.status.enabled}
            aria-label={i18n.t('v2ICloudSyncToggleTitle')}
            disabled={!canToggleSync}
            onclick={() => void toggleSync()}
          >
            <span
              class={`absolute left-1 top-1/2 h-5 w-5 -translate-y-1/2 rounded-full border border-stroke bg-white shadow-sm transition-transform ${
                v2ICloudSyncStore.status.enabled ? 'translate-x-7' : 'translate-x-0'
              }`}
            ></span>
          </button>
        </div>
      </div>
    </V2SettingsGroup>

    <V2SettingsGroup title={i18n.t('v2ICloudSyncStatusTitle')}>
      <div class="divide-y divide-stroke">
        <div class="flex min-h-14 items-center justify-between gap-3 px-4 py-3">
          <span class="text-sm font-semibold text-ink-muted">{i18n.t('v2ICloudSyncStatusLabel')}</span>
          <span class="text-right text-sm font-semibold text-ink">
            {statusText(v2ICloudSyncStore.status.status)}
          </span>
        </div>
        <div class="flex min-h-14 items-center justify-between gap-3 px-4 py-3">
          <span class="text-sm font-semibold text-ink-muted">{i18n.t('v2ICloudSyncAvailabilityLabel')}</span>
          <span class="text-right text-sm font-semibold text-ink">
            {v2ICloudSyncStore.status.available
              ? i18n.t('v2ICloudSyncAvailable')
              : i18n.t('v2ICloudSyncUnavailable')}
          </span>
        </div>
        <div class="flex min-h-14 items-center justify-between gap-3 px-4 py-3">
          <span class="text-sm font-semibold text-ink-muted">{i18n.t('v2ICloudSyncLastSyncedLabel')}</span>
          <span class="text-right text-sm font-semibold text-ink">
            {formatLastSynced(v2ICloudSyncStore.status.last_synced_at)}
          </span>
        </div>
      </div>
    </V2SettingsGroup>

    <button
      type="button"
      class="flex min-h-12 items-center justify-center gap-2 rounded-[6px_24px_6px_24px] border-2 border-ink bg-accent-sky-strong px-4 text-[15px] font-semibold text-ink transition-colors hover:bg-accent-sky disabled:cursor-not-allowed disabled:opacity-50"
      disabled={!canSyncNow}
      onclick={() => void syncNow()}
    >
      <RefreshCw
        size={18}
        strokeWidth={2.2}
        class={v2ICloudSyncStore.isSyncing ? 'animate-spin' : ''}
        aria-hidden="true"
      />
      {v2ICloudSyncStore.isSyncing
        ? i18n.t('v2ICloudSyncSyncing')
        : i18n.t('v2ICloudSyncNow')}
    </button>
  </div>
</V2SettingsShell>
