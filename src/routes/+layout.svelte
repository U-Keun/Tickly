<script lang="ts">
  import '../app.css';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';
  import { checklistStore } from '$lib/checklist/checklistStore.svelte';

  let { children }: { children: Snippet } = $props();

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

</script>

<div class="page-wrapper">
  {@render children()}
</div>

<style>
  :global(body) {
    overflow: hidden;
  }

  .page-wrapper {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    background-color: var(--color-canvas);
  }
</style>
