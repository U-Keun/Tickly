import { invoke } from './client';
import type {
  ArchivedItem,
  Category,
  GraphData,
  ItemSearchResult,
  RepeatType,
  StreakHeatmap,
  Tag,
  TagSummary,
  TodoItem
} from '../../types';

export async function getCategories(): Promise<Category[]> {
  return invoke<Category[]>('v2_get_categories');
}

export async function createCategory(name: string): Promise<Category> {
  return invoke<Category>('v2_create_category', { name });
}

export async function updateCategory(id: number, name: string): Promise<void> {
  return invoke<void>('v2_update_category', { id, name });
}

export async function deleteCategory(id: number): Promise<void> {
  return invoke<void>('v2_delete_category', { id });
}

export async function reorderCategories(categoryIds: number[]): Promise<void> {
  return invoke<void>('v2_reorder_categories', { categoryIds });
}

export async function getItems(categoryId: number): Promise<TodoItem[]> {
  return invoke<TodoItem[]>('v2_get_items', { categoryId });
}

export async function getActiveReminderItems(): Promise<TodoItem[]> {
  return invoke<TodoItem[]>('v2_get_active_reminder_items');
}

export async function getTags(): Promise<Tag[]> {
  return invoke<Tag[]>('v2_get_tags');
}

export async function getTagSummaries(): Promise<TagSummary[]> {
  return invoke<TagSummary[]>('v2_get_tag_summaries');
}

export async function renameTag(id: number, name: string): Promise<Tag> {
  return invoke<Tag>('v2_rename_tag', { id, name });
}

export async function deleteTag(id: number): Promise<void> {
  return invoke<void>('v2_delete_tag', { id });
}

export async function searchItems(
  query: string,
  limit: number
): Promise<ItemSearchResult[]> {
  return invoke<ItemSearchResult[]>('v2_search_items', { query, limit });
}

export async function createItem(
  categoryId: number,
  text: string,
  tagNames: string[] = []
): Promise<TodoItem> {
  return invoke<TodoItem>('v2_create_item', { categoryId, text, tagNames });
}

export async function updateItemText(id: number, text: string): Promise<void> {
  return invoke<void>('v2_update_item_text', { id, text });
}

export async function updateItemDetails(
  id: number,
  text: string,
  memo: string | null,
  tagNames: string[] = [],
  repeatType: RepeatType = 'none',
  repeatDetail: string | null = null,
  reminderAt: string | null = null,
  trackStreak = false
): Promise<TodoItem> {
  return invoke<TodoItem>('v2_update_item_details', {
    id,
    text,
    memo,
    tagNames,
    repeatType,
    repeatDetail,
    reminderAt,
    trackStreak
  });
}

export async function toggleItem(id: number): Promise<TodoItem> {
  return invoke<TodoItem>('v2_toggle_item', { id });
}

export async function processRepeats(): Promise<number> {
  return invoke<number>('v2_process_repeats');
}

export async function archiveCompletedItems(categoryId: number): Promise<number> {
  return invoke<number>('v2_archive_completed_items', { categoryId });
}

export async function getArchivedItems(): Promise<ArchivedItem[]> {
  return invoke<ArchivedItem[]>('v2_get_archived_items');
}

export async function restoreArchivedItem(id: number): Promise<TodoItem> {
  return invoke<TodoItem>('v2_restore_archived_item', { id });
}

export async function deleteArchivedItem(id: number): Promise<void> {
  return invoke<void>('v2_delete_archived_item', { id });
}

export async function getStreakHeatmaps(): Promise<StreakHeatmap[]> {
  return invoke<StreakHeatmap[]>('v2_get_streak_heatmaps');
}

export async function getGraphData(): Promise<GraphData> {
  return invoke<GraphData>('v2_get_graph_data');
}

export async function deleteItem(id: number): Promise<void> {
  return invoke<void>('v2_delete_item', { id });
}

export async function reorderItems(categoryId: number, itemIds: number[]): Promise<void> {
  return invoke<void>('v2_reorder_items', { categoryId, itemIds });
}
