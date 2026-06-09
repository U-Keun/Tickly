<script lang="ts">
  import type { V2Category } from '../../types';
  import V2CategoryRail from './V2CategoryRail.svelte';

  const now = '2026-06-08T00:00:00Z';

  const categories: V2Category[] = [
    'Home',
    'Travel',
    'Work',
    'Groceries',
    'Reading',
    'Long walks',
    'Errands',
    'Family plans',
    'Someday'
  ].map((name, index) => ({
    id: index + 1,
    name,
    display_order: (index + 1) * 1000,
    created_at: now,
    updated_at: now
  }));

  let selectedCategoryId = $state(7);
</script>

<div class="min-h-screen bg-[var(--color-canvas)] p-4">
  <div class="mx-auto w-[360px] max-w-full">
    <V2CategoryRail
      {categories}
      {selectedCategoryId}
      onSelectCategory={async (id) => {
        selectedCategoryId = id;
      }}
      onCreateCategory={() => {}}
      onManageCategory={() => {}}
    />
  </div>
</div>
