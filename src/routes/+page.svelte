<script lang="ts">
  import { onMount } from 'svelte';

  import V2ChecklistScreen from '../components/v2/V2ChecklistScreen.svelte';
  import { initializeFonts } from '../lib/fonts';
  import { i18n } from '../lib/i18n';
  import { initializeTheme } from '../lib/themes';
  import { v2ChecklistStore } from '../lib/v2/v2ChecklistStore.svelte';

  onMount(() => {
    initializeTheme();
    initializeFonts();
    void i18n.loadLocale();
    void v2ChecklistStore.load().catch(() => undefined);
  });
</script>

<V2ChecklistScreen
  categories={v2ChecklistStore.categories}
  selectedCategoryId={v2ChecklistStore.selectedCategoryId}
  items={v2ChecklistStore.items}
  availableTags={v2ChecklistStore.tags}
  errorMessage={v2ChecklistStore.errorMessage}
  onSelectCategory={v2ChecklistStore.selectCategory}
  onAddCategory={v2ChecklistStore.addCategory}
  onUpdateCategory={v2ChecklistStore.updateCategory}
  onDeleteCategory={v2ChecklistStore.deleteCategory}
  onReorderCategories={v2ChecklistStore.reorderCategories}
  onAddItem={v2ChecklistStore.addItem}
  onToggleItem={v2ChecklistStore.toggleItem}
  onUpdateItemDetails={v2ChecklistStore.updateItemDetails}
  onDeleteItem={v2ChecklistStore.deleteItem}
  onReorderItems={v2ChecklistStore.reorderItems}
  onSearchItems={v2ChecklistStore.searchItems}
/>
