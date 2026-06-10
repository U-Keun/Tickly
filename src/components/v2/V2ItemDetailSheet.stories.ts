import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2TodoItem } from '../../types';
import V2ItemDetailSheet from './V2ItemDetailSheet.svelte';

const item: V2TodoItem = {
  id: 1,
  category_id: 1,
  text: 'Wallet',
  memo: null,
  tags: [],
  done: false,
  display_order: 1000,
  created_at: '2026-06-08T00:00:00Z',
  updated_at: '2026-06-08T00:00:00Z'
};

const meta = {
  title: 'v2/V2ItemDetailSheet',
  component: V2ItemDetailSheet,
  tags: ['autodocs'],
  args: {
    show: true,
    item,
    availableTags: [
      { id: 1, name: 'home', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' },
      { id: 2, name: 'travel', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' }
    ],
    isSaving: false,
    onSaveDetails: async () => {},
    onClose: () => {}
  },
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof V2ItemDetailSheet>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const LongItemName: Story = {
  args: {
    item: {
      ...item,
      text: 'Umbrella before leaving for a very long commute day with errands after work'
    }
  }
};

export const WithMemo: Story = {
  args: {
    item: {
      ...item,
      memo: 'Keep this with the keys by the door.',
      tags: [{ id: 1, name: 'home', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' }]
    }
  }
};

export const Saving: Story = {
  args: {
    isSaving: true
  }
};

export const EmptyInputDisabled: Story = {
  args: {
    item: {
      ...item,
      text: ''
    }
  }
};
