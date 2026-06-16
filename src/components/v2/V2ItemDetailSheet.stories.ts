import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2TodoItem } from '../../types';
import V2ItemDetailSheet from './V2ItemDetailSheet.svelte';

const item: V2TodoItem = {
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

export const WeeklyRepeat: Story = {
  args: {
    item: {
      ...item,
      repeat_type: 'weekly',
      repeat_detail: '[1,3,5]',
      text: 'Plan weekly meals'
    }
  }
};

export const WithReminder: Story = {
  args: {
    item: {
      ...item,
      reminder_at: '09:30'
    }
  }
};

export const FullEditingSurface: Story = {
  args: {
    item: {
      ...item,
      text: 'Review the redesigned editing sheet before the next build',
      memo: 'Compare this fallback surface with the native Swift sheet: field height, border weight, repeat controls, tags, reminder, and action buttons should feel like the same product.',
      tags: [
        { id: 1, name: 'design', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' },
        { id: 2, name: 'ios', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' }
      ],
      repeat_type: 'weekly',
      repeat_detail: '[1,3,5]',
      reminder_at: '18:30'
    },
    availableTags: [
      { id: 1, name: 'design', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' },
      { id: 2, name: 'ios', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' },
      { id: 3, name: 'review', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' }
    ]
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
