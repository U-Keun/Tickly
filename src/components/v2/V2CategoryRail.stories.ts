import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2Category } from '../../types';
import V2CategoryRail from './V2CategoryRail.svelte';

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

const meta = {
  title: 'v2/V2CategoryRail',
  component: V2CategoryRail,
  tags: ['autodocs'],
  args: {
    categories,
    selectedCategoryId: 1,
    onSelectCategory: async () => {},
    onCreateCategory: () => {},
    onManageCategory: () => {}
  },
  parameters: {
    layout: 'padded'
  }
} satisfies Meta<typeof V2CategoryRail>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const SelectedMiddle: Story = {
  args: {
    selectedCategoryId: 2
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
      },
      {
        id: 5,
        name: 'Reading',
        display_order: 5000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
      },
      {
        id: 6,
        name: 'Long walks',
        display_order: 6000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
      }
    ]
  }
};

export const LongName: Story = {
  args: {
    categories: [
      ...categories,
      {
        id: 4,
        name: 'A very long category name for careful truncation',
        display_order: 4000,
        created_at: '2026-06-08T00:00:00Z',
        updated_at: '2026-06-08T00:00:00Z'
      }
    ],
    selectedCategoryId: 4
  }
};
