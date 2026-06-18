<script lang="ts">
  import type { Category } from '../../types';
  import CategoryRail from './CategoryRail.svelte';

  const now = '2026-06-08T00:00:00Z';

  const initialCategories: Category[] = ['Home', 'Travel', 'Work', 'Groceries'].map(
    (name, index) => ({
      id: index + 1,
      name,
      display_order: (index + 1) * 1000,
      created_at: now,
      updated_at: now
    })
  );

  let categories = $state<Category[]>(initialCategories);
  let selectedCategoryId = $state(1);
  let isReorderMode = $state(true);
  let isReorderBusy = $state(false);

  function reindexCategories(nextCategories: Category[]): Category[] {
    return nextCategories.map((category, index) => ({
      ...category,
      display_order: (index + 1) * 1000
    }));
  }

  async function saveCategoryOrder(nextCategories: Category[]): Promise<void> {
    isReorderBusy = true;
    await new Promise((resolve) => setTimeout(resolve, 180));
    categories = reindexCategories(nextCategories);
    isReorderBusy = false;
  }
</script>

<div class="min-h-screen bg-[var(--color-canvas)] p-4">
  <div class="mx-auto w-[390px] max-w-full">
    <CategoryRail
      {categories}
      {selectedCategoryId}
      {isReorderMode}
      {isReorderBusy}
      onSelectCategory={async (id) => {
        selectedCategoryId = id;
      }}
      onManageCategory={() => {}}
      onEnterReorderMode={() => {
        isReorderMode = true;
      }}
      onFinishReorderMode={() => {
        isReorderMode = false;
      }}
      onReorderConsider={(nextCategories) => {
        categories = nextCategories;
      }}
      onReorderFinalize={saveCategoryOrder}
    />
  </div>
</div>
