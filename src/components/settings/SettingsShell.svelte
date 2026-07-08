<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';
  import { ArrowLeft } from '@lucide/svelte';

  import { i18n } from '$lib/i18n';

  const ROUTE_MOTION_KEY = 'tickly:route:last-pathname';

  interface Props {
    title: string;
    onBack: () => void;
    children: Snippet;
    footer?: Snippet;
    contentClass?: string;
  }

  let { title, onBack, children, footer, contentClass = '' }: Props = $props();

  let routeMotionClass = $state('settings-route-enter-idle');

  function settingsDepth(pathname: string): number {
    return pathname.split('/').filter(Boolean).length;
  }

  function getEnterDirection(pathname: string): 'forward' | 'back' | 'neutral' {
    if (typeof sessionStorage === 'undefined') return 'neutral';

    const previousPathname = sessionStorage.getItem(ROUTE_MOTION_KEY);
    sessionStorage.setItem(ROUTE_MOTION_KEY, pathname);

    if (previousPathname === '/') return 'forward';
    if (!previousPathname?.startsWith('/settings')) return 'neutral';

    const previousDepth = settingsDepth(previousPathname);
    const nextDepth = settingsDepth(pathname);

    if (nextDepth > previousDepth) return 'forward';
    if (nextDepth < previousDepth) return 'back';
    return 'neutral';
  }

  onMount(() => {
    const direction = getEnterDirection($page.url.pathname);
    routeMotionClass = `settings-route-enter-${direction}`;
  });
</script>

<div
  class={`app-container full-bleed-app-container isolate flex min-w-0 flex-col overflow-hidden bg-canvas text-ink ${routeMotionClass}`}
>
  <main class="mx-auto flex min-h-0 w-full min-w-0 max-w-2xl flex-1 flex-col overflow-hidden">
    <section class="flex min-h-0 w-full min-w-0 flex-1 flex-col overflow-hidden px-[max(1rem,var(--safe-area-left))] pb-[max(1rem,var(--safe-area-bottom))] pt-[max(0.75rem,var(--safe-area-top))]">
      <header class="flex shrink-0 items-center gap-3 pb-5">
        <button
          type="button"
          class="flex h-11 w-11 shrink-0 items-center justify-center rounded-[6px_16px_6px_16px] border-2 border-ink bg-white text-ink transition-colors hover:bg-canvas active:bg-accent-sky"
          aria-label={i18n.t('back')}
          onclick={onBack}
        >
          <ArrowLeft size={22} strokeWidth={2.4} aria-hidden="true" />
        </button>

        <h1 class="min-w-0 flex-1 truncate text-[22px] font-semibold leading-7 text-ink">{title}</h1>
        <div class="h-11 w-11 shrink-0" aria-hidden="true"></div>
      </header>

      <div class={`min-h-0 flex-1 overflow-y-auto pb-6 ${contentClass}`}>
        {@render children()}
      </div>

      {#if footer}
        {@render footer()}
      {/if}
    </section>
  </main>
</div>

<style>
  .settings-route-enter-forward,
  .settings-route-enter-back,
  .settings-route-enter-neutral {
    animation: settingsRouteEnter 400ms cubic-bezier(0.22, 1, 0.36, 1) backwards;
    will-change: transform, opacity;
  }

  .settings-route-enter-forward {
    --settings-route-enter-x: 22px;
  }

  .settings-route-enter-back {
    --settings-route-enter-x: -18px;
  }

  .settings-route-enter-neutral {
    --settings-route-enter-x: 0;
  }

  @keyframes settingsRouteEnter {
    from {
      opacity: 0.92;
      transform: translate3d(var(--settings-route-enter-x), 0, 0);
    }

    to {
      opacity: 1;
      transform: translate3d(0, 0, 0);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .settings-route-enter-forward,
    .settings-route-enter-back,
    .settings-route-enter-neutral {
      animation: none;
      will-change: auto;
    }
  }
</style>
