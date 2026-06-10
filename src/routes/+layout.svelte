<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { fly, fade } from 'svelte/transition';
  import { beforeNavigate, goto } from '$app/navigation';
  import { cubicOut } from 'svelte/easing';
  import { onMount, onDestroy } from 'svelte';
  import type { Snippet } from 'svelte';
  import { appStore } from '$lib/stores';
  import {
    authStore,
    handleOAuthCallback,
    handleOAuthCallbackError,
  } from '$lib/stores/authStore.svelte';
  import { syncStore } from '$lib/stores/syncStore.svelte';
  import { ensurePermission } from '$lib/notification';

  let { children }: { children: Snippet } = $props();

  let direction = $state(1); // 1: 오른쪽에서, -1: 왼쪽에서
  let hasNavigated = $state(false); // 네비게이션 발생 여부

  // Connect realtime when logged in
  async function connectRealtimeIfLoggedIn() {
    if (!syncStore.isEnabled) {
      await syncStore.disconnectRealtime();
      return;
    }

    if (authStore.isLoggedIn && authStore.session) {
      const { access_token, user_id } = authStore.session;
      await syncStore.connectRealtime(access_token, user_id);
    }
  }

  async function handleWidgetDeepLink(parsedUrl: URL): Promise<boolean> {
    if (parsedUrl.host !== 'widget') {
      return false;
    }

    if (parsedUrl.pathname === '/toggle') {
      const itemIdParam = parsedUrl.searchParams.get('itemId') ?? parsedUrl.searchParams.get('id');
      const itemId = Number(itemIdParam);

      if (!Number.isInteger(itemId) || itemId <= 0) {
        console.error('Invalid widget item id:', itemIdParam);
        return true;
      }

      await appStore.toggleItemFromWidget(itemId);
      await goto('/v1');
      return true;
    }

    if (parsedUrl.pathname === '/category') {
      const categoryIdParam = parsedUrl.searchParams.get('categoryId') ?? parsedUrl.searchParams.get('id');
      if (!categoryIdParam) {
        await goto('/v1');
        return true;
      }

      const categoryId = Number(categoryIdParam);
      if (!Number.isInteger(categoryId) || categoryId <= 0) {
        console.error('Invalid widget category id:', categoryIdParam);
        return true;
      }

      await appStore.loadCategories();
      const categoryExists = appStore.categories.some(category => category.id === categoryId);

      if (categoryExists) {
        await appStore.selectCategory(categoryId);
      } else {
        console.error('Widget category id does not exist:', categoryId);
      }

      await goto('/v1');
      return true;
    }

    return false;
  }

  async function handleIncomingDeepLinks(urls: string[]): Promise<void> {
    for (const url of urls) {

      // Parse the URL to extract OAuth callback parameters
      try {
        const parsedUrl = new URL(url);

        if (await handleWidgetDeepLink(parsedUrl)) {
          continue;
        }

        // Check if this is an OAuth callback
        if (parsedUrl.host === 'auth' && parsedUrl.pathname === '/callback') {
          const code = parsedUrl.searchParams.get('code');
          const error = parsedUrl.searchParams.get('error');
          const errorDescription = parsedUrl.searchParams.get('error_description');

          if (error) {
            const message = errorDescription ? `${error}: ${errorDescription}` : error;
            console.error('OAuth error:', message);
            handleOAuthCallbackError(new Error(message));
            continue;
          }

          if (code) {
            await handleOAuthCallback(code);
            // Connect to realtime after successful OAuth
            await connectRealtimeIfLoggedIn();
            continue;
          }

          const message = 'OAuth callback missing authorization code';
          console.error(message);
          handleOAuthCallbackError(new Error(message));
        }
      } catch (e) {
        console.error('Failed to parse deep link URL:', e);
      }
    }
  }

  // Check session and set up deep link listener
  onMount(async () => {
    // Request notification permission
    await ensurePermission();

    // Restore login state from saved session
    await authStore.checkSession();

    // Load persisted sync state before any sync-related decisions
    await syncStore.loadStatus();

    // Apply widget-only check actions queued in app group storage
    await appStore.processWidgetActions();

    if (authStore.isLoggedIn && syncStore.isEnabled) {
      await syncStore.sync();
    }

    // Connect to realtime if logged in and sync is enabled
    await connectRealtimeIfLoggedIn();

    try {
      const { getCurrent, onOpenUrl } = await import('@tauri-apps/plugin-deep-link');

      const currentUrls = (await getCurrent()) ?? [];
      if (currentUrls.length > 0) {
        await handleIncomingDeepLinks(currentUrls);
      }

      // Listen for deep link events
      await onOpenUrl(async (urls) => {
        await handleIncomingDeepLinks(urls);
      });
    } catch (e) {
      // Deep link plugin might not be available on all platforms
      // Deep link plugin not available on this platform
    }
  });

  // Disconnect realtime on component destroy
  onDestroy(() => {
    syncStore.disconnectRealtime();
  });

  // 경로 깊이 계산
  function getDepth(path: string): number {
    if (path === '/') return 0;
    return path.split('/').filter(Boolean).length;
  }

  beforeNavigate(({ from, to }) => {
    hasNavigated = true;

    if (!from?.url || !to?.url) return;

    const fromDepth = getDepth(from.url.pathname);
    const toDepth = getDepth(to.url.pathname);

    // 하위 페이지로 가면 오른쪽에서, 상위로 가면 왼쪽에서
    direction = toDepth > fromDepth ? 1 : -1;
  });

  // 첫 로드: fade, 이후: fly
  function transitionIn(node: Element) {
    const el = node as HTMLElement;
    el.style.zIndex = '1'; // 들어오는 페이지가 위에

    if (!hasNavigated) {
      return fade(node, { duration: 3000 });
    }
    return fly(node, { x: direction * 60, duration: 600, easing: cubicOut });
  }

  function transitionOut(node: Element) {
    const el = node as HTMLElement;
    el.style.zIndex = '0'; // 나가는 페이지가 아래에
    el.style.pointerEvents = 'none'; // 클릭 방지

    if (!hasNavigated) {
      return fade(node, { duration: 0 });
    }
    return fly(node, { x: direction * -60, duration: 550, easing: cubicOut });
  }
</script>

{#key $page.url.pathname}
  <div
    class="page-wrapper"
    in:transitionIn
  >
    {@render children()}
  </div>
{/key}

<style>
  :global(body) {
    overflow: hidden;
  }

  .page-wrapper {
    position: absolute;
    width: 100%;
    height: 100%;
  }
</style>
