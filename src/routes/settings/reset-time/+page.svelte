<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { i18n } from '$lib/i18n';
  import * as settingsApi from '$lib/api/settingsApi';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';
  import SettingsLayout from '../../../components/SettingsLayout.svelte';
  import SaveFooter from '../../../components/SaveFooter.svelte';

  const hours = Array.from({ length: 24 }, (_, i) => i.toString().padStart(2, '0'));
  const minutes = Array.from({ length: 12 }, (_, i) => (i * 5).toString().padStart(2, '0'));

  let hour = $state('00');
  let minute = $state('00');
  let originalHour = $state('00');
  let originalMinute = $state('00');
  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));
  let hasChanges = $derived(hour !== originalHour || minute !== originalMinute);

  onMount(async () => {
    const saved = await settingsApi.getSetting('reset_time');
    if (saved) {
      const [h, m] = saved.split(':');
      hour = h;
      minute = m;
      originalHour = h;
      originalMinute = m;
    }
  });

  async function saveResetTime() {
    const time = `${hour}:${minute}`;
    await settingsApi.setSetting('reset_time', time);
    goto(settingsPathWithReturnTo('/settings', returnTo));
  }
</script>

<SettingsLayout title={i18n.t('resetTimeTitle')} onBack={() => goto(settingsPathWithReturnTo('/settings', returnTo))}>
  <p class="description">{i18n.t('resetTimeDescription')}</p>

  <div class="time-picker-container">
    <select class="time-select" bind:value={hour}>
      {#each hours as h}
        <option value={h}>{h}</option>
      {/each}
    </select>
    <span class="time-separator">:</span>
    <select class="time-select" bind:value={minute}>
      {#each minutes as m}
        <option value={m}>{m}</option>
      {/each}
    </select>
  </div>

  {#snippet footer()}
    <SaveFooter onSave={saveResetTime} disabled={!hasChanges} />
  {/snippet}
</SettingsLayout>

<style>
  .description {
    font-size: 14px;
    color: var(--color-ink-muted);
    margin-bottom: 24px;
    padding: 0 4px;
  }

  .time-picker-container {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 8px;
    padding: 24px 16px;
    background: var(--color-canvas);
    border-radius: 12px;
  }

  .time-select {
    font-size: 18px;
    font-weight: 500;
    color: var(--color-ink);
    background: var(--color-paper);
    border: 1px solid var(--color-stroke);
    border-radius: 8px;
    padding: 8px 12px;
    outline: none;
    cursor: pointer;
    appearance: none;
    text-align: center;
    text-align-last: center;
    min-width: 64px;
  }

  .time-select:focus {
    border-color: var(--color-accent-sky-strong);
  }

  .time-separator {
    font-size: 20px;
    font-weight: 600;
    color: var(--color-ink);
  }
</style>
