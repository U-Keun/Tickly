import type {
  V2Category,
  V2ItemSearchResult,
  V2RepeatType,
  V2StreakHeatmap,
  V2Tag,
  V2TodoItem
} from '../../types';
import * as settingsApi from '../api/settingsApi';
import * as v2ChecklistApi from '../api/v2ChecklistApi';
import * as v2ReminderNotificationApi from '../api/v2ReminderNotificationApi';

const MIN_REPEAT_TIMER_DELAY_MS = 1000;

let categories = $state<V2Category[]>([]);
let items = $state<V2TodoItem[]>([]);
let tags = $state<V2Tag[]>([]);
let selectedCategoryId = $state<number | null>(null);
let isLoading = $state(false);
let errorMessage = $state<string | null>(null);
let repeatProcessingTimeout: ReturnType<typeof setTimeout> | null = null;
let repeatProcessingScheduleToken = 0;

function sortCategories(nextCategories: V2Category[]): V2Category[] {
  return [...nextCategories].sort((a, b) => a.display_order - b.display_order);
}

function sortItems(nextItems: V2TodoItem[]): V2TodoItem[] {
  return [...nextItems].sort((a, b) => {
    if (a.done !== b.done) {
      return a.done ? 1 : -1;
    }
    return a.display_order - b.display_order;
  });
}

function sortTags(nextTags: V2Tag[]): V2Tag[] {
  return [...nextTags].sort((a, b) => a.name.localeCompare(b.name));
}

function moveInList<T extends { id: number }>(list: T[], id: number, delta: number): T[] {
  const index = list.findIndex((item) => item.id === id);
  if (index < 0) return list;

  const nextIndex = index + delta;
  if (nextIndex < 0 || nextIndex >= list.length) return list;

  const nextList = [...list];
  const [item] = nextList.splice(index, 1);
  nextList.splice(nextIndex, 0, item);
  return nextList;
}

function reindexCategories(nextCategories: V2Category[]): V2Category[] {
  return nextCategories.map((category, index) => ({
    ...category,
    display_order: (index + 1) * 1000
  }));
}

function reindexItems(nextItems: V2TodoItem[]): V2TodoItem[] {
  return nextItems.map((item, index) => ({
    ...item,
    display_order: (index + 1) * 1000
  }));
}

function setError(error: unknown, fallback: string): Error {
  const nextError = error instanceof Error ? error : new Error(String(error || fallback));
  errorMessage = nextError.message;
  console.error(fallback, error);
  return nextError;
}

function parseResetTime(resetTime: string | null): { hour: number; minute: number } {
  const [rawHour, rawMinute] = (resetTime || '00:00').split(':');
  const hour = Number.parseInt(rawHour, 10);
  const minute = Number.parseInt(rawMinute, 10);

  return {
    hour: Number.isInteger(hour) && hour >= 0 && hour <= 23 ? hour : 0,
    minute: Number.isInteger(minute) && minute >= 0 && minute <= 59 ? minute : 0
  };
}

function getNextResetAt(resetTime: string | null, now = new Date()): Date {
  const { hour, minute } = parseResetTime(resetTime);
  const nextResetAt = new Date(now);
  nextResetAt.setHours(hour, minute, 0, 0);

  if (nextResetAt.getTime() <= now.getTime()) {
    nextResetAt.setDate(nextResetAt.getDate() + 1);
  }

  return nextResetAt;
}

function clearRepeatProcessingTimer(): void {
  if (repeatProcessingTimeout === null) return;
  clearTimeout(repeatProcessingTimeout);
  repeatProcessingTimeout = null;
}

async function loadItemsForSelectedCategory(): Promise<void> {
  if (selectedCategoryId === null) {
    items = [];
    return;
  }

  items = sortItems(await v2ChecklistApi.v2GetItems(selectedCategoryId));
}

async function syncReminderNotifications(): Promise<void> {
  try {
    const reminderItems = await v2ChecklistApi.v2GetActiveReminderItems();
    await v2ReminderNotificationApi.syncActiveReminderNotifications(reminderItems);
  } catch (error) {
    console.error('Failed to sync v2 reminder notifications.', error);
  }
}

async function load(): Promise<void> {
  isLoading = true;
  errorMessage = null;

  try {
    await v2ChecklistApi.v2ProcessRepeats();
    const [nextCategoriesRaw, nextTagsRaw] = await Promise.all([
      v2ChecklistApi.v2GetCategories(),
      v2ChecklistApi.v2GetTags()
    ]);
    const nextCategories = sortCategories(nextCategoriesRaw);
    categories = nextCategories;
    tags = sortTags(nextTagsRaw);

    if (nextCategories.length === 0) {
      selectedCategoryId = null;
      items = [];
      return;
    }

    const selectedStillExists = nextCategories.some(
      (category) => category.id === selectedCategoryId
    );
    if (!selectedStillExists) {
      selectedCategoryId = nextCategories[0].id;
    }

    await loadItemsForSelectedCategory();
    await syncReminderNotifications();
  } catch (error) {
    throw setError(error, 'Failed to load v2 checklist.');
  } finally {
    isLoading = false;
  }
}

