import type { Meta, StoryObj } from '@storybook/sveltekit';

import V2LeafCommandBar from './V2LeafCommandBar.svelte';

const meta = {
  title: 'v2/V2LeafCommandBar',
  component: V2LeafCommandBar,
  tags: ['autodocs'],
  args: {
    disabled: false,
    initialInput: 'Wallet',
    mode: 'add',
    searchQuery: '',
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
