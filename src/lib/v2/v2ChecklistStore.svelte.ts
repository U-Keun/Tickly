import type { V2Category, V2ItemSearchResult, V2TodoItem } from '../../types';
import * as v2ChecklistApi from '../api/v2ChecklistApi';

let categories = $state<V2Category[]>([]);
let items = $state<V2TodoItem[]>([]);
let selectedCategoryId = $state<number | null>(null);
let isLoading = $state(false);
let errorMessage = $state<string | null>(null);

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

async function loadItemsForSelectedCategory(): Promise<void> {
  if (selectedCategoryId === null) {
    items = [];
    return;
  }

  items = sortItems(await v2ChecklistApi.v2GetItems(selectedCategoryId));
}

async function load(): Promise<void> {
  isLoading = true;
  errorMessage = null;

  try {
    const nextCategories = sortCategories(await v2ChecklistApi.v2GetCategories());
    categories = nextCategories;

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

async function addItem(text: string): Promise<void> {
  if (selectedCategoryId === null) return;
  errorMessage = null;

  try {
    const item = await v2ChecklistApi.v2CreateItem(selectedCategoryId, text);
    items = sortItems([...items, item]);
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
  memo: string | null
): Promise<void> {
  errorMessage = null;

  try {
    await v2ChecklistApi.v2UpdateItemDetails(id, text, memo);
    const trimmedText = text.trim();
    const trimmedMemo = memo?.trim() ?? '';
    const normalizedMemo = trimmedMemo ? trimmedMemo : null;
    items = items.map((item) =>
      item.id === id
        ? { ...item, text: trimmedText, memo: normalizedMemo }
        : item
    );
  } catch (error) {
    throw setError(error, 'Failed to update v2 item details.');
  }
}

async function toggleItem(id: number): Promise<void> {
  errorMessage = null;

  try {
    const updatedItem = await v2ChecklistApi.v2ToggleItem(id);
    items = sortItems(items.map((item) => (item.id === id ? updatedItem : item)));
  } catch (error) {
    throw setError(error, 'Failed to toggle v2 item.');
  }
}

async function deleteItem(id: number): Promise<void> {
  errorMessage = null;

  try {
    await v2ChecklistApi.v2DeleteItem(id);
    items = items.filter((item) => item.id !== id);
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
  toggleItem,
  deleteItem,
  moveItem,
  reorderItems
};
