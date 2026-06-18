import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { Category, GraphData, Tag, TodoItem } from '../../types';
import GraphOverlay from './GraphOverlay.svelte';

const now = '2026-06-17T00:00:00Z';
const repeatDefaults = {
  repeat_type: 'none' as const,
  repeat_detail: null,
  next_due_at: null,
  last_completed_at: null,
  reminder_at: null,
  archived_at: null,
  track_streak: false,
  streak_started_on: null
};

const categories: Category[] = [
  { id: 1, name: 'Home', display_order: 1000, created_at: now, updated_at: now },
  { id: 2, name: 'Work', display_order: 2000, created_at: now, updated_at: now },
  { id: 3, name: 'Travel', display_order: 3000, created_at: now, updated_at: now }
];

const tags: Tag[] = [
  { id: 1, name: 'morning', created_at: now, updated_at: now },
  { id: 2, name: 'health', created_at: now, updated_at: now },
  { id: 3, name: 'focus', created_at: now, updated_at: now },
  { id: 4, name: 'packing', created_at: now, updated_at: now }
];

const items: TodoItem[] = [
  {
    id: 1,
    category_id: 1,
    text: 'Drink water before coffee',
    memo: null,
    tags: [tags[0], tags[1]],
    ...repeatDefaults,
    done: false,
    display_order: 1000,
    created_at: now,
    updated_at: now
  },
  {
    id: 2,
    category_id: 1,
    text: 'Stretch shoulders',
    memo: null,
    tags: [tags[1]],
    ...repeatDefaults,
    done: true,
    display_order: 2000,
    created_at: now,
    updated_at: now
  },
  {
    id: 3,
    category_id: 2,
    text: 'Review launch notes with a deliberately longer label',
    memo: null,
    tags: [tags[2]],
    ...repeatDefaults,
    done: false,
    display_order: 1000,
    created_at: now,
    updated_at: now
  },
  {
    id: 4,
    category_id: 2,
    text: 'Email design feedback',
    memo: null,
    tags: [tags[0], tags[2]],
    ...repeatDefaults,
    done: false,
    display_order: 2000,
    created_at: now,
    updated_at: now
  },
  {
    id: 5,
    category_id: 3,
    text: 'Pack charger',
    memo: null,
    tags: [tags[3]],
    ...repeatDefaults,
    done: false,
    display_order: 1000,
    created_at: now,
    updated_at: now
  },
  {
    id: 6,
    category_id: 3,
    text: 'Passport check',
    memo: null,
    tags: [tags[3], tags[0]],
    ...repeatDefaults,
    done: true,
    display_order: 2000,
    created_at: now,
    updated_at: now
  }
];

const graphData: GraphData = {
  categories,
  items,
  tags,
  tag_edges: items.flatMap((item) =>
    item.tags.map((tag) => ({
      tag_id: tag.id,
      item_id: item.id
    }))
  )
};

const meta = {
  title: 'Checklist/GraphOverlay',
  component: GraphOverlay,
  tags: ['autodocs'],
  args: {
    show: true,
    data: graphData,
    initialSelectedItemId: null,
    isLoading: false,
    errorMessage: null,
    onClose: () => {},
    onRefresh: async () => {},
    onItemEdit: async () => {},
    onItemToggle: async (itemId: number) => {
      const item = items.find((candidate) => candidate.id === itemId) ?? items[0];
      return { ...item, done: !item.done };
    }
  },
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof GraphOverlay>;

export default meta;

type Story = StoryObj<typeof meta>;

export const CategoryMembranes: Story = {};

export const ActionHalo: Story = {
  args: {
    initialSelectedItemId: 1
  }
};

export const Empty: Story = {
  args: {
    data: {
      categories: [],
      items: [],
      tags: [],
      tag_edges: []
    }
  }
};

export const Loading: Story = {
  args: {
    isLoading: true
  }
};

export const ErrorState: Story = {
  args: {
    errorMessage: 'Graph data could not be loaded.'
  }
};
