import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { Category } from '../../types';
import CategoryRail from './CategoryRail.svelte';
import CategoryRailHorizontalScrollStory from './CategoryRailHorizontalScrollStory.svelte';
import CategoryRailReorderStory from './CategoryRailReorderStory.svelte';

const categories: Category[] = [
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
  title: 'Checklist/CategoryRail',
  component: CategoryRail,
  tags: ['autodocs'],
  args: {
    categories,
    selectedCategoryId: 1,
    isReorderMode: false,
    isReorderBusy: false,
    onSelectCategory: async () => {},
    onManageCategory: () => {},
    onEnterReorderMode: () => {},
    onFinishReorderMode: () => {},
    onReorderConsider: () => {},
    onReorderFinalize: async () => {}
  },
  parameters: {
    layout: 'padded'
  }
} satisfies Meta<typeof CategoryRail>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const SelectedMiddle: Story = {
  args: {
    selectedCategoryId: 2
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

export const ReorderMode: StoryObj<typeof CategoryRailReorderStory> = {
  render: () => ({
    Component: CategoryRailReorderStory
  }),
  parameters: {
    layout: 'fullscreen'
  }
};

export const HorizontalScroll: StoryObj<typeof CategoryRailHorizontalScrollStory> = {
  render: () => ({
    Component: CategoryRailHorizontalScrollStory
  }),
  parameters: {
    layout: 'fullscreen'
  }
};
