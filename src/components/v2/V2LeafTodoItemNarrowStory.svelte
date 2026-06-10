<script lang="ts">
  import type { V2TodoItem } from '../../types';
  import V2LeafTodoItem from './V2LeafTodoItem.svelte';

  interface Props {
    text: string;
  }

  let { text }: Props = $props();

  const now = '2026-06-08T00:00:00Z';

  let done = $state(false);
  let item = $derived<V2TodoItem>({
    id: 1,
    category_id: 1,
    text,
    memo: null,
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
    <V2LeafTodoItem
      {item}
      onToggleItem={toggleItem}
      onRequestEditItem={async () => {}}
      onRequestDeleteItem={async () => {}}
    />
  </div>
</div>
