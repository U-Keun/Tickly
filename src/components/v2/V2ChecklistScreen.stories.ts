import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2Category, V2ItemSearchResult, V2TodoItem } from '../../types';
import V2ChecklistScreen from './V2ChecklistScreen.svelte';

const categories: V2Category[] = [
  {
    id: 1,
    name: 'Home',
    display_order: 1000,
    created_at: '2026-06-08T00:00:00Z',
    updated_at: '2026-06-08T00:00:00Z'
  },
  {
    id: 2,
    name: 'Travel',
    display_order: 2000,
    created_at: '2026-06-08T00:00:00Z',
    updated_at: '2026-06-08T00:00:00Z'
  },
  {
    id: 3,
    name: 'Work',
    display_order: 3000,
    created_at: '2026-06-08T00:00:00Z',
    updated_at: '2026-06-08T00:00:00Z'
  }
];

const items: V2TodoItem[] = [
  {
    id: 1,
    category_id: 1,
    text: 'Wallet',
    memo: 'Keep this in the front pocket.',
    done: false,
    display_order: 1000,
    created_at: '2026-06-08T00:00:00Z',
    updated_at: '2026-06-08T00:00:00Z'
  },
  {
    id: 2,
    category_id: 1,
    text: 'Umbrella before leaving for a very long commute day',
    memo: null,
    done: false,
    display_order: 2000,
    created_at: '2026-06-08T00:00:00Z',
    updated_at: '2026-06-08T00:00:00Z'
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
      done: false,
      display_order: 1000,
      created_at: '2026-06-08T00:00:00Z',
      updated_at: '2026-06-08T00:00:00Z'
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
            (result.item.memo ?? '').toLocaleLowerCase().includes(normalizedQuery)
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
        done: true,
        display_order: 3000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
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
        done: false,
        display_order: 1000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
      },
      {
        id: 11,
        category_id: 1,
        text: 'SuperLongUnbrokenChecklistItemNameThatShouldNeverPushTheCardWiderThanTheiPhoneViewportEvenWhenTypedWithoutSpaces',
        memo: null,
        done: false,
        display_order: 2000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
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
        done: false,
        display_order: 3000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
      },
      {
        id: 4,
        category_id: 1,
        text: 'Water bottle',
        memo: null,
        done: false,
        display_order: 4000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
      },
      {
        id: 5,
        category_id: 1,
        text: 'Portable charger',
        memo: null,
        done: true,
        display_order: 5000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
      }
    ]
  }
};
