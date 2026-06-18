<script lang="ts">
  import type { TodoItem } from '../../types';
  import LeafTodoItem from './LeafTodoItem.svelte';

  interface Props {
    text: string;
  }

  let { text }: Props = $props();

  const now = '2026-06-08T00:00:00Z';

  let done = $state(false);
  let item = $derived<TodoItem>({
    id: 1,
    category_id: 1,
    text,
    memo: null,
    tags: [{ id: 1, name: 'mobile', created_at: now, updated_at: now }],
    repeat_type: 'none',
    repeat_detail: null,
    next_due_at: null,
    last_completed_at: null,
    reminder_at: null,
    archived_at: null,
    track_streak: false,
    streak_started_on: null,
    done,
    display_order: 1000,
    created_at: now,
    updated_at: now
  });

  async function toggleItem(): Promise<void> {
    done = !done;
  }
</script>

<div class="min-h-screen bg-[var(--color-canvas)] p-4">
  <div class="mx-auto w-[360px] max-w-full">
    <LeafTodoItem
      {item}
      onToggleItem={toggleItem}
      onRequestEditItem={async () => {}}
      onRequestDeleteItem={async () => {}}
    />
  </div>
</div>
