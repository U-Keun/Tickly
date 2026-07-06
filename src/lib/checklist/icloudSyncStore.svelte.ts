import * as icloudSyncApi from '../api/icloudSyncApi';
import { i18n } from '../i18n';

const SYNC_DEBOUNCE_MS = 1200;
const AUTO_SYNC_DEBOUNCE_MS = 3500;
const SYNC_MIN_INTERVAL_MS = 5000;
const FOREGROUND_PULL_INTERVAL_MS = 30000;

let enabled = $state(false);
let isSyncing = $state(false);
let lastSyncedAt = $state<string | null>(null);
let lastError = $state<string | null>(null);
let availabilityMessage = $state<string | null>(null);
let syncTimeout: ReturnType<typeof setTimeout> | null = null;
let foregroundPullTimeout: ReturnType<typeof setTimeout> | null = null;
let lastSyncStartedAt = 0;
const idleResolvers = new Set<() => void>();

function localizedUnavailableMessage(status: string, message: string | null): string {
  if (status === 'accountUnavailable') {
    return i18n.t('icloudSyncAccountUnavailable');
  }
  if (status === 'unavailable') {
    return message ?? i18n.t('icloudSyncUnavailable');
  }
  return message ?? i18n.t('icloudSyncGenericError');
}

function clearSyncTimer(): void {
  if (syncTimeout === null) return;
  clearTimeout(syncTimeout);
  syncTimeout = null;
}

function clearForegroundPullTimer(): void {
  if (foregroundPullTimeout === null) return;
  clearTimeout(foregroundPullTimeout);
  foregroundPullTimeout = null;
}

function dispatchSynced(appliedCount: number): void {
  if (typeof window === 'undefined') return;
  window.dispatchEvent(
    new CustomEvent('tickly:iCloudSyncCompleted', { detail: { appliedCount } })
  );
}

function yieldToUI(): Promise<void> {
  if (typeof window === 'undefined') {
    return Promise.resolve();
  }

  return new Promise((resolve) => {
    window.setTimeout(resolve, 0);
  });
}

function notifySyncIdle(): void {
  idleResolvers.forEach((resolve) => resolve());
  idleResolvers.clear();
}

function waitUntilIdle(timeoutMs = 20000): Promise<void> {
  if (!isSyncing) return Promise.resolve();

  return new Promise((resolve) => {
    let timeout: ReturnType<typeof setTimeout> | null = null;

    const finish = (): void => {
      idleResolvers.delete(finish);
      if (timeout !== null) {
        clearTimeout(timeout);
        timeout = null;
      }
      resolve();
    };

    idleResolvers.add(finish);
    timeout = setTimeout(finish, timeoutMs);
  });
}

async function loadStatus(): Promise<void> {
  const status = await icloudSyncApi.getStatus();
  enabled = status.enabled;
  lastSyncedAt = status.lastSyncedAt;
  lastError = status.lastError;
  availabilityMessage = icloudSyncApi.shouldUseNativeICloudSync()
    ? null
    : i18n.t('icloudSyncUnavailableIOSApp');
}

async function setEnabled(nextEnabled: boolean): Promise<void> {
  const status = await icloudSyncApi.setEnabled(nextEnabled);
  enabled = status.enabled;
  lastSyncedAt = status.lastSyncedAt;
  lastError = status.lastError;
  if (enabled) {
    void syncNow();
  } else {
    clearSyncTimer();
  }
}

async function syncNow(): Promise<void> {
  if (!enabled || isSyncing) return;
  clearSyncTimer();
  isSyncing = true;
  lastSyncStartedAt = Date.now();
  lastError = null;

  try {
    const localRecords = await icloudSyncApi.exportRecords();
    await yieldToUI();
    const exchangeResult = await icloudSyncApi.exchangeWithICloud(localRecords);
    await yieldToUI();

    if (exchangeResult.status !== 'success') {
      const message = localizedUnavailableMessage(exchangeResult.status, exchangeResult.message);
      availabilityMessage =
        exchangeResult.status === 'unavailable' || exchangeResult.status === 'accountUnavailable'
          ? message
          : availabilityMessage;
      lastError = message;
      await icloudSyncApi.setLastError(message);
      return;
    }

    availabilityMessage = null;
    const appliedCount = await icloudSyncApi.applyRemoteRecords(exchangeResult.remoteRecords);
    await yieldToUI();
    const syncedSyncIds = [
      ...exchangeResult.syncedSyncIds,
      ...exchangeResult.remoteRecords.map((record) => record.syncId)
    ];
    await icloudSyncApi.markRecordsSynced([...new Set(syncedSyncIds)]);
    await yieldToUI();
    const status = await icloudSyncApi.getStatus();
    lastSyncedAt = status.lastSyncedAt;
    lastError = status.lastError;
    dispatchSynced(appliedCount);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    lastError = message;
    await icloudSyncApi.setLastError(message).catch(() => undefined);
  } finally {
    isSyncing = false;
    notifySyncIdle();
  }
}

function scheduleSync(delayMs = SYNC_DEBOUNCE_MS): void {
  clearSyncTimer();
  if (!enabled || isSyncing) return;
  const elapsedSinceLastSync = Date.now() - lastSyncStartedAt;
  const intervalDelay = Math.max(0, SYNC_MIN_INTERVAL_MS - elapsedSinceLastSync);
  syncTimeout = setTimeout(() => {
    syncTimeout = null;
    void syncNow();
  }, Math.max(delayMs, intervalDelay));
}

function scheduleForegroundPull(delayMs = FOREGROUND_PULL_INTERVAL_MS): void {
  clearForegroundPullTimer();
  if (!enabled || typeof document === 'undefined' || document.visibilityState !== 'visible') {
    return;
  }

  foregroundPullTimeout = setTimeout(() => {
    foregroundPullTimeout = null;
    void syncNow().finally(() => {
      scheduleForegroundPull();
    });
  }, delayMs);
}

function dispose(): void {
  clearSyncTimer();
  clearForegroundPullTimer();
}

export const icloudSyncStore = {
  get enabled() {
    return enabled;
  },
  get isSyncing() {
    return isSyncing;
  },
  get lastSyncedAt() {
    return lastSyncedAt;
  },
  get lastError() {
    return lastError;
  },
  get availabilityMessage() {
    return availabilityMessage;
  },
  get canUseNative() {
    return icloudSyncApi.shouldUseNativeICloudSync();
  },
  loadStatus,
  setEnabled,
  syncNow,
  waitUntilIdle,
  scheduleSync,
  scheduleAutoSync: () => scheduleSync(AUTO_SYNC_DEBOUNCE_MS),
  scheduleForegroundPull,
  disposeForegroundPull: clearForegroundPullTimer,
  dispose
};
