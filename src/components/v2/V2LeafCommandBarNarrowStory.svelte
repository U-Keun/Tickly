<script lang="ts">
  import type { V2Tag } from '../../types';
  import V2LeafCommandBar from './V2LeafCommandBar.svelte';

  type Mode = 'add' | 'search';

  const now = '2026-06-08T00:00:00Z';
  const availableTags: V2Tag[] = [
    { id: 1, name: 'travel', created_at: now, updated_at: now },
    { id: 2, name: 'today', created_at: now, updated_at: now },
    { id: 3, name: 'home', created_at: now, updated_at: now }
  ];

  let mode = $state<Mode>('add');
  let searchQuery = $state('');

  function enterSearch(): void {
    mode = 'search';
  }

  function exitSearch(): void {
    mode = 'add';
    searchQuery = '';
  }
</script>

<div class="min-h-screen bg-[var(--color-canvas)] p-4">
  <div class="mx-auto w-[360px] max-w-full">
    <V2LeafCommandBar
      initialInput="Groceries"
      {mode}
      {searchQuery}
      {availableTags}
      onAddItem={async () => {}}
      onEnterSearch={enterSearch}
      onExitSearch={exitSearch}
      onSearchQueryChange={(query) => (searchQuery = query)}
      onSearchInputFocus={() => {}}
    />
  </div>
</div>
