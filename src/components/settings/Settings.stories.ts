import type { Meta, StoryObj } from '@storybook/sveltekit';

import ArchiveSettingsPreviewStory from './ArchiveSettingsPreviewStory.svelte';
import SettingsPreviewStory from './SettingsPreviewStory.svelte';

const meta = {
  title: 'Settings/Settings',
  component: SettingsPreviewStory,
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof SettingsPreviewStory>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Main: Story = {};

export const ArchiveList: StoryObj<typeof ArchiveSettingsPreviewStory> = {
  render: () => ({
    Component: ArchiveSettingsPreviewStory
  })
};

export const ArchiveEmpty: StoryObj<typeof ArchiveSettingsPreviewStory> = {
  render: () => ({
    Component: ArchiveSettingsPreviewStory,
    props: {
      state: 'empty'
    }
  })
};
