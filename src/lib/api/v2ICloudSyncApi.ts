import { invoke } from './client';
import type { V2ICloudSyncResult, V2ICloudSyncStatus } from '../../types';

export async function v2GetICloudSyncStatus(): Promise<V2ICloudSyncStatus> {
  return invoke<V2ICloudSyncStatus>('v2_get_icloud_sync_status');
}

export async function v2SetICloudSyncEnabled(
  enabled: boolean
): Promise<V2ICloudSyncStatus> {
  return invoke<V2ICloudSyncStatus>('v2_set_icloud_sync_enabled', { enabled });
}

export async function v2TriggerICloudSync(): Promise<V2ICloudSyncResult> {
  return invoke<V2ICloudSyncResult>('v2_trigger_icloud_sync');
}
