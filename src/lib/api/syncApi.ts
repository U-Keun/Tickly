import type { SyncResult, SyncStatusInfo } from '../../types';
import { invoke } from './client';

export async function triggerSync(): Promise<SyncResult> {
  return invoke<SyncResult>('trigger_sync');
}

export async function getSyncStatus(): Promise<SyncStatusInfo> {
  return invoke<SyncStatusInfo>('get_sync_status');
}

export async function getPendingCount(): Promise<number> {
  return invoke<number>('get_pending_count');
}

export async function setSyncEnabled(enabled: boolean): Promise<void> {
  return invoke<void>('set_sync_enabled', { enabled });
}

export async function forcePull(): Promise<void> {
  return invoke<void>('force_pull');
}
