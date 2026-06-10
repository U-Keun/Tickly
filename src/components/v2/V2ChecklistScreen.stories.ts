import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2Category, V2ItemSearchResult, V2Tag, V2TodoItem } from '../../types';
import V2ChecklistScreen from './V2ChecklistScreen.svelte';

const now = '2026-06-08T00:00:00Z';

const tags: V2Tag[] = [
  { id: 1, name: 'home', created_at: now, updated_at: now },
  { id: 2, name: 'travel', created_at: now, updated_at: now },
  { id: 3, name: 'church', created_at: now, updated_at: now }
];

const categories: V2Category[] = [
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
  }
];

const items: V2TodoItem[] = [
  {
    id: 1,
    category_id: 1,
    text: 'Wallet',
    memo: 'Keep this in the front pocket.',
    tags: [tags[0]],
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
    tags: [tags[1]],
    done: false,
    display_order: 2000,
    created_at: now,
    updated_at: now
  }
];

const searchResults: V2ItemSearchResult[] = [
  {
    item: items[0],
    category: categories[0]
  },
  {
    item: {
      id: 6,
      category_id: 2,
      text: 'Travel wallet pouch',
      memo: null,
      tags: [tags[1]],
      done: false,
      display_order: 1000,
      created_at: now,
      updated_at: now
    },
    category: categories[1]
  }
];

const meta = {
  title: 'v2/V2ChecklistScreen',
  component: V2ChecklistScreen,
  tags: ['autodocs'],
  args: {
    categories,
    selectedCategoryId: 1,
    items,
    availableTags: tags,
    errorMessage: null,
    onSelectCategory: () => {},
    onAddCategory: async () => {},
    onUpdateCategory: async () => {},
    onDeleteCategory: async () => {},
    onReorderCategories: async () => {},
    onAddItem: async () => {},
    onToggleItem: async () => {},
    onUpdateItemDetails: async () => {},
    onDeleteItem: async () => {},
    onReorderItems: async () => {},
    onSearchItems: async (query: string, limit: number) =>
      searchResults
        .filter((result) => {
          const normalizedQuery = query.toLocaleLowerCase();
          return (
            result.item.text.toLocaleLowerCase().includes(normalizedQuery) ||
            (result.item.memo ?? '').toLocaleLowerCase().includes(normalizedQuery) ||
            result.item.tags.some((tag) => tag.name.toLocaleLowerCase().includes(normalizedQuery))
          );
        })
        .slice(0, limit)
  }
} satisfies Meta<typeof V2ChecklistScreen>;

export default meta;

type Story = StoryObj<typeof meta>;

export const DefaultList: Story = {};

export const EmptyList: Story = {
  args: {
    items: []
  }
};

export const CompletedItems: Story = {
  args: {
    items: [
      ...items,
      {
        id: 3,
        category_id: 1,
        text: 'Keys',
        memo: null,
        tags: [],
        done: true,
        display_order: 3000,
        created_at: now,
        updated_at: now
      }
    ]
  }
};

export const LongTextItems: Story = {
  args: {
    items: [
      {
        id: 10,
        category_id: 1,
        text: 'Umbrella before leaving for a very long commute day with errands after work and grocery pickup',
        memo: null,
        tags: [tags[1]],
        done: false,
        display_order: 1000,
        created_at: now,
        updated_at: now
      },
      {
        id: 11,
        category_id: 1,
        text: 'SuperLongUnbrokenChecklistItemNameThatShouldNeverPushTheCardWiderThanTheiPhoneViewportEvenWhenTypedWithoutSpaces',
        memo: null,
        tags: [tags[2]],
        done: false,
        display_order: 2000,
        created_at: now,
        updated_at: now
      }
    ]
  }
};

export const SearchActive: Story = {
  args: {
    initialSearchMode: true,
    initialSearchQuery: 'wallet'
  }
};

export const MemoSearch: Story = {
  args: {
    initialSearchMode: true,
    initialSearchQuery: 'front',
    items
  }
};

export const CategoryReorderMode: Story = {
  args: {
    initialCategoryReorderMode: true
  }
};

export const LongListDragReady: Story = {
  args: {
    items: [
      ...items,
      {
        id: 3,
        category_id: 1,
        text: 'Keys',
        memo: null,
        tags: [],
        done: false,
        display_order: 3000,
        created_at: now,
        updated_at: now
      },
      {
        id: 4,
        category_id: 1,
        text: 'Water bottle',
        memo: null,
        tags: [tags[0]],
        done: false,
        display_order: 4000,
        created_at: now,
        updated_at: now
      },
      {
        id: 5,
        category_id: 1,
        text: 'Portable charger',
        memo: null,
        tags: [tags[1]],
        done: true,
        display_order: 5000,
        created_at: now,
        updated_at: now
      }
    ]
  }
};
