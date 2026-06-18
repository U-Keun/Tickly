import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { Category } from '../../types';
import CategoryDetailSheet from './CategoryDetailSheet.svelte';

const category: Category = {
  id: 1,
  name: 'Home',
  display_order: 1000,
  created_at: '2026-06-08T00:00:00Z',
  updated_at: '2026-06-08T00:00:00Z'
};

const meta = {
  title: 'Checklist/CategoryDetailSheet',
  component: CategoryDetailSheet,
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
} satisfies Meta<typeof CategoryDetailSheet>;

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
