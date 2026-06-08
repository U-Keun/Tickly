import type { Meta, StoryObj } from '@storybook/sveltekit';

import V2LeafCommandBar from './V2LeafCommandBar.svelte';

const meta = {
  title: 'v2/V2LeafCommandBar',
  component: V2LeafCommandBar,
  tags: ['autodocs'],
  args: {
    disabled: false,
    initialInput: 'Wallet',
    onAddItem: async () => {}
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
