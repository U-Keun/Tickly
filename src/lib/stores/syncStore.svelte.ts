import type {
  SyncResult,
  SyncStatusInfo,
  RealtimeConnectionState,
  RealtimeEvent
} from '../../types';
import { formatLastSyncedAt as formatLastSyncedAtLabel } from '$lib/sync/formatLastSynced';
import { RealtimeBridge } from './syncStoreRealtime';
import { appStore } from './appStore.svelte';
import * as syncApi from '$lib/api/syncApi';

// Supabase config - loaded from environment at build time
const SUPABASE_URL = import.meta.env.VITE_SUPABASE_URL || '';
const SUPABASE_ANON_KEY = import.meta.env.VITE_SUPABASE_ANON_KEY || '';

// Debounce delay for auto-sync (in milliseconds)
const SYNC_DEBOUNCE_MS = 2000;

function getErrorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

class SyncStore {
  // Manual sync state
  isEnabled = $state(false);
  isSyncing = $state(false);
  isOnline = $state(false);
  lastSyncedAt = $state<string | null>(null);
  pendingCount = $state(0);
  lastSyncResult = $state<SyncResult | null>(null);
  error = $state<string | null>(null);

  // Realtime state
  realtimeState = $state<RealtimeConnectionState>('disconnected');
  realtimeError = $state<string | null>(null);

  // Debounced sync timer
  private syncDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  private readonly realtimeBridge = new RealtimeBridge({
    supabaseUrl: SUPABASE_URL,
    supabaseAnonKey: SUPABASE_ANON_KEY,
    setRealtimeState: (state) => this.setRealtimeState(state),
    setRealtimeError: (error) => this.setRealtimeError(error),
    sync: async () => {
      await this.sync();
    },
    getErrorMessage
  });

  get isRealtimeConnected(): boolean {
    return this.realtimeState === 'connected';
  }

  // Setter methods for external callback reactivity
  private setRealtimeState(state: RealtimeConnectionState) {
    this.realtimeState = state;
  }

  private setRealtimeError(error: string | null) {
    this.realtimeError = error;
  }

  private applySyncStatus(status: SyncStatusInfo): void {
    this.isEnabled = status.is_enabled;
    this.isSyncing = status.is_syncing;
    this.isOnline = status.is_online;
    this.lastSyncedAt = status.last_synced_at;
    this.pendingCount = status.pending_count;
  }

  async loadStatus(): Promise<void> {
    try {
      const status = await syncApi.getSyncStatus();
      this.applySyncStatus(status);
      this.error = null;

      // Also load realtime status
      const realtimeStatus = await this.realtimeBridge.getStatus();
      this.setRealtimeState(realtimeStatus.state);
      this.setRealtimeError(realtimeStatus.last_error ?? null);
    } catch (error) {
      console.error('Failed to load sync status:', error);
      this.error = getErrorMessage(error);
    }
  }

  async sync(): Promise<SyncResult | null> {
    if (this.isSyncing) {
      return null;
    }

    this.isSyncing = true;
    this.error = null;

    try {
      const result = await syncApi.triggerSync();
      this.lastSyncResult = result;
      this.lastSyncedAt = result.last_synced_at;
      this.pendingCount = 0;

      // Always refresh app data after sync to reflect any changes
      await appStore.refreshAll();

      return result;
    } catch (error) {
      console.error('Sync failed:', error);
      this.error = getErrorMessage(error);
      return null;
    } finally {
      this.isSyncing = false;
    }
  }

  async forcePull(): Promise<SyncResult | null> {
    if (this.isSyncing) {
      return null;
    }

    this.isSyncing = true;
    this.error = null;

    try {
      await syncApi.forcePull();
      await appStore.refreshAll();

      const result = await syncApi.triggerSync();
      this.lastSyncResult = result;
      this.lastSyncedAt = result.last_synced_at;
      this.pendingCount = 0;

      await appStore.refreshAll();
      return result;
    } catch (error) {
      console.error('Force pull failed:', error);
      this.error = getErrorMessage(error);
      return null;
    } finally {
      this.isSyncing = false;
    }
  }

  async setEnabled(enabled: boolean): Promise<void> {
    try {
      await syncApi.setSyncEnabled(enabled);
      this.isEnabled = enabled;
    } catch (error) {
      console.error('Failed to set sync enabled:', error);
      throw error;
    }
  }

  async refreshPendingCount(): Promise<void> {
    try {
      this.pendingCount = await syncApi.getPendingCount();
    } catch (error) {
      console.error('Failed to get pending count:', error);
    }
  }

  /**
   * Schedule a debounced sync.
   * Call this when local data changes to auto-sync after a delay.
   */
  scheduleSync(): void {
    // Only schedule if sync is enabled and realtime is connected
    if (!this.isEnabled || !this.isRealtimeConnected) {
      return;
    }

    // Clear existing timer
    if (this.syncDebounceTimer) {
      clearTimeout(this.syncDebounceTimer);
    }

    // Schedule new sync
    this.syncDebounceTimer = setTimeout(async () => {
      this.syncDebounceTimer = null;
      await this.sync();
    }, SYNC_DEBOUNCE_MS);
  }

  // ===== Realtime Methods =====

  /**
   * Connect to Supabase Realtime
   */
  async connectRealtime(accessToken: string, userId: string): Promise<void> {
    await this.realtimeBridge.connect(accessToken, userId);
  }

  /**
   * Disconnect from Supabase Realtime
   */
  async disconnectRealtime(): Promise<void> {
    await this.realtimeBridge.disconnect();
  }

  formatLastSyncedAt(): string {
    return formatLastSyncedAtLabel(this.lastSyncedAt);
  }
}

export const syncStore = new SyncStore();
