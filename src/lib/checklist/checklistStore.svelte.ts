import type {
  Category,
  GraphData,
  ItemSearchResult,
  RepeatType,
  StreakHeatmap,
  Tag,
  TodoItem
} from '../../types';
import * as settingsApi from '../api/settingsApi';
import * as checklistApi from '../api/checklistApi';
import * as reminderNotificationApi from '../api/reminderNotificationApi';
import * as widgetApi from '../api/widgetApi';
import { icloudSyncStore } from './icloudSyncStore.svelte';

const MIN_REPEAT_TIMER_DELAY_MS = 1000;
const WIDGET_REFRESH_DEBOUNCE_MS = 300;

let categories = $state<Category[]>([]);
let items = $state<TodoItem[]>([]);
let tags = $state<Tag[]>([]);
let selectedCategoryId = $state<number | null>(null);
let isLoading = $state(false);
let errorMessage = $state<string | null>(null);
let repeatProcessingTimeout: ReturnType<typeof setTimeout> | null = null;
let repeatProcessingScheduleToken = 0;
let widgetRefreshTimeout: ReturnType<typeof setTimeout> | null = null;

function sortCategories(nextCategories: Category[]): Category[] {
  return [...nextCategories].sort((a, b) => a.display_order - b.display_order);
}

function sortItems(nextItems: TodoItem[]): TodoItem[] {
  return [...nextItems].sort((a, b) => {
    if (a.done !== b.done) {
      return a.done ? 1 : -1;
    }
    return a.display_order - b.display_order;
  });
}

