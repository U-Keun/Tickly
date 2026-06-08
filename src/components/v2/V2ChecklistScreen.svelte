<script lang="ts">
  import type { V2Category, V2TodoItem } from '../../types';
  import { i18n } from '$lib/i18n';
  import V2LeafCommandBar from './V2LeafCommandBar.svelte';

  type MaybePromise = void | Promise<void>;

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

  let newCategoryName = $state('');
  let editingCategoryId = $state<number | null>(null);
  let editingCategoryName = $state('');
  let editingItemId = $state<number | null>(null);
  let editingItemText = $state('');
  let isReorderMode = $state(false);
  let didApplyInitialReorderMode = $state(false);

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

  function isFirstItem(id: number): boolean {
    return items.findIndex((item) => item.id === id) <= 0;
  }

  function isLastItem(id: number): boolean {
    const index = items.findIndex((item) => item.id === id);
    return index < 0 || index >= items.length - 1;
  }

  async function submitCategory(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    const trimmedName = newCategoryName.trim();
    if (!trimmedName) return;

    await onAddCategory(trimmedName);
    newCategoryName = '';
  }

  function beginCategoryEdit(): void {
    if (!selectedCategory) return;
    editingCategoryId = selectedCategory.id;
    editingCategoryName = selectedCategory.name;
  }

  async function submitCategoryEdit(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    if (editingCategoryId === null) return;

    const trimmedName = editingCategoryName.trim();
    if (!trimmedName) return;

    await onUpdateCategory(editingCategoryId, trimmedName);
    editingCategoryId = null;
    editingCategoryName = '';
  }

  function cancelCategoryEdit(): void {
    editingCategoryId = null;
    editingCategoryName = '';
  }

  function beginItemEdit(item: V2TodoItem): void {
    editingItemId = item.id;
    editingItemText = item.text;
  }

  async function submitItemEdit(event: SubmitEvent, itemId: number): Promise<void> {
    event.preventDefault();
    const trimmedText = editingItemText.trim();
    if (!trimmedText) return;

    await onUpdateItemText(itemId, trimmedText);
    editingItemId = null;
    editingItemText = '';
  }

  function cancelItemEdit(): void {
    editingItemId = null;
    editingItemText = '';
  }
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

    <section class="shrink-0 border-b border-stroke bg-paper px-4 py-3">
      <div class="mb-2 flex items-center justify-between gap-3">
        <h2 class="text-sm font-semibold text-ink">{i18n.t('v2Categories')}</h2>
        <button
          type="button"
          class="min-h-11 rounded-md border border-stroke bg-white px-3 text-sm text-ink"
          onclick={() => (isReorderMode = !isReorderMode)}
        >
          {isReorderMode ? i18n.t('v2ExitReorderMode') : i18n.t('v2ReorderMode')}
        </button>
      </div>

      <div class="scrollbar-hide -mx-1 flex gap-2 overflow-x-auto px-1 pb-2">
        {#each categories as category (category.id)}
          <button
            type="button"
            class={`min-h-11 shrink-0 rounded-md border px-4 text-sm font-medium ${
              category.id === selectedCategoryId
                ? 'border-accent-sky-strong bg-accent-sky text-ink'
                : 'border-stroke bg-white text-ink-muted'
            }`}
            onclick={() => onSelectCategory(category.id)}
          >
            {category.name}
          </button>
        {/each}
      </div>

      {#if selectedCategory}
        <div class="mt-1 flex flex-wrap items-center gap-2">
          {#if editingCategoryId === selectedCategory.id}
            <form class="flex min-w-0 flex-1 gap-2" onsubmit={submitCategoryEdit}>
              <input
                class="min-h-11 min-w-0 flex-1 rounded-md border border-stroke bg-white px-3 text-base text-ink"
                bind:value={editingCategoryName}
                aria-label={i18n.t('v2EditCategory')}
              />
              <button
                type="submit"
                class="min-h-11 rounded-md bg-accent-mint-strong px-3 text-sm font-medium text-ink"
              >
                {i18n.t('v2SaveCategory')}
              </button>
              <button
                type="button"
                class="min-h-11 rounded-md border border-stroke bg-white px-3 text-sm text-ink"
                onclick={cancelCategoryEdit}
              >
                {i18n.t('cancel')}
              </button>
            </form>
          {:else}
            <button
              type="button"
              class="min-h-11 rounded-md border border-stroke bg-white px-3 text-sm text-ink"
              onclick={beginCategoryEdit}
            >
              {i18n.t('v2EditCategory')}
            </button>
            <button
              type="button"
              class="min-h-11 rounded-md border border-accent-peach-strong bg-white px-3 text-sm text-ink"
              onclick={() => onDeleteCategory(selectedCategory.id)}
            >
              {i18n.t('v2DeleteCategory')}
            </button>
          {/if}

          {#if isReorderMode}
            <button
              type="button"
              class="min-h-11 rounded-md border border-stroke bg-white px-3 text-sm text-ink disabled:opacity-40"
              disabled={isFirstCategory(selectedCategory.id)}
              onclick={() => onMoveCategory(selectedCategory.id, -1)}
            >
              {i18n.t('v2MoveLeft')}
            </button>
            <button
              type="button"
              class="min-h-11 rounded-md border border-stroke bg-white px-3 text-sm text-ink disabled:opacity-40"
              disabled={isLastCategory(selectedCategory.id)}
              onclick={() => onMoveCategory(selectedCategory.id, 1)}
            >
              {i18n.t('v2MoveRight')}
            </button>
          {/if}
        </div>
      {/if}

      <form class="mt-3 flex gap-2" onsubmit={submitCategory}>
        <input
          class="min-h-11 min-w-0 flex-1 rounded-md border border-stroke bg-white px-3 text-base text-ink"
          bind:value={newCategoryName}
          placeholder={i18n.t('v2NewCategoryPlaceholder')}
          aria-label={i18n.t('v2NewCategoryPlaceholder')}
        />
        <button
          type="submit"
          class="min-h-11 rounded-md bg-accent-sky-strong px-3 text-sm font-semibold text-ink"
        >
          {i18n.t('v2AddCategory')}
        </button>
      </form>
    </section>

    <section class="flex min-h-0 flex-1 flex-col px-4 py-3">
      <div class="mb-3 flex items-center justify-between gap-3">
        <h2 class="text-sm font-semibold text-ink">{i18n.t('v2Items')}</h2>
        {#if isLoading}
          <span class="text-sm text-ink-muted">{i18n.t('v2Loading')}</span>
        {/if}
      </div>

      <div class="mb-3">
        <V2LeafCommandBar
          disabled={selectedCategoryId === null}
          onAddItem={onAddItem}
        />
      </div>

      <div class="todo-list-scroll rounded-md border border-stroke bg-paper">
        {#if items.length === 0}
          <div class="px-6 py-10 text-center text-ink-muted">
            <p class="font-medium text-ink">{i18n.t('v2EmptyItemsTitle')}</p>
            <p class="mt-1 text-sm">{i18n.t('v2EmptyItemsSubtitle')}</p>
          </div>
        {:else}
          <div class="flex flex-col divide-y divide-stroke">
            {#each items as item (item.id)}
              <div class="flex gap-3 bg-white px-3 py-3">
                <button
                  type="button"
                  class={`mt-0.5 flex h-9 w-9 shrink-0 items-center justify-center rounded-full border text-sm font-bold ${
                    item.done
                      ? 'border-accent-mint-strong bg-accent-mint text-ink'
                      : 'border-stroke bg-paper text-ink-muted'
                  }`}
                  aria-label={item.done ? i18n.t('v2RestoreItem') : i18n.t('v2CompleteItem')}
                  onclick={() => onToggleItem(item.id)}
                >
                  {item.done ? 'OK' : ''}
                </button>

                <div class="min-w-0 flex-1">
                  {#if editingItemId === item.id}
                    <form class="flex gap-2" onsubmit={(event) => submitItemEdit(event, item.id)}>
                      <input
                        class="min-h-11 min-w-0 flex-1 rounded-md border border-stroke bg-paper px-3 text-base text-ink"
                        bind:value={editingItemText}
                        aria-label={i18n.t('v2EditItem')}
                      />
                      <button
                        type="submit"
                        class="min-h-11 rounded-md bg-accent-mint-strong px-3 text-sm font-medium text-ink"
                      >
                        {i18n.t('v2SaveItem')}
                      </button>
                      <button
                        type="button"
                        class="min-h-11 rounded-md border border-stroke bg-white px-3 text-sm text-ink"
                        onclick={cancelItemEdit}
                      >
                        {i18n.t('cancel')}
                      </button>
                    </form>
                  {:else}
                    <p
                      class={`break-words text-base leading-6 ${
                        item.done ? 'text-ink-muted line-through' : 'text-ink'
                      }`}
                    >
                      {item.text}
                    </p>
                    <div class="mt-2 flex flex-wrap gap-2">
                      <button
                        type="button"
                        class="min-h-11 rounded-md border border-stroke bg-paper px-3 text-sm text-ink"
                        onclick={() => beginItemEdit(item)}
                      >
                        {i18n.t('v2EditItem')}
                      </button>
                      <button
                        type="button"
                        class="min-h-11 rounded-md border border-accent-peach-strong bg-paper px-3 text-sm text-ink"
                        onclick={() => onDeleteItem(item.id)}
                      >
                        {i18n.t('v2DeleteItem')}
                      </button>

                      {#if isReorderMode}
                        <button
                          type="button"
                          class="min-h-11 rounded-md border border-stroke bg-paper px-3 text-sm text-ink disabled:opacity-40"
                          disabled={isFirstItem(item.id)}
                          onclick={() => onMoveItem(item.id, -1)}
                        >
                          {i18n.t('v2MoveUp')}
                        </button>
                        <button
                          type="button"
                          class="min-h-11 rounded-md border border-stroke bg-paper px-3 text-sm text-ink disabled:opacity-40"
                          disabled={isLastItem(item.id)}
                          onclick={() => onMoveItem(item.id, 1)}
                        >
                          {i18n.t('v2MoveDown')}
                        </button>
                      {/if}
                    </div>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </section>
  </main>
</div>