async function selectCategory(categoryId: number): Promise<void> {
  errorMessage = null;

  try {
    const nextItems = sortItems(await v2ChecklistApi.v2GetItems(categoryId));
    selectedCategoryId = categoryId;
    items = nextItems;
  } catch (error) {
    throw setError(error, 'Failed to select v2 category.');
  }
}

async function addCategory(name: string): Promise<void> {
  errorMessage = null;

  try {
    const category = await v2ChecklistApi.v2CreateCategory(name);
    categories = sortCategories([...categories, category]);
    selectedCategoryId = category.id;
    items = [];
  } catch (error) {
    throw setError(error, 'Failed to add v2 category.');
  }
}

async function updateCategory(id: number, name: string): Promise<void> {
  errorMessage = null;

  try {
    await v2ChecklistApi.v2UpdateCategory(id, name);
    const trimmedName = name.trim();
    categories = categories.map((category) =>
      category.id === id ? { ...category, name: trimmedName } : category
    );
  } catch (error) {
    throw setError(error, 'Failed to update v2 category.');
  }
}

async function deleteCategory(id: number): Promise<void> {
  errorMessage = null;

  try {
    const reminderItemIds = items
      .filter((item) => item.category_id === id && item.reminder_at)
      .map((item) => item.id);
    await v2ChecklistApi.v2DeleteCategory(id);
    const nextCategories = categories.filter((category) => category.id !== id);
    categories = nextCategories;

    if (selectedCategoryId === id) {
      const nextSelectedCategoryId = nextCategories[0]?.id ?? null;
      const nextItems =
        nextSelectedCategoryId === null
          ? []
          : sortItems(await v2ChecklistApi.v2GetItems(nextSelectedCategoryId));

      selectedCategoryId = nextSelectedCategoryId;
      items = nextItems;
    }
    await refreshTags();
    await Promise.all(
      reminderItemIds.map((itemId) => v2ReminderNotificationApi.cancelReminderForItem(itemId))
    );
    await syncReminderNotifications();
  } catch (error) {
    throw setError(error, 'Failed to delete v2 category.');
  }
}

async function moveCategory(id: number, delta: number): Promise<void> {
  const movedRaw = moveInList(categories, id, delta);
  if (movedRaw === categories) return;
  await reorderCategories(movedRaw.map((category) => category.id));
}

async function reorderCategories(categoryIds: number[]): Promise<void> {
  const categoriesById = new Map(categories.map((category) => [category.id, category]));
  const orderedCategories = categoryIds
    .map((id) => categoriesById.get(id))
    .filter((category): category is V2Category => category !== undefined);
  if (orderedCategories.length !== categories.length) return;

  const moved = reindexCategories(orderedCategories);

  errorMessage = null;
  try {
    await v2ChecklistApi.v2ReorderCategories(moved.map((category) => category.id));
    categories = moved;
  } catch (error) {
    await load();
    throw setError(error, 'Failed to reorder v2 categories.');
  }
}

async function refreshTags(): Promise<void> {
  tags = sortTags(await v2ChecklistApi.v2GetTags());
}

async function addItem(text: string, tagNames: string[] = []): Promise<void> {
  if (selectedCategoryId === null) return;
  errorMessage = null;

  try {
    const item = await v2ChecklistApi.v2CreateItem(selectedCategoryId, text, tagNames);
    items = sortItems([...items, item]);
    await refreshTags();
  } catch (error) {
    throw setError(error, 'Failed to add v2 item.');
  }
}

async function searchItems(query: string, limit: number): Promise<V2ItemSearchResult[]> {
  errorMessage = null;

  try {
    return await v2ChecklistApi.v2SearchItems(query, limit);
  } catch (error) {
    throw setError(error, 'Failed to search v2 items.');
  }
}

async function updateItemText(id: number, text: string): Promise<void> {
  errorMessage = null;

  try {
    await v2ChecklistApi.v2UpdateItemText(id, text);
    const trimmedText = text.trim();
    items = items.map((item) => (item.id === id ? { ...item, text: trimmedText } : item));
  } catch (error) {
    throw setError(error, 'Failed to update v2 item.');
  }
}

async function updateItemDetails(
  id: number,
  text: string,
  memo: string | null,
  tagNames: string[] = [],
  repeatType: V2RepeatType = 'none',
  repeatDetail: string | null = null,
  reminderAt: string | null = null,
  trackStreak?: boolean
): Promise<void> {
  errorMessage = null;

  try {
    const nextTrackStreak =
      trackStreak ?? items.find((item) => item.id === id)?.track_streak ?? false;
    const updatedItem = await v2ChecklistApi.v2UpdateItemDetails(
      id,
      text,
      memo,
      tagNames,
      repeatType,
      repeatDetail,
      reminderAt,
      nextTrackStreak
    );
    items = sortItems(items.map((item) => (item.id === id ? updatedItem : item)));
    await refreshTags();
    await v2ReminderNotificationApi.syncReminderForItem(updatedItem);
  } catch (error) {
    throw setError(error, 'Failed to update v2 item details.');
  }
}

