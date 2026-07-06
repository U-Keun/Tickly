import type { TodoItem } from '../../types';
import { invoke } from './client';

interface PendingNotification {
  id: number;
}

interface ActiveReminder {
  item: TodoItem;
  id: number;
  reminderTime: {
    hour: number;
    minute: number;
  };
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

function yieldToUI(): Promise<void> {
  if (typeof window === 'undefined') {
    return Promise.resolve();
  }

  return new Promise((resolve) => {
    window.setTimeout(resolve, 0);
  });
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

  await scheduleReminder(item, id, reminderTime);
}

async function scheduleReminder(
  item: TodoItem,
  id: number,
  reminderTime: { hour: number; minute: number }
): Promise<void> {
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
  const activeReminders = items
    .map((item): ActiveReminder | null => {
      if (item.done) return null;
      const id = notificationId(item.id);
      const reminderTime = parseReminderTime(item.reminder_at);
      if (id === null || reminderTime === null) return null;
      return { item, id, reminderTime };
    })
    .filter((reminder): reminder is ActiveReminder => reminder !== null);
  const activeIds = new Set(activeReminders.map((reminder) => reminder.id));

  try {
    const pending = await nativeGetPending();
    const staleIds = pending
      .map((notification) => notification.id)
      .filter((id) => isChecklistNotificationId(id) && !activeIds.has(id));
    await nativeCancel(staleIds);
  } catch (error) {
    console.error('Failed to prune stale reminders:', error);
  }

  if (activeReminders.length === 0) return;

  const granted = await ensurePermission();
  if (!granted) return;

  for (const reminder of activeReminders) {
    await scheduleReminder(reminder.item, reminder.id, reminder.reminderTime);
    await yieldToUI();
  }
}
