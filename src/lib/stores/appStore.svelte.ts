import type { TodoItem, Category, RepeatType, Tag } from '../../types';
import { createCategoryActions } from './appStoreCategoryActions';
import { createItemActions } from './appStoreItemActions';
import { createTagActions } from './appStoreTagActions';
import { syncStore } from './syncStore.svelte';
import * as todoApi from '../api/todoApi';

// Core app state
let items = $state<TodoItem[]>([]);
let categories = $state<Category[]>([]);
let selectedCategoryId = $state<number | null>(null);

// Tag state
let allTags = $state<Tag[]>([]);
let activeTagFilter = $state<number | null>(null);
let filteredItems = $state<TodoItem[]>([]);
let itemTagsMap = $state<Record<number, Tag[]>>({});

async function finalizeMutation(): Promise<void> {
  syncStore.scheduleSync();
}

const tagActions = createTagActions({
  getItemTagsMap: () => itemTagsMap,
  setItemTagsMap: (nextMap) => {
    itemTagsMap = nextMap;
  },
  getAllTags: () => allTags,
  setAllTags: (nextTags) => {
    allTags = nextTags;
  },
  getActiveTagFilter: () => activeTagFilter,
  setActiveTagFilter: (nextFilter) => {
    activeTagFilter = nextFilter;
  },
  setFilteredItems: (nextItems) => {
    filteredItems = nextItems;
  },
  finalizeMutation
});

async function loadItems(): Promise<void> {
  try {
    // Check and perform auto-reset if needed before loading items
    await todoApi.checkAndAutoReset();
    items = await todoApi.getItems(selectedCategoryId);
  } catch (error) {
    console.error('Failed to load items:', error);
  }
}

const categoryActions = createCategoryActions({
  getCategories: () => categories,
  setCategories: (nextCategories) => {
    categories = nextCategories;
  },
  getSelectedCategoryId: () => selectedCategoryId,
  setSelectedCategoryId: (nextCategoryId) => {
    selectedCategoryId = nextCategoryId;
  },
  getItems: () => items,
  loadItems,
  loadTagsForItems: tagActions.loadTagsForItems,
  finalizeMutation
});

async function refreshAll(): Promise<void> {
  await categoryActions.loadCategories();

  // If no category selected or selected category doesn't exist, select the first one
  if (categories.length > 0) {
    const selectedExists = categories.some(c => c.id === selectedCategoryId);
    if (!selectedExists) {
      selectedCategoryId = categories[0].id;
    }
  }

  await loadItems();
  await tagActions.loadAllTags();
}

const itemActions = createItemActions({
  getItems: () => items,
  setItems: (nextItems) => {
    items = nextItems;
  },
  getSelectedCategoryId: () => selectedCategoryId,
  getItemTagsMap: () => itemTagsMap,
  setItemTagsMap: (nextMap) => {
    itemTagsMap = nextMap;
  },
  loadAllTags: tagActions.loadAllTags,
  refreshAll,
  finalizeMutation,
  scheduleSync: () => syncStore.scheduleSync()
});

// Export store with getters and actions
export const appStore = {
  // Getters (reactive)
  get items() { return items; },
  get categories() { return categories; },
  get selectedCategoryId() { return selectedCategoryId; },

  // Tag getters (reactive)
  get allTags() { return allTags; },
  get activeTagFilter() { return activeTagFilter; },
  get filteredItems() { return filteredItems; },
  get itemTagsMap() { return itemTagsMap; },

  // Data loading
  loadCategories: categoryActions.loadCategories,
  loadItems,
  refreshAll,

  // Category actions
  selectCategory: categoryActions.selectCategory,
  addCategory: categoryActions.addCategory,
  editCategory: categoryActions.editCategory,
  deleteCategory: categoryActions.deleteCategory,
  setCategories: categoryActions.setCategories,
  goToFirstCategory: categoryActions.goToFirstCategory,

  // Item actions
  addItem: itemActions.addItem,
  toggleItem: itemActions.toggleItem,
  deleteItem: itemActions.deleteItem,
  editItem: itemActions.editItem,
  updateMemo: itemActions.updateMemo,
  updateRepeat: itemActions.updateRepeat,
  updateTrackStreak: itemActions.updateTrackStreak,
  updateLinkedApp: itemActions.updateLinkedApp,
  updateReminder: itemActions.updateReminder,
  resetAllItems: itemActions.resetAllItems,
  setItems: itemActions.setItems,

  // Tag actions
  loadAllTags: tagActions.loadAllTags,
  loadTagsForItems: tagActions.loadTagsForItems,
  addTagToItem: tagActions.addTagToItem,
  removeTagFromItem: tagActions.removeTagFromItem,
  deleteTag: tagActions.deleteTag,
  setTagFilter: tagActions.setTagFilter,
  clearTagFilter: tagActions.clearTagFilter,
};
