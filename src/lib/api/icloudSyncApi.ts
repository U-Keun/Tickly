import { invoke } from './client';
import type { ChecklistSyncRecord, ChecklistSyncStatus } from '../../types';

export type ICloudSyncResultStatus =
  | 'success'
  | 'unavailable'
  | 'accountUnavailable'
  | 'error';

export interface ICloudSyncExchangeResult {
  token: string;
  status: ICloudSyncResultStatus;
  message: string | null;
  remoteRecords: ChecklistSyncRecord[];
  syncedSyncIds: string[];
}

interface TauriWindow extends Window {
  __TAURI_INTERNALS__?: unknown;
}

function isIOS(): boolean {
  if (typeof navigator === 'undefined') return false;

  return (
    /iPad|iPhone|iPod/.test(navigator.userAgent) ||
    (navigator.platform === 'MacIntel' && navigator.maxTouchPoints > 1)
  );
}

function canAttemptNativeICloudSync(): boolean {
  if (typeof window === 'undefined') return false;
  return isIOS() && '__TAURI_INTERNALS__' in (window as TauriWindow);
}

function createToken(): string {
  if (typeof crypto !== 'undefined' && 'randomUUID' in crypto) {
    return crypto.randomUUID();
  }

  return `${Date.now()}-${Math.random().toString(36).slice(2)}`;
}

function parseExchangeResult(detail: unknown): ICloudSyncExchangeResult | null {
  if (!detail || typeof detail !== 'object') return null;
  const value = detail as Partial<ICloudSyncExchangeResult>;
  if (typeof value.token !== 'string') return null;
  if (
    value.status !== 'success' &&
    value.status !== 'unavailable' &&
    value.status !== 'accountUnavailable' &&
    value.status !== 'error'
  ) {
    return null;
  }

  return {
    token: value.token,
    status: value.status,
    message: typeof value.message === 'string' ? value.message : null,
    remoteRecords: Array.isArray(value.remoteRecords) ? value.remoteRecords : [],
    syncedSyncIds: Array.isArray(value.syncedSyncIds) ? value.syncedSyncIds : []
  };
}

export function shouldUseNativeICloudSync(): boolean {
  return canAttemptNativeICloudSync();
}

export async function getStatus(): Promise<ChecklistSyncStatus> {
  return invoke<ChecklistSyncStatus>('checklist_sync_get_status');
}

export async function setEnabled(enabled: boolean): Promise<ChecklistSyncStatus> {
  return invoke<ChecklistSyncStatus>('checklist_sync_set_enabled', { enabled });
}

export async function exportRecords(): Promise<ChecklistSyncRecord[]> {
  return invoke<ChecklistSyncRecord[]>('checklist_sync_export_records');
}

export async function applyRemoteRecords(records: ChecklistSyncRecord[]): Promise<number> {
  return invoke<number>('checklist_sync_apply_remote_records', { records });
}

export async function markRecordsSynced(syncIds: string[]): Promise<void> {
  return invoke<void>('checklist_sync_mark_records_synced', { syncIds });
}

export async function setLastError(error: string | null): Promise<void> {
  return invoke<void>('checklist_sync_set_last_error', { error });
}

export async function exchangeWithICloud(
  records: ChecklistSyncRecord[]
): Promise<ICloudSyncExchangeResult> {
  const token = createToken();

  if (!canAttemptNativeICloudSync()) {
    return {
      token,
      status: 'unavailable',
      message: null,
      remoteRecords: [],
      syncedSyncIds: []
    };
  }

  return new Promise<ICloudSyncExchangeResult>((resolve) => {
    let timeout: ReturnType<typeof setTimeout> | null = null;

    const cleanup = (): void => {
      window.removeEventListener('tickly:iCloudSyncResult', handleResult);
      if (timeout !== null) {
        clearTimeout(timeout);
        timeout = null;
      }
    };

    const finish = (result: ICloudSyncExchangeResult): void => {
      cleanup();
      resolve(result);
    };

    const handleResult = (event: Event): void => {
      const result = parseExchangeResult((event as CustomEvent<unknown>).detail);
      if (!result || result.token !== token) return;
      finish(result);
    };

    window.addEventListener('tickly:iCloudSyncResult', handleResult);
    timeout = setTimeout(() => {
      finish({
        token,
        status: 'error',
        message: 'iCloud sync timed out.',
        remoteRecords: [],
        syncedSyncIds: []
      });
    }, 30000);

    invoke<boolean>('checklist_icloud_exchange', { request: { token, records } })
      .then((started) => {
        if (!started) {
          finish({
            token,
            status: 'unavailable',
            message: null,
            remoteRecords: [],
            syncedSyncIds: []
          });
        }
      })
      .catch((error) => {
        finish({
          token,
          status: 'error',
          message: error instanceof Error ? error.message : String(error),
          remoteRecords: [],
          syncedSyncIds: []
        });
      });
  });
}
