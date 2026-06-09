import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2ItemSearchResult } from '../../types';
import V2SearchSuggestionBoard from './V2SearchSuggestionBoard.svelte';

const now = '2026-06-08T00:00:00Z';

const results: V2ItemSearchResult[] = [
  {
    item: {
      id: 1,
      category_id: 1,
      text: 'Wallet',
      done: false,
      display_order: 1000,
      created_at: now,
      updated_at: now
    },
    category: {
      id: 1,
      name: 'Home',
      display_order: 1000,
      created_at: now,
      updated_at: now
    }
  },
  {
    item: {
      id: 2,
      category_id: 2,
      text: 'Portable charger before leaving for a very long commute',
      done: true,
      display_order: 2000,
      created_at: now,
      updated_at: now
    },
    category: {
      id: 2,
      name: 'Travel',
      display_order: 2000,
      created_at: now,
      updated_at: now
    }
  }
];

const meta = {
  title: 'v2/V2SearchSuggestionBoard',
  component: V2SearchSuggestionBoard,
  tags: ['autodocs'],
  args: {
    query: 'wall',
    results,
    isLoading: false,
    onSelectResult: async () => {}
  },
  parameters: {
    layout: 'padded'
  }
} satisfies Meta<typeof V2SearchSuggestionBoard>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const NoResults: Story = {
  args: {
    query: 'passport',
    results: []
  }
};

export const Loading: Story = {
  args: {
    results: [],
    isLoading: true
  }
};
