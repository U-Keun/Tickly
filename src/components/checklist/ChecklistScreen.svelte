<script lang="ts">
  import { onDestroy, onMount, tick, untrack } from 'svelte';
  import { flip } from 'svelte/animate';
  import { cubicIn, cubicOut } from 'svelte/easing';
  import { fade, fly } from 'svelte/transition';
  import { dragHandleZone } from 'svelte-dnd-action';
  import type { DndEvent } from 'svelte-dnd-action';

  import type { Category, ItemSearchResult, RepeatType, Tag, TodoItem } from '../../types';
  import { i18n } from '$lib/i18n';
  import { openNativeItemDetailSheet } from '$lib/checklist/itemDetailLauncher';
  import CategoryDetailSheet from './CategoryDetailSheet.svelte';
  import CategoryManageSheet from './CategoryManageSheet.svelte';
  import CheckboxFanfare from './CheckboxFanfare.svelte';
  import CategoryRail from './CategoryRail.svelte';
  import ConfirmModal from './ConfirmModal.svelte';
  import ItemDetailSheet from './ItemDetailSheet.svelte';
  import LeafCommandBar from './LeafCommandBar.svelte';
  import LeafTodoItem from './LeafTodoItem.svelte';
  import ModalShell from './ModalShell.svelte';
  import SearchSuggestionBoard from './SearchSuggestionBoard.svelte';
  import * as nativeSheetApi from '$lib/api/nativeSheetApi';

  type MaybePromise<T = void> = T | Promise<T>;
  const LIST_EXIT_DURATION_MS = 80;
  const LIST_EXIT_GAP_MS = 70;
  const LIST_ENTER_DURATION_MS = 160;
  const REORDER_FLIP_DURATION_MS = 180;
  const TODO_COMPLETION_MOVE_DURATION_MS = 420;
  const TODO_COMPLETION_CHECKBOX_HOP_LEAD_MS = 1060;
  const TODO_COMPLETION_MOVE_CLEANUP_MS = TODO_COMPLETION_MOVE_DURATION_MS + 80;
  const TODO_COMPLETION_FANFARE_DURATION_MS = 560;
  const ITEM_ENTRY_DURATION_MS = 300;
  const ITEM_ENTRY_CLEANUP_MS = ITEM_ENTRY_DURATION_MS + 100;
  const ITEM_EXIT_DURATION_MS = 300;
  const SEARCH_DEBOUNCE_MS = 150;
  const SEARCH_RESULT_LIMIT = 8;

  interface Props {
    categories: Category[];
    selectedCategoryId: number | null;
    items: TodoItem[];
    availableTags?: Tag[];
    errorMessage?: string | null;
    isInitialLoading?: boolean;
    initialSearchMode?: boolean;
    initialSearchQuery?: string;
    initialCategoryReorderMode?: boolean;
    initialOpenDrawerItemIds?: number[];
    routeMotionClass?: string;
    archiveRequestToken?: number;
    nativeDockVisible?: boolean;
    onSelectCategory: (id: number) => MaybePromise;
    onAddCategory: (name: string) => MaybePromise;
    onUpdateCategory: (id: number, name: string) => MaybePromise;
    onDeleteCategory: (id: number) => MaybePromise;
    onReorderCategories: (categoryIds: number[]) => MaybePromise;
    onAddItem: (text: string, tagNames?: string[]) => MaybePromise;
    onToggleItem: (id: number) => MaybePromise<TodoItem | void>;
    onUpdateItemDetails: (
      id: number,
      text: string,
      memo: string | null,
      tagNames?: string[],
      repeatType?: RepeatType,
      repeatDetail?: string | null,
      reminderAt?: string | null,
      trackStreak?: boolean
    ) => MaybePromise;
    onDeleteItem: (id: number) => MaybePromise;
    onReorderItems: (itemIds: number[]) => MaybePromise;
    onSearchItems: (query: string, limit: number) => Promise<ItemSearchResult[]>;
    onArchiveCompletedItems: (categoryId: number) => Promise<number>;
    onNativeDockVisibilityChange?: (visible: boolean) => MaybePromise;
  }

  let {
    categories,
    selectedCategoryId,
    items,
    availableTags = [],
    errorMessage = null,
    isInitialLoading = false,
    initialSearchMode = false,
    initialSearchQuery = '',
    initialCategoryReorderMode = false,
    initialOpenDrawerItemIds = [],
    routeMotionClass = '',
    archiveRequestToken = 0,
    nativeDockVisible = false,
    onSelectCategory,
    onAddCategory,
    onUpdateCategory,
    onDeleteCategory,
    onReorderCategories,
    onAddItem,
    onToggleItem,
    onUpdateItemDetails,
    onDeleteItem,
    onReorderItems,
    onSearchItems,
    onArchiveCompletedItems,
    onNativeDockVisibilityChange = () => undefined
  }: Props = $props();

  type CategoryDetailMode = 'create' | 'rename';
  type CategoryManageActionId = 'create' | 'rename' | 'editOrder' | 'delete';
  type ReorderGroup = 'active' | 'done';
  type RectSnapshot = {
    left: number;
    top: number;
    width: number;
    height: number;
  };
  type CompletionMoveOverlay = {
    id: number;
    html: string;
    from: RectSnapshot;
    deltaX: number;
    deltaY: number;
    scaleX: number;
    scaleY: number;
    isAnimating: boolean;
  };
  type CompletionFanfareOverlay = {
    id: number;
    left: number;
    top: number;
  };

  let categoryDetailMode = $state<CategoryDetailMode>('create');
  let categoryPendingDetail = $state<Category | null>(null);
  let showCategoryDetailSheet = $state(false);
  let showCategoryManageSheet = $state(false);
  let categoryPendingDeletion = $state<Category | null>(null);
  let isSavingCategory = $state(false);
  let isDeletingCategory = $state(false);
  let isCategoryReorderMode = $state(false);
  let isSavingCategoryOrder = $state(false);
  let itemPendingEdit = $state<TodoItem | null>(null);
  let isSavingItemEdit = $state(false);
  let itemPendingDeletion = $state<TodoItem | null>(null);
  let isDeletingItem = $state(false);
  let showArchiveConfirm = $state(false);
  let showArchiveEmptyNotice = $state(false);
  let isArchivingCompletedItems = $state(false);
  let lastHandledArchiveRequestToken = $state(0);
  let isSavingReorder = $state(false);
  let searchMode = $state(false);
  let searchQuery = $state('');
  let appliedSearchQuery = $state('');
  let isSuggestionBoardOpen = $state(false);
  let didApplyInitialSearchState = $state(false);
  let didApplyInitialOpenDrawerState = $state(false);
  let searchSuggestions = $state<ItemSearchResult[]>([]);
  let isSearching = $state(false);
  let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  let searchRequestToken = 0;
  let activeItems = $state<TodoItem[]>([]);
  let doneItems = $state<TodoItem[]>([]);
  let openDrawerItemIds = $state<Set<number>>(new Set());
  let lastRequestedNativeDockVisible: boolean | null = null;
  let categoryReorderDraft = $state<Category[] | null>(null);
  let isTextClickSuppressed = $state(false);
  let textClickSuppressTimer: ReturnType<typeof setTimeout> | null = null;
  let completionMoveTimer: ReturnType<typeof setTimeout> | null = null;
  let completionMoveFrame: number | null = null;
  let completionMoveFinishFrame: number | null = null;
  let completionFanfareTimer: ReturnType<typeof setTimeout> | null = null;
  let enteringItemTimers = new Map<number, ReturnType<typeof setTimeout>>();
  const itemExitAnimations = new Map<number, Animation>();
  let completionMoveToken = 0;
  let completionFanfareToken = 0;
  const itemNodes = new Map<number, HTMLElement>();
  const itemShiftAnimations = new Map<number, Animation>();
  let movingItemId = $state<number | null>(null);
  let completionMoveOverlay = $state<CompletionMoveOverlay | null>(null);
  let completionFanfareOverlay = $state<CompletionFanfareOverlay | null>(null);
  let enteringItemIds = $state<Set<number>>(new Set());
  let exitingItemIds = $state<Set<number>>(new Set());
  let displayedCategoryId = $state<number | null>(null);
  let displayedItems = $state<TodoItem[]>([]);
  let hasDisplayedList = $state(false);
  let isListContentVisible = $state(true);
  let isListSwitching = $state(false);
  let listTransitionToken = 0;
  let prefersReducedMotion = $state(false);
  let itemSignature = $derived(
    items
      .map(
        (item) =>
          `${item.id}:${item.text}:${item.memo ?? ''}:${item.done}:${item.display_order}:${item.tags.map((tag) => tag.name).join(',')}`
      )
      .join('|')
  );
  let categorySignature = $derived(
    categories
      .map((category) => `${category.id}:${category.name}:${category.display_order}`)
      .join('|')
  );
  let searchTerm = $derived(searchQuery.trim().toLocaleLowerCase());
  let hasSearchQuery = $derived(searchTerm.length > 0);
  let appliedSearchTerm = $derived(appliedSearchQuery.trim().toLocaleLowerCase());
  let hasAppliedSearchQuery = $derived(appliedSearchTerm.length > 0);
  let hasDockBlockingSurface = $derived(
    showCategoryDetailSheet ||
      showCategoryManageSheet ||
      categoryPendingDeletion !== null ||
      itemPendingEdit !== null ||
      itemPendingDeletion !== null ||
      showArchiveConfirm ||
      showArchiveEmptyNotice
  );
  let listEnterDuration = $derived(prefersReducedMotion ? 0 : LIST_ENTER_DURATION_MS);
  let listExitDuration = $derived(prefersReducedMotion ? 0 : LIST_EXIT_DURATION_MS);
  let listEnterY = $derived(prefersReducedMotion ? 0 : 4);
  let listTransitionOpacity = $derived(prefersReducedMotion ? 1 : 0.18);
  let reorderFlipDuration = $derived(prefersReducedMotion ? 0 : REORDER_FLIP_DURATION_MS);
  let itemFlipDuration = $derived(movingItemId !== null ? 0 : reorderFlipDuration);

  let selectedCategory = $derived(
    categories.find((category) => category.id === selectedCategoryId) ?? null
  );
  let displayedCategories = $derived(categoryReorderDraft ?? categories);
  let archivableCompletedItems = $derived(
    items.filter((item) => item.done && item.repeat_type === 'none')
  );
  let archivableCompletedItemCount = $derived(archivableCompletedItems.length);
  const loadingItemRows = [
    { textWidth: '62%', tagWidth: '46px' },
    { textWidth: '78%', tagWidth: '34px' },
    { textWidth: '52%', tagWidth: '58px' }
  ];

  function wait(ms: number): Promise<void> {
    return new Promise((resolve) => window.setTimeout(resolve, ms));
  }

  function itemEntry(node: Element, params: { enabled: boolean }) {
    if (!params.enabled || prefersReducedMotion) {
      return { duration: 0 };
    }

    const element = node as HTMLElement;
    const height = element.offsetHeight;

    return {
      duration: ITEM_ENTRY_DURATION_MS,
      easing: cubicOut,
      css: (t: number) => `
        max-height: ${height * t}px;
        opacity: ${t};
        overflow: hidden;
        transform: translateY(${10 * (1 - t)}px);
      `
    };
  }

  function snapshotRect(rect: DOMRect): RectSnapshot {
    return {
      left: rect.left,
      top: rect.top,
      width: rect.width,
      height: rect.height
    };
  }

  function snapshotItemRects(): Map<number, RectSnapshot> {
    const rects = new Map<number, RectSnapshot>();

    itemNodes.forEach((node, id) => {
      rects.set(id, snapshotRect(node.getBoundingClientRect()));
    });

    return rects;
  }

  function snapshotCompletionMoveHtml(node: HTMLElement): string {
    const clone = node.cloneNode(true) as HTMLElement;
    clone.querySelectorAll('.checkboxSoftHop').forEach((element) => {
      element.classList.remove('checkboxSoftHop');
    });
    clone.querySelectorAll('.tickCheck').forEach((element) => {
      element.querySelectorAll('path').forEach((path) => {
        path.setAttribute('style', 'animation: none; stroke-dashoffset: 0;');
      });
    });

    return clone.innerHTML;
  }

  function trackItemNode(node: HTMLElement, id: number): { update: (nextId: number) => void; destroy: () => void } {
    itemNodes.set(id, node);

    return {
      update(nextId: number) {
        if (nextId === id) return;
        itemNodes.delete(id);
        id = nextId;
        itemNodes.set(id, node);
      },
      destroy() {
        itemNodes.delete(id);
      }
    };
  }

  function clearItemShiftAnimations(): void {
    itemShiftAnimations.forEach((animation) => {
      animation.cancel();
    });
    itemShiftAnimations.clear();
  }

  function clearEnteringItems(): void {
    enteringItemTimers.forEach((timer) => clearTimeout(timer));
    enteringItemTimers.clear();
    enteringItemIds = new Set();
  }

  function clearEnteringItem(id: number): void {
    const timer = enteringItemTimers.get(id);
    if (timer) {
      clearTimeout(timer);
      enteringItemTimers.delete(id);
    }

    if (!enteringItemIds.has(id)) return;

    const nextEnteringIds = new Set(enteringItemIds);
    nextEnteringIds.delete(id);
    enteringItemIds = nextEnteringIds;
  }

  function setExitingItem(id: number, isExiting: boolean): void {
    const nextExitingIds = new Set(exitingItemIds);

    if (isExiting) {
      nextExitingIds.add(id);
    } else {
      nextExitingIds.delete(id);
    }

    exitingItemIds = nextExitingIds;
  }

  function clearExitingItem(id: number): void {
    const animation = itemExitAnimations.get(id);
    if (animation) {
      animation.cancel();
      itemExitAnimations.delete(id);
    }

    const node = itemNodes.get(id);
    if (node) {
      node.style.overflow = '';
      node.style.maxHeight = '';
      node.style.willChange = '';
    }

    if (exitingItemIds.has(id)) {
      setExitingItem(id, false);
    }
  }

  function clearItemExitAnimations(): void {
    itemExitAnimations.forEach((animation, id) => {
      animation.cancel();
      const node = itemNodes.get(id);
      if (node) {
        node.style.overflow = '';
        node.style.maxHeight = '';
        node.style.willChange = '';
      }
    });
    itemExitAnimations.clear();
    exitingItemIds = new Set();
  }

  function canAnimateItemEntry(categoryId: number | null): boolean {
    return (
      hasDisplayedList &&
      displayedCategoryId === categoryId &&
      !prefersReducedMotion &&
      !hasAppliedSearchQuery &&
      !isListSwitching &&
      !isCategoryReorderMode &&
      !isSavingReorder &&
      movingItemId === null
    );
  }

  function canAnimateItemExit(): boolean {
    return (
      !prefersReducedMotion &&
      !hasAppliedSearchQuery &&
      !isListSwitching &&
      !isCategoryReorderMode &&
      !isSavingReorder &&
      movingItemId === null
    );
  }

  function markEnteringItems(categoryId: number | null, nextItems: TodoItem[]): void {
    if (!canAnimateItemEntry(categoryId)) return;

    const previousIds = new Set(displayedItems.map((item) => item.id));
    const nextEnteringIds = nextItems
      .filter((item) => !item.done && !previousIds.has(item.id))
      .map((item) => item.id);

    if (nextEnteringIds.length === 0) return;

    enteringItemIds = new Set([...enteringItemIds, ...nextEnteringIds]);

    nextEnteringIds.forEach((id) => {
      const existingTimer = enteringItemTimers.get(id);
      if (existingTimer) clearTimeout(existingTimer);

      const timer = setTimeout(() => {
        clearEnteringItem(id);
      }, ITEM_ENTRY_CLEANUP_MS);

      enteringItemTimers.set(id, timer);
    });
  }

  async function animateItemExit(id: number): Promise<boolean> {
    if (!canAnimateItemExit()) return false;

    const node = itemNodes.get(id);
    if (!node) return false;

    clearEnteringItem(id);
    setExitingItem(id, true);

    const height = node.offsetHeight;
    node.style.overflow = 'hidden';
    node.style.maxHeight = `${height}px`;
    node.style.willChange = 'max-height, opacity, transform';

    const animation = node.animate(
      [
        {
          maxHeight: `${height}px`,
          opacity: '1',
          transform: 'translateY(0)'
        },
        {
          maxHeight: '0px',
          opacity: '0',
          transform: 'translateY(10px)'
        }
      ],
      {
        duration: ITEM_EXIT_DURATION_MS,
        easing: 'cubic-bezier(0.2, 0.8, 0.2, 1)',
        fill: 'forwards'
      }
    );

    itemExitAnimations.set(id, animation);

    try {
      await animation.finished;
    } catch {
      return false;
    }

    return true;
  }

  function animateItemShifts(beforeRects: Map<number, RectSnapshot>, movingId: number): void {
    clearItemShiftAnimations();
    if (prefersReducedMotion) return;

    beforeRects.forEach((before, id) => {
      if (id === movingId) return;

      const node = itemNodes.get(id);
      if (!node) return;

      const after = node.getBoundingClientRect();
      const deltaX = before.left - after.left;
      const deltaY = before.top - after.top;
      if (Math.abs(deltaX) < 0.5 && Math.abs(deltaY) < 0.5) return;

      const animation = node.animate(
        [
          { transform: `translate3d(${deltaX}px, ${deltaY}px, 0)` },
          { transform: 'translate3d(0, 0, 0)' }
        ],
        {
          duration: TODO_COMPLETION_MOVE_DURATION_MS,
          easing: 'cubic-bezier(0.2, 0.8, 0.2, 1)'
        }
      );

      itemShiftAnimations.set(id, animation);

      const clearAnimation = () => {
        if (itemShiftAnimations.get(id) === animation) {
          itemShiftAnimations.delete(id);
        }
      };

      animation.onfinish = clearAnimation;
      animation.oncancel = clearAnimation;
    });
  }

  function clearCompletionMove(): void {
    completionMoveToken += 1;
    clearItemShiftAnimations();

    if (completionMoveTimer) {
      clearTimeout(completionMoveTimer);
      completionMoveTimer = null;
    }

    if (completionMoveFrame !== null) {
      cancelAnimationFrame(completionMoveFrame);
      completionMoveFrame = null;
    }

    if (completionMoveFinishFrame !== null) {
      cancelAnimationFrame(completionMoveFinishFrame);
      completionMoveFinishFrame = null;
    }

    completionMoveOverlay = null;
    movingItemId = null;
  }

  function clearCompletionFanfare(): void {
    completionFanfareToken += 1;

    if (completionFanfareTimer) {
      clearTimeout(completionFanfareTimer);
      completionFanfareTimer = null;
    }

    completionFanfareOverlay = null;
  }

  function finishCompletionMove(id: number, token: number): void {
    if (completionMoveTimer) {
      clearTimeout(completionMoveTimer);
      completionMoveTimer = null;
    }

    if (token !== completionMoveToken || completionMoveOverlay?.id !== id) return;

    movingItemId = null;

    completionMoveFinishFrame = requestAnimationFrame(() => {
      completionMoveFinishFrame = requestAnimationFrame(() => {
        completionMoveFinishFrame = null;
        if (token === completionMoveToken && completionMoveOverlay?.id === id) {
          completionMoveOverlay = null;
        }
      });
    });
  }

  function canAnimateCompletionMove(): boolean {
    return (
      !prefersReducedMotion &&
      !hasAppliedSearchQuery &&
      !isListSwitching &&
      !isCategoryReorderMode &&
      !isSavingReorder
    );
  }

  function completionMoveGhostStyle(overlay: CompletionMoveOverlay): string {
    const transform = overlay.isAnimating
      ? `translate3d(${overlay.deltaX}px, ${overlay.deltaY}px, 0) scale(${overlay.scaleX}, ${overlay.scaleY})`
      : 'translate3d(0, 0, 0) scale(1)';

    return [
      `left: ${overlay.from.left}px`,
      `top: ${overlay.from.top}px`,
      `width: ${overlay.from.width}px`,
      `height: ${overlay.from.height}px`,
      `transform: ${transform}`,
      `--completion-move-duration: ${TODO_COMPLETION_MOVE_DURATION_MS}ms`
    ].join('; ');
  }

  function completionFanfareStyle(overlay: CompletionFanfareOverlay): string {
    return [`left: ${overlay.left}px`, `top: ${overlay.top}px`].join('; ');
  }

  function updateDisplayedList(categoryId: number | null, nextItems: TodoItem[]): void {
    displayedCategoryId = categoryId;
    displayedItems = nextItems;
    splitDisplayedItems(nextItems);
  }

  function splitDisplayedItems(nextItems: TodoItem[]): void {
    const nextVisibleItems = filterItemsForSearch(nextItems);
    activeItems = nextVisibleItems.filter((item) => !item.done);
    doneItems = nextVisibleItems.filter((item) => item.done);
  }

  function setItemDrawerOpen(id: number, open: boolean): void {
    const nextOpenDrawerItemIds = new Set(openDrawerItemIds);

    if (open) {
      nextOpenDrawerItemIds.add(id);
    } else {
      nextOpenDrawerItemIds.delete(id);
    }

    openDrawerItemIds = nextOpenDrawerItemIds;
  }

  function clearOpenItemDrawers(): void {
    if (openDrawerItemIds.size === 0) return;
    openDrawerItemIds = new Set();
  }

  function pruneOpenItemDrawers(validItems: TodoItem[]): void {
    if (openDrawerItemIds.size === 0) return;

    const validIds = new Set(validItems.map((item) => item.id));
    const nextOpenDrawerItemIds = new Set(
      [...openDrawerItemIds].filter((id) => validIds.has(id))
    );

    if (nextOpenDrawerItemIds.size !== openDrawerItemIds.size) {
      openDrawerItemIds = nextOpenDrawerItemIds;
    }
  }

  function filterItemsForSearch(nextItems: TodoItem[]): TodoItem[] {
    if (!hasAppliedSearchQuery) return nextItems;
    return nextItems.filter((item) => {
      const memo = item.memo ?? '';
      return (
        item.text.toLocaleLowerCase().includes(appliedSearchTerm) ||
        memo.toLocaleLowerCase().includes(appliedSearchTerm) ||
        item.tags.some((tag) => tag.name.toLocaleLowerCase().includes(appliedSearchTerm))
      );
    });
  }

  async function transitionDisplayedList(
    categoryId: number | null,
    nextItems: TodoItem[]
  ): Promise<void> {
    const token = ++listTransitionToken;
    clearEnteringItems();
    clearOpenItemDrawers();

    if (prefersReducedMotion) {
      updateDisplayedList(categoryId, nextItems);
      isListContentVisible = true;
      isListSwitching = false;
      return;
    }

    isListSwitching = true;
    isListContentVisible = false;

    await wait(LIST_EXIT_DURATION_MS + LIST_EXIT_GAP_MS);
    if (token !== listTransitionToken) return;

    updateDisplayedList(categoryId, nextItems);
    isListContentVisible = true;

    await wait(LIST_ENTER_DURATION_MS);
    if (token === listTransitionToken) {
      isListSwitching = false;
    }
  }

  $effect(() => {
    if (didApplyInitialSearchState) return;
    searchMode = initialSearchMode;
    searchQuery = initialSearchQuery;
    isSuggestionBoardOpen = initialSearchMode && initialSearchQuery.trim().length > 0;
    isCategoryReorderMode = initialCategoryReorderMode && categories.length > 1;
    didApplyInitialSearchState = true;
  });

  $effect(() => {
    if (didApplyInitialOpenDrawerState) return;
    if (initialOpenDrawerItemIds.length > 0) {
      openDrawerItemIds = new Set(initialOpenDrawerItemIds);
    }
    didApplyInitialOpenDrawerState = true;
  });

  $effect(() => {
    const nextCategoryId = selectedCategoryId;
    const nextItems = items;
    itemSignature;

    untrack(() => {
      if (!hasDisplayedList || displayedCategoryId === null || nextCategoryId === displayedCategoryId) {
        markEnteringItems(nextCategoryId, nextItems);
        updateDisplayedList(nextCategoryId, nextItems);
        hasDisplayedList = true;
        isListContentVisible = true;
        isListSwitching = false;
        return;
      }

      void transitionDisplayedList(nextCategoryId, nextItems);
    });
  });

  $effect(() => {
    itemSignature;
    pruneOpenItemDrawers(items);
  });

  $effect(() => {
    displayedItems;
    appliedSearchTerm;
    if (isSavingReorder) return;
    splitDisplayedItems(displayedItems);
  });

  $effect(() => {
    categorySignature;
    isSavingCategoryOrder;
    if (!isSavingCategoryOrder) {
      categoryReorderDraft = null;
    }
    if (isCategoryReorderMode && categories.length <= 1) {
      isCategoryReorderMode = false;
    }
  });

  $effect(() => {
    searchMode;
    searchTerm;
    selectedCategoryId;
    scheduleSearchSuggestions();
  });

  $effect(() => {
    const nextArchiveRequestToken = archiveRequestToken;
    if (nextArchiveRequestToken === lastHandledArchiveRequestToken) return;

    lastHandledArchiveRequestToken = nextArchiveRequestToken;
    untrack(openArchivePrompt);
  });

  $effect(() => {
    const nextVisible = !hasDockBlockingSurface;
    if (lastRequestedNativeDockVisible === nextVisible) return;

    lastRequestedNativeDockVisible = nextVisible;
    untrack(() => {
      void onNativeDockVisibilityChange(nextVisible);
    });
  });

  function openCreateCategorySheet(): void {
    if (isCategoryReorderMode) return;

    showCategoryManageSheet = false;
    categoryPendingDetail = null;
    void openCategoryTextSheet('create', null);
  }

  function openCategoryManageSheet(category: Category): void {
    if (isCategoryReorderMode) return;

    void openCategoryManageActions(category);
  }

  function openWebCategoryManageSheet(category: Category): void {
    categoryPendingDetail = category;
    showCategoryManageSheet = true;
  }

  function canOpenWebBottomSheetFallback(): boolean {
    return !nativeSheetApi.shouldUseNativeSheets();
  }

  async function openCategoryManageActions(category: Category): Promise<void> {
    showCategoryManageSheet = false;
    showCategoryDetailSheet = false;
    categoryPendingDetail = category;

    const nativeResult = await nativeSheetApi.openNativeActionSheet({
      title: i18n.t('checklistManageCategory'),
      message: category.name,
      cancelLabel: i18n.t('cancel'),
      actions: [
        {
          id: 'create',
          label: i18n.t('checklistAddCategory'),
          tone: 'neutral'
        },
        {
          id: 'rename',
          label: i18n.t('checklistRenameCategoryActionTemplate')(category.name),
          tone: 'neutral'
        },
        {
          id: 'editOrder',
          label: i18n.t('checklistEditCategoryOrder'),
          tone: 'neutral',
          disabled: categories.length <= 1
        },
        {
          id: 'delete',
          label: i18n.t('checklistDeleteCategory'),
          tone: 'danger',
          disabled: categories.length <= 1
        }
      ]
    });

    if (nativeResult.status === 'unavailable') {
      if (!canOpenWebBottomSheetFallback()) {
        categoryPendingDetail = null;
        return;
      }

      openWebCategoryManageSheet(category);
      return;
    }

    if (nativeResult.status !== 'action') {
      categoryPendingDetail = null;
      return;
    }

    handleNativeCategoryManageAction(category, nativeResult.actionId as CategoryManageActionId);
  }

  function handleNativeCategoryManageAction(
    category: Category,
    actionId: CategoryManageActionId
  ): void {
    if (actionId === 'create') {
      openCreateCategorySheet();
      return;
    }

    if (actionId === 'rename') {
      void openCategoryTextSheet('rename', category);
      return;
    }

    if (actionId === 'editOrder') {
      categoryPendingDetail = null;
      enterCategoryReorderMode();
      return;
    }

    if (actionId === 'delete' && categories.length > 1) {
      categoryPendingDeletion = category;
      categoryPendingDetail = null;
    }
  }

  function enterCategoryReorderMode(): void {
    if (categories.length <= 1) return;

    isCategoryReorderMode = true;
    isSuggestionBoardOpen = false;
    clearOpenItemDrawers();
    showCategoryManageSheet = false;
    showCategoryDetailSheet = false;
    categoryPendingDetail = null;
  }

  function finishCategoryReorderMode(): void {
    if (isSavingCategoryOrder) return;

    isCategoryReorderMode = false;
    categoryReorderDraft = null;
  }

  function handleCategoryReorderConsider(nextCategories: Category[]): void {
    categoryReorderDraft = nextCategories;
  }

  async function handleCategoryReorderFinalize(nextCategories: Category[]): Promise<void> {
    categoryReorderDraft = nextCategories;
    if (isSavingCategoryOrder) return;

    isSavingCategoryOrder = true;
    try {
      await onReorderCategories(nextCategories.map((category) => category.id));
    } catch {
      categoryReorderDraft = null;
    } finally {
      isSavingCategoryOrder = false;
    }
  }

  async function selectCategoryWithTransition(id: number): Promise<void> {
    if (isCategoryReorderMode) return;
    if (id === selectedCategoryId) return;

    clearOpenItemDrawers();
    await onSelectCategory(id);
  }

  function clearSearchDebounceTimer(): void {
    if (!searchDebounceTimer) return;
    clearTimeout(searchDebounceTimer);
    searchDebounceTimer = null;
  }

  function resetSearchSuggestions(): void {
    clearSearchDebounceTimer();
    searchRequestToken += 1;
    searchSuggestions = [];
    isSearching = false;
  }

  function prioritizeSearchResults(results: ItemSearchResult[]): ItemSearchResult[] {
    if (selectedCategoryId === null) return results;

    return results
      .map((result, index) => ({ result, index }))
      .sort((a, b) => {
        const aSelected = a.result.category.id === selectedCategoryId;
        const bSelected = b.result.category.id === selectedCategoryId;
        if (aSelected !== bSelected) return aSelected ? -1 : 1;
        return a.index - b.index;
      })
      .map(({ result }) => result);
  }

  function scheduleSearchSuggestions(): void {
    clearSearchDebounceTimer();
    const query = searchQuery.trim();
    const token = ++searchRequestToken;

    if (!searchMode || !query) {
      searchSuggestions = [];
      isSearching = false;
      return;
    }

    isSearching = true;
    searchDebounceTimer = setTimeout(() => {
      searchDebounceTimer = null;
      void (async () => {
        try {
          const results = await onSearchItems(query, SEARCH_RESULT_LIMIT);
          if (token === searchRequestToken) {
            searchSuggestions = prioritizeSearchResults(results);
          }
        } catch {
          if (token === searchRequestToken) {
            searchSuggestions = [];
          }
        } finally {
          if (token === searchRequestToken) {
            isSearching = false;
          }
        }
      })();
    }, SEARCH_DEBOUNCE_MS);
  }

  function enterSearchMode(): void {
    if (isCategoryReorderMode) return;

    searchMode = true;
    isSuggestionBoardOpen = hasSearchQuery;
  }

  function exitSearchMode(): void {
    searchMode = false;
    searchQuery = '';
    appliedSearchQuery = '';
    isSuggestionBoardOpen = false;
    clearOpenItemDrawers();
    resetSearchSuggestions();
    splitDisplayedItems(displayedItems);
  }

  function updateSearchQuery(query: string): void {
    searchQuery = query;
    isSuggestionBoardOpen = query.trim().length > 0;
    if (!query.trim()) {
      appliedSearchQuery = '';
      clearOpenItemDrawers();
    }
  }

  function openSearchSuggestionBoard(): void {
    if (searchMode && hasSearchQuery) {
      isSuggestionBoardOpen = true;
    }
  }

  async function selectSearchSuggestion(result: ItemSearchResult): Promise<void> {
    isSuggestionBoardOpen = false;
    appliedSearchQuery = searchQuery.trim();
    clearOpenItemDrawers();
    if (result.category.id === selectedCategoryId) return;

    try {
      await onSelectCategory(result.category.id);
    } catch {
      // The checklist store owns the visible error banner; keep the search query in place.
    }
  }

  function suppressTextClickFor(durationMs: number): void {
    if (textClickSuppressTimer) {
      clearTimeout(textClickSuppressTimer);
    }

    isTextClickSuppressed = true;
    textClickSuppressTimer = setTimeout(() => {
      isTextClickSuppressed = false;
      textClickSuppressTimer = null;
    }, durationMs);
  }

  function reorderGroupItems(group: ReorderGroup, nextItems: TodoItem[]): void {
    if (group === 'active') {
      activeItems = nextItems;
      return;
    }

    doneItems = nextItems;
  }

  function orderedReorderItemIds(): number[] {
    return [...activeItems, ...doneItems].map((item) => item.id);
  }

  function keepDraggedItemQuiet(element: HTMLElement | undefined): void {
    if (!element) return;
    element.style.outline = 'none';
  }

  function handleReorderConsider(
    group: ReorderGroup,
    event: CustomEvent<DndEvent<TodoItem>>
  ): void {
    reorderGroupItems(group, event.detail.items);
    suppressTextClickFor(900);
  }

  async function handleReorderFinalize(
    group: ReorderGroup,
    event: CustomEvent<DndEvent<TodoItem>>
  ): Promise<void> {
    reorderGroupItems(group, event.detail.items);
    if (isSavingReorder) return;

    isSavingReorder = true;
    try {
      await onReorderItems(orderedReorderItemIds());
    } catch {
      splitDisplayedItems(displayedItems);
    } finally {
      isSavingReorder = false;
      suppressTextClickFor(280);
    }
  }

  async function handleToggleItem(id: number): Promise<void> {
    if (!canAnimateCompletionMove()) {
      await onToggleItem(id);
      return;
    }

    await tick();

    const sourceItem = displayedItems.find((item) => item.id === id);
    if (sourceItem && !sourceItem.done) {
      await wait(TODO_COMPLETION_CHECKBOX_HOP_LEAD_MS);
      await tick();
    }

    const sourceNode = itemNodes.get(id);
    if (!sourceNode) {
      await onToggleItem(id);
      return;
    }

    const sourceRect = snapshotRect(sourceNode.getBoundingClientRect());
    const sourceHtml = snapshotCompletionMoveHtml(sourceNode);
    const sourceDrawerOpen = openDrawerItemIds.has(id);
    const beforeRects = snapshotItemRects();

    clearCompletionMove();
    const moveToken = ++completionMoveToken;
    completionMoveOverlay = {
      id,
      html: sourceHtml,
      from: sourceRect,
      deltaX: 0,
      deltaY: 0,
      scaleX: 1,
      scaleY: 1,
      isAnimating: false
    };
    movingItemId = id;

    try {
      await onToggleItem(id);
      await tick();
      if (sourceDrawerOpen) {
        await tick();
      }
      animateItemShifts(beforeRects, id);

      const targetNode = itemNodes.get(id);
      if (!targetNode) {
        clearCompletionMove();
        return;
      }

      const targetRect = snapshotRect(targetNode.getBoundingClientRect());
      const scaleX = sourceRect.width > 0 ? targetRect.width / sourceRect.width : 1;
      const scaleY = sourceDrawerOpen
        ? 1
        : sourceRect.height > 0
          ? targetRect.height / sourceRect.height
          : 1;

      completionMoveOverlay = {
        id,
        html: sourceHtml,
        from: sourceRect,
        deltaX: targetRect.left - sourceRect.left,
        deltaY: targetRect.top - sourceRect.top,
        scaleX,
        scaleY,
        isAnimating: false
      };

      await tick();

      completionMoveFrame = requestAnimationFrame(() => {
        completionMoveFrame = null;
        if (moveToken === completionMoveToken && completionMoveOverlay?.id === id) {
          completionMoveOverlay = {
            ...completionMoveOverlay,
            isAnimating: true
          };
        }
      });

      completionMoveTimer = setTimeout(() => {
        finishCompletionMove(id, moveToken);
      }, TODO_COMPLETION_MOVE_CLEANUP_MS);
    } catch (error) {
      clearCompletionMove();
      throw error;
    }
  }

  function requestCompletionFanfare(rect: DOMRect): void {
    if (prefersReducedMotion) return;

    if (completionFanfareTimer) {
      clearTimeout(completionFanfareTimer);
      completionFanfareTimer = null;
    }

    const fanfareToken = ++completionFanfareToken;
    completionFanfareOverlay = {
      id: fanfareToken,
      left: rect.right - 18,
      top: rect.top - 20
    };

    completionFanfareTimer = setTimeout(() => {
      if (fanfareToken === completionFanfareToken) {
        completionFanfareOverlay = null;
        completionFanfareTimer = null;
      }
    }, TODO_COMPLETION_FANFARE_DURATION_MS);
  }

  function openArchivePrompt(): void {
    isSuggestionBoardOpen = false;
    clearOpenItemDrawers();

    if (!selectedCategory || archivableCompletedItemCount === 0) {
      showArchiveConfirm = false;
      showArchiveEmptyNotice = true;
      return;
    }

    showArchiveEmptyNotice = false;
    showArchiveConfirm = true;
  }

  function cancelArchiveCompletedItems(): void {
    if (isArchivingCompletedItems) return;
    showArchiveConfirm = false;
  }

  function closeArchiveEmptyNotice(): void {
    showArchiveEmptyNotice = false;
  }

  async function confirmArchiveCompletedItems(): Promise<void> {
    if (selectedCategoryId === null || isArchivingCompletedItems) return;

    isArchivingCompletedItems = true;
    try {
      const archivedCount = await onArchiveCompletedItems(selectedCategoryId);
      showArchiveConfirm = false;
      if (archivedCount === 0) {
        showArchiveEmptyNotice = true;
      }
      clearOpenItemDrawers();
    } catch {
      // The checklist store owns the visible error banner; keep the confirm modal open.
    } finally {
      isArchivingCompletedItems = false;
    }
  }

  function requestRenameCategory(): void {
    const category = categoryPendingDetail ?? selectedCategory;
    if (!category) return;

    void openCategoryTextSheet('rename', category);
  }

  function requestEditCategoryOrder(): void {
    showCategoryManageSheet = false;
    enterCategoryReorderMode();
  }

  function closeCategoryDetailSheet(): void {
    if (isSavingCategory) return;

    showCategoryDetailSheet = false;
    categoryPendingDetail = null;
  }

  function closeCategoryManageSheet(): void {
    if (isDeletingCategory) return;

    showCategoryManageSheet = false;
    categoryPendingDetail = null;
  }

  function openWebCategoryDetailSheet(mode: CategoryDetailMode, category: Category | null): void {
    categoryDetailMode = mode;
    categoryPendingDetail = category;
    showCategoryManageSheet = false;
    showCategoryDetailSheet = true;
  }

  async function openCategoryTextSheet(
    mode: CategoryDetailMode,
    category: Category | null
  ): Promise<void> {
    showCategoryManageSheet = false;
    showCategoryDetailSheet = false;
    categoryPendingDetail = null;

    const nativeResult = await nativeSheetApi.openNativeTextSheet({
      title:
        mode === 'create'
          ? i18n.t('checklistCreateCategoryTitle')
          : i18n.t('checklistRenameCategoryTitle'),
      label: i18n.t('checklistCategoryNameLabel'),
      placeholder: i18n.t('checklistNewCategoryPlaceholder'),
      initialValue: mode === 'rename' ? (category?.name ?? '') : '',
      confirmLabel: i18n.t('checklistSaveCategory'),
      cancelLabel: i18n.t('cancel')
    });

    if (nativeResult.status === 'unavailable') {
      if (!canOpenWebBottomSheetFallback()) {
        categoryPendingDetail = null;
        return;
      }

      openWebCategoryDetailSheet(mode, category);
      return;
    }

    if (nativeResult.status === 'saved') {
      try {
        await persistCategoryName(mode, category, nativeResult.value);
      } catch {
        // The checklist store owns the visible error banner.
      }
    }
  }

  async function persistCategoryName(
    mode: CategoryDetailMode,
    category: Category | null,
    name: string
  ): Promise<void> {
    if (isSavingCategory) return;

    isSavingCategory = true;
    try {
      if (mode === 'create') {
        await onAddCategory(name);
      } else if (category) {
        await onUpdateCategory(category.id, name);
      }
    } finally {
      isSavingCategory = false;
    }
  }

  async function saveCategoryName(name: string): Promise<void> {
    await persistCategoryName(categoryDetailMode, categoryPendingDetail, name);
  }

  function requestDeleteCategory(): void {
    const category = categoryPendingDetail ?? selectedCategory;
    if (!category || categories.length <= 1) return;

    categoryPendingDeletion = category;
  }

  function cancelDeleteCategory(): void {
    if (isDeletingCategory) return;
    categoryPendingDeletion = null;
  }

  async function confirmDeleteCategory(): Promise<void> {
    if (!categoryPendingDeletion || isDeletingCategory) return;

    isDeletingCategory = true;
    try {
      await onDeleteCategory(categoryPendingDeletion.id);
      categoryPendingDeletion = null;
      showCategoryManageSheet = false;
    } catch {
      // The checklist store owns the visible error banner; keep the confirm modal open.
    } finally {
      isDeletingCategory = false;
    }
  }

  function requestEditItem(item: TodoItem): void {
    void openItemDetailSheet(item);
  }

  function cancelEditItem(): void {
    if (isSavingItemEdit) return;
    itemPendingEdit = null;
  }

  async function openItemDetailSheet(item: TodoItem): Promise<void> {
    const nativeResult = await openNativeItemDetailSheet(item, availableTags);

    if (nativeResult.status === 'unavailable') {
      if (!canOpenWebBottomSheetFallback()) {
        return;
      }

      itemPendingEdit = item;
      return;
    }

    if (nativeResult.status === 'saved') {
      try {
        const { values } = nativeResult;
        await saveItemDetails(
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

  async function saveItemDetails(
    id: number,
    text: string,
    memo: string | null,
    tagNames: string[] = [],
    repeatType: RepeatType = 'none',
    repeatDetail: string | null = null,
    reminderAt: string | null = null,
    trackStreak = false
  ): Promise<void> {
    if (isSavingItemEdit) return;

    isSavingItemEdit = true;
    try {
      await onUpdateItemDetails(
        id,
        text,
        memo,
        tagNames,
        repeatType,
        repeatDetail,
        reminderAt,
        repeatType !== 'none' && trackStreak
      );
    } finally {
      isSavingItemEdit = false;
    }
  }

  function requestDeleteItem(item: TodoItem): void {
    itemPendingDeletion = item;
  }

  function cancelDeleteItem(): void {
    if (isDeletingItem) return;
    itemPendingDeletion = null;
  }

  async function confirmDeleteItem(): Promise<void> {
    if (!itemPendingDeletion || isDeletingItem) return;

    const itemToDelete = itemPendingDeletion;
    isDeletingItem = true;
    itemPendingDeletion = null;
    setItemDrawerOpen(itemToDelete.id, false);

    try {
      const didAnimateExit = await animateItemExit(itemToDelete.id);
      await onDeleteItem(itemToDelete.id);
      await tick();

      if (didAnimateExit) {
        clearExitingItem(itemToDelete.id);
      }
    } catch {
      clearExitingItem(itemToDelete.id);
      // The checklist store owns the visible error banner; keep the item visible for retry.
    } finally {
      isDeletingItem = false;
    }
  }

  onMount(() => {
    const motionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
    const handleMotionPreferenceChange = (event: MediaQueryListEvent): void => {
      prefersReducedMotion = event.matches;
    };

    prefersReducedMotion = motionQuery.matches;
    motionQuery.addEventListener('change', handleMotionPreferenceChange);

    return () => {
      motionQuery.removeEventListener('change', handleMotionPreferenceChange);
    };
  });

  onDestroy(() => {
    clearSearchDebounceTimer();
    clearCompletionMove();
    clearCompletionFanfare();
    clearItemShiftAnimations();
    clearEnteringItems();
    clearItemExitAnimations();
    if (textClickSuppressTimer) {
      clearTimeout(textClickSuppressTimer);
      textClickSuppressTimer = null;
    }
  });

</script>

<div
  class={`app-container full-bleed-app-container isolate flex min-w-0 flex-col overflow-hidden bg-canvas text-ink ${routeMotionClass}`}
>
  <main class="relative z-10 mx-auto flex min-h-0 w-full min-w-0 max-w-2xl flex-1 flex-col overflow-hidden">
    <section
      class={`relative flex min-h-0 w-full min-w-0 flex-1 flex-col overflow-hidden pl-[max(1rem,var(--safe-area-left))] pr-[max(1rem,var(--safe-area-right))] pt-[max(0.75rem,var(--safe-area-top))] ${
        nativeDockVisible ? 'pb-0' : 'pb-[max(0.75rem,var(--safe-area-bottom))]'
      }`}
    >
      <div class="relative z-50 mb-4">
        <LeafCommandBar
          mode={searchMode ? 'search' : 'add'}
          searchQuery={searchQuery}
          {availableTags}
          disabled={selectedCategoryId === null || isCategoryReorderMode}
          onAddItem={onAddItem}
          onEnterSearch={enterSearchMode}
          onExitSearch={exitSearchMode}
          onSearchQueryChange={updateSearchQuery}
          onSearchInputFocus={openSearchSuggestionBoard}
        />

        {#if searchMode && hasSearchQuery && isSuggestionBoardOpen}
          <div class="absolute left-0 right-0 top-[calc(100%+8px)] z-40">
            <SearchSuggestionBoard
              query={searchQuery}
              results={searchSuggestions}
              isLoading={isSearching}
              onSelectResult={selectSearchSuggestion}
            />
          </div>
        {/if}
      </div>

      {#if errorMessage}
        <div class="mb-3 rounded-md border border-accent-peach-strong bg-accent-peach px-3 py-2 text-sm text-ink">
          {errorMessage}
        </div>
      {/if}

      <div class="relative z-30 mb-4">
        <CategoryRail
          categories={displayedCategories}
          {selectedCategoryId}
          isReorderMode={isCategoryReorderMode}
          isReorderBusy={isSavingCategoryOrder}
          onSelectCategory={selectCategoryWithTransition}
          onManageCategory={openCategoryManageSheet}
          onEnterReorderMode={enterCategoryReorderMode}
          onFinishReorderMode={finishCategoryReorderMode}
          onReorderConsider={handleCategoryReorderConsider}
          onReorderFinalize={handleCategoryReorderFinalize}
        />
      </div>

      <div
        class="todoListViewport relative min-h-0 w-full min-w-0 max-w-full flex-1"
        class:hasNativeDock={nativeDockVisible}
      >
        <div class="todo-list-scroll h-full w-full min-w-0 max-w-full" aria-busy={isListSwitching}>
          <div class={`grid min-h-full w-full min-w-0 max-w-full overflow-hidden ${isListSwitching ? 'pointer-events-none' : ''}`}>
            {#if isListContentVisible}
              <div
                class="col-start-1 row-start-1 min-h-0 w-full min-w-0 max-w-full will-change-transform"
                in:fly={{
                  y: listEnterY,
                  duration: listEnterDuration,
                  opacity: listTransitionOpacity,
                  easing: cubicOut
                }}
                out:fade={{ duration: listExitDuration, easing: cubicIn }}
              >
                {#if isInitialLoading}
                  <div
                    class={`flex w-full min-w-0 max-w-full flex-col gap-2 ${nativeDockVisible ? 'pb-28' : 'pb-16'}`}
                    aria-busy="true"
                    aria-label={i18n.t('checklistLoadingItems')}
                  >
                    {#each loadingItemRows as row}
                      <div class="min-h-[60px] w-full min-w-0 overflow-hidden rounded-[6px_24px_6px_24px] border-2 border-stroke bg-paper px-3 py-2">
                        <div class="flex min-h-11 items-center gap-3">
                          <div class="h-10 w-10 shrink-0 animate-pulse rounded-[10px] border-2 border-stroke bg-white/80"></div>
                          <div class="min-w-0 flex-1">
                            <div
                              class="h-4 animate-pulse rounded-full bg-stroke/70"
                              style={`width: ${row.textWidth}`}
                            ></div>
                          </div>
                          <div
                            class="h-5 shrink-0 animate-pulse rounded-full bg-canvas"
                            style={`width: ${row.tagWidth}`}
                          ></div>
                        </div>
                      </div>
                    {/each}
                  </div>
                {:else if activeItems.length === 0 && doneItems.length === 0}
                  <div class="px-6 py-10 text-center text-ink-muted">
                    {#if hasAppliedSearchQuery}
                      <p class="font-medium text-ink">{i18n.t('checklistNoSearchResultsTemplate')(appliedSearchQuery)}</p>
                      <p class="mt-1 text-sm">{selectedCategory?.name ?? i18n.t('checklistCategories')}</p>
                    {:else}
                      <p class="font-medium text-ink">{i18n.t('checklistEmptyItemsTitle')}</p>
                      <p class="mt-1 text-sm">{i18n.t('checklistEmptyItemsSubtitle')}</p>
                    {/if}
                  </div>
                {:else}
                  <div class={`flex w-full min-w-0 max-w-full flex-col gap-2 ${nativeDockVisible ? 'pb-28' : 'pb-16'}`}>
                    {#if activeItems.length > 0}
                      <div
                        use:dragHandleZone={{
                          items: activeItems,
                          flipDurationMs: reorderFlipDuration,
                          type: 'checklist-active-items',
                          dragDisabled:
                            isSavingReorder ||
                            isListSwitching ||
                            hasAppliedSearchQuery ||
                            isCategoryReorderMode,
                          morphDisabled: true,
                          dropFromOthersDisabled: true,
                          dropTargetStyle: { outline: 'none' },
                          dropTargetClasses: [],
                          delayTouchStart: 450,
                          transformDraggedElement: keepDraggedItemQuiet
                        }}
                        onconsider={(event) => handleReorderConsider('active', event)}
                        onfinalize={(event) => void handleReorderFinalize('active', event)}
                        class="flex w-full min-w-0 max-w-full flex-col gap-2"
                      >
                        {#each activeItems as item (item.id)}
                          <div
                            use:trackItemNode={item.id}
                            in:itemEntry={{ enabled: enteringItemIds.has(item.id) }}
                            animate:flip={{ duration: itemFlipDuration }}
                            class={`relative z-10 w-full min-w-0 max-w-full outline-none focus:outline-none focus-visible:outline-none ${
                              movingItemId === item.id ? 'completionMoveHidden' : ''
                            } ${exitingItemIds.has(item.id) ? 'itemExitCollapsing' : ''}`}
                          >
                            <LeafTodoItem
                              {item}
                              drawerOpen={openDrawerItemIds.has(item.id)}
                              drawerOpenImmediate={movingItemId === item.id}
                              onToggleItem={handleToggleItem}
                              onDrawerOpenChange={setItemDrawerOpen}
                              isTextClickSuppressed={isTextClickSuppressed}
                              onRequestEditItem={requestEditItem}
                              onRequestDeleteItem={requestDeleteItem}
                              onRequestCompleteFanfare={requestCompletionFanfare}
                            />
                          </div>
                        {/each}
                      </div>
                    {/if}

                    {#if doneItems.length > 0}
                      <div
                        use:dragHandleZone={{
                          items: doneItems,
                          flipDurationMs: reorderFlipDuration,
                          type: 'checklist-done-items',
                          dragDisabled:
                            isSavingReorder ||
                            isListSwitching ||
                            hasAppliedSearchQuery ||
                            isCategoryReorderMode,
                          morphDisabled: true,
                          dropFromOthersDisabled: true,
                          dropTargetStyle: { outline: 'none' },
                          dropTargetClasses: [],
                          delayTouchStart: 450,
                          transformDraggedElement: keepDraggedItemQuiet
                        }}
                        onconsider={(event) => handleReorderConsider('done', event)}
                        onfinalize={(event) => void handleReorderFinalize('done', event)}
                        class="flex w-full min-w-0 max-w-full flex-col gap-2"
                      >
                        {#each doneItems as item (item.id)}
                          <div
                            use:trackItemNode={item.id}
                            in:itemEntry={{ enabled: enteringItemIds.has(item.id) }}
                            animate:flip={{ duration: itemFlipDuration }}
                            class={`relative z-10 w-full min-w-0 max-w-full outline-none focus:outline-none focus-visible:outline-none ${
                              movingItemId === item.id ? 'completionMoveHidden' : ''
                            } ${exitingItemIds.has(item.id) ? 'itemExitCollapsing' : ''}`}
                          >
                            <LeafTodoItem
                              {item}
                              drawerOpen={openDrawerItemIds.has(item.id)}
                              drawerOpenImmediate={movingItemId === item.id}
                              onToggleItem={handleToggleItem}
                              onDrawerOpenChange={setItemDrawerOpen}
                              isTextClickSuppressed={isTextClickSuppressed}
                              onRequestEditItem={requestEditItem}
                              onRequestDeleteItem={requestDeleteItem}
                              onRequestCompleteFanfare={requestCompletionFanfare}
                            />
                          </div>
                        {/each}
                      </div>
                    {/if}
                  </div>
                {/if}
              </div>
            {/if}
          </div>
        </div>

        <div class="listTopFade" aria-hidden="true"></div>
        <div class="listBottomFade" aria-hidden="true"></div>
      </div>
    </section>
  </main>

  {#if completionMoveOverlay}
    <div
      class="completionMoveGhost"
      style={completionMoveGhostStyle(completionMoveOverlay)}
      aria-hidden="true"
    >
      {@html completionMoveOverlay.html}
    </div>
  {/if}

  {#if completionFanfareOverlay}
    <div
      class="completionFanfareOverlay"
      style={completionFanfareStyle(completionFanfareOverlay)}
      aria-hidden="true"
    >
      {#key completionFanfareOverlay.id}
        <CheckboxFanfare />
      {/key}
    </div>
  {/if}

  <CategoryDetailSheet
    show={showCategoryDetailSheet}
    mode={categoryDetailMode}
    category={categoryPendingDetail}
    isSaving={isSavingCategory}
    onSave={saveCategoryName}
    onClose={closeCategoryDetailSheet}
  />

  <CategoryManageSheet
    show={showCategoryManageSheet}
    category={categoryPendingDetail}
    isOnlyCategory={categories.length <= 1}
    isBusy={isDeletingCategory}
    onCreate={openCreateCategorySheet}
    onRename={requestRenameCategory}
    onEditOrder={requestEditCategoryOrder}
    onDeleteRequest={requestDeleteCategory}
    onClose={closeCategoryManageSheet}
  />

  <ConfirmModal
    show={categoryPendingDeletion !== null}
    title={i18n.t('checklistDeleteCategoryConfirmTitle')}
    message={categoryPendingDeletion
      ? i18n.t('checklistDeleteCategoryConfirmMessageTemplate')(categoryPendingDeletion.name)
      : ''}
    confirmLabel={isDeletingCategory
      ? i18n.t('checklistDeletingCategory')
      : i18n.t('checklistDeleteCategoryConfirmAction')}
    cancelLabel={i18n.t('cancel')}
    tone="danger"
    isBusy={isDeletingCategory}
    onConfirm={confirmDeleteCategory}
    onCancel={cancelDeleteCategory}
  />

  <ItemDetailSheet
    show={itemPendingEdit !== null}
    item={itemPendingEdit}
    {availableTags}
    isSaving={isSavingItemEdit}
    onSaveDetails={saveItemDetails}
    onClose={cancelEditItem}
  />

  <ConfirmModal
    show={itemPendingDeletion !== null}
    title={i18n.t('checklistDeleteItemConfirmTitle')}
    message={itemPendingDeletion
      ? i18n.t('checklistDeleteItemConfirmMessageTemplate')(itemPendingDeletion.text)
      : ''}
    confirmLabel={isDeletingItem ? i18n.t('checklistDeletingItem') : i18n.t('checklistDeleteItemConfirmAction')}
    cancelLabel={i18n.t('cancel')}
    tone="danger"
    isBusy={isDeletingItem}
    onConfirm={confirmDeleteItem}
    onCancel={cancelDeleteItem}
  />

  <ConfirmModal
    show={showArchiveConfirm}
    title={i18n.t('checklistArchiveCompletedConfirmTitle')}
    message={selectedCategory
      ? i18n.t('checklistArchiveCompletedConfirmMessageTemplate')(
          selectedCategory.name,
          archivableCompletedItemCount
        )
      : ''}
    confirmLabel={isArchivingCompletedItems
      ? i18n.t('checklistArchivingCompletedItems')
      : i18n.t('checklistArchiveCompletedConfirmAction')}
    cancelLabel={i18n.t('cancel')}
    tone="primary"
    isBusy={isArchivingCompletedItems}
    onConfirm={confirmArchiveCompletedItems}
    onCancel={cancelArchiveCompletedItems}
  />

  <ModalShell
    show={showArchiveEmptyNotice}
    title={i18n.t('checklistArchiveEmptyTitle')}
    description={selectedCategory
      ? i18n.t('checklistArchiveEmptyMessageTemplate')(selectedCategory.name)
      : i18n.t('checklistArchiveEmptyMessage')}
    onClose={closeArchiveEmptyNotice}
  >
    <button
      type="button"
      class="min-h-11 w-full rounded-[12px] bg-[var(--color-accent-sky-strong)] px-4 text-sm font-semibold text-[var(--color-ink)] transition-colors hover:bg-[var(--color-accent-sky)]"
      onclick={closeArchiveEmptyNotice}
    >
      {i18n.t('done')}
    </button>
  </ModalShell>
</div>

<style>
  .completionMoveHidden {
    opacity: 0;
    pointer-events: none;
  }

  .mainRouteEnterBack {
    animation: mainRouteEnter 400ms cubic-bezier(0.22, 1, 0.36, 1) backwards;
    will-change: transform, opacity;
  }

  @keyframes mainRouteEnter {
    from {
      opacity: 0.94;
      transform: translate3d(-16px, 0, 0);
    }

    to {
      opacity: 1;
      transform: translate3d(0, 0, 0);
    }
  }

  .itemExitCollapsing {
    pointer-events: none;
  }

  .completionMoveGhost {
    position: fixed;
    z-index: 1;
    pointer-events: none;
    transform-origin: top left;
    transition: transform var(--completion-move-duration) cubic-bezier(0.2, 0.8, 0.2, 1);
    will-change: transform;
  }

  .completionMoveGhost :global(*) {
    pointer-events: none !important;
  }

  .completionMoveGhost :global(.tickCheck path) {
    animation: none !important;
    stroke-dashoffset: 0 !important;
  }

  .completionMoveGhost :global(.tickTextDone)::after {
    animation: none !important;
    transform: scaleX(1) !important;
  }

  .completionFanfareOverlay {
    position: fixed;
    z-index: 20;
    color: var(--color-ink);
    pointer-events: none;
  }

  .listTopFade {
    position: absolute;
    top: 0;
    right: 0;
    left: 0;
    z-index: 20;
    height: 0.375rem;
    background: var(--color-canvas);
    pointer-events: none;
    -webkit-mask-image: linear-gradient(
      to bottom,
      rgb(0 0 0 / 0.45) 0%,
      rgb(0 0 0 / 0.18) 58%,
      transparent 100%
    );
    mask-image: linear-gradient(
      to bottom,
      rgb(0 0 0 / 0.45) 0%,
      rgb(0 0 0 / 0.18) 58%,
      transparent 100%
    );
  }

  .listBottomFade {
    position: absolute;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: 20;
    height: 4.75rem;
    background: var(--color-canvas);
    pointer-events: none;
    -webkit-mask-image: linear-gradient(
      to top,
      #000 0%,
      rgb(0 0 0 / 0.88) 22%,
      rgb(0 0 0 / 0.52) 58%,
      rgb(0 0 0 / 0.18) 82%,
      transparent 100%
    );
    mask-image: linear-gradient(
      to top,
      #000 0%,
      rgb(0 0 0 / 0.88) 22%,
      rgb(0 0 0 / 0.52) 58%,
      rgb(0 0 0 / 0.18) 82%,
      transparent 100%
    );
  }

  .todoListViewport.hasNativeDock .listBottomFade {
    display: none;
  }

  @media (prefers-reduced-motion: reduce) {
    .mainRouteEnterBack {
      animation: none;
      will-change: auto;
    }

    .completionMoveGhost {
      transition: none;
    }
  }
</style>