async function getStreakHeatmaps(): Promise<V2StreakHeatmap[]> {
  errorMessage = null;

  try {
    return await v2ChecklistApi.v2GetStreakHeatmaps();
  } catch (error) {
    throw setError(error, 'Failed to load v2 streak heatmaps.');
  }
}

async function toggleItem(id: number): Promise<void> {
  errorMessage = null;

  try {
    const updatedItem = await v2ChecklistApi.v2ToggleItem(id);
    items = sortItems(items.map((item) => (item.id === id ? updatedItem : item)));
    await v2ReminderNotificationApi.syncReminderForItem(updatedItem);
  } catch (error) {
    throw setError(error, 'Failed to toggle v2 item.');
  }
}

async function processRepeatsAndReload(): Promise<number> {
  errorMessage = null;

  try {
    const reactivatedCount = await v2ChecklistApi.v2ProcessRepeats();
    if (reactivatedCount > 0) {
      await loadItemsForSelectedCategory();
    }
    await syncReminderNotifications();
    return reactivatedCount;
  } catch (error) {
    throw setError(error, 'Failed to process v2 repeats.');
  }
}

async function archiveCompletedItems(categoryId: number): Promise<number> {
  errorMessage = null;

  try {
    const archivedCount = await v2ChecklistApi.v2ArchiveCompletedItems(categoryId);
    if (archivedCount > 0 && selectedCategoryId === categoryId) {
      await loadItemsForSelectedCategory();
    }
    return archivedCount;
  } catch (error) {
    throw setError(error, 'Failed to archive v2 completed items.');
  }
}

async function scheduleRepeatProcessing(): Promise<void> {
  const scheduleToken = ++repeatProcessingScheduleToken;
  clearRepeatProcessingTimer();

  try {
    const resetTime = await settingsApi.getSetting('reset_time');
    if (scheduleToken !== repeatProcessingScheduleToken) return;

    const nextResetAt = getNextResetAt(resetTime);
    const delay = Math.max(
      MIN_REPEAT_TIMER_DELAY_MS,
      nextResetAt.getTime() - Date.now()
    );

    repeatProcessingTimeout = setTimeout(() => {
      repeatProcessingTimeout = null;
      void processRepeatsAndReload()
        .catch(() => undefined)
        .finally(() => {
          void scheduleRepeatProcessing();
        });
    }, delay);
  } catch (error) {
    console.error('Failed to schedule v2 repeat processing.', error);
  }
}

function disposeRepeatProcessingTimer(): void {
  repeatProcessingScheduleToken += 1;
  clearRepeatProcessingTimer();
}

async function deleteItem(id: number): Promise<void> {
  errorMessage = null;

  try {
    await v2ChecklistApi.v2DeleteItem(id);
    items = items.filter((item) => item.id !== id);
    await refreshTags();
    await v2ReminderNotificationApi.cancelReminderForItem(id);
  } catch (error) {
    throw setError(error, 'Failed to delete v2 item.');
  }
}

async function moveItem(id: number, delta: number): Promise<void> {
  if (selectedCategoryId === null) return;

  const movedRaw = moveInList(items, id, delta);
  if (movedRaw === items) return;
  await reorderItems(movedRaw.map((item) => item.id));
}

async function reorderItems(itemIds: number[]): Promise<void> {
  if (selectedCategoryId === null) return;

  const itemsById = new Map(items.map((item) => [item.id, item]));
  const orderedItems = itemIds
    .map((id) => itemsById.get(id))
    .filter((item): item is V2TodoItem => item !== undefined);
  if (orderedItems.length !== items.length) return;

  const moved = reindexItems(orderedItems);

  errorMessage = null;
  try {
    await v2ChecklistApi.v2ReorderItems(
      selectedCategoryId,
      moved.map((item) => item.id)
    );
    items = sortItems(moved);
  } catch (error) {
    await loadItemsForSelectedCategory();
    throw setError(error, 'Failed to reorder v2 items.');
  }
}

export const v2ChecklistStore = {
  get categories() {
    return categories;
  },
  get items() {
    return items;
  },
  get tags() {
    return tags;
  },
  get selectedCategoryId() {
    return selectedCategoryId;
  },
  get isLoading() {
    return isLoading;
  },
  get errorMessage() {
    return errorMessage;
  },
  load,
  selectCategory,
  addCategory,
  updateCategory,
  deleteCategory,
  moveCategory,
  reorderCategories,
  addItem,
  searchItems,
  updateItemText,
  updateItemDetails,
  getStreakHeatmaps,
  toggleItem,
  processRepeatsAndReload,
  archiveCompletedItems,
  scheduleRepeatProcessing,
  disposeRepeatProcessingTimer,
  deleteItem,
  moveItem,
  reorderItems
};
