import type { Meta, StoryObj } from '@storybook/sveltekit';

import V2ChecklistScreenCategorySwitchingStory from './V2ChecklistScreenCategorySwitchingStory.svelte';

const meta = {
  title: 'v2/V2ChecklistScreen/Interactive',
  component: V2ChecklistScreenCategorySwitchingStory,
  tags: ['autodocs'],
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof V2ChecklistScreenCategorySwitchingStory>;

export default meta;

type Story = StoryObj<typeof meta>;

export const CategorySwitching: Story = {};
