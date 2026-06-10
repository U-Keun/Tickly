<script lang="ts">
  import { onDestroy } from 'svelte';
  import { cubicOut } from 'svelte/easing';
  import { slide } from 'svelte/transition';
  import { Pencil, Trash2 } from '@lucide/svelte';
  import { dragHandle } from 'svelte-dnd-action';

  import type { V2TodoItem } from '../../types';
  import { i18n } from '$lib/i18n';
  import V2CheckboxFanfare from './V2CheckboxFanfare.svelte';

  type MaybePromise = void | Promise<void>;

  const DRAWER_SLIDE_DURATION_MS = 280;
  const DRAWER_CONTENT_DELAY_MS = 60;
  const DRAWER_CONTENT_TRANSITION_MS = 160;
  const DRAWER_CLOSE_COLLAPSE_DELAY_MS = DRAWER_CONTENT_TRANSITION_MS + DRAWER_CONTENT_DELAY_MS;
  const CHECKBOX_HOP_DURATION_MS = 980;
  const CHECKBOX_CHECK_REVEAL_DELAY_MS = 260;
  const TEXT_DONE_DELAY_MS = CHECKBOX_CHECK_REVEAL_DELAY_MS + 230;
  const CHECKBOX_HOP_RESET_DELAY_MS = CHECKBOX_HOP_DURATION_MS + 40;
  const TEXT_PRESS_RELEASE_DELAY_MS = 90;

  interface Props {
    item: V2TodoItem;
    initialDrawerOpen?: boolean;
    isTextClickSuppressed?: boolean;
    onToggleItem: (id: number) => MaybePromise;
    onRequestEditItem: (item: V2TodoItem) => MaybePromise;
    onRequestDeleteItem: (item: V2TodoItem) => MaybePromise;
    onRequestCompleteFanfare?: (rect: DOMRect) => void;
  }

  let {
    item,
    initialDrawerOpen = false,
    isTextClickSuppressed = false,
    onToggleItem,
    onRequestEditItem,
    onRequestDeleteItem,
    onRequestCompleteFanfare
  }: Props = $props();

  let isDrawerOpen = $state(false);
  let isDrawerRendered = $state(false);
  let displayedDone = $state(false);
  let textDone = $state(false);
  let checkVisible = $state(false);
  let tickPulse = $state(false);
  let tickAnimationKey = $state(0);
  let isToggling = $state(false);
  let isDrawerContentVisible = $state(false);
  let didApplyInitialDrawerOpen = $state(false);
  let lastSyncedItemId = $state<number | null>(null);
  let lastSyncedDone = $state<boolean | null>(null);
  let textDoneTimer: ReturnType<typeof setTimeout> | null = null;
  let checkRevealTimer: ReturnType<typeof setTimeout> | null = null;
  let tickPulseTimer: ReturnType<typeof setTimeout> | null = null;
  let drawerContentTimer: ReturnType<typeof setTimeout> | null = null;
  let textPressTimer: ReturnType<typeof setTimeout> | null = null;
  let isTextPressing = $state(false);
  let checkboxButton: HTMLButtonElement | null = $state(null);
  let drawerId = $derived(`v2-todo-drawer-${item.id}`);

  $effect(() => {
    if (didApplyInitialDrawerOpen) return;
    if (initialDrawerOpen) {
      openDrawer();
    }
    didApplyInitialDrawerOpen = true;
  });

  $effect(() => {
    const itemChanged = item.id !== lastSyncedItemId;

    if (itemChanged && lastSyncedItemId !== null) {
      resetDrawer();
    }

    if (!itemChanged && isToggling && item.done === displayedDone) {
      lastSyncedDone = item.done;
      return;
    }

    if (!itemChanged && item.done === lastSyncedDone) return;

    setDisplayedDone(item.done, false);
    lastSyncedItemId = item.id;
    lastSyncedDone = item.done;
  });

  $effect(() => {
    if (isTextClickSuppressed) {
      resetTextPressState();
    }
  });

  onDestroy(() => {
    clearTextDoneTimer();
    clearCheckRevealTimer();
    clearTickPulseTimer();
    clearDrawerContentTimer();
    clearTextPressTimer();
  });

  function clearTextDoneTimer(): void {
    if (!textDoneTimer) return;
    clearTimeout(textDoneTimer);
    textDoneTimer = null;
  }

  function clearTickPulseTimer(): void {
    if (!tickPulseTimer) return;
    clearTimeout(tickPulseTimer);
    tickPulseTimer = null;
  }

  function clearCheckRevealTimer(): void {
    if (!checkRevealTimer) return;
    clearTimeout(checkRevealTimer);
    checkRevealTimer = null;
  }

  function clearDrawerContentTimer(): void {
    if (!drawerContentTimer) return;
    clearTimeout(drawerContentTimer);
    drawerContentTimer = null;
  }

  function clearTextPressTimer(): void {
    if (!textPressTimer) return;
    clearTimeout(textPressTimer);
    textPressTimer = null;
  }

  function resetTextPressState(): void {
    clearTextPressTimer();
    isTextPressing = false;
  }

  function releaseTextPressSoon(): void {
    clearTextPressTimer();
    textPressTimer = setTimeout(() => {
      isTextPressing = false;
      textPressTimer = null;
    }, TEXT_PRESS_RELEASE_DELAY_MS);
  }

  function prefersReducedMotion(): boolean {
    return typeof window !== 'undefined' && window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  }

  function resetDrawer(): void {
    clearDrawerContentTimer();
    isDrawerOpen = false;
    isDrawerRendered = false;
    isDrawerContentVisible = false;
  }

  function openDrawer(): void {
    clearDrawerContentTimer();
    isDrawerOpen = true;
    isDrawerRendered = true;

    if (prefersReducedMotion()) {
      isDrawerContentVisible = true;
      return;
    }

    isDrawerContentVisible = false;
    drawerContentTimer = setTimeout(() => {
      isDrawerContentVisible = true;
      drawerContentTimer = null;
    }, DRAWER_SLIDE_DURATION_MS + DRAWER_CONTENT_DELAY_MS);
  }

  function closeDrawer(): void {
    clearDrawerContentTimer();
    isDrawerOpen = false;
    isDrawerContentVisible = false;

    if (prefersReducedMotion() || !isDrawerRendered) {
      isDrawerRendered = false;
      return;
    }

    drawerContentTimer = setTimeout(() => {
      isDrawerRendered = false;
      drawerContentTimer = null;
    }, DRAWER_CLOSE_COLLAPSE_DELAY_MS);
  }

  function toggleDrawer(): void {
    if (isDrawerOpen) {
      closeDrawer();
      return;
    }

    openDrawer();
  }

  function handleTextClick(event: MouseEvent): void {
    if (isTextClickSuppressed) {
      resetTextPressState();
      event.preventDefault();
      return;
    }

    toggleDrawer();
  }

  function handleTextPointerDown(event: PointerEvent): void {
    if (event.pointerType === 'mouse' && event.button !== 0) return;
    if (isTextClickSuppressed) return;

    clearTextPressTimer();
    isTextPressing = true;
  }

  function handleTextPointerUp(): void {
    if (!isTextPressing) return;
    releaseTextPressSoon();
  }

  function setDisplayedDone(nextDone: boolean, shouldAnimate: boolean): void {
    displayedDone = nextDone;
    clearTextDoneTimer();
    clearCheckRevealTimer();

    if (!nextDone) {
      textDone = false;
      checkVisible = false;
      tickPulse = false;
      clearTickPulseTimer();
      return;
    }

    if (!shouldAnimate || prefersReducedMotion()) {
      textDone = true;
      checkVisible = true;
      tickPulse = false;
      return;
    }

    textDone = false;
    checkVisible = false;
    tickPulse = true;

    textDoneTimer = setTimeout(() => {
      textDone = true;
      textDoneTimer = null;
    }, TEXT_DONE_DELAY_MS);

    checkRevealTimer = setTimeout(() => {
      checkVisible = true;
      tickAnimationKey += 1;
      if (checkboxButton) {
        onRequestCompleteFanfare?.(checkboxButton.getBoundingClientRect());
      }
      checkRevealTimer = null;
    }, CHECKBOX_CHECK_REVEAL_DELAY_MS);

    clearTickPulseTimer();
    tickPulseTimer = setTimeout(() => {
      tickPulse = false;
      tickPulseTimer = null;
    }, CHECKBOX_HOP_RESET_DELAY_MS);
  }

  async function handleToggleItem(): Promise<void> {
    if (isToggling) return;

    setDisplayedDone(!displayedDone, !displayedDone);
    isToggling = true;
    try {
      await onToggleItem(item.id);
    } catch {
      setDisplayedDone(item.done, false);
    } finally {
      isToggling = false;
    }
  }

  async function handleRequestEdit(): Promise<void> {
    await onRequestEditItem(item);
  }

  async function handleRequestDelete(): Promise<void> {
    await onRequestDeleteItem(item);
  }

