<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { Clock3 } from '@lucide/svelte';

  import { i18n } from '$lib/i18n';
  import * as settingsApi from '$lib/api/settingsApi';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';
  import SettingsActionFooter from '../../../components/settings/SettingsActionFooter.svelte';
  import SettingsGroup from '../../../components/settings/SettingsGroup.svelte';
  import SettingsShell from '../../../components/settings/SettingsShell.svelte';

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
    await goto(settingsPathWithReturnTo('/settings', returnTo));
  }
</script>

<SettingsShell
  title={i18n.t('resetTimeTitle')}
  onBack={() => void goto(settingsPathWithReturnTo('/settings', returnTo))}
>
  <SettingsGroup title={i18n.t('resetTimeChange')} description={i18n.t('resetTimeDescription')}>
    <div class="px-4 py-4">
      <div
        class="flex min-h-[96px] items-center gap-4 rounded-[6px_20px_6px_20px] border border-stroke bg-canvas px-4 py-4"
      >
        <span
          class="flex h-11 w-11 shrink-0 items-center justify-center rounded-[5px_14px_5px_14px] bg-accent-sky text-ink"
          aria-hidden="true"
        >
          <Clock3 size={22} strokeWidth={2.2} />
        </span>

        <div class="flex min-w-0 flex-1 items-center justify-end gap-2">
          <label class="sr-only" for="reset-hour">{i18n.t('resetTimeHourLabel')}</label>
          <select
            id="reset-hour"
            class="h-12 min-w-[74px] rounded-[6px_16px_6px_16px] border-2 border-ink bg-paper px-3 text-center text-[18px] font-semibold leading-6 text-ink outline-none transition-colors focus:bg-white"
            bind:value={hour}
          >
            {#each hours as h}
              <option value={h}>{h}</option>
            {/each}
          </select>

          <span class="text-[22px] font-semibold leading-none text-ink" aria-hidden="true">:</span>

          <label class="sr-only" for="reset-minute">{i18n.t('resetTimeMinuteLabel')}</label>
          <select
            id="reset-minute"
            class="h-12 min-w-[74px] rounded-[6px_16px_6px_16px] border-2 border-ink bg-paper px-3 text-center text-[18px] font-semibold leading-6 text-ink outline-none transition-colors focus:bg-white"
            bind:value={minute}
          >
            {#each minutes as m}
              <option value={m}>{m}</option>
            {/each}
          </select>
        </div>
      </div>

      <p class="mt-3 px-1 text-[13px] font-medium leading-5 text-ink-muted">
        {i18n.t('resetTimeRepeatHint')}
      </p>
    </div>
  </SettingsGroup>

  {#snippet footer()}
    <SettingsActionFooter onSave={saveResetTime} disabled={!hasChanges} />
  {/snippet}
</SettingsShell>
