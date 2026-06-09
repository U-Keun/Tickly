<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { flip } from 'svelte/animate';
  import { cubicIn, cubicOut } from 'svelte/easing';
  import { fade, fly } from 'svelte/transition';
  import { dragHandleZone } from 'svelte-dnd-action';
  import type { DndEvent } from 'svelte-dnd-action';

  import type { V2Category, V2ItemSearchResult, V2TodoItem } from '../../types';
  import { i18n } from '$lib/i18n';
  import V2CategoryDetailSheet from './V2CategoryDetailSheet.svelte';
  import V2CategoryManageSheet from './V2CategoryManageSheet.svelte';
  import V2CategoryRail from './V2CategoryRail.svelte';
  import V2ConfirmModal from './V2ConfirmModal.svelte';
  import V2ItemDetailSheet from './V2ItemDetailSheet.svelte';
  import V2LeafCommandBar from './V2LeafCommandBar.svelte';
  import V2LeafTodoItem from './V2LeafTodoItem.svelte';
  import V2SearchSuggestionBoard from './V2SearchSuggestionBoard.svelte';

  type MaybePromise = void | Promise<void>;
  const LIST_EXIT_DURATION_MS = 80;
  const LIST_EXIT_GAP_MS = 70;
  const LIST_ENTER_DURATION_MS = 160;
  const REORDER_FLIP_DURATION_MS = 180;
  const SEARCH_DEBOUNCE_MS = 150;
  const SEARCH_RESULT_LIMIT = 8;

  interface Props {
    categories: V2Category[];
    selectedCategoryId: number | null;
    items: V2TodoItem[];
    errorMessage?: string | null;
    initialSearchMode?: boolean;
    initialSearchQuery?: string;
    initialCategoryReorderMode?: boolean;
    onSelectCategory: (id: number) => MaybePromise;
    onAddCategory: (name: string) => MaybePromise;
    onUpdateCategory: (id: number, name: string) => MaybePromise;
    onDeleteCategory: (id: number) => MaybePromise;
    onReorderCategories: (categoryIds: number[]) => MaybePromise;
    onAddItem: (text: string) => MaybePromise;
    onToggleItem: (id: number) => MaybePromise;
    onUpdateItemText: (id: number, text: string) => MaybePromise;
    onDeleteItem: (id: number) => MaybePromise;
    onReorderItems: (itemIds: number[]) => MaybePromise;
    onSearchItems: (query: string, limit: number) => Promise<V2ItemSearchResult[]>;
  }

  let {
    categories,
    selectedCategoryId,
    items,
    errorMessage = null,
    initialSearchMode = false,
    initialSearchQuery = '',
    initialCategoryReorderMode = false,
    onSelectCategory,
    onAddCategory,
    onUpdateCategory,
    onDeleteCategory,
    onReorderCategories,
    onAddItem,
    onToggleItem,
    onUpdateItemText,
    onDeleteItem,
    onReorderItems,
    onSearchItems
  }: Props = $props();

  type CategoryDetailMode = 'create' | 'rename';
  type ReorderGroup = 'active' | 'done';

  let categoryDetailMode = $state<CategoryDetailMode>('create');
  let categoryPendingDetail = $state<V2Category | null>(null);
  let showCategoryDetailSheet = $state(false);
  let showCategoryManageSheet = $state(false);
  let categoryPendingDeletion = $state<V2Category | null>(null);
  let isSavingCategory = $state(false);
  let isDeletingCategory = $state(false);
  let isCategoryReorderMode = $state(false);
  let isSavingCategoryOrder = $state(false);
  let itemPendingEdit = $state<V2TodoItem | null>(null);
  let isSavingItemEdit = $state(false);
  let itemPendingDeletion = $state<V2TodoItem | null>(null);
  let isDeletingItem = $state(false);
  let isSavingReorder = $state(false);
  let searchMode = $state(false);
  let searchQuery = $state('');
  let appliedSearchQuery = $state('');
  let isSuggestionBoardOpen = $state(false);
  let didApplyInitialSearchState = $state(false);
  let searchSuggestions = $state<V2ItemSearchResult[]>([]);
  let isSearching = $state(false);
  let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  let searchRequestToken = 0;
  let activeItems = $state<V2TodoItem[]>([]);
  let doneItems = $state<V2TodoItem[]>([]);
  let categoryReorderDraft = $state<V2Category[] | null>(null);
  let isTextClickSuppressed = $state(false);
  let textClickSuppressTimer: ReturnType<typeof setTimeout> | null = null;
  let displayedCategoryId = $state<number | null>(null);
  let displayedItems = $state<V2TodoItem[]>([]);
  let hasDisplayedList = $state(false);
  let isListContentVisible = $state(true);
  let isListSwitching = $state(false);
  let listTransitionToken = 0;
  let prefersReducedMotion = $state(false);
  let itemSignature = $derived(
    items
      .map((item) => `${item.id}:${item.text}:${item.done}:${item.display_order}`)
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
  let listEnterDuration = $derived(prefersReducedMotion ? 0 : LIST_ENTER_DURATION_MS);
  let listExitDuration = $derived(prefersReducedMotion ? 0 : LIST_EXIT_DURATION_MS);
  let listEnterY = $derived(prefersReducedMotion ? 0 : 4);
  let listTransitionOpacity = $derived(prefersReducedMotion ? 1 : 0.18);
  let reorderFlipDuration = $derived(prefersReducedMotion ? 0 : REORDER_FLIP_DURATION_MS);

  let selectedCategory = $derived(
    categories.find((category) => category.id === selectedCategoryId) ?? null
  );
  let displayedCategories = $derived(categoryReorderDraft ?? categories);

  function wait(ms: number): Promise<void> {
    return new Promise((resolve) => window.setTimeout(resolve, ms));
  }

  function updateDisplayedList(categoryId: number | null, nextItems: V2TodoItem[]): void {
    displayedCategoryId = categoryId;
    displayedItems = nextItems;
    splitDisplayedItems(nextItems);
  }

  function splitDisplayedItems(nextItems: V2TodoItem[]): void {
    const nextVisibleItems = filterItemsForSearch(nextItems);
    activeItems = nextVisibleItems.filter((item) => !item.done);
    doneItems = nextVisibleItems.filter((item) => item.done);
  }

  function filterItemsForSearch(nextItems: V2TodoItem[]): V2TodoItem[] {
    if (!hasAppliedSearchQuery) return nextItems;
    return nextItems.filter((item) =>
      item.text.toLocaleLowerCase().includes(appliedSearchTerm)
    );
  }

  async function transitionDisplayedList(
    categoryId: number | null,
    nextItems: V2TodoItem[]
  ): Promise<void> {
    const token = ++listTransitionToken;

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
    const nextCategoryId = selectedCategoryId;
    const nextItems = items;
    itemSignature;

    if (!hasDisplayedList || displayedCategoryId === null || nextCategoryId === displayedCategoryId) {
      updateDisplayedList(nextCategoryId, nextItems);
      hasDisplayedList = true;
      isListContentVisible = true;
      isListSwitching = false;
      return;
    }

    void transitionDisplayedList(nextCategoryId, nextItems);
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

  function openCreateCategorySheet(): void {
    if (isCategoryReorderMode) return;

    categoryDetailMode = 'create';
    categoryPendingDetail = null;
    showCategoryManageSheet = false;
    showCategoryDetailSheet = true;
  }

  function openCategoryManageSheet(category: V2Category): void {
    if (isCategoryReorderMode) return;
    categoryPendingDetail = category;
    showCategoryManageSheet = true;
  }

  function enterCategoryReorderMode(): void {
    if (categories.length <= 1) return;

    isCategoryReorderMode = true;
    isSuggestionBoardOpen = false;
    showCategoryManageSheet = false;
    showCategoryDetailSheet = false;
    categoryPendingDetail = null;
  }

  function finishCategoryReorderMode(): void {
    if (isSavingCategoryOrder) return;

    isCategoryReorderMode = false;
    categoryReorderDraft = null;
  }

  function handleCategoryReorderConsider(nextCategories: V2Category[]): void {
    categoryReorderDraft = nextCategories;
  }

  async function handleCategoryReorderFinalize(nextCategories: V2Category[]): Promise<void> {
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

  function prioritizeSearchResults(results: V2ItemSearchResult[]): V2ItemSearchResult[] {
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
    resetSearchSuggestions();
    splitDisplayedItems(displayedItems);
  }

  function updateSearchQuery(query: string): void {
    searchQuery = query;
    isSuggestionBoardOpen = query.trim().length > 0;
    if (!query.trim()) {
      appliedSearchQuery = '';
    }
  }

  function openSearchSuggestionBoard(): void {
    if (searchMode && hasSearchQuery) {
      isSuggestionBoardOpen = true;
    }
  }

  async function selectSearchSuggestion(result: V2ItemSearchResult): Promise<void> {
    isSuggestionBoardOpen = false;
    appliedSearchQuery = searchQuery.trim();
    if (result.category.id === selectedCategoryId) return;

    try {
      await onSelectCategory(result.category.id);
    } catch {
      // The v2 store owns the visible error banner; keep the search query in place.
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

  function reorderGroupItems(group: ReorderGroup, nextItems: V2TodoItem[]): void {
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
    event: CustomEvent<DndEvent<V2TodoItem>>
  ): void {
    reorderGroupItems(group, event.detail.items);
    suppressTextClickFor(900);
  }

  async function handleReorderFinalize(
    group: ReorderGroup,
    event: CustomEvent<DndEvent<V2TodoItem>>
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

  function requestRenameCategory(): void {
    if (!selectedCategory) return;

    categoryDetailMode = 'rename';
    categoryPendingDetail = selectedCategory;
    showCategoryManageSheet = false;
    showCategoryDetailSheet = true;
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
  }

  async function saveCategoryName(name: string): Promise<void> {
    if (isSavingCategory) return;

    isSavingCategory = true;
    try {
      if (categoryDetailMode === 'create') {
        await onAddCategory(name);
      } else if (categoryPendingDetail) {
        await onUpdateCategory(categoryPendingDetail.id, name);
      }
    } finally {
      isSavingCategory = false;
    }
  }

  function requestDeleteCategory(): void {
    if (!selectedCategory || categories.length <= 1) return;
    categoryPendingDeletion = selectedCategory;
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
      // The v2 store owns the visible error banner; keep the confirm modal open.
    } finally {
      isDeletingCategory = false;
    }
  }

  function requestEditItem(item: V2TodoItem): void {
    itemPendingEdit = item;
  }

  function cancelEditItem(): void {
    if (isSavingItemEdit) return;
    itemPendingEdit = null;
  }

  async function saveItemText(id: number, text: string): Promise<void> {
    if (isSavingItemEdit) return;

    isSavingItemEdit = true;
    try {
      await onUpdateItemText(id, text);
    } finally {
      isSavingItemEdit = false;
    }
  }

  function requestDeleteItem(item: V2TodoItem): void {
    itemPendingDeletion = item;
  }

  function cancelDeleteItem(): void {
    if (isDeletingItem) return;
    itemPendingDeletion = null;
  }

  async function confirmDeleteItem(): Promise<void> {
    if (!itemPendingDeletion || isDeletingItem) return;

    isDeletingItem = true;
    try {
      await onDeleteItem(itemPendingDeletion.id);
      itemPendingDeletion = null;
    } catch {
      // The v2 store owns the visible error banner; keep the confirm modal open.
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
    if (textClickSuppressTimer) {
      clearTimeout(textClickSuppressTimer);
      textClickSuppressTimer = null;
    }
  });

</script>

<div class="app-container bg-canvas text-ink flex flex-col">
  <main class="mx-auto flex min-h-0 w-full max-w-2xl flex-1 flex-col">
    <section class="flex min-h-0 flex-1 flex-col px-4 pb-3 pt-[calc(var(--safe-area-top)+12px)]">
      <div class="relative z-30 mb-3">
        <V2LeafCommandBar
          mode={searchMode ? 'search' : 'add'}
          searchQuery={searchQuery}
          disabled={selectedCategoryId === null || isCategoryReorderMode}
          onAddItem={onAddItem}
          onEnterSearch={enterSearchMode}
          onExitSearch={exitSearchMode}
          onSearchQueryChange={updateSearchQuery}
          onSearchInputFocus={openSearchSuggestionBoard}
        />

        {#if searchMode && hasSearchQuery && isSuggestionBoardOpen}
          <div class="absolute left-0 right-0 top-[calc(100%+8px)] z-40">
            <V2SearchSuggestionBoard
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

      <div class="mb-3">
        <V2CategoryRail
          categories={displayedCategories}
          {selectedCategoryId}
          isReorderMode={isCategoryReorderMode}
          isReorderBusy={isSavingCategoryOrder}
          onSelectCategory={selectCategoryWithTransition}
          onCreateCategory={openCreateCategorySheet}
          onManageCategory={openCategoryManageSheet}
          onEnterReorderMode={enterCategoryReorderMode}
          onFinishReorderMode={finishCategoryReorderMode}
          onReorderConsider={handleCategoryReorderConsider}
          onReorderFinalize={handleCategoryReorderFinalize}
        />
      </div>

      <div class="todo-list-scroll" aria-busy={isListSwitching}>
        <div class={`grid min-h-full overflow-hidden ${isListSwitching ? 'pointer-events-none' : ''}`}>
          {#if isListContentVisible}
            <div
              class="col-start-1 row-start-1 min-h-0 will-change-transform"
              in:fly={{
                y: listEnterY,
                duration: listEnterDuration,
                opacity: listTransitionOpacity,
                easing: cubicOut
              }}
              out:fade={{ duration: listExitDuration, easing: cubicIn }}
            >
              {#if activeItems.length === 0 && doneItems.length === 0}
                <div class="px-6 py-10 text-center text-ink-muted">
                  {#if hasAppliedSearchQuery}
                    <p class="font-medium text-ink">{i18n.t('v2NoSearchResultsTemplate')(appliedSearchQuery)}</p>
                    <p class="mt-1 text-sm">{selectedCategory?.name ?? i18n.t('v2Categories')}</p>
                  {:else}
                    <p class="font-medium text-ink">{i18n.t('v2EmptyItemsTitle')}</p>
                    <p class="mt-1 text-sm">{i18n.t('v2EmptyItemsSubtitle')}</p>
                  {/if}
                </div>
              {:else}
                <div class="flex flex-col gap-2 pb-16">
                  {#if activeItems.length > 0}
                    <div
                      use:dragHandleZone={{
                        items: activeItems,
                        flipDurationMs: reorderFlipDuration,
                        type: 'v2-active-items',
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
                      class="flex flex-col gap-2"
                    >
                      {#each activeItems as item (item.id)}
                        <div
                          animate:flip={{ duration: reorderFlipDuration }}
                          class="outline-none focus:outline-none focus-visible:outline-none"
                        >
                          <V2LeafTodoItem
                            {item}
                            {onToggleItem}
                            isTextClickSuppressed={isTextClickSuppressed}
                            onRequestEditItem={requestEditItem}
                            onRequestDeleteItem={requestDeleteItem}
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
                        type: 'v2-done-items',
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
                      class="flex flex-col gap-2"
                    >
                      {#each doneItems as item (item.id)}
                        <div
                          animate:flip={{ duration: reorderFlipDuration }}
                          class="outline-none focus:outline-none focus-visible:outline-none"
                        >
                          <V2LeafTodoItem
                            {item}
                            {onToggleItem}
                            isTextClickSuppressed={isTextClickSuppressed}
                            onRequestEditItem={requestEditItem}
                            onRequestDeleteItem={requestDeleteItem}
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
    </section>
  </main>

  <V2CategoryDetailSheet
    show={showCategoryDetailSheet}
    mode={categoryDetailMode}
    category={categoryPendingDetail}
    isSaving={isSavingCategory}
    onSave={saveCategoryName}
    onClose={closeCategoryDetailSheet}
  />

  <V2CategoryManageSheet
    show={showCategoryManageSheet}
    category={selectedCategory}
    isOnlyCategory={categories.length <= 1}
    isBusy={isDeletingCategory}
    onRename={requestRenameCategory}
    onEditOrder={requestEditCategoryOrder}
    onDeleteRequest={requestDeleteCategory}
    onClose={closeCategoryManageSheet}
  />

  <V2ConfirmModal
    show={categoryPendingDeletion !== null}
    title={i18n.t('v2DeleteCategoryConfirmTitle')}
    message={categoryPendingDeletion
      ? i18n.t('v2DeleteCategoryConfirmMessageTemplate')(categoryPendingDeletion.name)
      : ''}
    confirmLabel={isDeletingCategory
      ? i18n.t('v2DeletingCategory')
      : i18n.t('v2DeleteCategoryConfirmAction')}
    cancelLabel={i18n.t('cancel')}
    tone="danger"
    isBusy={isDeletingCategory}
    onConfirm={confirmDeleteCategory}
    onCancel={cancelDeleteCategory}
  />

  <V2ItemDetailSheet
    show={itemPendingEdit !== null}
    item={itemPendingEdit}
    isSaving={isSavingItemEdit}
    onSaveText={saveItemText}
    onClose={cancelEditItem}
  />

  <V2ConfirmModal
    show={itemPendingDeletion !== null}
    title={i18n.t('v2DeleteItemConfirmTitle')}
    message={itemPendingDeletion
      ? i18n.t('v2DeleteItemConfirmMessageTemplate')(itemPendingDeletion.text)
      : ''}
    confirmLabel={isDeletingItem ? i18n.t('v2DeletingItem') : i18n.t('v2DeleteItemConfirmAction')}
    cancelLabel={i18n.t('cancel')}
    tone="danger"
    isBusy={isDeletingItem}
    onConfirm={confirmDeleteItem}
    onCancel={cancelDeleteItem}
  />
</div>
