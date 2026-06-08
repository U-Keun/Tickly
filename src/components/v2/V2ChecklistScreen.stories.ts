import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2Category, V2TodoItem } from '../../types';
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
    done: false,
    display_order: 1000,
    created_at: '2026-06-08T00:00:00Z',
    updated_at: '2026-06-08T00:00:00Z'
  },
  {
    id: 2,
    category_id: 1,
    text: 'Umbrella before leaving for a very long commute day',
    done: false,
    display_order: 2000,
    created_at: '2026-06-08T00:00:00Z',
    updated_at: '2026-06-08T00:00:00Z'
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
    isLoading: false,
    errorMessage: null,
    initialReorderMode: false,
    onBackHome: () => {},
    onRefresh: () => {},
    onSelectCategory: () => {},
    onAddCategory: async () => {},
    onUpdateCategory: async () => {},
    onDeleteCategory: async () => {},
    onMoveCategory: async () => {},
    onAddItem: async () => {},
    onToggleItem: async () => {},
    onUpdateItemText: async () => {},
    onDeleteItem: async () => {},
    onMoveItem: async () => {}
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
        done: true,
        display_order: 3000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
      }
    ]
  }
};

export const ManyCategories: Story = {
  args: {
    categories: [
      ...categories,
      {
        id: 4,
        name: 'Groceries',
        display_order: 4000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
      }
    ],
    selectedCategoryId: 3
  }
};

export const ReorderMode: Story = {
  args: {
    initialReorderMode: true
  }
};