function sortTags(nextTags: Tag[]): Tag[] {
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

function reindexCategories(nextCategories: Category[]): Category[] {
  return nextCategories.map((category, index) => ({
    ...category,
    display_order: (index + 1) * 1000
  }));
}

function reindexItems(nextItems: TodoItem[]): TodoItem[] {
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

function clearWidgetRefreshTimer(): void {
  if (widgetRefreshTimeout === null) return;
  clearTimeout(widgetRefreshTimeout);
  widgetRefreshTimeout = null;
}

async function refreshWidgetCacheQuietly(): Promise<void> {
  try {
    await widgetApi.refreshWidgetCache();
  } catch (error) {
    console.error('Failed to refresh widget cache.', error);
  }
}

function scheduleWidgetRefresh(): void {
  clearWidgetRefreshTimer();
  widgetRefreshTimeout = setTimeout(() => {
    widgetRefreshTimeout = null;
    void refreshWidgetCacheQuietly();
  }, WIDGET_REFRESH_DEBOUNCE_MS);
}

function finalizeLocalMutation(): void {
  scheduleWidgetRefresh();
  icloudSyncStore.scheduleSync();
}

async function loadItemsForSelectedCategory(): Promise<void> {
  if (selectedCategoryId === null) {
    items = [];
    return;
  }

  items = sortItems(await checklistApi.getItems(selectedCategoryId));
}

async function syncReminderNotifications(): Promise<void> {
  try {
    const reminderItems = await checklistApi.getActiveReminderItems();
    await reminderNotificationApi.syncActiveReminderNotifications(reminderItems);
  } catch (error) {
    console.error('Failed to sync reminder notifications.', error);
  }
}

async function load(): Promise<void> {
  isLoading = true;
  errorMessage = null;

  try {
    await checklistApi.processRepeats();
    const [nextCategoriesRaw, nextTagsRaw] = await Promise.all([
      checklistApi.getCategories(),
      checklistApi.getTags()
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
    void syncReminderNotifications();
    scheduleWidgetRefresh();
  } catch (error) {
    throw setError(error, 'Failed to load checklist.');
  } finally {
    isLoading = false;
  }
}

async function selectCategory(categoryId: number): Promise<void> {
  errorMessage = null;

  try {
    const nextItems = sortItems(await checklistApi.getItems(categoryId));
    selectedCategoryId = categoryId;
    items = nextItems;
  } catch (error) {
    throw setError(error, 'Failed to select category.');
  }
}

async function addCategory(name: string): Promise<void> {
  errorMessage = null;

  try {
    const category = await checklistApi.createCategory(name);
    categories = sortCategories([...categories, category]);
    selectedCategoryId = category.id;
    items = [];
    finalizeLocalMutation();
  } catch (error) {
    throw setError(error, 'Failed to add category.');
  }
}

async function updateCategory(id: number, name: string): Promise<void> {
  errorMessage = null;

  try {
    await checklistApi.updateCategory(id, name);
    const trimmedName = name.trim();
    categories = categories.map((category) =>
      category.id === id ? { ...category, name: trimmedName } : category
    );
    finalizeLocalMutation();
  } catch (error) {
    throw setError(error, 'Failed to update category.');
  }
}

async function deleteCategory(id: number): Promise<void> {
  errorMessage = null;

  try {
    const reminderItemIds = items
      .filter((item) => item.category_id === id && item.reminder_at)
      .map((item) => item.id);
    await checklistApi.deleteCategory(id);
    const nextCategories = categories.filter((category) => category.id !== id);
    categories = nextCategories;

    if (selectedCategoryId === id) {
      const nextSelectedCategoryId = nextCategories[0]?.id ?? null;
      const nextItems =
        nextSelectedCategoryId === null
          ? []
          : sortItems(await checklistApi.getItems(nextSelectedCategoryId));

      selectedCategoryId = nextSelectedCategoryId;
      items = nextItems;
    }
    await refreshTags();
    await Promise.all(
      reminderItemIds.map((itemId) => reminderNotificationApi.cancelReminderForItem(itemId))
    );
    await syncReminderNotifications();
    finalizeLocalMutation();
  } catch (error) {
    throw setError(error, 'Failed to delete category.');
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
    .filter((category): category is Category => category !== undefined);
  if (orderedCategories.length !== categories.length) return;

  const moved = reindexCategories(orderedCategories);

  errorMessage = null;
  try {
    await checklistApi.reorderCategories(moved.map((category) => category.id));
    categories = moved;
    finalizeLocalMutation();
  } catch (error) {
    await load();
    throw setError(error, 'Failed to reorder categories.');
  }
}

async function refreshTags(): Promise<void> {
  tags = sortTags(await checklistApi.getTags());
}

async function addItem(text: string, tagNames: string[] = []): Promise<void> {
  if (selectedCategoryId === null) return;
  errorMessage = null;

  try {
    const item = await checklistApi.createItem(selectedCategoryId, text, tagNames);
    items = sortItems([...items, item]);
    await refreshTags();
    finalizeLocalMutation();
  } catch (error) {
    throw setError(error, 'Failed to add item.');
  }
}

async function searchItems(query: string, limit: number): Promise<ItemSearchResult[]> {
  errorMessage = null;

  try {
    return await checklistApi.searchItems(query, limit);
  } catch (error) {
    throw setError(error, 'Failed to search items.');
  }
}

async function updateItemText(id: number, text: string): Promise<void> {
  errorMessage = null;

  try {
    await checklistApi.updateItemText(id, text);
    const trimmedText = text.trim();
    items = items.map((item) => (item.id === id ? { ...item, text: trimmedText } : item));
    finalizeLocalMutation();
  } catch (error) {
    throw setError(error, 'Failed to update item.');
  }
}

async function updateItemDetails(
  id: number,
  text: string,
  memo: string | null,
  tagNames: string[] = [],
  repeatType: RepeatType = 'none',
  repeatDetail: string | null = null,
  reminderAt: string | null = null,
  trackStreak?: boolean
): Promise<void> {
  errorMessage = null;

  try {
    const nextTrackStreak =
      trackStreak ?? items.find((item) => item.id === id)?.track_streak ?? false;
    const normalizedTrackStreak = repeatType !== 'none' && nextTrackStreak;
    const updatedItem = await checklistApi.updateItemDetails(
      id,
      text,
      memo,
      tagNames,
      repeatType,
      repeatDetail,
      reminderAt,
      normalizedTrackStreak
    );
    items = sortItems(items.map((item) => (item.id === id ? updatedItem : item)));
    await refreshTags();
    await reminderNotificationApi.syncReminderForItem(updatedItem);
    finalizeLocalMutation();
  } catch (error) {
    throw setError(error, 'Failed to update item details.');
  }
}

async function getStreakHeatmaps(): Promise<StreakHeatmap[]> {
  errorMessage = null;

  try {
    return await checklistApi.getStreakHeatmaps();
  } catch (error) {
    throw setError(error, 'Failed to load streak heatmaps.');
  }
}

async function getGraphData(): Promise<GraphData> {
  errorMessage = null;

  try {
    return await checklistApi.getGraphData();
  } catch (error) {
    throw setError(error, 'Failed to load graph data.');
  }
}

async function toggleItem(id: number): Promise<TodoItem> {
  errorMessage = null;

  try {
    const updatedItem = await checklistApi.toggleItem(id);
    items = sortItems(items.map((item) => (item.id === id ? updatedItem : item)));
    await reminderNotificationApi.syncReminderForItem(updatedItem);
    finalizeLocalMutation();
    return updatedItem;
  } catch (error) {
    throw setError(error, 'Failed to toggle item.');
  }
}

async function processRepeatsAndReload(): Promise<number> {
  errorMessage = null;

  try {
    const reactivatedCount = await checklistApi.processRepeats();
    if (reactivatedCount > 0) {
      await loadItemsForSelectedCategory();
      finalizeLocalMutation();
    }
    await syncReminderNotifications();
    return reactivatedCount;
  } catch (error) {
    throw setError(error, 'Failed to process repeats.');
  }
}

async function archiveCompletedItems(categoryId: number): Promise<number> {
  errorMessage = null;

  try {
    const archivedCount = await checklistApi.archiveCompletedItems(categoryId);
    if (archivedCount > 0 && selectedCategoryId === categoryId) {
      await loadItemsForSelectedCategory();
    }
    if (archivedCount > 0) {
      finalizeLocalMutation();
    }
    return archivedCount;
  } catch (error) {
    throw setError(error, 'Failed to archive completed items.');
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
    console.error('Failed to schedule repeat processing.', error);
  }
}

function disposeRepeatProcessingTimer(): void {
  repeatProcessingScheduleToken += 1;
  clearRepeatProcessingTimer();
}

function disposeWidgetRefreshTimer(): void {
  clearWidgetRefreshTimer();
}

async function processWidgetActionsAndReload(): Promise<number> {
  errorMessage = null;

  try {
    const processedCount = await widgetApi.processWidgetActions();
    if (processedCount > 0) {
      await load();
      scheduleWidgetRefresh();
    }
    return processedCount;
  } catch (error) {
    console.error('Failed to process widget actions.', error);
    return 0;
  }
}

async function toggleItemFromWidget(id: number): Promise<void> {
  errorMessage = null;

  try {
    await widgetApi.toggleItemFromWidget(id);
    await load();
    scheduleWidgetRefresh();
  } catch (error) {
    throw setError(error, 'Failed to toggle item from widget.');
  }
}

async function deleteItem(id: number): Promise<void> {
  errorMessage = null;

  try {
    await checklistApi.deleteItem(id);
    items = items.filter((item) => item.id !== id);
    await refreshTags();
    await reminderNotificationApi.cancelReminderForItem(id);
    finalizeLocalMutation();
  } catch (error) {
    throw setError(error, 'Failed to delete item.');
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
    .filter((item): item is TodoItem => item !== undefined);
  if (orderedItems.length !== items.length) return;

  const moved = reindexItems(orderedItems);

  errorMessage = null;
  try {
    await checklistApi.reorderItems(
      selectedCategoryId,
      moved.map((item) => item.id)
    );
    items = sortItems(moved);
    finalizeLocalMutation();
  } catch (error) {
    await loadItemsForSelectedCategory();
    throw setError(error, 'Failed to reorder items.');
  }
}

export const checklistStore = {
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
  getGraphData,
  toggleItem,
  processRepeatsAndReload,
  archiveCompletedItems,
  scheduleRepeatProcessing,
  disposeRepeatProcessingTimer,
  disposeWidgetRefreshTimer,
  refreshWidgetCache: refreshWidgetCacheQuietly,
  processWidgetActions: processWidgetActionsAndReload,
  toggleItemFromWidget,
  deleteItem,
  moveItem,
  reorderItems
};
