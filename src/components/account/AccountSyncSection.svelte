<script lang="ts">
  import type { RealtimeConnectionState } from '../../types';
  import { i18n } from '$lib/i18n';

  let {
    isEnabled,
    isSyncing,
    realtimeState,
    lastSyncedText,
    syncError = null,
    onToggleSync,
    onSync,
    onForcePull
  } = $props<{
    isEnabled: boolean;
    isSyncing: boolean;
    realtimeState: RealtimeConnectionState;
    lastSyncedText: string;
    syncError?: string | null;
    onToggleSync: () => void | Promise<void>;
    onSync: () => void | Promise<void>;
    onForcePull: () => void | Promise<void>;
  }>();
</script>

<div class="sync-section">
  <h3>{i18n.t('syncTitle')}</h3>

  <div class="sync-toggle">
    <span>{i18n.t('syncEnabled')}</span>
    <button
      class="toggle-switch"
      class:active={isEnabled}
      onclick={onToggleSync}
      aria-label="Toggle sync"
    >
      <span class="toggle-thumb"></span>
    </button>
  </div>

  {#if isEnabled}
    <div class="sync-status">
      <div class="status-row">
        <span class="status-label">{i18n.t('lastSynced')}</span>
        <span class="status-value">{lastSyncedText}</span>
      </div>
      <div class="status-row">
        <span class="status-label">{i18n.t('realtimeSync')}</span>
        <span class="status-value realtime-status" class:connected={realtimeState === 'connected'} class:connecting={realtimeState === 'connecting' || realtimeState === 'reconnecting'}>
          {#if realtimeState === 'connected'}
            <span class="realtime-dot connected"></span> {i18n.t('realtimeConnected')}
          {:else if realtimeState === 'connecting'}
            <span class="realtime-dot connecting"></span> {i18n.t('realtimeConnecting')}
          {:else if realtimeState === 'reconnecting'}
            <span class="realtime-dot connecting"></span> {i18n.t('realtimeReconnecting')}
          {:else}
            <span class="realtime-dot disconnected"></span> {i18n.t('realtimeDisconnected')}
          {/if}
        </span>
      </div>
    </div>

    <button
      class="sync-btn"
      onclick={onSync}
      disabled={isSyncing}
    >
      {#if isSyncing}
        <span class="sync-spinner"></span>
        {i18n.t('syncing')}
      {:else}
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
        {i18n.t('syncNow')}
      {/if}
    </button>

    <button
      class="force-pull-btn"
      onclick={onForcePull}
      disabled={isSyncing}
    >
      {i18n.t('forcePull')}
    </button>

    {#if syncError}
      <p class="sync-error">{syncError}</p>
    {/if}
  {/if}
</div>

<style>
  .sync-section {
    background: var(--color-canvas);
    border-radius: 12px;
    padding: 20px;
  }

  .sync-section h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--color-ink);
    margin-bottom: 16px;
  }

  .sync-toggle {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .sync-toggle span {
    font-size: 15px;
    color: var(--color-ink);
  }

  .toggle-switch {
    width: 51px;
    height: 31px;
    background: var(--color-stroke);
    border: none;
    border-radius: 16px;
    position: relative;
    cursor: pointer;
    transition: background 0.2s;
  }

  .toggle-switch.active {
    background: var(--color-accent-mint-strong);
  }

  .toggle-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 27px;
    height: 27px;
    background: var(--color-white);
    border-radius: 50%;
    box-shadow: 0 2px 4px rgba(0,0,0,0.2);
    transition: transform 0.2s;
  }

  .toggle-switch.active .toggle-thumb {
    transform: translateX(20px);
  }

  .sync-status {
    background: var(--color-mist);
    border-radius: 8px;
    padding: 12px;
    margin-bottom: 16px;
  }

  .status-row {
    display: flex;
    justify-content: space-between;
    font-size: 14px;
  }

  .status-row:not(:last-child) {
    margin-bottom: 8px;
  }

  .status-label {
    color: var(--color-ink-muted);
  }

  .status-value {
    color: var(--color-ink);
    font-weight: 500;
  }

  .realtime-status {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .realtime-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .realtime-dot.connected {
    background: var(--color-accent-mint-strong);
  }

  .realtime-dot.connecting {
    background: var(--color-accent-sky-strong);
    animation: pulse 1s infinite;
  }

  .realtime-dot.disconnected {
    background: var(--color-ink-muted);
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }

  .sync-btn {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 14px;
    background: var(--color-accent-sky);
    border: none;
    border-radius: 10px;
    color: var(--color-ink);
    font-size: 15px;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .force-pull-btn {
    width: 100%;
    margin-top: 10px;
    padding: 12px;
    background: transparent;
    border: 1px solid var(--color-stroke);
    border-radius: 10px;
    color: var(--color-ink-muted);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .sync-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .force-pull-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .sync-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid var(--color-ink-muted);
    border-top-color: var(--color-ink);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .sync-error {
    margin-top: 12px;
    padding: 12px;
    background: var(--color-accent-peach);
    border-radius: 8px;
    color: var(--color-accent-peach-strong);
    font-size: 14px;
  }
</style>
