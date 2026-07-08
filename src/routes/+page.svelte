<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  import GraphOverlay from '../components/checklist/GraphOverlay.svelte';
  import ItemDetailSheet from '../components/checklist/ItemDetailSheet.svelte';
  import ChecklistScreen from '../components/checklist/ChecklistScreen.svelte';
  import StreakOverlay from '../components/checklist/StreakOverlay.svelte';
  import * as nativeDockApi from '../lib/api/nativeDockApi';
  import { initializeFonts } from '../lib/fonts';
  import { i18n } from '../lib/i18n';
  import { initializeTheme } from '../lib/themes';
  import { openNativeItemDetailSheet } from '../lib/checklist/itemDetailLauncher';
  import { checklistStore } from '../lib/checklist/checklistStore.svelte';
  import { icloudSyncStore } from '../lib/checklist/icloudSyncStore.svelte';
  import type { GraphData, RepeatType, StreakHeatmap, TodoItem } from '../types';

  const ICLOUD_STATUS_AFTER_LOAD_DELAY_MS = 1500;
  const ROUTE_MOTION_KEY = 'tickly:route:last-pathname';

  let mainRouteMotionClass = $state('main-route-enter-idle');
  let nativeDockSupported = $state(false);
  let nativeDockRequestedVisible = $state(true);
  let nativeSheetOpen = $state(false);
  let webEditableFocused = $state(false);
  let archiveRequestToken = $state(0);
  let showStreakOverlay = $state(false);
  let isLoadingStreakHeatmaps = $state(false);
  let streakHeatmapError = $state<string | null>(null);
  let streakHeatmaps = $state<StreakHeatmap[]>([]);
  let showGraphOverlay = $state(false);
  let isLoadingGraphData = $state(false);
  let graphError = $state<string | null>(null);
  let graphData = $state<GraphData | null>(null);
  let graphItemPendingEdit = $state<TodoItem | null>(null);
  let isSavingGraphItemEdit = $state(false);
  let isInitialChecklistLoading = $state(true);
  let iCloudSyncScheduleTimeout: ReturnType<typeof setTimeout> | null = null;
  let nativeDockVisible = $derived(
    nativeDockSupported &&
      nativeDockRequestedVisible &&
      !nativeSheetOpen &&
      !webEditableFocused &&
      !showStreakOverlay &&
      !showGraphOverlay
  );

  function shouldShowNativeDock(): boolean {
    return (
      nativeDockSupported &&
      nativeDockRequestedVisible &&
      !nativeSheetOpen &&
      !webEditableFocused &&
      !showStreakOverlay &&
      !showGraphOverlay
    );
  }

  function getMainEnterClass(): string {
    if (typeof sessionStorage === 'undefined') return 'main-route-enter-idle';

    const previousPathname = sessionStorage.getItem(ROUTE_MOTION_KEY);
    sessionStorage.setItem(ROUTE_MOTION_KEY, '/');

    if (previousPathname?.startsWith('/settings')) return 'mainRouteEnterBack';
    return 'main-route-enter-idle';
  }

  function buildNativeDockRequest(visible: boolean): nativeDockApi.NativeDockRequest {
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
    void nativeDockApi.configureNativeDock(buildNativeDockRequest(shouldShowNativeDock()));
  }

  function setNativeDockRequestedVisible(visible: boolean): void {
    if (nativeDockRequestedVisible === visible) return;
    nativeDockRequestedVisible = visible;
    syncNativeDock();
  }

  function handleNativeDockAction(actionId: nativeDockApi.NativeDockActionId): void {
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

    if (actionId === 'graph') {
      void openGraphOverlay();
      return;
    }

    console.info(`Tickly native ${actionId} dock action requested`);
  }

  async function loadStreakHeatmaps(): Promise<void> {
    isLoadingStreakHeatmaps = true;
    streakHeatmapError = null;
    try {
      streakHeatmaps = await checklistStore.getStreakHeatmaps();
    } catch (error) {
      streakHeatmapError =
        error instanceof Error ? error.message : i18n.t('checklistStreakLoadErrorMessage');
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

  async function loadGraphData(): Promise<void> {
    isLoadingGraphData = true;
    graphError = null;
    try {
      graphData = await checklistStore.getGraphData();
    } catch (error) {
      graphError = error instanceof Error ? error.message : i18n.t('checklistGraphLoadErrorMessage');
    } finally {
      isLoadingGraphData = false;
    }
  }

  async function syncGraphDataQuietly(): Promise<void> {
    try {
      graphData = await checklistStore.getGraphData();
    } catch (error) {
      console.error('Failed to quietly sync graph data.', error);
    }
  }

  async function openGraphOverlay(): Promise<void> {
    showGraphOverlay = true;
    syncNativeDock();
    await loadGraphData();
  }

  function closeGraphOverlay(): void {
    if (isSavingGraphItemEdit) return;
    graphItemPendingEdit = null;
    showGraphOverlay = false;
    syncNativeDock();
  }

  async function saveGraphItemDetails(
    id: number,
    text: string,
    memo: string | null,
    tagNames: string[] = [],
    repeatType: RepeatType = 'none',
    repeatDetail: string | null = null,
    reminderAt: string | null = null,
    trackStreak = false
  ): Promise<void> {
    if (isSavingGraphItemEdit) return;

    isSavingGraphItemEdit = true;
    try {
      await checklistStore.updateItemDetails(
        id,
        text,
        memo,
        tagNames,
        repeatType,
        repeatDetail,
        reminderAt,
        trackStreak
      );
      await loadGraphData();
    } finally {
      isSavingGraphItemEdit = false;
    }
  }

  async function openGraphItemDetail(item: TodoItem): Promise<void> {
    const nativeResult = await openNativeItemDetailSheet(item, checklistStore.tags);

    if (nativeResult.status === 'unavailable') {
      graphItemPendingEdit = item;
      return;
    }

    if (nativeResult.status === 'saved') {
      const { values } = nativeResult;
      try {
        await saveGraphItemDetails(
          values.id,
          values.text,
          values.memo,
          values.tagNames,
          values.repeatType,
          values.repeatDetail,
          values.reminderAt,
          values.trackStreak
        );
      } catch {
        // The checklist store owns the visible error banner.
      }
    }
  }

  function handleGraphItemEdit(itemId: number): void {
    const item = graphData?.items.find((candidate) => candidate.id === itemId);
    if (!item) return;
    void openGraphItemDetail(item);
  }

  async function handleGraphItemToggle(itemId: number): Promise<TodoItem> {
    const updatedItem = await checklistStore.toggleItem(itemId);
    void syncGraphDataQuietly();
    return updatedItem;
  }

  function cancelGraphItemEdit(): void {
    if (isSavingGraphItemEdit) return;
    graphItemPendingEdit = null;
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

  async function loadChecklist(): Promise<void> {
    await checklistStore.load().catch(() => undefined);
  }

  function reloadChecklistAfterICloudIdle(isCancelled: () => boolean): void {
    if (!icloudSyncStore.isSyncing) return;

    void icloudSyncStore
      .waitUntilIdle()
      .then(() => {
        if (isCancelled() || document.visibilityState !== 'visible') return;
        return loadChecklist().finally(() => {
          if (!isCancelled() && document.visibilityState === 'visible') {
            void checklistStore.scheduleRepeatProcessing();
          }
        });
      })
      .catch(() => undefined);
  }

  function waitForNextFrame(): Promise<void> {
    return new Promise((resolve) => {
      window.requestAnimationFrame(() => resolve());
    });
  }

  function clearICloudSyncScheduleTimeout(): void {
    if (iCloudSyncScheduleTimeout === null) return;
    window.clearTimeout(iCloudSyncScheduleTimeout);
    iCloudSyncScheduleTimeout = null;
  }

  async function runChecklistMaintenance(isCancelled: () => boolean): Promise<void> {
    await new Promise((resolve) => window.setTimeout(resolve, 250));
    if (isCancelled() || document.visibilityState !== 'visible') return;

    try {
      const processedCount = await checklistStore.processWidgetActions();
      if (isCancelled() || document.visibilityState !== 'visible') return;

      if (processedCount <= 0) {
        await checklistStore.processRepeatsAndReload();
      }
    } catch {
      // The checklist store keeps the visible error state; boot should continue.
    } finally {
      if (isCancelled() || document.visibilityState !== 'visible') return;
      void checklistStore.scheduleRepeatProcessing();
    }
  }

  function scheduleICloudSyncTimersAfterChecklistIdle(
    isCancelled: () => boolean,
    delayMs = 2500
  ): void {
    clearICloudSyncScheduleTimeout();

    iCloudSyncScheduleTimeout = window.setTimeout(() => {
      iCloudSyncScheduleTimeout = null;
      if (isCancelled() || document.visibilityState !== 'visible') return;
      if (checklistStore.isLoading) {
        scheduleICloudSyncTimersAfterChecklistIdle(isCancelled, delayMs);
        return;
      }

      icloudSyncStore.scheduleAutoSync();
      icloudSyncStore.scheduleForegroundPull();
    }, delayMs);
  }

  async function loadICloudStatusThenSchedule(isCancelled: () => boolean): Promise<void> {
    await new Promise((resolve) => window.setTimeout(resolve, ICLOUD_STATUS_AFTER_LOAD_DELAY_MS));
    if (isCancelled() || document.visibilityState !== 'visible') return;
    await icloudSyncStore.loadStatus().catch(() => undefined);
    if (isCancelled() || document.visibilityState !== 'visible') return;
    await waitForNextFrame();
    scheduleICloudSyncTimersAfterChecklistIdle(isCancelled);
  }

  onMount(() => {
    let isUnmounted = false;

    mainRouteMotionClass = getMainEnterClass();
    initializeTheme();
    initializeFonts();
    void loadChecklist()
      .finally(() => {
        if (!isUnmounted) {
          isInitialChecklistLoading = false;
        }
      })
      .then(async () => {
        await runChecklistMaintenance(() => isUnmounted);
        if (isUnmounted || document.visibilityState !== 'visible') return;
        reloadChecklistAfterICloudIdle(() => isUnmounted);
        await loadICloudStatusThenSchedule(() => isUnmounted);
      })
      .catch(() => undefined);
    nativeDockSupported = nativeDockApi.shouldUseNativeDock();
    const removeNativeDockActionListener =
      nativeDockApi.addNativeDockActionListener(handleNativeDockAction);
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
      window.requestAnimationFrame(() => {
        setWebEditableFocused(isEditableElement(document.activeElement));
      });
    };
    const handleVisibilityChange = (): void => {
      if (document.visibilityState !== 'visible') {
        clearICloudSyncScheduleTimeout();
        checklistStore.disposeRepeatProcessingTimer();
        icloudSyncStore.disposeForegroundPull();
        return;
      }

      void runChecklistMaintenance(() => false).finally(() => {
        scheduleICloudSyncTimersAfterChecklistIdle(() => false);
      });
    };
    const handleICloudSyncCompleted = (event: Event): void => {
      const detail = (event as CustomEvent<{ appliedCount?: number }>).detail;
      if ((detail?.appliedCount ?? 0) <= 0) return;
      void checklistStore
        .load()
        .catch(() => undefined)
        .finally(() => {
          void checklistStore.scheduleRepeatProcessing();
        });
    };
    window.addEventListener('tickly:nativeSheetState', handleNativeSheetState);
    window.addEventListener('tickly:iCloudSyncCompleted', handleICloudSyncCompleted);
    document.addEventListener('focusin', handleFocusIn);
    document.addEventListener('focusout', handleFocusOut);
    document.addEventListener('visibilitychange', handleVisibilityChange);

    void i18n.loadLocale().finally(() => {
      syncNativeDock();
    });

    return () => {
      isUnmounted = true;
      clearICloudSyncScheduleTimeout();
      removeNativeDockActionListener();
      window.removeEventListener('tickly:nativeSheetState', handleNativeSheetState);
      window.removeEventListener('tickly:iCloudSyncCompleted', handleICloudSyncCompleted);
      document.removeEventListener('focusin', handleFocusIn);
      document.removeEventListener('focusout', handleFocusOut);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
      checklistStore.disposeRepeatProcessingTimer();
      checklistStore.disposeWidgetRefreshTimer();
      checklistStore.disposeReminderNotificationSync();
      icloudSyncStore.dispose();
      if (nativeDockSupported) {
        void nativeDockApi.configureNativeDock(buildNativeDockRequest(false));
      }
    };
  });
</script>

<ChecklistScreen
  categories={checklistStore.categories}
  selectedCategoryId={checklistStore.selectedCategoryId}
  items={checklistStore.items}
  availableTags={checklistStore.tags}
  errorMessage={checklistStore.errorMessage}
  isInitialLoading={isInitialChecklistLoading}
  routeMotionClass={mainRouteMotionClass}
  onSelectCategory={checklistStore.selectCategory}
  onAddCategory={checklistStore.addCategory}
  onUpdateCategory={checklistStore.updateCategory}
  onDeleteCategory={checklistStore.deleteCategory}
  onReorderCategories={checklistStore.reorderCategories}
  onAddItem={checklistStore.addItem}
  onToggleItem={checklistStore.toggleItem}
  onUpdateItemDetails={checklistStore.updateItemDetails}
  onDeleteItem={checklistStore.deleteItem}
  onReorderItems={checklistStore.reorderItems}
  onSearchItems={checklistStore.searchItems}
  {archiveRequestToken}
  onArchiveCompletedItems={checklistStore.archiveCompletedItems}
  {nativeDockVisible}
  onNativeDockVisibilityChange={setNativeDockRequestedVisible}
/>

<StreakOverlay
  show={showStreakOverlay}
  heatmaps={streakHeatmaps}
  isLoading={isLoadingStreakHeatmaps}
  errorMessage={streakHeatmapError}
  onRefresh={loadStreakHeatmaps}
  onClose={closeStreakOverlay}
/>

<GraphOverlay
  show={showGraphOverlay}
  data={graphData}
  isLoading={isLoadingGraphData}
  errorMessage={graphError}
  onRefresh={loadGraphData}
  onItemEdit={handleGraphItemEdit}
  onItemToggle={handleGraphItemToggle}
  onClose={closeGraphOverlay}
/>

<ItemDetailSheet
  show={graphItemPendingEdit !== null}
  item={graphItemPendingEdit}
  availableTags={checklistStore.tags}
  isSaving={isSavingGraphItemEdit}
  onSaveDetails={saveGraphItemDetails}
  onClose={cancelGraphItemEdit}
/>
