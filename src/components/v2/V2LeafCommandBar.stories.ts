import type { Meta, StoryObj } from '@storybook/sveltekit';

import V2LeafCommandBar from './V2LeafCommandBar.svelte';
import V2LeafCommandBarNarrowStory from './V2LeafCommandBarNarrowStory.svelte';

const meta = {
  title: 'v2/V2LeafCommandBar',
  component: V2LeafCommandBar,
  tags: ['autodocs'],
  args: {
    disabled: false,
    initialInput: 'Wallet',
    mode: 'add',
    searchQuery: '',
    availableTags: [
      { id: 1, name: 'travel', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' },
      { id: 2, name: 'today', created_at: '2026-06-08T00:00:00Z', updated_at: '2026-06-08T00:00:00Z' }
    ],
    onAddItem: async () => {},
    onEnterSearch: async () => {},
    onExitSearch: async () => {},
    onSearchQueryChange: () => {},
    onSearchInputFocus: () => {}
  },
  parameters: {
    layout: 'centered'
  }
} satisfies Meta<typeof V2LeafCommandBar>;

export default meta;

type Story = StoryObj<typeof meta>;

export const WithInput: Story = {};

export const Empty: Story = {
  args: {
    initialInput: ''
  }
};

export const Disabled: Story = {
  args: {
    disabled: true,
    initialInput: ''
  }
};

export const SearchMode: Story = {
  args: {
    mode: 'search',
    searchQuery: 'wallet',
    initialInput: ''
  }
};

export const TagDraft: Story = {
  args: {
    initialInput: 'Read #t'
  }
};

export const NarrowActions: StoryObj<typeof V2LeafCommandBarNarrowStory> = {
  render: () => ({
    Component: V2LeafCommandBarNarrowStory
  }),
  parameters: {
    layout: 'fullscreen'
  }
};
