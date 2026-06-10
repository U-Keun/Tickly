<script lang="ts">
  import { goto } from '$app/navigation';
  import { onDestroy, onMount } from 'svelte';

  import type { RepeatType, TodoItem } from '../../types';

  import ActiveTagFilterBanner from '../../components/ActiveTagFilterBanner.svelte';
  import CategoryTabs from '../../components/CategoryTabs.svelte';
  import FloatingActions from '../../components/FloatingActions.svelte';
  import HomeModals from '../../components/HomeModals.svelte';
  import HomeTodoList from '../../components/HomeTodoList.svelte';
  import IntroAnimation from '../../components/IntroAnimation.svelte';
  import { createHomeBootstrap } from '../../lib/home/homeBootstrap';
  import { createHomeCategoryActions } from '../../lib/home/homeCategoryActions';
  import { createHomeEditModalActions } from '../../lib/home/homeEditModalActions';
  import { initializeFonts } from '../../lib/fonts';
  import { createHomeItemActions } from '../../lib/home/homeItemActions';
  import { createHomeLifecycle } from '../../lib/home/homeLifecycle';
  import { i18n } from '../../lib/i18n';
  import { cancelReminder, rescheduleAll, scheduleReminder } from '../../lib/notification';
  import { appStore, modalStore } from '../../lib/stores';
  import { initializeTheme } from '../../lib/themes';
  import * as settingsApi from '../../lib/api/settingsApi';
  import * as todoApi from '../../lib/api/todoApi';

  // Local UI state only
  let isEditingItem = $state(false);
  let showFab = $state(false);

  // Edit item modal state (rendered at root to avoid CSS transform clipping)
  let editingItem = $state<TodoItem | null>(null);
  let showEditModal = $state(false);
  let capturedCloseDrawer = $state<(() => void) | null>(null);
  const { openEditModal, handleEditSave, handleEditCancel } = createHomeEditModalActions({
    getCapturedCloseDrawer: () => capturedCloseDrawer,
    setCapturedCloseDrawer: (handler) => {
      capturedCloseDrawer = handler;
    },
    setEditingItem: (item) => {
      editingItem = item;
    },
    setShowEditModal: (show) => {
      showEditModal = show;
    },
    setIsEditingItem: (editing) => {
      isEditingItem = editing;
    }
  });

  // Computed display items (tag filter aware)
  let displayItems = $derived(
    appStore.activeTagFilter !== null ? appStore.filteredItems : appStore.items
  );
  let activeTagName = $derived(
    appStore.activeTagFilter !== null
      ? (appStore.allTags.find((tag) => tag.id === appStore.activeTagFilter)?.name ?? '')
      : ''
  );

  // Reference to CategoryTabs component
  let categoryTabsComponent: any;
  const {
    handleCategoryLongPress,
    handleEditFromMenu,
    confirmDeleteCategory,
    confirmReset
  } = createHomeCategoryActions({
    openCategoryMenu: modalStore.openCategoryMenu,
    closeCategoryMenu: modalStore.closeCategoryMenu,
    closeDeleteCategoryConfirm: modalStore.closeDeleteCategoryConfirm,
    closeResetConfirm: modalStore.closeResetConfirm,
    getSelectedCategoryForMenu: () => modalStore.selectedCategoryForMenu,
    getCategoryTabsComponent: () => categoryTabsComponent,
    deleteCategory: appStore.deleteCategory,
    resetAllItems: appStore.resetAllItems
  });

  const homeLifecycle = createHomeLifecycle({
    loadItems: appStore.loadItems,
    loadTagsForItems: appStore.loadTagsForItems,
    getItems: () => appStore.items,
    processRepeats: todoApi.processRepeats,
    checkAndAutoReset: todoApi.checkAndAutoReset,
    getResetTime: () => settingsApi.getSetting('reset_time'),
    rescheduleAll
  });

  const {
    processRepeatsAndReload,
    scheduleResetTimer,
    clearResetTimer,
    handleVisibilityChange
  } = homeLifecycle;

  const { handleAddItem, handleUpdateReminder, handleToggleItem, handleDeleteItem } =
    createHomeItemActions({
      addItem: appStore.addItem,
      updateReminder: appStore.updateReminder,
      toggleItem: appStore.toggleItem,
      deleteItem: appStore.deleteItem,
      getItems: () => appStore.items,
      scheduleReminder,
      cancelReminder
    });

  const { run: runHomeBootstrap } = createHomeBootstrap({
    initializeTheme,
    initializeFonts,
    loadLocale: i18n.loadLocale,
    processRepeatsAndReload,
    loadCategories: appStore.loadCategories,
    loadItems: appStore.loadItems,
    loadAllTags: appStore.loadAllTags,
    loadTagsForItems: appStore.loadTagsForItems,
    getItems: () => appStore.items,
    scheduleResetTimer,
    rescheduleAll
  });

  onMount(async () => {
    await runHomeBootstrap();

    showFab = true;

    // Listen for visibility changes (app coming back to foreground)
    document.addEventListener('visibilitychange', handleVisibilityChange);
  });

  onDestroy(() => {
    document.removeEventListener('visibilitychange', handleVisibilityChange);
    clearResetTimer();
  });
</script>

<div class="app-container bg-paper flex flex-col">
  <!-- Category Tabs Component -->
  <div>
    <CategoryTabs
      bind:this={categoryTabsComponent}
      categories={appStore.categories}
      selectedCategoryId={appStore.selectedCategoryId}
      hasActiveTagFilter={appStore.activeTagFilter !== null}
      onSelectCategory={appStore.selectCategory}
      onAddCategory={appStore.addCategory}
      onEditCategory={appStore.editCategory}
      onCategoryLongPress={handleCategoryLongPress}
      onReorderCategories={modalStore.openReorderCategoriesModal}
    />
  </div>

  <!-- Main Content -->
  <main class="main-content max-w-2xl w-full mx-auto flex flex-col flex-1 min-h-0">
    <!-- Tag Filter Banner -->
    <ActiveTagFilterBanner
      show={appStore.activeTagFilter !== null}
      tagName={activeTagName}
      onClear={appStore.clearTagFilter}
    />

    <HomeTodoList
      {displayItems}
      itemTagsMap={appStore.itemTagsMap}
      onDeleteItem={handleDeleteItem}
      onToggleItem={handleToggleItem}
      onEditItem={appStore.editItem}
      onOpenEditModal={openEditModal}
    />
  </main>

  <!-- Floating Action Buttons -->
  <FloatingActions
    show={showFab && !isEditingItem}
    onAdd={modalStore.openAddItemModal}
    onGraph={() => goto('/graph')}
    onReorder={modalStore.openReorderModal}
    onStreak={modalStore.openStreakModal}
    onTagFilter={modalStore.openTagFilterModal}
    onSettings={() => goto('/settings')}
  />

  <HomeModals
    {handleAddItem}
    {handleEditFromMenu}
    {confirmReset}
    {confirmDeleteCategory}
    {editingItem}
    {showEditModal}
    {handleUpdateReminder}
    {handleEditSave}
    {handleEditCancel}
  />

  <!-- Intro Animation Component -->
  <IntroAnimation />
</div>
