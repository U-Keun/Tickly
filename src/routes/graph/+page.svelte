<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { i18n } from '$lib/i18n';
  import SettingsLayout from '../../components/SettingsLayout.svelte';
  import GraphCanvas from '../../components/GraphCanvas.svelte';
  import * as graphApi from '$lib/api/graphApi';
  import * as todoApi from '$lib/api/todoApi';
  import type { GraphData } from '$lib/api/graphApi';
  import { appStore } from '$lib/stores';

  let graphData = $state<GraphData | null>(null);
  let loading = $state(true);

  onMount(async () => {
    try {
      graphData = await graphApi.getGraphData();
    } catch (error) {
      console.error('Failed to load graph data:', error);
    } finally {
      loading = false;
    }
  });

  let isEmpty = $derived(!loading && graphData !== null && graphData.nodes.length === 0);
  let hasData = $derived(!loading && graphData !== null && graphData.nodes.length > 0);

  function handleCategoryTap(categoryId: number) {
    appStore.selectCategory(categoryId);
    goto('/v1');
  }

  async function handleItemToggle(itemId: number, _done: boolean) {
    await todoApi.toggleItem(itemId);
  }
</script>

<SettingsLayout title={i18n.t('graphView')} onBack={() => goto('/v1')} contentClass="graph-content-override">
  {#if loading}
    <div class="empty-state">
      <p>{i18n.t('loading')}</p>
    </div>
  {:else if isEmpty}
    <div class="empty-state">
      <p>{i18n.t('graphEmpty')}</p>
      <p class="hint">{i18n.t('graphEmptyHint')}</p>
    </div>
  {:else if hasData && graphData}
    <div class="graph-wrapper">
      <GraphCanvas
        data={graphData}
        onCategoryTap={handleCategoryTap}
        onItemToggle={handleItemToggle}
      />
    </div>
  {/if}
</SettingsLayout>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--color-ink-muted);
    font-size: 15px;
    text-align: center;
    padding: 32px 16px;
  }

  .empty-state .hint {
    font-size: 13px;
    margin-top: 6px;
  }

  .graph-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
  }

  :global(.graph-content-override) {
    padding: 0 !important;
    position: relative;
    overflow: hidden !important;
  }
</style>
