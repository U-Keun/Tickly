import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2Category } from '../../types';
import V2CategoryDetailSheet from './V2CategoryDetailSheet.svelte';

const category: V2Category = {
  id: 1,
  name: 'Home',
  display_order: 1000,
  created_at: '2026-06-08T00:00:00Z',
  updated_at: '2026-06-08T00:00:00Z'
};

const meta = {
  title: 'v2/V2CategoryDetailSheet',
  component: V2CategoryDetailSheet,
  tags: ['autodocs'],
  args: {
    show: true,
    mode: 'rename',
    category,
    isSaving: false,
    onSave: async () => {},
    onClose: () => {}
  },
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof V2CategoryDetailSheet>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Rename: Story = {};

export const Create: Story = {
  args: {
    mode: 'create',
    category: null
  }
};

export const LongName: Story = {
  args: {
    category: {
      ...category,
      name: 'A very long category name for careful editing on iPhone'
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
    category: {
      ...category,
      name: ''
    }
  }
};
