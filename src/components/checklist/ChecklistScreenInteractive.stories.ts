import type { Meta, StoryObj } from '@storybook/sveltekit';

import ChecklistScreenCategorySwitchingStory from './ChecklistScreenCategorySwitchingStory.svelte';

const meta = {
  title: 'Checklist/ChecklistScreen/Interactive',
  component: ChecklistScreenCategorySwitchingStory,
  tags: ['autodocs'],
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof ChecklistScreenCategorySwitchingStory>;

export default meta;

type Story = StoryObj<typeof meta>;

export const CategorySwitching: Story = {};

export const DrawerOpenCompletion: Story = {
  args: {
    initialOpenDrawerItemIds: [1]
  }
};
