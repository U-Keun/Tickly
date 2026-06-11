<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import V2SettingsGroup from '../../components/settings/V2SettingsGroup.svelte';
  import V2SettingsRow from '../../components/settings/V2SettingsRow.svelte';
  import V2SettingsShell from '../../components/settings/V2SettingsShell.svelte';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';

  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));

  function goToSetting(path: string): void {
    void goto(settingsPathWithReturnTo(path, returnTo));
  }
</script>

<V2SettingsShell title={i18n.t('settingsTitle')} onBack={() => void goto(returnTo)}>
  <div class="flex flex-col gap-5">
    <V2SettingsGroup title={i18n.t('settingsGroupAppearance')}>
      <V2SettingsRow
        icon="palette"
        tone="sky"
        label={i18n.t('themeChange')}
        onSelect={() => goToSetting('/settings/theme')}
      />
      <V2SettingsRow
        icon="type"
        tone="peach"
        label={i18n.t('fontChange')}
        onSelect={() => goToSetting('/settings/font')}
      />
      <V2SettingsRow
        icon="languages"
        tone="mint"
        label={i18n.t('languageChange')}
        onSelect={() => goToSetting('/settings/language')}
      />
    </V2SettingsGroup>

    <V2SettingsGroup title={i18n.t('settingsGroupDaily')}>
      <V2SettingsRow
        icon="clock"
        tone="peach"
        label={i18n.t('resetTimeChange')}
        onSelect={() => goToSetting('/settings/reset-time')}
      />
    </V2SettingsGroup>

    <V2SettingsGroup title={i18n.t('settingsGroupLegacy')}>
      <V2SettingsRow
        icon="tags"
        tone="mint"
        label={i18n.t('tagManage')}
        badge={i18n.t('settingsLegacyBadge')}
        onSelect={() => goToSetting('/settings/tags')}
      />
      <V2SettingsRow
        icon="cloud"
        tone="sky"
        label={i18n.t('cloudSync')}
        badge={i18n.t('settingsLegacyBadge')}
        onSelect={() => goToSetting('/settings/account')}
      />
    </V2SettingsGroup>
  </div>
</V2SettingsShell>
