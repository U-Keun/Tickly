import { invoke } from './client';

export type NativeDockActionId = 'streak' | 'graph' | 'archive' | 'settings';

export interface NativeDockRequest {
  visible: boolean;
  streakLabel: string;
  graphLabel: string;
  archiveLabel: string;
  settingsLabel: string;
  streakEnabled: boolean;
  graphEnabled: boolean;
  archiveEnabled: boolean;
  settingsEnabled: boolean;
}

interface NativeDockEventDetail {
  actionId: NativeDockActionId;
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

export function shouldUseNativeDock(): boolean {
  if (typeof window === 'undefined') return false;
  return isIOS() && '__TAURI_INTERNALS__' in (window as TauriWindow);
}

export async function configureNativeDock(request: NativeDockRequest): Promise<boolean> {
  if (!shouldUseNativeDock()) {
    return false;
  }

  try {
    return await invoke<boolean>('configure_native_dock', { request });
  } catch (error) {
    console.error('Failed to configure native dock:', error);
    return false;
  }
}

export function addNativeDockActionListener(
  listener: (actionId: NativeDockActionId) => void
): () => void {
  if (typeof window === 'undefined') {
    return () => undefined;
  }

  const handleAction = (event: Event): void => {
    const detail = (event as CustomEvent<Partial<NativeDockEventDetail>>).detail;
    if (
      detail?.actionId !== 'streak' &&
      detail?.actionId !== 'graph' &&
      detail?.actionId !== 'archive' &&
      detail?.actionId !== 'settings'
    ) {
      return;
    }

    listener(detail.actionId);
  };

  window.addEventListener('tickly:nativeDockAction', handleAction);
  return () => window.removeEventListener('tickly:nativeDockAction', handleAction);
}
