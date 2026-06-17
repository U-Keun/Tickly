<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  import V2ChecklistScreen from '../components/v2/V2ChecklistScreen.svelte';
  import V2StreakOverlay from '../components/v2/V2StreakOverlay.svelte';
  import * as v2NativeDockApi from '../lib/api/v2NativeDockApi';
  import { initializeFonts } from '../lib/fonts';
  import { i18n } from '../lib/i18n';
  import { initializeTheme } from '../lib/themes';
  import { v2ChecklistStore } from '../lib/v2/v2ChecklistStore.svelte';
  import type { V2StreakHeatmap } from '../types';

  let nativeDockSupported = $state(false);
  let nativeDockRequestedVisible = $state(true);
  let nativeSheetOpen = $state(false);
  let webEditableFocused = $state(false);
  let archiveRequestToken = $state(0);
  let showStreakOverlay = $state(false);
  let isLoadingStreakHeatmaps = $state(false);
  let streakHeatmapError = $state<string | null>(null);
  let streakHeatmaps = $state<V2StreakHeatmap[]>([]);
  let nativeDockVisible = $derived(
    nativeDockSupported &&
      nativeDockRequestedVisible &&
      !nativeSheetOpen &&
      !webEditableFocused &&
      !showStreakOverlay
  );

  function shouldShowNativeDock(): boolean {
    return (
      nativeDockSupported &&
      nativeDockRequestedVisible &&
      !nativeSheetOpen &&
      !webEditableFocused &&
      !showStreakOverlay
    );
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

    if (actionId === 'archive') {
      archiveRequestToken += 1;
      return;
    }

    if (actionId === 'streak') {
      void openStreakOverlay();
      return;
    }

    console.info(`Tickly native ${actionId} dock action requested`);
  }

  async function loadStreakHeatmaps(): Promise<void> {
    isLoadingStreakHeatmaps = true;
    streakHeatmapError = null;
    try {
      streakHeatmaps = await v2ChecklistStore.getStreakHeatmaps();
    } catch (error) {
      streakHeatmapError =
        error instanceof Error ? error.message : i18n.t('v2StreakLoadErrorMessage');
    } finally {
      isLoadingStreakHeatmaps = false;
    }
  }

  async function openStreakOverlay(): Promise<void> {
    showStreakOverlay = true;
    syncNativeDock();
    await loadStreakHeatmaps();
  }

  function closeStreakOverlay(): void {
    showStreakOverlay = false;
    syncNativeDock();
  }

  function isEditableElement(target: EventTarget | Element | null): boolean {
    if (!(target instanceof HTMLElement)) return false;

    return (
      target instanceof HTMLInputElement ||
      target instanceof HTMLTextAreaElement ||
      target.isContentEditable
    );
  }

  function setWebEditableFocused(focused: boolean): void {
    if (webEditableFocused === focused) return;

    webEditableFocused = focused;
    syncNativeDock();
  }

  onMount(() => {
    initializeTheme();
    initializeFonts();
    void v2ChecklistStore
      .load()
      .catch(() => undefined)
      .finally(() => {
        if (document.visibilityState === 'visible') {
          void v2ChecklistStore.scheduleRepeatProcessing();
        }
      });

    nativeDockSupported = v2NativeDockApi.shouldUseNativeDock();
    const removeNativeDockActionListener =
      v2NativeDockApi.addNativeDockActionListener(handleNativeDockAction);
    const handleNativeSheetState = (event: Event): void => {
      const detail = (event as CustomEvent<{ isOpen?: boolean }>).detail;
      nativeSheetOpen = detail?.isOpen === true;
      syncNativeDock();
    };
    const handleFocusIn = (event: FocusEvent): void => {
      if (isEditableElement(event.target)) {
        setWebEditableFocused(true);
      }
    };
    const handleFocusOut = (): void => {
      window.setTimeout(() => {
        setWebEditableFocused(isEditableElement(document.activeElement));
      }, 0);
    };
    const handleVisibilityChange = (): void => {
      if (document.visibilityState !== 'visible') {
        v2ChecklistStore.disposeRepeatProcessingTimer();
        return;
      }

      void v2ChecklistStore
        .processRepeatsAndReload()
        .catch(() => undefined)
        .finally(() => {
          void v2ChecklistStore.scheduleRepeatProcessing();
        });
    };

    window.addEventListener('tickly:nativeSheetState', handleNativeSheetState);
    document.addEventListener('focusin', handleFocusIn);
    document.addEventListener('focusout', handleFocusOut);
    document.addEventListener('visibilitychange', handleVisibilityChange);

    void i18n.loadLocale().finally(() => {
      syncNativeDock();
    });

    return () => {
      removeNativeDockActionListener();
      window.removeEventListener('tickly:nativeSheetState', handleNativeSheetState);
      document.removeEventListener('focusin', handleFocusIn);
      document.removeEventListener('focusout', handleFocusOut);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
      v2ChecklistStore.disposeRepeatProcessingTimer();
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
  {archiveRequestToken}
  onArchiveCompletedItems={v2ChecklistStore.archiveCompletedItems}
  {nativeDockVisible}
  onNativeDockVisibilityChange={setNativeDockRequestedVisible}
/>

<V2StreakOverlay
  show={showStreakOverlay}
  heatmaps={streakHeatmaps}
  isLoading={isLoadingStreakHeatmaps}
  errorMessage={streakHeatmapError}
  onRefresh={loadStreakHeatmaps}
  onClose={closeStreakOverlay}
/>
