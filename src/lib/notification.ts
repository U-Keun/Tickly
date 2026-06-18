import { invoke } from './api/client';

/**
 * Check permission via native Tauri invoke (NOT window.Notification which doesn't work on iOS).
 */
async function nativeIsPermissionGranted(): Promise<boolean> {
  return await invoke<boolean>('plugin:notification|is_permission_granted');
}

/**
 * Request permission via native Tauri invoke.
 */
async function nativeRequestPermission(): Promise<string> {
  return await invoke<string>('plugin:notification|request_permission');
}

/**
 * Request notification permission if not already granted.
 * Returns true if permission is granted.
 */
export async function ensurePermission(): Promise<boolean> {
  try {
    let granted = await nativeIsPermissionGranted();
    if (!granted) {
      const result = await nativeRequestPermission();
      granted = result === 'granted';
    }
    return granted;
  } catch (e) {
    console.error('Notification permission error:', e);
    return false;
  }
}
