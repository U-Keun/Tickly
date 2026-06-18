import type { Meta, StoryObj } from '@storybook/sveltekit';

import TagEditor from './TagEditor.svelte';

const now = '2026-06-08T00:00:00Z';

const meta = {
  title: 'Checklist/TagEditor',
  component: TagEditor,
  tags: ['autodocs'],
  args: {
    tagNames: ['home', 'morning'],
    availableTags: [
      { id: 1, name: 'home', created_at: now, updated_at: now },
      { id: 2, name: 'morning', created_at: now, updated_at: now },
      { id: 3, name: 'travel', created_at: now, updated_at: now },
      { id: 4, name: 'church', created_at: now, updated_at: now }
    ],
    disabled: false,
    onChange: async () => {}
  },
  parameters: {
    layout: 'padded'
  }
} satisfies Meta<typeof TagEditor>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Empty: Story = {
  args: {
    tagNames: []
  }
};

export const Disabled: Story = {
  args: {
    disabled: true
  }
};
