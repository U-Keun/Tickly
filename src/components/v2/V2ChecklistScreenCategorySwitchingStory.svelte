<script lang="ts">
  import type { V2Category, V2ItemSearchResult, V2RepeatType, V2Tag, V2TodoItem } from '../../types';
  import V2ChecklistScreen from './V2ChecklistScreen.svelte';

  interface Props {
    initialOpenDrawerItemIds?: number[];
  }

  let { initialOpenDrawerItemIds = [] }: Props = $props();

  const now = '2026-06-08T00:00:00Z';
  const orderStep = 1000;
  const repeatDefaults = {
    repeat_type: 'none' as const,
    repeat_detail: null,
    next_due_at: null,
    last_completed_at: null,
    reminder_at: null,
    archived_at: null
  };
  const initialTags: V2Tag[] = [
    { id: 1, name: 'home', created_at: now, updated_at: now },
    { id: 2, name: 'travel', created_at: now, updated_at: now },
    { id: 3, name: 'work', created_at: now, updated_at: now }
  ];

  const initialCategories: V2Category[] = [
    {
      id: 1,
      name: 'Home',
      display_order: 1000,
      created_at: now,
      updated_at: now
    },
    {
      id: 2,
      name: 'Travel',
      display_order: 2000,
      created_at: now,
      updated_at: now
    },
    {
      id: 3,
      name: 'Work',
      display_order: 3000,
      created_at: now,
      updated_at: now
    },
    {
      id: 4,
      name: 'Groceries',
      display_order: 4000,
      created_at: now,
      updated_at: now
    }
  ];

  const initialItemsByCategory: Record<number, V2TodoItem[]> = {
    1: [
      {
        id: 1,
        category_id: 1,
        text: 'Wallet',
        memo: 'Keep this in the small front pocket.',
        tags: [initialTags[0]],
        ...repeatDefaults,
        done: false,
        display_order: 1000,
        created_at: now,
        updated_at: now
      },
      {
        id: 2,
        category_id: 1,
        text: 'Umbrella before leaving for a very long commute day',
        memo: null,
        tags: [initialTags[1]],
        ...repeatDefaults,
        done: false,
        display_order: 2000,
        created_at: now,
        updated_at: now
      }
    ],
    2: [
      {
        id: 3,
        category_id: 2,
        text: 'Passport',
        memo: 'Check the hotel booking folder before packing.',
        tags: [initialTags[1]],
        ...repeatDefaults,
        done: false,
        display_order: 1000,
        created_at: now,
        updated_at: now
      },
      {
        id: 4,
        category_id: 2,
        text: 'Portable charger',
        memo: null,
        tags: [initialTags[1]],
        ...repeatDefaults,
        done: true,
        display_order: 2000,
        created_at: now,
        updated_at: now
      }
    ],
    3: [
      {
        id: 5,
        category_id: 3,
        text: 'Review v2 category surface',
        memo: null,
        tags: [initialTags[2]],
        ...repeatDefaults,
        done: false,
        display_order: 1000,
        created_at: now,
        updated_at: now
      }
    ],
    4: []
  };

  let categories = $state<V2Category[]>(initialCategories);
  let selectedCategoryId = $state<number | null>(1);
  let itemsByCategory = $state<Record<number, V2TodoItem[]>>(initialItemsByCategory);
  let availableTags = $state<V2Tag[]>(initialTags);
  let nextCategoryId = $state(5);
  let nextItemId = $state(6);
  let nextTagId = $state(4);

  let items = $derived(selectedCategoryId === null ? [] : (itemsByCategory[selectedCategoryId] ?? []));

  function reindexCategories(nextCategories: V2Category[]): V2Category[] {
    return nextCategories.map((category, index) => ({
      ...category,
      display_order: (index + 1) * orderStep
    }));
  }

  function reindexItems(nextItems: V2TodoItem[]): V2TodoItem[] {
    return nextItems.map((item, index) => ({
      ...item,
      display_order: (index + 1) * orderStep
    }));
  }

  function setItemsForCategory(categoryId: number, nextItems: V2TodoItem[]): void {
    itemsByCategory = {
      ...itemsByCategory,
      [categoryId]: reindexItems(nextItems)
    };
  }

  async function selectCategory(id: number): Promise<void> {
    selectedCategoryId = id;
  }

  async function addCategory(name: string): Promise<void> {
    const category: V2Category = {
      id: nextCategoryId,
      name: name.trim(),
      display_order: (categories.length + 1) * orderStep,
      created_at: now,
      updated_at: now
    };

    nextCategoryId += 1;
    categories = [...categories, category];
    itemsByCategory = { ...itemsByCategory, [category.id]: [] };
    selectedCategoryId = category.id;
  }

  async function updateCategory(id: number, name: string): Promise<void> {
    categories = categories.map((category) =>
      category.id === id ? { ...category, name: name.trim(), updated_at: now } : category
    );
  }

  async function deleteCategory(id: number): Promise<void> {
    if (categories.length <= 1) return;

    const nextCategories = categories.filter((category) => category.id !== id);
    const { [id]: _deletedItems, ...nextItemsByCategory } = itemsByCategory;
    categories = reindexCategories(nextCategories);
    itemsByCategory = nextItemsByCategory;

    if (selectedCategoryId === id) {
      selectedCategoryId = categories[0]?.id ?? null;
    }
  }

  async function reorderCategories(categoryIds: number[]): Promise<void> {
    const categoriesById = new Map(categories.map((category) => [category.id, category]));
    const nextCategories = categoryIds
      .map((id) => categoriesById.get(id))
      .filter((category): category is V2Category => category !== undefined);
    if (nextCategories.length !== categories.length) return;

    categories = reindexCategories(nextCategories);
  }

  function resolveTags(tagNames: string[]): V2Tag[] {
    const nextTags = [...availableTags];
    const resolvedTags = tagNames.map((name) => {
      const existingTag = nextTags.find(
        (tag) => tag.name.toLocaleLowerCase() === name.toLocaleLowerCase()
      );
      if (existingTag) return existingTag;

      const tag: V2Tag = {
        id: nextTagId,
        name,
        created_at: now,
        updated_at: now
      };
      nextTagId += 1;
      nextTags.push(tag);
      return tag;
    });
    availableTags = nextTags;
    return resolvedTags;
  }

  async function addItem(text: string, tagNames: string[] = []): Promise<void> {
    if (selectedCategoryId === null) return;

    const currentItems = itemsByCategory[selectedCategoryId] ?? [];
    const item: V2TodoItem = {
      id: nextItemId,
      category_id: selectedCategoryId,
      text: text.trim(),
      memo: null,
      tags: resolveTags(tagNames),
      ...repeatDefaults,
      done: false,
      display_order: (currentItems.length + 1) * orderStep,
      created_at: now,
      updated_at: now
    };

    nextItemId += 1;
    setItemsForCategory(selectedCategoryId, [...currentItems, item]);
  }

  async function toggleItem(id: number): Promise<void> {
    if (selectedCategoryId === null) return;

    setItemsForCategory(
      selectedCategoryId,
      items.map((item) => (item.id === id ? { ...item, done: !item.done, updated_at: now } : item))
    );
  }

  async function updateItemDetails(
    id: number,
    text: string,
    memo: string | null,
    tagNames: string[] = [],
    repeatType: V2RepeatType = 'none',
    repeatDetail: string | null = null,
    reminderAt: string | null = null
  ): Promise<void> {
    if (selectedCategoryId === null) return;

    setItemsForCategory(
      selectedCategoryId,
      items.map((item) =>
        item.id === id
          ? {
              ...item,
              text: text.trim(),
              memo: memo?.trim() || null,
              tags: resolveTags(tagNames),
              repeat_type: repeatType,
              repeat_detail: repeatDetail,
              reminder_at: reminderAt,
              next_due_at: null,
              updated_at: now
            }
          : item
      )
    );
  }

  async function deleteItem(id: number): Promise<void> {
    if (selectedCategoryId === null) return;

    setItemsForCategory(
      selectedCategoryId,
      items.filter((item) => item.id !== id)
    );
  }

  async function reorderItems(itemIds: number[]): Promise<void> {
    if (selectedCategoryId === null) return;

    const itemsById = new Map(items.map((item) => [item.id, item]));
    const nextItems = itemIds
      .map((id) => itemsById.get(id))
      .filter((item): item is V2TodoItem => item !== undefined);
    if (nextItems.length !== items.length) return;

    setItemsForCategory(selectedCategoryId, nextItems);
  }

  async function archiveCompletedItems(categoryId: number): Promise<number> {
    const currentItems = itemsByCategory[categoryId] ?? [];
    const archivableIds = new Set(
      currentItems
        .filter((item) => item.done && item.repeat_type === 'none' && item.archived_at === null)
        .map((item) => item.id)
    );

    if (archivableIds.size === 0) return 0;

    setItemsForCategory(
      categoryId,
      currentItems.filter((item) => !archivableIds.has(item.id))
    );
    return archivableIds.size;
  }

  async function searchItems(query: string, limit: number): Promise<V2ItemSearchResult[]> {
    const normalizedQuery = query.trim().toLocaleLowerCase();
    if (!normalizedQuery) return [];

    const results = categories.flatMap((category) =>
      (itemsByCategory[category.id] ?? [])
        .filter(
          (item) =>
            item.text.toLocaleLowerCase().includes(normalizedQuery) ||
            (item.memo ?? '').toLocaleLowerCase().includes(normalizedQuery) ||
            item.tags.some((tag) => tag.name.toLocaleLowerCase().includes(normalizedQuery))
        )
        .map((item) => ({ item, category }))
    );

    return results.slice(0, limit);
  }
</script>

<V2ChecklistScreen
  {categories}
  {selectedCategoryId}
  {items}
  {availableTags}
  errorMessage={null}
  {initialOpenDrawerItemIds}
  onSelectCategory={selectCategory}
  onAddCategory={addCategory}
  onUpdateCategory={updateCategory}
  onDeleteCategory={deleteCategory}
  onReorderCategories={reorderCategories}
  onAddItem={addItem}
  onToggleItem={toggleItem}
  onUpdateItemDetails={updateItemDetails}
  onDeleteItem={deleteItem}
  onReorderItems={reorderItems}
  onArchiveCompletedItems={archiveCompletedItems}
  onSearchItems={searchItems}
/>
