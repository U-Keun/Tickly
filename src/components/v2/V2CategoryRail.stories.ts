import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2Category } from '../../types';
import V2CategoryRail from './V2CategoryRail.svelte';
import V2CategoryRailHorizontalScrollStory from './V2CategoryRailHorizontalScrollStory.svelte';
import V2CategoryRailReorderStory from './V2CategoryRailReorderStory.svelte';

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
    isReorderMode: false,
    isReorderBusy: false,
    onSelectCategory: async () => {},
    onCreateCategory: () => {},
    onManageCategory: () => {},
    onEnterReorderMode: () => {},
    onFinishReorderMode: () => {},
    onReorderConsider: () => {},
    onReorderFinalize: async () => {}
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

export const ReorderMode: StoryObj<typeof V2CategoryRailReorderStory> = {
  render: () => ({
    Component: V2CategoryRailReorderStory
  }),
  parameters: {
    layout: 'fullscreen'
  }
};

export const HorizontalScroll: StoryObj<typeof V2CategoryRailHorizontalScrollStory> = {
  render: () => ({
    Component: V2CategoryRailHorizontalScrollStory
  }),
  parameters: {
    layout: 'fullscreen'
  }
};
