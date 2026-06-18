import { invoke } from './client';
import type {
  V2ArchivedItem,
  V2Category,
  V2GraphData,
  V2ItemSearchResult,
  V2RepeatType,
  V2StreakHeatmap,
  V2Tag,
  V2TodoItem
} from '../../types';

export async function v2GetCategories(): Promise<V2Category[]> {
  return invoke<V2Category[]>('v2_get_categories');
}

export async function v2CreateCategory(name: string): Promise<V2Category> {
  return invoke<V2Category>('v2_create_category', { name });
}

export async function v2UpdateCategory(id: number, name: string): Promise<void> {
  return invoke<void>('v2_update_category', { id, name });
}

export async function v2DeleteCategory(id: number): Promise<void> {
  return invoke<void>('v2_delete_category', { id });
}

export async function v2ReorderCategories(categoryIds: number[]): Promise<void> {
  return invoke<void>('v2_reorder_categories', { categoryIds });
}

export async function v2GetItems(categoryId: number): Promise<V2TodoItem[]> {
  return invoke<V2TodoItem[]>('v2_get_items', { categoryId });
}

export async function v2GetActiveReminderItems(): Promise<V2TodoItem[]> {
  return invoke<V2TodoItem[]>('v2_get_active_reminder_items');
}

export async function v2GetTags(): Promise<V2Tag[]> {
  return invoke<V2Tag[]>('v2_get_tags');
}

export async function v2SearchItems(
  query: string,
  limit: number
): Promise<V2ItemSearchResult[]> {
  return invoke<V2ItemSearchResult[]>('v2_search_items', { query, limit });
}

export async function v2CreateItem(
  categoryId: number,
  text: string,
  tagNames: string[] = []
): Promise<V2TodoItem> {
  return invoke<V2TodoItem>('v2_create_item', { categoryId, text, tagNames });
}

export async function v2UpdateItemText(id: number, text: string): Promise<void> {
  return invoke<void>('v2_update_item_text', { id, text });
}

export async function v2UpdateItemDetails(
  id: number,
  text: string,
  memo: string | null,
  tagNames: string[] = [],
  repeatType: V2RepeatType = 'none',
  repeatDetail: string | null = null,
  reminderAt: string | null = null,
  trackStreak = false
): Promise<V2TodoItem> {
  return invoke<V2TodoItem>('v2_update_item_details', {
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

export async function v2ToggleItem(id: number): Promise<V2TodoItem> {
  return invoke<V2TodoItem>('v2_toggle_item', { id });
}

export async function v2ProcessRepeats(): Promise<number> {
  return invoke<number>('v2_process_repeats');
}

export async function v2ArchiveCompletedItems(categoryId: number): Promise<number> {
  return invoke<number>('v2_archive_completed_items', { categoryId });
}

export async function v2GetArchivedItems(): Promise<V2ArchivedItem[]> {
  return invoke<V2ArchivedItem[]>('v2_get_archived_items');
}

export async function v2RestoreArchivedItem(id: number): Promise<V2TodoItem> {
  return invoke<V2TodoItem>('v2_restore_archived_item', { id });
}

export async function v2DeleteArchivedItem(id: number): Promise<void> {
  return invoke<void>('v2_delete_archived_item', { id });
}

export async function v2GetStreakHeatmaps(): Promise<V2StreakHeatmap[]> {
  return invoke<V2StreakHeatmap[]>('v2_get_streak_heatmaps');
}

export async function v2GetGraphData(): Promise<V2GraphData> {
  return invoke<V2GraphData>('v2_get_graph_data');
}

export async function v2DeleteItem(id: number): Promise<void> {
  return invoke<void>('v2_delete_item', { id });
}

export async function v2ReorderItems(categoryId: number, itemIds: number[]): Promise<void> {
  return invoke<void>('v2_reorder_items', { categoryId, itemIds });
}
