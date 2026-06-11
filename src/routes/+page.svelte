<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  import V2ChecklistScreen from '../components/v2/V2ChecklistScreen.svelte';
  import * as v2NativeDockApi from '../lib/api/v2NativeDockApi';
  import { initializeFonts } from '../lib/fonts';
  import { i18n } from '../lib/i18n';
  import { initializeTheme } from '../lib/themes';
  import { v2ChecklistStore } from '../lib/v2/v2ChecklistStore.svelte';

  let nativeDockSupported = $state(false);
  let nativeDockRequestedVisible = $state(true);
  let nativeSheetOpen = $state(false);
  let nativeDockVisible = $derived(nativeDockSupported && nativeDockRequestedVisible && !nativeSheetOpen);

  function shouldShowNativeDock(): boolean {
    return nativeDockSupported && nativeDockRequestedVisible && !nativeSheetOpen;
  }

  function buildNativeDockRequest(visible: boolean): v2NativeDockApi.V2NativeDockRequest {
    return {
      visible,
      streakLabel: i18n.t('streak'),
      graphLabel: i18n.t('graph'),
      archiveLabel: i18n.t('archive'),
      settingsLabel: i18n.t('settings'),
      streakEnabled: true,
      graphEnabled: true,
      archiveEnabled: true,
      settingsEnabled: true
    };
  }

  function syncNativeDock(): void {
    if (!nativeDockSupported) return;
    void v2NativeDockApi.configureNativeDock(buildNativeDockRequest(shouldShowNativeDock()));
  }

  function setNativeDockRequestedVisible(visible: boolean): void {
    if (nativeDockRequestedVisible === visible) return;
    nativeDockRequestedVisible = visible;
    syncNativeDock();
  }

  function handleNativeDockAction(actionId: v2NativeDockApi.V2NativeDockActionId): void {
    if (actionId === 'settings') {
      void goto('/settings?returnTo=%2F');
      return;
    }

    console.info(`Tickly native ${actionId} dock action requested`);
  }

  onMount(() => {
    initializeTheme();
    initializeFonts();
    void v2ChecklistStore.load().catch(() => undefined);

    nativeDockSupported = v2NativeDockApi.shouldUseNativeDock();
    const removeNativeDockActionListener =
      v2NativeDockApi.addNativeDockActionListener(handleNativeDockAction);
    const handleNativeSheetState = (event: Event): void => {
      const detail = (event as CustomEvent<{ isOpen?: boolean }>).detail;
      nativeSheetOpen = detail?.isOpen === true;
      syncNativeDock();
    };

    window.addEventListener('tickly:nativeSheetState', handleNativeSheetState);

    void i18n.loadLocale().finally(() => {
      syncNativeDock();
    });

    return () => {
      removeNativeDockActionListener();
      window.removeEventListener('tickly:nativeSheetState', handleNativeSheetState);
      if (nativeDockSupported) {
        void v2NativeDockApi.configureNativeDock(buildNativeDockRequest(false));
      }
    };
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
  {nativeDockVisible}
  onNativeDockVisibilityChange={setNativeDockRequestedVisible}
/>
