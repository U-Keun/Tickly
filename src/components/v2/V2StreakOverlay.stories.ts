import type { Meta, StoryObj } from '@storybook/sveltekit';

import type { V2StreakHeatmap, V2Tag } from '../../types';
import V2StreakOverlay from './V2StreakOverlay.svelte';

const now = '2026-06-17T00:00:00Z';
const tags: V2Tag[] = [{ id: 1, name: 'health', created_at: now, updated_at: now }];

function logs(startDay: number, count: number) {
  return Array.from({ length: count }, (_, index) => ({
    completed_on: `2026-06-${String(startDay + index).padStart(2, '0')}`,
    completed_count: 1,
    combo_intensity: Math.min(index + 1, 10)
  }));
}

const heatmaps: V2StreakHeatmap[] = [
  {
    item: {
      id: 1,
      category_id: 1,
      text: 'Morning walk',
      memo: null,
      tags,
      repeat_type: 'none',
      repeat_detail: null,
      next_due_at: null,
      last_completed_at: '2026-06-17',
      reminder_at: null,
      archived_at: null,
      track_streak: true,
      streak_started_on: '2026-06-10',
      done: true,
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
    },
    logs: logs(10, 8),
    combo_intensity: 8,
    total_days: 8,
    current_streak: 8,
    longest_streak: 8,
    current_streak_dates: logs(10, 8).map((log) => log.completed_on),
    longest_streak_dates: logs(10, 8).map((log) => log.completed_on)
  },
  {
    item: {
      id: 2,
      category_id: 1,
      text: 'Plan weekly meals before Monday starts',
      memo: null,
      tags: [],
      repeat_type: 'weekly',
      repeat_detail: '[1,3,5]',
      next_due_at: null,
      last_completed_at: null,
      reminder_at: '18:30',
      archived_at: null,
      track_streak: true,
      streak_started_on: '2026-06-01',
      done: false,
      display_order: 2000,
      created_at: now,
      updated_at: now
    },
    category: {
      id: 2,
      name: 'Work',
      display_order: 2000,
      created_at: now,
      updated_at: now
    },
    logs: [
      { completed_on: '2026-06-02', completed_count: 1, combo_intensity: 1 },
      { completed_on: '2026-06-04', completed_count: 1, combo_intensity: 2 },
      { completed_on: '2026-06-09', completed_count: 1, combo_intensity: 1 }
    ],
    combo_intensity: 2,
    total_days: 3,
    current_streak: 0,
    longest_streak: 2,
    current_streak_dates: [],
    longest_streak_dates: ['2026-06-02', '2026-06-04']
  }
];

const meta = {
  title: 'v2/V2StreakOverlay',
  component: V2StreakOverlay,
  tags: ['autodocs'],
  args: {
    show: true,
    heatmaps,
    isLoading: false,
    errorMessage: null,
    onClose: () => {},
    onRefresh: async () => {}
  },
  parameters: {
    layout: 'fullscreen'
  }
} satisfies Meta<typeof V2StreakOverlay>;

export default meta;

type Story = StoryObj<typeof meta>;

export const WithCards: Story = {};

export const Empty: Story = {
  args: {
    heatmaps: []
  }
};

export const Loading: Story = {
  args: {
    isLoading: true
  }
};
