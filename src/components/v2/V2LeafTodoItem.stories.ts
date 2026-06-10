import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2TodoItem } from '../../types';
import V2LeafTodoItem from './V2LeafTodoItem.svelte';
import V2LeafTodoItemNarrowStory from './V2LeafTodoItemNarrowStory.svelte';
import V2LeafTodoItemSoftHopStory from './V2LeafTodoItemSoftHopStory.svelte';

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
    initialDrawerOpen: false,
    onToggleItem: async () => {},
    onRequestEditItem: async () => {},
    onRequestDeleteItem: async () => {}
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

export const LongText: StoryObj<typeof V2LeafTodoItemNarrowStory> = {
  render: () => ({
    Component: V2LeafTodoItemNarrowStory,
    props: {
      text: 'Umbrella before leaving for a very long commute day with errands after work and grocery pickup'
    }
  }),
  parameters: {
    layout: 'fullscreen'
  }
};

export const LongUnbrokenText: StoryObj<typeof V2LeafTodoItemNarrowStory> = {
  render: () => ({
    Component: V2LeafTodoItemNarrowStory,
    props: {
      text: 'SuperLongUnbrokenChecklistItemNameThatShouldNeverPushTheCardWiderThanTheiPhoneViewport'
    }
  }),
  parameters: {
    layout: 'fullscreen'
  }
};

export const SoftHop: StoryObj<typeof V2LeafTodoItemSoftHopStory> = {
  render: () => ({
    Component: V2LeafTodoItemSoftHopStory
  })
};
