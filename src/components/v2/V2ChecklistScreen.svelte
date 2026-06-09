<script lang="ts">
  import { onMount } from 'svelte';
  import { cubicIn, cubicOut } from 'svelte/easing';
  import { fade, fly } from 'svelte/transition';

  import type { V2Category, V2TodoItem } from '../../types';
  import { i18n } from '$lib/i18n';
  import V2CategoryDetailSheet from './V2CategoryDetailSheet.svelte';
  import V2CategoryManageSheet from './V2CategoryManageSheet.svelte';
  import V2CategoryRail from './V2CategoryRail.svelte';
  import V2ConfirmModal from './V2ConfirmModal.svelte';
  import V2ItemDetailSheet from './V2ItemDetailSheet.svelte';
  import V2LeafCommandBar from './V2LeafCommandBar.svelte';
  import V2LeafTodoItem from './V2LeafTodoItem.svelte';

  type MaybePromise = void | Promise<void>;
  const LIST_EXIT_DURATION_MS = 80;
  const LIST_EXIT_GAP_MS = 70;
  const LIST_ENTER_DURATION_MS = 160;

  interface Props {
    categories: V2Category[];
    selectedCategoryId: number | null;
    items: V2TodoItem[];
    isLoading?: boolean;
    errorMessage?: string | null;
    initialReorderMode?: boolean;
    onBackHome: () => MaybePromise;
    onRefresh: () => MaybePromise;
    onSelectCategory: (id: number) => MaybePromise;
    onAddCategory: (name: string) => MaybePromise;
    onUpdateCategory: (id: number, name: string) => MaybePromise;
    onDeleteCategory: (id: number) => MaybePromise;
    onMoveCategory: (id: number, delta: number) => MaybePromise;
    onAddItem: (text: string) => MaybePromise;
    onToggleItem: (id: number) => MaybePromise;
    onUpdateItemText: (id: number, text: string) => MaybePromise;
    onDeleteItem: (id: number) => MaybePromise;
    onMoveItem: (id: number, delta: number) => MaybePromise;
  }

  let {
    categories,
    selectedCategoryId,
    items,
    isLoading = false,
    errorMessage = null,
    initialReorderMode = false,
    onBackHome,
    onRefresh,
    onSelectCategory,
    onAddCategory,
    onUpdateCategory,
    onDeleteCategory,
    onMoveCategory,
    onAddItem,
    onToggleItem,
    onUpdateItemText,
    onDeleteItem,
    onMoveItem
  }: Props = $props();

  type CategoryDetailMode = 'create' | 'rename';

  let categoryDetailMode = $state<CategoryDetailMode>('create');
  let categoryPendingDetail = $state<V2Category | null>(null);
  let showCategoryDetailSheet = $state(false);
  let showCategoryManageSheet = $state(false);
  let categoryPendingDeletion = $state<V2Category | null>(null);
  let isSavingCategory = $state(false);
  let isMovingCategory = $state(false);
  let isDeletingCategory = $state(false);
  let isReorderMode = $state(false);
  let didApplyInitialReorderMode = $state(false);
  let itemPendingEdit = $state<V2TodoItem | null>(null);
  let isSavingItemEdit = $state(false);
  let itemPendingDeletion = $state<V2TodoItem | null>(null);
  let isDeletingItem = $state(false);
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
  let listEnterDuration = $derived(prefersReducedMotion ? 0 : LIST_ENTER_DURATION_MS);
  let listExitDuration = $derived(prefersReducedMotion ? 0 : LIST_EXIT_DURATION_MS);
  let listEnterY = $derived(prefersReducedMotion ? 0 : 4);
  let listTransitionOpacity = $derived(prefersReducedMotion ? 1 : 0.18);

  let selectedCategory = $derived(
    categories.find((category) => category.id === selectedCategoryId) ?? null
  );

  $effect(() => {
    if (didApplyInitialReorderMode) return;
    isReorderMode = initialReorderMode;
    didApplyInitialReorderMode = true;
  });

  function isFirstCategory(id: number): boolean {
    return categories.findIndex((category) => category.id === id) <= 0;
  }

  function isLastCategory(id: number): boolean {
    const index = categories.findIndex((category) => category.id === id);
    return index < 0 || index >= categories.length - 1;
  }

  function wait(ms: number): Promise<void> {
    return new Promise((resolve) => window.setTimeout(resolve, ms));
  }

  function updateDisplayedList(categoryId: number | null, nextItems: V2TodoItem[]): void {
    displayedCategoryId = categoryId;
    displayedItems = nextItems;
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

  function isFirstItem(id: number): boolean {
    return displayedItems.findIndex((item) => item.id === id) <= 0;
  }

  function isLastItem(id: number): boolean {
    const index = displayedItems.findIndex((item) => item.id === id);
    return index < 0 || index >= displayedItems.length - 1;
  }

  function openCreateCategorySheet(): void {
    categoryDetailMode = 'create';
    categoryPendingDetail = null;
    showCategoryManageSheet = false;
    showCategoryDetailSheet = true;
  }

  function openCategoryManageSheet(category: V2Category): void {
    categoryPendingDetail = category;
    showCategoryManageSheet = true;
  }

  async function selectCategoryWithTransition(id: number): Promise<void> {
    if (id === selectedCategoryId) return;

    await onSelectCategory(id);
  }

  function requestRenameCategory(): void {
    if (!selectedCategory) return;

    categoryDetailMode = 'rename';
    categoryPendingDetail = selectedCategory;
    showCategoryManageSheet = false;
    showCategoryDetailSheet = true;
  }

  function closeCategoryDetailSheet(): void {
    if (isSavingCategory) return;

    showCategoryDetailSheet = false;
    categoryPendingDetail = null;
  }

  function closeCategoryManageSheet(): void {
    if (isMovingCategory || isDeletingCategory) return;

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

  async function moveSelectedCategory(delta: number): Promise<void> {
    if (!selectedCategory || isMovingCategory) return;

    isMovingCategory = true;
    try {
      await onMoveCategory(selectedCategory.id, delta);
    } finally {
      isMovingCategory = false;
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

</script>

<div class="app-container bg-canvas text-ink flex flex-col">
  <header
    class="shrink-0 border-b border-stroke bg-paper px-4 pb-3 pt-[calc(var(--safe-area-top)+12px)]"
  >
    <div class="mx-auto flex w-full max-w-2xl items-center justify-between gap-3">
      <div class="min-w-0">
        <p class="text-xs font-semibold uppercase tracking-normal text-ink-muted">
          {i18n.t('v2Subtitle')}
        </p>
        <h1 class="truncate text-lg font-semibold text-ink">{i18n.t('v2Title')}</h1>
      </div>

      <div class="flex shrink-0 items-center gap-2">
        <button
          type="button"
          class="min-h-11 rounded-md border border-stroke bg-white px-3 text-sm font-medium text-ink"
          onclick={onRefresh}
        >
          {i18n.t('v2Refresh')}
        </button>
        <button
          type="button"
          class="min-h-11 rounded-md bg-ink px-3 text-sm font-medium text-white"
          onclick={onBackHome}
        >
          {i18n.t('v2BackHome')}
        </button>
      </div>
    </div>
  </header>

  <main class="mx-auto flex min-h-0 w-full max-w-2xl flex-1 flex-col">
    {#if errorMessage}
      <div class="mx-4 mt-3 rounded-md border border-accent-peach-strong bg-accent-peach px-3 py-2 text-sm text-ink">
        {errorMessage}
      </div>
    {/if}

    <section class="flex min-h-0 flex-1 flex-col px-4 py-3">
      <div class="mb-3 flex items-center justify-between gap-3">
        <h2 class="text-sm font-semibold text-ink">{i18n.t('v2Items')}</h2>
        <div class="flex shrink-0 items-center gap-2">
          {#if isLoading}
            <span class="text-sm text-ink-muted">{i18n.t('v2Loading')}</span>
          {/if}
          <button
            type="button"
            class="min-h-11 rounded-[12px] border border-[var(--color-stroke)] bg-[var(--color-white)] px-3 text-sm font-semibold text-[var(--color-ink)] transition-colors hover:border-[var(--color-ink)] disabled:opacity-40"
            disabled={items.length === 0}
            onclick={() => (isReorderMode = !isReorderMode)}
          >
            {isReorderMode ? i18n.t('v2ExitReorderMode') : i18n.t('v2ReorderMode')}
          </button>
        </div>
      </div>

      <div class="mb-3">
        <V2LeafCommandBar
          disabled={selectedCategoryId === null}
          onAddItem={onAddItem}
        />
      </div>

      <div class="mb-3">
        <V2CategoryRail
          {categories}
          {selectedCategoryId}
          onSelectCategory={selectCategoryWithTransition}
          onCreateCategory={openCreateCategorySheet}
          onManageCategory={openCategoryManageSheet}
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
              {#if displayedItems.length === 0}
                <div class="rounded-[0_24px_0_24px] border-2 border-[var(--color-ink)] bg-[var(--color-white)] px-6 py-10 text-center text-ink-muted shadow-sm">
                  <p class="font-medium text-ink">{i18n.t('v2EmptyItemsTitle')}</p>
                  <p class="mt-1 text-sm">{i18n.t('v2EmptyItemsSubtitle')}</p>
                </div>
              {:else}
                <div class="flex flex-col gap-2 pb-16">
                  {#each displayedItems as item (item.id)}
                    <V2LeafTodoItem
                      {item}
                      {isReorderMode}
                      isFirst={isFirstItem(item.id)}
                      isLast={isLastItem(item.id)}
                      {onToggleItem}
                      onRequestEditItem={requestEditItem}
                      onRequestDeleteItem={requestDeleteItem}
                      {onMoveItem}
                    />
                  {/each}
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
    isFirst={selectedCategory ? isFirstCategory(selectedCategory.id) : true}
    isLast={selectedCategory ? isLastCategory(selectedCategory.id) : true}
    isBusy={isMovingCategory || isDeletingCategory}
    onRename={requestRenameCategory}
    onDeleteRequest={requestDeleteCategory}
    onMove={moveSelectedCategory}
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
