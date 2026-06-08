<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  import V2ChecklistScreen from '../../components/v2/V2ChecklistScreen.svelte';
  import { initializeFonts } from '../../lib/fonts';
  import { i18n } from '../../lib/i18n';
  import { initializeTheme } from '../../lib/themes';
  import { v2ChecklistStore } from '../../lib/v2/v2ChecklistStore.svelte';

  onMount(() => {
    initializeTheme();
    initializeFonts();
    void i18n.loadLocale();
    void v2ChecklistStore.load();
  });
</script>

<V2ChecklistScreen
  categories={v2ChecklistStore.categories}
  selectedCategoryId={v2ChecklistStore.selectedCategoryId}
  items={v2ChecklistStore.items}
  isLoading={v2ChecklistStore.isLoading}
  errorMessage={v2ChecklistStore.errorMessage}
  onBackHome={() => goto('/')}
  onRefresh={v2ChecklistStore.load}
  onSelectCategory={v2ChecklistStore.selectCategory}
  onAddCategory={v2ChecklistStore.addCategory}
  onUpdateCategory={v2ChecklistStore.updateCategory}
  onDeleteCategory={v2ChecklistStore.deleteCategory}
  onMoveCategory={v2ChecklistStore.moveCategory}
  onAddItem={v2ChecklistStore.addItem}
  onToggleItem={v2ChecklistStore.toggleItem}
  onUpdateItemText={v2ChecklistStore.updateItemText}
  onDeleteItem={v2ChecklistStore.deleteItem}
  onMoveItem={v2ChecklistStore.moveItem}
/>
