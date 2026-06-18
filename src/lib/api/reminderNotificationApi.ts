import type { TodoItem } from '../../types';
import { invoke } from './client';

interface PendingNotification {
  id: number;
}

const CHECKLIST_NOTIFICATION_ID_OFFSET = 200_000_000;
const MAX_I32 = 2_147_483_647;

function notificationId(itemId: number): number | null {
  const id = CHECKLIST_NOTIFICATION_ID_OFFSET + Math.trunc(itemId);
  return id > CHECKLIST_NOTIFICATION_ID_OFFSET && id <= MAX_I32 ? id : null;
}

function isChecklistNotificationId(id: number): boolean {
  return id > CHECKLIST_NOTIFICATION_ID_OFFSET && id <= MAX_I32;
}

function parseReminderTime(reminderAt: string | null): { hour: number; minute: number } | null {
  if (!reminderAt) return null;
  const [rawHour, rawMinute] = reminderAt.split(':');
  if (rawHour?.length !== 2 || rawMinute?.length !== 2) return null;

  const hour = Number.parseInt(rawHour, 10);
  const minute = Number.parseInt(rawMinute, 10);
  if (!Number.isInteger(hour) || !Number.isInteger(minute)) return null;
  if (hour < 0 || hour > 23 || minute < 0 || minute > 59) return null;

  return { hour, minute };
}

async function nativeIsPermissionGranted(): Promise<boolean> {
  return invoke<boolean>('plugin:notification|is_permission_granted');
}

async function nativeRequestPermission(): Promise<string> {
  return invoke<string>('plugin:notification|request_permission');
}

async function nativeSendNotification(options: Record<string, unknown>): Promise<void> {
  await invoke('plugin:notification|notify', { options });
}

async function nativeGetPending(): Promise<PendingNotification[]> {
  return invoke<PendingNotification[]>('plugin:notification|get_pending');
}

async function nativeCancel(ids: number[]): Promise<void> {
  if (ids.length === 0) return;
  await invoke('plugin:notification|cancel', { notifications: ids });
}

async function ensurePermission(): Promise<boolean> {
  try {
    let granted = await nativeIsPermissionGranted();
    if (!granted) {
      granted = (await nativeRequestPermission()) === 'granted';
    }
    return granted;
  } catch (error) {
    console.error('Checklist notification permission error:', error);
    return false;
  }
}

export async function cancelReminderForItem(itemId: number): Promise<void> {
  const id = notificationId(itemId);
  if (id === null) return;

  try {
    await nativeCancel([id]);
  } catch (error) {
    console.error('Failed to cancel reminder:', error);
  }
}

export async function syncReminderForItem(item: TodoItem): Promise<void> {
  const id = notificationId(item.id);
  if (id === null) return;

  const reminderTime = parseReminderTime(item.reminder_at);
  if (item.done || reminderTime === null) {
    await cancelReminderForItem(item.id);
    return;
  }

  const granted = await ensurePermission();
  if (!granted) return;

  try {
    await nativeCancel([id]);
    await nativeSendNotification({
      id,
      title: 'Tickly',
      body: item.text,
      schedule: {
        interval: {
          interval: {
            hour: reminderTime.hour,
            minute: reminderTime.minute
          },
          allowWhileIdle: true
        }
      }
    });
  } catch (error) {
    console.error('Failed to schedule reminder:', error);
  }
}

export async function syncActiveReminderNotifications(items: TodoItem[]): Promise<void> {
  const activeItems = items.filter(
    (item) => !item.done && parseReminderTime(item.reminder_at) !== null
  );
  const activeIds = new Set(
    activeItems
      .map((item) => notificationId(item.id))
      .filter((id): id is number => id !== null)
  );

  try {
    const pending = await nativeGetPending();
    const staleIds = pending
      .map((notification) => notification.id)
      .filter((id) => isChecklistNotificationId(id) && !activeIds.has(id));
    await nativeCancel(staleIds);
  } catch (error) {
    console.error('Failed to prune stale reminders:', error);
  }

  if (activeItems.length === 0) return;

  const granted = await ensurePermission();
  if (!granted) return;

  await Promise.all(activeItems.map(syncReminderForItem));
}
