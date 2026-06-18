import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { Category } from '../../types';
import CategoryManageSheet from './CategoryManageSheet.svelte';

const category: Category = {
  id: 2,
  name: 'Travel',
  display_order: 2000,
  created_at: '2026-06-08T00:00:00Z',
  updated_at: '2026-06-08T00:00:00Z'
};

const meta = {
  title: 'Checklist/CategoryManageSheet',
  component: CategoryManageSheet,
  tags: ['autodocs'],
  args: {
    show: true,
    category,
    isOnlyCategory: false,
    isBusy: false,
    onCreate: () => {},
    onRename: () => {},
    onEditOrder: () => {},
    onDeleteRequest: () => {},
    onClose: () => {}
  },
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof CategoryManageSheet>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const OnlyCategory: Story = {
  args: {
    category: {
      ...category,
      id: 1,
      name: 'Home',
      display_order: 1000
    },
    isOnlyCategory: true,
  }
};

export const Busy: Story = {
  args: {
    isBusy: true
  }
};
