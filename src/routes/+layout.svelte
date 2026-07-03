<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { fly, fade } from 'svelte/transition';
  import { beforeNavigate, goto } from '$app/navigation';
  import { cubicOut } from 'svelte/easing';
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';
  import { checklistStore } from '$lib/checklist/checklistStore.svelte';

  let { children }: { children: Snippet } = $props();

  let direction = $state(1); // 1: 오른쪽에서, -1: 왼쪽에서
  let hasNavigated = $state(false); // 네비게이션 발생 여부

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

      await checklistStore.toggleItemFromWidget(itemId);
      await goto('/');
      return true;
    }

    if (parsedUrl.pathname === '/category') {
      const categoryIdParam = parsedUrl.searchParams.get('categoryId') ?? parsedUrl.searchParams.get('id');
      if (!categoryIdParam) {
        await goto('/');
        return true;
      }

      const categoryId = Number(categoryIdParam);
      if (!Number.isInteger(categoryId) || categoryId <= 0) {
        console.error('Invalid widget category id:', categoryIdParam);
        return true;
      }

      await checklistStore.load();
      const categoryExists = checklistStore.categories.some(category => category.id === categoryId);

      if (categoryExists) {
        await checklistStore.selectCategory(categoryId);
      } else {
        console.error('Widget category id does not exist:', categoryId);
      }

      await goto('/');
      return true;
    }

    return false;
  }

  async function handleIncomingDeepLinks(urls: string[]): Promise<void> {
    for (const url of urls) {
      try {
        const parsedUrl = new URL(url);

        if (await handleWidgetDeepLink(parsedUrl)) {
          continue;
        }
      } catch (e) {
        console.error('Failed to parse deep link URL:', e);
      }
    }
  }

  onMount(async () => {
    // Apply widget-only check actions queued in app group storage to the checklist.
    await checklistStore.processWidgetActions();

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
      return fade(node, { duration: 120 });
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
