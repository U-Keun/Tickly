import { invoke } from './client';
import type { V2Category, V2TodoItem } from '../../types';

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

export async function v2CreateItem(
  categoryId: number,
  text: string
): Promise<V2TodoItem> {
  return invoke<V2TodoItem>('v2_create_item', { categoryId, text });
}

export async function v2UpdateItemText(id: number, text: string): Promise<void> {
  return invoke<void>('v2_update_item_text', { id, text });
}

export async function v2ToggleItem(id: number): Promise<V2TodoItem> {
  return invoke<V2TodoItem>('v2_toggle_item', { id });
}

export async function v2DeleteItem(id: number): Promise<void> {
  return invoke<void>('v2_delete_item', { id });
}

export async function v2ReorderItems(categoryId: number, itemIds: number[]): Promise<void> {
  return invoke<void>('v2_reorder_items', { categoryId, itemIds });
}
