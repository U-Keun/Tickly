import type { Meta, StoryObj } from '@storybook/sveltekit';

import V2SettingsPreviewStory from './V2SettingsPreviewStory.svelte';

const meta = {
  title: 'settings/V2Settings',
  component: V2SettingsPreviewStory,
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof V2SettingsPreviewStory>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Main: Story = {};
