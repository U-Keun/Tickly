import type { V2ICloudSyncResult, V2ICloudSyncStatus } from '../../types';
import * as v2ICloudSyncApi from '../api/v2ICloudSyncApi';

const SYNC_DEBOUNCE_MS = 1200;

let status = $state<V2ICloudSyncStatus>({
  enabled: false,
  available: false,
  status: 'idle',
  last_synced_at: null,
  error: null
});
let isLoading = $state(false);
let isSyncing = $state(false);
let debounceTimer: ReturnType<typeof setTimeout> | null = null;

function clearDebounce(): void {
  if (debounceTimer === null) return;
  clearTimeout(debounceTimer);
  debounceTimer = null;
}

function applyResult(result: V2ICloudSyncResult): void {
  status = {
    enabled: status.enabled,
    available: result.available,
    status: result.status,
    last_synced_at: result.last_synced_at ?? status.last_synced_at,
    error: result.error
  };
}

async function loadStatus(): Promise<void> {
  isLoading = true;
  try {
    status = await v2ICloudSyncApi.v2GetICloudSyncStatus();
  } finally {
    isLoading = false;
  }
}

async function setEnabled(enabled: boolean): Promise<void> {
  isLoading = true;
  clearDebounce();
  try {
    status = await v2ICloudSyncApi.v2SetICloudSyncEnabled(enabled);
    if (enabled && status.available) {
      await syncNow();
    }
  } finally {
    isLoading = false;
  }
}

async function syncNow(): Promise<void> {
  if (!status.enabled || isSyncing) return;
  isSyncing = true;
  status = { ...status, status: 'syncing', error: null };
  try {
    const result = await v2ICloudSyncApi.v2TriggerICloudSync();
    applyResult(result);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    status = { ...status, status: 'error', error: message };
    throw error;
  } finally {
    isSyncing = false;
  }
}

function scheduleSync(): void {
  if (!status.enabled || !status.available) return;
  clearDebounce();
  debounceTimer = setTimeout(() => {
    debounceTimer = null;
    void syncNow().catch((error) => {
      console.error('Failed to run scheduled v2 iCloud sync.', error);
    });
  }, SYNC_DEBOUNCE_MS);
}

function dispose(): void {
  clearDebounce();
}

export const v2ICloudSyncStore = {
  get status() {
    return status;
  },
  get isLoading() {
    return isLoading;
  },
  get isSyncing() {
    return isSyncing;
  },
  loadStatus,
  setEnabled,
  syncNow,
  scheduleSync,
  dispose
};
