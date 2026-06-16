<script lang="ts">
  import type { V2TodoItem } from '../../types';
  import V2LeafTodoItem from './V2LeafTodoItem.svelte';

  const now = '2026-06-08T00:00:00Z';

  let item = $state<V2TodoItem>({
    id: 1,
    category_id: 1,
    text: 'Soft hop checkbox',
    memo: null,
    tags: [],
    repeat_type: 'none',
    repeat_detail: null,
    next_due_at: null,
    last_completed_at: null,
    done: false,
    display_order: 1000,
    created_at: now,
    updated_at: now
  });

  async function toggleItem(): Promise<void> {
    item = {
      ...item,
      done: !item.done,
      updated_at: now
    };
  }
</script>

<div class="max-w-xl">
  <V2LeafTodoItem
    {item}
    onToggleItem={toggleItem}
    onRequestEditItem={async () => {}}
    onRequestDeleteItem={async () => {}}
  />
</div>
