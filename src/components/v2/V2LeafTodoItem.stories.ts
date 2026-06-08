import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2TodoItem } from '../../types';
import V2LeafTodoItem from './V2LeafTodoItem.svelte';

const item: V2TodoItem = {
  id: 1,
  category_id: 1,
  text: 'Wallet',
  done: false,
  display_order: 1000,
  created_at: '2026-06-08T00:00:00Z',
  updated_at: '2026-06-08T00:00:00Z'
};

const meta = {
  title: 'v2/V2LeafTodoItem',
  component: V2LeafTodoItem,
  tags: ['autodocs'],
  args: {
    item,
    isReorderMode: false,
    isFirst: false,
    isLast: false,
    initialDrawerOpen: false,
    onToggleItem: async () => {},
    onRequestEditItem: async () => {},
    onRequestDeleteItem: async () => {},
    onMoveItem: async () => {}
  },
  parameters: {
    layout: 'padded'
  }
} satisfies Meta<typeof V2LeafTodoItem>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Active: Story = {};

export const Completed: Story = {
  args: {
    item: {
      ...item,
      done: true,
      text: 'Keys'
    }
  }
};

export const DrawerOpen: Story = {
  args: {
    initialDrawerOpen: true
  }
};

export const LongText: Story = {
  args: {
    item: {
      ...item,
      text: 'Umbrella before leaving for a very long commute day with errands after work'
    }
  }
};

export const ReorderMiddle: Story = {
  args: {
    isReorderMode: true
  }
};

export const ReorderDrawerOpen: Story = {
  args: {
    isReorderMode: true,
    initialDrawerOpen: true
  }
};

export const ReorderFirst: Story = {
  args: {
    isReorderMode: true,
    isFirst: true
  }
};

export const ReorderLast: Story = {
  args: {
    isReorderMode: true,
    isLast: true
  }
};

export const ReorderDrawerFirst: Story = {
  args: {
    isReorderMode: true,
    isFirst: true,
    initialDrawerOpen: true
  }
};

export const ReorderDrawerLast: Story = {
  args: {
    isReorderMode: true,
    isLast: true,
    initialDrawerOpen: true
  }
};
