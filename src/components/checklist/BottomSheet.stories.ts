import { createRawSnippet } from 'svelte';
import type { Meta, StoryObj } from '@storybook/sveltekit';

import BottomSheet from './BottomSheet.svelte';

const shortContent = createRawSnippet(() => ({
  render: () => `
    <div class="space-y-3">
      <p class="text-sm leading-6 text-[var(--color-ink-muted)]">Bottom sheet content stays close to the thumb area.</p>
      <button class="min-h-11 w-full rounded-[12px] bg-[var(--color-accent-sky-strong)] px-4 text-sm font-semibold text-[var(--color-ink)]">Action</button>
    </div>
  `
}));

const longContent = createRawSnippet(() => ({
  render: () => `
    <div class="space-y-3">
      ${Array.from({ length: 12 }, (_, index) => `
        <p class="rounded-[12px] bg-[var(--color-paper)] px-3 py-2 text-sm text-[var(--color-ink-muted)]">
          Detail row ${index + 1}
        </p>
      `).join('')}
    </div>
  `
}));

const meta = {
  title: 'Checklist/BottomSheet',
  component: BottomSheet,
  tags: ['autodocs'],
  args: {
    show: true,
    title: 'Item details',
    description: '',
    children: shortContent,
    onClose: () => {}
  },
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof BottomSheet>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Open: Story = {};

export const Closed: Story = {
  args: {
    show: false
  }
};

export const LongContent: Story = {
  args: {
    title: 'Long content',
    children: longContent
  }
};