</script>

<article
  style={`--drawer-content-duration: ${DRAWER_CONTENT_TRANSITION_MS}ms; --checkbox-hop-duration: ${CHECKBOX_HOP_DURATION_MS}ms;`}
  class={`todoSurface w-full min-w-0 max-w-full rounded-[6px_24px_6px_24px] border-2 border-[var(--color-ink)] p-2 shadow-sm ${
    displayedDone
      ? 'bg-[var(--color-canvas)]'
      : 'bg-[var(--color-paper)]'
  }`}
  class:todoSurfacePressed={isTextPressing}
>
  <div class="flex min-h-11 w-full min-w-0 items-center gap-2.5">
    <button
      type="button"
      class={`relative grid h-11 w-11 flex-shrink-0 place-items-center overflow-visible border-2 transition-colors ${
        displayedDone
          ? 'rounded-[12px] border-[var(--color-ink)] bg-[var(--color-white)] text-[var(--color-ink)]'
          : 'rounded-[12px] border-[var(--color-ink)] bg-[var(--color-white)] text-transparent hover:bg-[var(--color-canvas)] active:bg-[var(--color-canvas)]'
      }`}
      class:checkboxSoftHop={tickPulse}
      aria-pressed={displayedDone}
      aria-label={displayedDone ? i18n.t('v2RestoreItem') : i18n.t('v2CompleteItem')}
      title={displayedDone ? i18n.t('v2RestoreItem') : i18n.t('v2CompleteItem')}
      bind:this={checkboxButton}
      onclick={() => void handleToggleItem()}
    >
      {#if checkVisible && tickPulse && !onRequestCompleteFanfare}
        <span class="fanfareInline">
          <V2CheckboxFanfare />
        </span>
      {/if}

      {#if checkVisible}
        {#key tickAnimationKey}
          <svg class="tickCheck h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="3"
              d="m5 12.5 4.2 4.2L19 7"
            />
          </svg>
        {/key}
      {/if}
    </button>

    <div class="flex min-w-0 flex-1 basis-0 items-center overflow-hidden">
      <button
        type="button"
        class={`flex min-h-11 min-w-0 flex-1 basis-0 items-center overflow-hidden rounded-[12px] pr-1 text-left text-base leading-6 transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--color-ink)] ${
          textDone
            ? 'text-[var(--color-ink-muted)]'
            : 'text-[var(--color-ink)]'
        }`}
        use:dragHandle
        aria-expanded={isDrawerOpen}
        aria-controls={drawerId}
        title={item.text}
        oncontextmenu={(event) => {
          event.preventDefault();
          resetTextPressState();
        }}
        onpointerdown={handleTextPointerDown}
        onpointerup={handleTextPointerUp}
        onpointercancel={resetTextPressState}
        onpointerleave={resetTextPressState}
        onclick={handleTextClick}
      >
        <span class="block min-w-0 flex-1 overflow-hidden whitespace-nowrap">
          <span class="tickText" class:tickTextDone={textDone}>{item.text}</span>
        </span>
      </button>

      <div class="h-11 w-[72px] shrink-0" aria-hidden="true"></div>
    </div>
  </div>

  {#if isDrawerRendered}
    <div
      id={drawerId}
      class="mt-2 overflow-hidden"
      transition:slide={{ duration: DRAWER_SLIDE_DURATION_MS, easing: cubicOut }}
    >
      <div class="drawerSurface" class:drawerSurfaceVisible={isDrawerContentVisible}>
        <div class="drawerContent" class:drawerContentVisible={isDrawerContentVisible}>
          <div class="drawerActions">
            <button
              type="button"
              class="drawerActionButton drawerActionEdit"
              aria-label={i18n.t('v2EditItem')}
              title={i18n.t('v2EditItem')}
              onclick={() => void handleRequestEdit()}
            >
              <Pencil size={20} strokeWidth={2.4} />
            </button>

            <button
              type="button"
              class="drawerActionButton drawerActionDelete"
              aria-label={i18n.t('v2DeleteItem')}
              title={i18n.t('v2DeleteItem')}
              onclick={() => void handleRequestDelete()}
            >
              <Trash2 size={20} strokeWidth={2.4} />
            </button>
          </div>
        </div>
      </div>
    </div>
  {/if}
</article>

<style>
  .todoSurface {
    transform-origin: center;
    transition:
      background-color 160ms ease-out,
      box-shadow 130ms cubic-bezier(0.2, 0.8, 0.2, 1),
      transform 130ms cubic-bezier(0.2, 0.8, 0.2, 1);
    will-change: transform;
  }

  .todoSurfacePressed {
    box-shadow: 0 1px 0 rgb(0 0 0 / 0.08);
    transform: translateY(1px) scale(0.992);
  }

  .tickCheck path {
    stroke-dasharray: 24;
    stroke-dashoffset: 0;
  }

  .checkboxSoftHop {
    animation: checkbox-soft-hop var(--checkbox-hop-duration) cubic-bezier(0.18, 0.95, 0.24, 1.05);
    transform-origin: 50% 75%;
  }

  .checkboxSoftHop .tickCheck path {
    stroke-dashoffset: 24;
    animation: tick-draw 190ms ease-out forwards;
  }

  .fanfareInline {
    position: absolute;
    right: -18px;
    top: -20px;
    color: currentColor;
    pointer-events: none;
  }

  .tickText {
    position: relative;
    display: inline-block;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    vertical-align: top;
  }

  .tickText::after {
    position: absolute;
    right: 0;
    left: 0;
    top: 52%;
    height: 2px;
    content: '';
    background: currentColor;
    border-radius: 999px;
    transform: scaleX(0);
    transform-origin: left center;
  }

  .tickTextDone::after {
    animation: tick-line 320ms cubic-bezier(0.2, 0.8, 0.2, 1) forwards;
  }

  .drawerSurface {
    position: relative;
    border-radius: 0 18px 0 18px;
    background-color: transparent;
    padding: 8px;
    transition: background-color var(--drawer-content-duration) ease-out;
  }

  .drawerSurface::before {
    position: absolute;
    inset: 0;
    border: 2px solid var(--color-ink);
    border-radius: inherit;
    content: '';
    opacity: 0;
    pointer-events: none;
    transition: opacity var(--drawer-content-duration) ease-out;
  }

  .drawerSurfaceVisible {
    background-color: var(--color-white);
  }

  .drawerSurfaceVisible::before {
    opacity: 1;
  }

  .drawerContent {
    position: relative;
    z-index: 1;
    opacity: 0;
    pointer-events: none;
    transform: translateY(-2px);
    transition:
      opacity var(--drawer-content-duration) ease-out,
      transform var(--drawer-content-duration) ease-out;
  }

  .drawerContentVisible {
    opacity: 1;
    pointer-events: auto;
    transform: translateY(0);
  }

  .drawerActions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .drawerActionButton {
    display: grid;
    width: 44px;
    height: 44px;
    place-items: center;
    border: 0;
    border-radius: 999px;
    color: var(--color-ink);
    cursor: pointer;
    transition:
      background-color 160ms ease-out,
      opacity 160ms ease-out,
      transform 160ms ease-out;
  }

  .drawerActionButton:hover {
    transform: translateY(-1px);
  }

  .drawerActionButton:active {
    transform: translateY(0);
  }

  .drawerActionButton:focus-visible {
    outline: 2px solid var(--color-ink);
    outline-offset: 3px;
  }

  .drawerActionButton:disabled {
    cursor: not-allowed;
    opacity: 0.35;
    transform: none;
  }

  .drawerActionEdit {
    background: var(--color-accent-sky);
  }

  .drawerActionEdit:hover,
  .drawerActionEdit:active {
    background: var(--color-accent-sky-strong);
  }

  .drawerActionNeutral {
    background: var(--color-paper);
  }

  .drawerActionNeutral:hover,
  .drawerActionNeutral:active {
    background: var(--color-canvas);
  }

  .drawerActionDelete {
    background: var(--color-accent-peach);
  }

  .drawerActionDelete:hover,
  .drawerActionDelete:active {
    background: var(--color-accent-peach-strong);
  }

  @keyframes checkbox-soft-hop {
    0% {
      transform: translateY(0) rotate(0deg) scale(1);
    }

    18% {
      transform: translateY(-6px) rotate(-10deg) scale(1.06);
    }

    78% {
      transform: translateY(-6px) rotate(-10deg) scale(1.06);
    }

    92% {
      transform: translateY(1px) rotate(3deg) scale(0.99);
    }

    100% {
      transform: translateY(0) rotate(0deg) scale(1);
    }
  }

  @keyframes tick-draw {
    to {
      stroke-dashoffset: 0;
    }
  }

  @keyframes tick-line {
    to {
      transform: scaleX(1);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .todoSurface,
    .todoSurfacePressed {
      box-shadow: var(--tw-shadow);
      transition: background-color 160ms ease-out;
      transform: none;
    }

    .checkboxSoftHop {
      animation: none;
    }

    .tickCheck path {
      animation: none;
      stroke-dashoffset: 0;
    }

    .drawerContent {
      opacity: 1;
      pointer-events: auto;
      transform: none;
      transition: none;
    }

    .drawerSurface {
      background-color: var(--color-white);
      transition: none;
    }

    .drawerSurface::before {
      opacity: 1;
      transition: none;
    }

    .tickTextDone::after {
      animation: none;
      transform: scaleX(1);
    }
  }
</style>
