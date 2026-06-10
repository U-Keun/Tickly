import type { Meta, StoryObj } from '@storybook/sveltekit';

import V2TagSuggestionBoard from './V2TagSuggestionBoard.svelte';

const now = '2026-06-08T00:00:00Z';

const meta = {
  title: 'v2/V2TagSuggestionBoard',
  component: V2TagSuggestionBoard,
  tags: ['autodocs'],
  args: {
    query: 'tr',
    suggestions: [
      { id: 1, name: 'travel', created_at: now, updated_at: now },
      { id: 2, name: 'train', created_at: now, updated_at: now },
      { id: 3, name: 'morning-trip', created_at: now, updated_at: now }
    ],
    selectedTagNames: [],
    onSelectTag: async () => {},
    onCreateTag: async () => {},
    onRemoveSelectedTag: async () => {}
  },
  parameters: {
    layout: 'padded'
  }
} satisfies Meta<typeof V2TagSuggestionBoard>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Empty: Story = {
  args: {
    query: 'church',
    suggestions: [],
    createTagName: 'church'
  }
};

export const WithSelectedTags: Story = {
  args: {
    query: '',
    selectedTagNames: ['travel', 'morning'],
    suggestions: []
  }
};
