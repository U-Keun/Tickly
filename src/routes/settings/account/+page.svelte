<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';

  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';

  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));

  onMount(() => {
    void goto(settingsPathWithReturnTo('/settings', returnTo), { replaceState: true });
  });
</script>
