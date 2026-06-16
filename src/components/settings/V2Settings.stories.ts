import type { Meta, StoryObj } from '@storybook/sveltekit';

import V2ArchiveSettingsPreviewStory from './V2ArchiveSettingsPreviewStory.svelte';
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

export const ArchiveList: StoryObj<typeof V2ArchiveSettingsPreviewStory> = {
  render: () => ({
    Component: V2ArchiveSettingsPreviewStory
  })
};

export const ArchiveEmpty: StoryObj<typeof V2ArchiveSettingsPreviewStory> = {
  render: () => ({
    Component: V2ArchiveSettingsPreviewStory,
    props: {
      state: 'empty'
    }
  })
};
