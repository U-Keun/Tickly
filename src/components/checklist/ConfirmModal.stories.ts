import type { Meta, StoryObj } from '@storybook/sveltekit';

import ConfirmModal from './ConfirmModal.svelte';

const meta = {
  title: 'Checklist/ConfirmModal',
  component: ConfirmModal,
  tags: ['autodocs'],
  args: {
    show: true,
    title: '항목을 삭제할까요?',
    message: '"Wallet" 항목이 삭제됩니다.',
    confirmLabel: '삭제',
    cancelLabel: '취소',
    tone: 'danger',
    isBusy: false,
    onConfirm: async () => {},
    onCancel: async () => {}
  },
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof ConfirmModal>;

export default meta;

type Story = StoryObj<typeof meta>;

export const DeleteItem: Story = {};

export const LongMessage: Story = {
  args: {
    message:
      '"Umbrella before leaving for a very long commute day with errands after work" 항목이 삭제됩니다.'
  }
};

export const Busy: Story = {
  args: {
    confirmLabel: '삭제 중...',
    isBusy: true
  }
};

export const Primary: Story = {
  args: {
    title: '변경사항을 저장할까요?',
    message: '현재 항목의 변경사항을 저장합니다.',
    confirmLabel: '저장',
    tone: 'primary'
  }
};
