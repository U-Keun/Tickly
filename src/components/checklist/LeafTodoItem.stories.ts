import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { TodoItem } from '../../types';
import LeafTodoItem from './LeafTodoItem.svelte';
import LeafTodoItemNarrowStory from './LeafTodoItemNarrowStory.svelte';
import LeafTodoItemSoftHopStory from './LeafTodoItemSoftHopStory.svelte';

const item: TodoItem = {
  id: 1,
  category_id: 1,
  text: 'Wallet',
  memo: null,
  tags: [],
  repeat_type: 'none',
  repeat_detail: null,
  next_due_at: null,
  last_completed_at: null,
  reminder_at: null,
  archived_at: null,
  track_streak: false,
  streak_started_on: null,
  done: false,
  display_order: 1000,
  created_at: '2026-06-08T00:00:00Z',
  updated_at: '2026-06-08T00:00:00Z'
};

const meta = {
  title: 'Checklist/LeafTodoItem',
  component: LeafTodoItem,
  tags: ['autodocs'],
  args: {
    item,
    initialDrawerOpen: false,
    onToggleItem: async () => {},
    onRequestEditItem: async () => {},
    onRequestDeleteItem: async () => {}
  },
  parameters: {
    layout: 'padded'
  }
} satisfies Meta<typeof LeafTodoItem>;

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

export const MemoPreview: Story = {
  args: {
    item: {
      ...item,
      memo: 'Keep this in the small front pocket so it is easy to find before leaving.',
      tags: [{ id: 1, name: 'travel', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' }]
    },
    initialDrawerOpen: true
  }
};

export const RepeatPreview: Story = {
  args: {
    item: {
      ...item,
      text: 'Plan weekly meals',
      memo: 'Use the shared grocery list.',
      repeat_type: 'weekly',
      repeat_detail: '[1,3,5]'
    },
    initialDrawerOpen: true
  }
};

export const ReminderPreview: Story = {
  args: {
    item: {
      ...item,
      reminder_at: '09:30'
    },
    initialDrawerOpen: true
  }
};

export const LongTitleDrawer: Story = {
  args: {
    item: {
      ...item,
      text: 'Umbrella before leaving for a very long commute day with errands after work and grocery pickup',
      memo: null,
      tags: [{ id: 1, name: 'errand', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' }]
    },
    initialDrawerOpen: true
  }
};

export const LongTitleAndMemoDrawer: Story = {
  args: {
    item: {
      ...item,
      text: 'SuperLongUnbrokenChecklistItemNameThatShouldNeverPushTheCardWiderThanTheiPhoneViewportEvenWhenTheDrawerIsOpen',
      memo: 'Check the side pocket before leaving.\nIf it rains after work, use the compact umbrella instead of the larger one in the hallway closet.\nThis memo intentionally has multiple lines so the drawer preview can be checked.',
      tags: [
        { id: 1, name: 'travel', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' },
        { id: 2, name: 'morning', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' }
      ]
    },
    initialDrawerOpen: true
  }
};

export const LongText: StoryObj<typeof LeafTodoItemNarrowStory> = {
  render: () => ({
    Component: LeafTodoItemNarrowStory,
    props: {
      text: 'Umbrella before leaving for a very long commute day with errands after work and grocery pickup'
    }
  }),
  parameters: {
    layout: 'fullscreen'
  }
};

export const LongUnbrokenText: StoryObj<typeof LeafTodoItemNarrowStory> = {
  render: () => ({
    Component: LeafTodoItemNarrowStory,
    props: {
      text: 'SuperLongUnbrokenChecklistItemNameThatShouldNeverPushTheCardWiderThanTheiPhoneViewport'
    }
  }),
  parameters: {
    layout: 'fullscreen'
  }
};

export const SoftHop: StoryObj<typeof LeafTodoItemSoftHopStory> = {
  render: () => ({
    Component: LeafTodoItemSoftHopStory
  })
};
